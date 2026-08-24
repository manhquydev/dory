import { test } from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { mkdtemp, readFile, writeFile, chmod } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { main } from "../src/cli.js";
import { journalPath, parseJournalLines } from "../src/journal.js";
import { HOST, startServer } from "../src/serve.js";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const bin = join(root, "bin", "dory.js");

function runCli(args, { env = process.env, timeoutMs = 5_000 } = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, [bin, ...args], {
      cwd: root,
      env,
      stdio: ["ignore", "pipe", "pipe"],
    });
    let stdout = "";
    let stderr = "";
    const timer = setTimeout(() => {
      child.kill("SIGKILL");
      reject(new Error(`cli timed out: ${args.join(" ")}`));
    }, timeoutMs);
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", (err) => {
      clearTimeout(timer);
      reject(err);
    });
    child.on("close", (code, signal) => {
      clearTimeout(timer);
      resolve({ code, signal, stdout, stderr });
    });
  });
}

async function tempWorkspace() {
  return mkdtemp(join(tmpdir(), "dory-phase1-"));
}

function escapeHtml(text) {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

test("parseJournalLines skips empty and marks broken", () => {
  const rows = parseJournalLines(
    Buffer.from('{"type":"session/open"}\n\nnot-json\n{"type":"journal/note","text":"x"}\n'),
  );
  assert.equal(rows.length, 3);
  assert.equal(rows[0].type, "session/open");
  assert.equal(rows[1].type, "_broken");
  assert.equal(rows[2].type, "journal/note");
  assert.equal(parseJournalLines(Buffer.alloc(0)).length, 0);
});

test("missing workspace fails", async () => {
  const none = await runCli(["serve"]);
  assert.notEqual(none.code, 0);
  assert.match(none.stderr, /missing --workspace/);

  const emptyFlag = await runCli(["serve", "--workspace"]);
  assert.notEqual(emptyFlag.code, 0);

  const code = await main(["serve"]);
  assert.equal(code, 2);

  const relative = await main(["serve", "--workspace", "relative/path"]);
  assert.equal(relative, 2);
});

test("loopback bind", async () => {
  const workspace = await tempWorkspace();
  await assert.rejects(
    () => startServer({ workspace, host: "0.0.0.0", port: 0 }),
    /non-loopback host/,
  );
  await assert.rejects(
    () => startServer({ workspace, host: "127.0.0.2", port: 0 }),
    /non-loopback host/,
  );

  const hostCode = await main([
    "serve",
    "--workspace",
    workspace,
    "--host",
    "0.0.0.0",
  ]);
  assert.equal(hostCode, 2);

  const started = await startServer({ workspace, host: HOST, port: 0 });
  try {
    assert.equal(started.host, "127.0.0.1");
    assert.ok(started.port > 0);
    const addr = started.server.address();
    assert.equal(addr.address, "127.0.0.1");
    assert.equal(addr.port, started.port);
  } finally {
    await started.close();
  }
});

test("journal append reconstructs on GET", async () => {
  const workspace = await tempWorkspace();
  const note = 'hello <b>journal</b> & "bytes"';
  const first = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const post = await fetch(`${first.url}/note`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ text: note }),
    });
    assert.equal(post.status, 200);
    const posted = await post.json();
    assert.equal(posted.ok, true);
    assert.equal(posted.event.type, "journal/note");
    assert.equal(posted.event.text, note);
  } finally {
    await first.close();
  }

  const fileBytes = await readFile(journalPath(workspace));
  const fileText = fileBytes.toString("utf8");
  const lines = fileText.trimEnd().split("\n").map((line) => JSON.parse(line));
  assert.equal(lines[0].type, "session/open");
  assert.equal(lines[0].workspace, workspace);
  assert.equal(lines.at(-1).type, "journal/note");
  assert.equal(lines.at(-1).text, note);

  const second = await startServer({ workspace, host: HOST, port: 0 });
  try {
    const res = await fetch(`${second.url}/`);
    assert.equal(res.status, 200);
    assert.match(res.headers.get("content-type"), /text\/html/);
    const html = await res.text();
    assert.match(html, /Log projection/);
    assert.match(html, /Not a workplace\. Not a pane\. Not a terminal\./);
    assert.match(html, /lang="vi"/);
    assert.match(html, /Mở phiên/);
    assert.match(html, /Ghi chú/);
    assert.match(html, /id="workspace"/);
    assert.ok(html.includes(escapeHtml(workspace)));
    assert.ok(html.includes(escapeHtml(note)));
    assert.doesNotMatch(html, /PTY|w1:t1|attach|detach/i);
    assert.ok(!html.includes(fileText.trim().split("\n")[0]));
    assert.ok(!html.includes("<b>journal</b>"));
  } finally {
    await second.close();
  }

  const afterRestart = await readFile(journalPath(workspace), "utf8");
  assert.ok(afterRestart.startsWith(fileText));
  const restartLines = afterRestart
    .trimEnd()
    .split("\n")
    .map((line) => JSON.parse(line));
  assert.equal(restartLines.filter((e) => e.type === "session/open").length, 2);
  assert.equal(
    restartLines.filter((e) => e.type === "journal/note" && e.text === note)
      .length,
    1,
  );
});

test("POST /flow invoke-and-exit records result", async () => {
  const workspace = await tempWorkspace();
  const flowBin = join(workspace, "fake-flow");
  await writeFile(
    flowBin,
    `#!/usr/bin/env node
process.stdout.write("cwd=" + process.cwd() + "\\n");
process.stdout.write("arg=" + process.argv[2] + "\\n");
process.exit(0);
`,
  );
  await chmod(flowBin, 0o755);

  const started = await startServer({ workspace, host: HOST, port: 0 });
  const prev = process.env.FLOW_BIN;
  process.env.FLOW_BIN = flowBin;
  try {
    const res = await fetch(`${started.url}/flow`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ confirm: true }),
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.ok, true);
    assert.equal(body.event.type, "flow/result");
    assert.equal(body.event.code, 0);
    assert.match(body.event.stdout, new RegExp(`cwd=${workspace}`));
    assert.match(body.event.stdout, /arg=status/);
  } finally {
    if (prev === undefined) delete process.env.FLOW_BIN;
    else process.env.FLOW_BIN = prev;
    await started.close();
  }

  const lines = (await readFile(journalPath(workspace), "utf8"))
    .trimEnd()
    .split("\n")
    .map((line) => JSON.parse(line));
  const types = lines.map((e) => e.type);
  assert.ok(types.includes("flow/invoke"));
  assert.ok(types.includes("flow/result"));
  const invoke = lines.find((e) => e.type === "flow/invoke");
  assert.equal(invoke.cwd, workspace);
  assert.deepEqual(invoke.args, ["status"]);
});
