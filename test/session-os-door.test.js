import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, readFile, writeFile, chmod, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { journalPath } from "../src/journal.js";
import { HOST, startServer } from "../src/serve.js";

async function tempWorkspace() {
  return mkdtemp(join(tmpdir(), "dory-session-door-"));
}

async function journalTypes(workspace) {
  try {
    const text = await readFile(journalPath(workspace), "utf8");
    return text
      .trimEnd()
      .split("\n")
      .filter(Boolean)
      .map((line) => JSON.parse(line).type);
  } catch (err) {
    if (err && err.code === "ENOENT") return [];
    throw err;
  }
}

test("POST /goal appends session/goal and paints Mục tiêu", async () => {
  const workspace = await tempWorkspace();
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const empty = await fetch(`${started.url}/goal`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ text: "   " }),
    });
    assert.equal(empty.status, 400);
    assert.ok(!(await journalTypes(workspace)).includes("session/goal"));

    const missing = await fetch(`${started.url}/goal`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({}),
    });
    assert.equal(missing.status, 400);

    const goal = "viết README <b>x</b>";
    const post = await fetch(`${started.url}/goal`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ text: `  ${goal}  ` }),
    });
    assert.equal(post.status, 200);
    const posted = await post.json();
    assert.equal(posted.ok, true);
    assert.equal(posted.event.type, "session/goal");
    assert.equal(posted.event.text, goal);

    const note = await fetch(`${started.url}/note`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ text: "ghi chú còn sống" }),
    });
    assert.equal(note.status, 200);
    assert.equal((await note.json()).event.type, "journal/note");

    const html = await (await fetch(`${started.url}/`)).text();
    assert.match(html, /Mục tiêu/);
    assert.match(html, /Ghi chú/);
    assert.ok(html.includes("viết README &lt;b&gt;x&lt;/b&gt;"));
    assert.ok(!html.includes("<b>x</b>"));
    assert.ok(html.includes(workspace));
    assert.match(html, /serve --workspace/);
    assert.doesNotMatch(html, /0\.0\.0\.0/);
    assert.doesNotMatch(html, /PTY|w1:t1|attach|detach/i);
  } finally {
    await started.close();
  }
});

test("POST /flow is fail-closed without confirm === true", async () => {
  const workspace = await tempWorkspace();
  const flowBin = join(workspace, "fake-flow");
  const stamp = join(workspace, "flow-ran");
  await writeFile(
    flowBin,
    `#!/usr/bin/env node
import { writeFileSync } from "node:fs";
writeFileSync(${JSON.stringify(stamp)}, "ran");
process.exit(0);
`,
  );
  await chmod(flowBin, 0o755);

  const started = await startServer({ workspace, host: HOST, port: 0 });
  const prev = process.env.FLOW_BIN;
  process.env.FLOW_BIN = flowBin;
  try {
    const denied = [
      await fetch(`${started.url}/flow`, { method: "POST" }),
      await fetch(`${started.url}/flow`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({}),
      }),
      await fetch(`${started.url}/flow`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ confirm: "true" }),
      }),
      await fetch(`${started.url}/flow`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ confirm: 1 }),
      }),
    ];
    for (const res of denied) {
      assert.equal(res.status, 403);
      assert.equal((await res.json()).ok, false);
    }
    assert.ok(!(await journalTypes(workspace)).includes("flow/invoke"));
    await assert.rejects(() => stat(stamp), { code: "ENOENT" });

    const html = await (await fetch(`${started.url}/`)).text();
    assert.ok(html.includes(basename(flowBin)));
    assert.match(html, /status/);
    assert.ok(html.includes(workspace));

    const ok = await fetch(`${started.url}/flow`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ confirm: true }),
    });
    assert.equal(ok.status, 200);
    assert.equal((await ok.json()).event.type, "flow/result");
    assert.equal(await readFile(stamp, "utf8"), "ran");
    const types = await journalTypes(workspace);
    assert.ok(types.includes("flow/invoke"));
    assert.ok(types.includes("flow/result"));
  } finally {
    if (prev === undefined) delete process.env.FLOW_BIN;
    else process.env.FLOW_BIN = prev;
    await started.close();
  }
});
