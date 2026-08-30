import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, mkdir, readFile, stat, writeFile, chmod, cp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { HOST, startServer } from "../src/serve.js";
import { journalPath } from "../src/journal.js";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const fixture = join(root, "eval", "phase5-project");

async function waitFor(fn, { timeoutMs = 8_000, intervalMs = 50 } = {}) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if (await fn()) return true;
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  throw new Error("waitFor timed out");
}

test("a fixture project is completed inside Dory; Flow judges", async () => {
  const workspace = await mkdtemp(join(tmpdir(), "dory-phase5-"));
  await cp(fixture, workspace, { recursive: true });
  const flowBin = join(workspace, "bin", "flow.sh");
  await chmod(flowBin, 0o755);
  await mkdir(join(workspace, "evidence"), { recursive: true });

  const prev = process.env.FLOW_BIN;
  process.env.FLOW_BIN = flowBin;
  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const fail = await (
      await fetch(`${started.url}/flow`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ confirm: true, args: ["check", "C-001"] }),
      })
    ).json();
    assert.equal(fail.event.code, 1);

    const opened = await (
      await fetch(`${started.url}/workplace/open`, { method: "POST" })
    ).json();
    const paneId = opened.workplace.panes[0].pane_id;
    await fetch(`${started.url}/workplace/panes/${paneId}/session`, {
      method: "POST",
    });
    await fetch(`${started.url}/workplace/panes/${paneId}/session/note`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ text: "write evidence/DONE from the hosted session" }),
    });
    await fetch(`${started.url}/workplace/panes/${paneId}/input`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        data: `mkdir -p evidence && echo done > evidence/DONE\n`,
      }),
    });

    const marker = join(workspace, "evidence", "DONE");
    await waitFor(async () => {
      try {
        await stat(marker);
        return true;
      } catch {
        return false;
      }
    });

    const pass = await (
      await fetch(`${started.url}/flow`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ confirm: true, args: ["check", "C-001"] }),
      })
    ).json();
    assert.equal(pass.ok, true);
    assert.equal(pass.event.type, "flow/result");
    assert.equal(pass.event.code, 0);
    assert.match(pass.event.stdout, /PASS C-001/);

    const parent = await readFile(journalPath(workspace), "utf8");
    assert.match(parent, /flow\/result/);
    assert.match(parent, /"code":0/);
  } finally {
    if (prev === undefined) delete process.env.FLOW_BIN;
    else process.env.FLOW_BIN = prev;
    await started.close();
  }
});

test("runtime source does not exec herdr or dsh", async () => {
  const files = [
    "src/cli.js",
    "src/serve.js",
    "src/page.js",
    "src/flow.js",
    "src/journal.js",
    "src/workplace/runtime.js",
    "src/workplace/http.js",
    "src/workplace/pty-hold.js",
    "src/workplace/ids.js",
    "bin/dory.js",
  ];
  for (const rel of files) {
    const text = await readFile(join(root, rel), "utf8");
    assert.doesNotMatch(text, /spawn\(\s*["']herdr["']/);
    assert.doesNotMatch(text, /spawn\(\s*["']dsh["']/);
    assert.doesNotMatch(text, /from ["']@deepseek-ai\/dsh["']/);
  }
  await writeFile(join(tmpdir(), "dory-phase5-grep-ok"), "ok");
});
