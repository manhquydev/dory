import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { HOST, startServer } from "../src/serve.js";

async function tempWorkspace() {
  return mkdtemp(join(tmpdir(), "dory-phase4-"));
}

async function attachAll(url) {
  const opened = await (await fetch(`${url}/workplace/open`, { method: "POST" })).json();
  const a = opened.workplace.panes[0].pane_id;
  await fetch(`${url}/workplace/panes/${a}/session`, { method: "POST" });
  const split = await (await fetch(`${url}/workplace/split`, { method: "POST" })).json();
  const b = split.workplace.panes[1].pane_id;
  await fetch(`${url}/workplace/panes/${b}/session`, { method: "POST" });
  return { a, b };
}

test("CLI read does not mark seen; focus turns done into idle", async () => {
  const workspace = await tempWorkspace();
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const { a } = await attachAll(started.url);
    await fetch(`${started.url}/workplace/panes/${a}/session/note`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ text: "unseen work" }),
    });
    const read = await (await fetch(`${started.url}/workplace/panes/${a}`)).json();
    assert.equal(read.pane.occupant.state, "done");
    assert.equal(read.pane.occupant.seen, false);
    const again = await (await fetch(`${started.url}/workplace`)).json();
    assert.equal(again.workplace.panes[0].occupant.state, "done");

    const focused = await (
      await fetch(`${started.url}/workplace/focus`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ pane_id: a }),
      })
    ).json();
    assert.equal(focused.workplace.panes[0].occupant.state, "idle");
    assert.equal(focused.workplace.panes[0].occupant.seen, true);
  } finally {
    await started.close();
  }
});

test("coordinate from outside is refused; inside drives the other session", async () => {
  const workspace = await tempWorkspace();
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const { a, b } = await attachAll(started.url);
    const denied = await fetch(`${started.url}/workplace/coordinate`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ from: a, to: b, text: "echo outside" }),
    });
    assert.equal(denied.status, 403);
    const body = await denied.json();
    assert.match(body.error, /inside the workplace/);

    const ok = await fetch(`${started.url}/workplace/coordinate`, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        "x-dory-inside": "1",
      },
      body: JSON.stringify({ from: a, to: b, text: "echo INSIDE" }),
    });
    assert.equal(ok.status, 200);
    const result = await ok.json();
    assert.equal(result.ok, true);
    assert.equal(result.event.type, "coordinate/in");
    assert.equal(result.event.from_pane, a);
    const to = result.workplace.panes.find((p) => p.pane_id === b);
    assert.equal(to.occupant.state, "done");
  } finally {
    await started.close();
  }
});
