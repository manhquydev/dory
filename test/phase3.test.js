import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { HOST, startServer } from "../src/serve.js";
import { journalPath, sessionJournalPath } from "../src/journal.js";

async function tempWorkspace() {
  return mkdtemp(join(tmpdir(), "dory-phase3-"));
}

test("pane hosts a session whose journal is not the pane", async () => {
  const workspace = await tempWorkspace();
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const opened = await (
      await fetch(`${started.url}/workplace/open`, { method: "POST" })
    ).json();
    const paneId = opened.workplace.panes[0].pane_id;
    const attached = await (
      await fetch(`${started.url}/workplace/panes/${paneId}/session`, {
        method: "POST",
      })
    ).json();
    assert.equal(attached.ok, true);
    assert.notEqual(attached.session_id, paneId);
    assert.match(attached.session_id, /^s\d+$/);
    assert.equal(attached.pane.occupant.kind, "session");
    assert.equal(attached.pane.occupant.session_id, attached.session_id);

    const note = await (
      await fetch(`${started.url}/workplace/panes/${paneId}/session/note`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ text: "child-only" }),
      })
    ).json();
    assert.equal(note.event.text, "child-only");

    const parent = await readFile(journalPath(workspace), "utf8");
    assert.doesNotMatch(parent, /child-only/);
    const child = await readFile(
      sessionJournalPath(workspace, attached.session_id),
      "utf8",
    );
    assert.match(child, /child-only/);
    assert.match(child, /session\/open/);
  } finally {
    await started.close();
  }
});
