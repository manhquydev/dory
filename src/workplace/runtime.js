import { createIds } from "./ids.js";
import { holdPty, isPidAlive } from "./pty-hold.js";
import { openSessionJournal } from "../journal.js";

const IDLE_LIKE = new Set(["idle", "done"]);

function displayState(occupant) {
  if (!occupant) return null;
  if (occupant.process === "idle") {
    return occupant.seen ? "idle" : "done";
  }
  return occupant.process;
}

export function createWorkplace({ workspace }) {
  const ids = createIds();
  let windowId = null;
  let tabId = null;
  const panes = new Map();
  let stopped = false;

  function requireOpen() {
    if (stopped || !windowId || panes.size === 0) {
      const err = new Error("dory: workplace is not open");
      err.statusCode = 409;
      throw err;
    }
  }

  function getPane(id) {
    requireOpen();
    const pane = panes.get(id);
    if (!pane) {
      const err = new Error(`dory: unknown pane ${id}`);
      err.statusCode = 404;
      throw err;
    }
    return pane;
  }

  function occupantView(occupant) {
    if (!occupant) return null;
    return {
      kind: "session",
      session_id: occupant.session_id,
      state: displayState(occupant),
      seen: occupant.seen,
    };
  }

  function paneView(pane) {
    return {
      pane_id: pane.pane_id,
      tab_id: pane.tab_id,
      window_id: pane.window_id,
      pid: pane.pty?.pid ?? null,
      alive: Boolean(pane.pty && pane.pty.alive()),
      occupant: occupantView(pane.occupant),
    };
  }

  function snapshot() {
    if (!windowId) {
      return { open: false, windows: [], tabs: [], panes: [] };
    }
    return {
      open: !stopped,
      windows: [{ window_id: windowId }],
      tabs: [{ tab_id: tabId, window_id: windowId }],
      panes: [...panes.values()].map(paneView),
    };
  }

  function addPane() {
    const pane_id = ids.pane(windowId);
    const pane = {
      pane_id,
      tab_id: tabId,
      window_id: windowId,
      pty: holdPty({ cwd: workspace }),
      occupant: null,
    };
    panes.set(pane_id, pane);
    return pane;
  }

  return {
    open() {
      if (stopped) {
        const err = new Error(
          "dory: workplace stopped; restore image is not a live PTY",
        );
        err.statusCode = 409;
        err.code = "DORY_WORKPLACE_STOPPED";
        throw err;
      }
      if (windowId) return snapshot();
      windowId = ids.window();
      tabId = ids.tab(windowId);
      addPane();
      return snapshot();
    },
    get() {
      return snapshot();
    },
    detach() {
      requireOpen();
      return snapshot();
    },
    split() {
      requireOpen();
      addPane();
      return snapshot();
    },
    async attachSession(paneId) {
      const pane = getPane(paneId);
      if (pane.occupant) {
        const err = new Error("dory: pane already hosts a session");
        err.statusCode = 409;
        throw err;
      }
      const session_id = ids.session();
      if (session_id === paneId) {
        const err = new Error("dory: session_id must not equal pane_id");
        err.statusCode = 500;
        throw err;
      }
      const journal = openSessionJournal(workspace, session_id);
      await journal.append("session/open", {
        session_id,
        host_pane: paneId,
      });
      pane.occupant = {
        session_id,
        journal,
        process: "idle",
        seen: false,
      };
      return {
        ok: true,
        pane: paneView(pane),
        session_id,
        journal: journal.file,
      };
    },
    async sessionNote(paneId, text) {
      const pane = getPane(paneId);
      if (!pane.occupant) {
        const err = new Error("dory: pane has no session");
        err.statusCode = 409;
        throw err;
      }
      if (typeof text !== "string" || !text) {
        const err = new Error("dory: text required");
        err.statusCode = 400;
        throw err;
      }
      pane.occupant.process = "working";
      const event = await pane.occupant.journal.append("journal/note", { text });
      pane.occupant.process = "idle";
      pane.occupant.seen = false;
      return { ok: true, event, occupant: occupantView(pane.occupant) };
    },
    focus(paneId) {
      const pane = getPane(paneId);
      if (pane.occupant) pane.occupant.seen = true;
      return snapshot();
    },
    async coordinate({ from, to, text, inside }) {
      if (!inside) {
        const err = new Error(
          "dory: coordination only valid from inside the workplace",
        );
        err.statusCode = 403;
        err.code = "DORY_OUTSIDE";
        throw err;
      }
      const src = getPane(from);
      const dst = getPane(to);
      if (!src.occupant || !dst.occupant) {
        const err = new Error("dory: both panes must host a session");
        err.statusCode = 409;
        throw err;
      }
      if (typeof text !== "string" || !text) {
        const err = new Error("dory: text required");
        err.statusCode = 400;
        throw err;
      }
      src.occupant.process = "working";
      dst.occupant.process = "working";
      dst.occupant.seen = false;
      const event = await dst.occupant.journal.append("coordinate/in", {
        from_pane: from,
        from_session: src.occupant.session_id,
        text,
      });
      dst.pty.write(text.endsWith("\n") ? text : `${text}\n`);
      src.occupant.process = "idle";
      src.occupant.seen = true;
      dst.occupant.process = "idle";
      return {
        ok: true,
        event,
        workplace: snapshot(),
      };
    },
    block(paneId) {
      const pane = getPane(paneId);
      if (!pane.occupant) {
        const err = new Error("dory: pane has no session");
        err.statusCode = 409;
        throw err;
      }
      pane.occupant.process = "blocked";
      return paneView(pane);
    },
    input(id, data) {
      const pane = getPane(id);
      if (typeof data !== "string") {
        const err = new Error("dory: input data must be a string");
        err.statusCode = 400;
        throw err;
      }
      if (pane.occupant?.process === "blocked") {
        const err = new Error("dory: do not send into blocked");
        err.statusCode = 409;
        throw err;
      }
      pane.pty.write(data);
      if (pane.occupant) {
        pane.occupant.process = "working";
        pane.occupant.seen = false;
      }
      return { ok: true, pane_id: id };
    },
    pane(id) {
      const pane = getPane(id);
      return { ...paneView(pane), output: pane.pty.read() };
    },
    shutdown() {
      if (stopped && panes.size === 0) {
        return {
          open: false,
          live: false,
          note: "server stop; this snapshot is not a live PTY",
          windows: [],
          tabs: [],
          panes: [],
        };
      }
      for (const pane of panes.values()) {
        pane.pty?.kill();
        ids.retire(pane.pane_id);
        if (pane.occupant) ids.retire(pane.occupant.session_id);
      }
      if (windowId) ids.retire(windowId);
      if (tabId) ids.retire(tabId);
      stopped = true;
      const image = {
        open: false,
        live: false,
        note: "server stop; this snapshot is not a live PTY",
        windows: windowId ? [{ window_id: windowId }] : [],
        tabs: tabId ? [{ tab_id: tabId, window_id: windowId }] : [],
        panes: [...panes.values()].map((pane) => ({
          ...paneView(pane),
          alive: false,
        })),
      };
      panes.clear();
      return image;
    },
    ids,
    isPidAlive,
    IDLE_LIKE,
  };
}
