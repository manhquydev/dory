import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, readFile, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { HOST, startServer } from "../src/serve.js";
import { createIds } from "../src/workplace/ids.js";
import { isPidAlive } from "../src/workplace/pty-hold.js";

async function tempWorkspace() {
  return mkdtemp(join(tmpdir(), "dory-phase2-"));
}

async function waitFor(fn, { timeoutMs = 8_000, intervalMs = 50 } = {}) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if (await fn()) return true;
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  throw new Error("waitFor timed out");
}

test("opaque IDs never reuse after retire", () => {
  const ids = createIds();
  const w1 = ids.window();
  const t1 = ids.tab(w1);
  const p1 = ids.pane(w1);
  ids.retire(w1);
  ids.retire(t1);
  ids.retire(p1);
  const w2 = ids.window();
  assert.equal(w1, "w1");
  assert.equal(w2, "w2");
  assert.notEqual(w2, w1);
});

test("Phase 1 journal page still hides workplace IDs", async () => {
  const workspace = await tempWorkspace();
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    await fetch(`${started.url}/workplace/open`, { method: "POST" });
    const html = await (await fetch(`${started.url}/`)).text();
    assert.match(html, /Not a workplace\. Not a pane\. Not a terminal\./);
    assert.doesNotMatch(html, /w1:t1|w1:p1/);
  } finally {
    await started.close();
  }
});

test("one window, one tab, one live PTY; detach does not kill", async () => {
  const workspace = await tempWorkspace();
  const marker = join(workspace, "phase2.marker");
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const opened = await (
      await fetch(`${started.url}/workplace/open`, { method: "POST" })
    ).json();
    assert.equal(opened.ok, true);
    assert.equal(opened.workplace.windows.length, 1);
    assert.equal(opened.workplace.tabs.length, 1);
    assert.equal(opened.workplace.panes.length, 1);
    const pane = opened.workplace.panes[0];
    assert.equal(pane.occupant, null);
    assert.equal(pane.alive, true);
    assert.ok(isPidAlive(pane.pid));

    const pidBefore = pane.pid;
    await fetch(`${started.url}/workplace/panes/${pane.pane_id}/input`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ data: `echo PHASE2 > "${marker}"\n` }),
    });

    await waitFor(async () => {
      try {
        await stat(marker);
        return true;
      } catch {
        return false;
      }
    });
    assert.equal((await readFile(marker, "utf8")).trim(), "PHASE2");

    const detached = await (
      await fetch(`${started.url}/workplace/detach`, { method: "POST" })
    ).json();
    assert.equal(detached.workplace.panes[0].alive, true);
    assert.equal(detached.workplace.panes[0].pid, pidBefore);
    assert.ok(isPidAlive(pidBefore));

    const again = await (await fetch(`${started.url}/workplace`)).json();
    assert.equal(again.workplace.panes[0].alive, true);
    assert.ok(isPidAlive(pidBefore));
  } finally {
    await started.close();
  }
});

test("server stop is not a live restore", async () => {
  const workspace = await tempWorkspace();
  const started = await startServer({ workspace, host: HOST, port: 0 });
  const opened = await (
    await fetch(`${started.url}/workplace/open`, { method: "POST" })
  ).json();
  const pid = opened.workplace.panes[0].pid;
  const image = started.workplace.shutdown();
  assert.equal(image.live, false);
  assert.equal(image.panes[0].alive, false);
  assert.match(image.note, /not a live PTY/);
  await started.close();
  await waitFor(() => !isPidAlive(pid), { timeoutMs: 3_000 });

  const reopen = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const fresh = await (await fetch(`${reopen.url}/workplace`)).json();
    assert.equal(fresh.workplace.open, false);
    const second = await (
      await fetch(`${reopen.url}/workplace/open`, { method: "POST" })
    ).json();
    assert.notEqual(second.workplace.panes[0].pid, pid);
    assert.equal(second.workplace.panes[0].alive, true);
  } finally {
    await reopen.close();
  }
});
