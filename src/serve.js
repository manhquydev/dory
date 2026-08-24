import { createServer } from "node:http";
import { statSync } from "node:fs";
import { isAbsolute } from "node:path";
import { openJournal } from "./journal.js";
import { invokeFlow, resolveFlowBin } from "./flow.js";
import { renderJournalPage } from "./page.js";
import { createWorkplace } from "./workplace/runtime.js";
import { handleWorkplace } from "./workplace/http.js";

export const HOST = "127.0.0.1";
export const PORT = 7380;
const BODY_LIMIT = 1_000_000;

export function assertWorkspace(workspace) {
  if (workspace == null || workspace === "") {
    const err = new Error("dory: missing --workspace <abs-dir>");
    err.exitCode = 2;
    throw err;
  }
  if (!isAbsolute(workspace)) {
    const err = new Error("dory: --workspace must be an absolute directory");
    err.exitCode = 2;
    throw err;
  }
  let st;
  try {
    st = statSync(workspace);
  } catch {
    const err = new Error(`dory: workspace not a directory: ${workspace}`);
    err.exitCode = 2;
    throw err;
  }
  if (!st.isDirectory()) {
    const err = new Error(`dory: workspace not a directory: ${workspace}`);
    err.exitCode = 2;
    throw err;
  }
}

export function assertLoopbackHost(host) {
  if (host !== HOST) {
    const err = new Error(`dory: refusing non-loopback host ${host}`);
    err.exitCode = 2;
    throw err;
  }
}

function previewFlow(workspace) {
  try {
    return {
      ok: true,
      bin: resolveFlowBin(process.env),
      args: ["status"],
      cwd: workspace,
    };
  } catch (err) {
    return {
      ok: false,
      error: err.message,
      args: ["status"],
      cwd: workspace,
    };
  }
}

async function readRequestObject(req) {
  const chunks = [];
  let n = 0;
  for await (const chunk of req) {
    n += chunk.length;
    if (n > BODY_LIMIT) {
      const err = new Error("payload too large");
      err.statusCode = 413;
      throw err;
    }
    chunks.push(chunk);
  }
  const raw = Buffer.concat(chunks).toString("utf8");
  const ctype = String(req.headers["content-type"] || "");
  if (ctype.includes("application/x-www-form-urlencoded")) {
    const obj = {};
    for (const [key, value] of new URLSearchParams(raw)) {
      obj[key] = value;
    }
    return obj;
  }
  if (n === 0) return {};
  try {
    return JSON.parse(raw);
  } catch {
    const err = new Error("invalid JSON");
    err.statusCode = 400;
    throw err;
  }
}

function send(res, status, body, headers = {}) {
  const buf = Buffer.from(body);
  res.writeHead(status, {
    "content-length": buf.length,
    ...headers,
  });
  res.end(buf);
}

function sendJson(res, status, obj) {
  send(res, status, `${JSON.stringify(obj)}\n`, {
    "content-type": "application/json; charset=utf-8",
  });
}

async function readJsonBody(req) {
  const chunks = [];
  let n = 0;
  for await (const chunk of req) {
    n += chunk.length;
    if (n > BODY_LIMIT) {
      const err = new Error("payload too large");
      err.statusCode = 413;
      throw err;
    }
    chunks.push(chunk);
  }
  if (n === 0) return {};
  try {
    return JSON.parse(Buffer.concat(chunks).toString("utf8"));
  } catch {
    const err = new Error("invalid JSON");
    err.statusCode = 400;
    throw err;
  }
}

function createHandler(workspace, journal, workplace) {
  return async function handler(req, res) {
    try {
      const url = new URL(req.url || "/", `http://${HOST}:${PORT}`);
      if (await handleWorkplace(req, res, url, workplace, readJsonBody)) {
        return;
      }
      if (req.method === "GET" && url.pathname === "/") {
        const bytes = await journal.readBytes();
        send(
          res,
          200,
          renderJournalPage(bytes, {
            workspace,
            flowPreview: previewFlow(workspace),
          }),
          {
            "content-type": "text/html; charset=utf-8",
          },
        );
        return;
      }
      if (req.method === "POST" && url.pathname === "/goal") {
        const body = await readRequestObject(req);
        if (typeof body.text !== "string" || body.text.trim() === "") {
          sendJson(res, 400, { ok: false, error: "text must be a string" });
          return;
        }
        const event = await journal.append("session/goal", {
          text: body.text.trim(),
        });
        const ctype = String(req.headers["content-type"] || "");
        if (ctype.includes("application/x-www-form-urlencoded")) {
          res.writeHead(303, { location: "/", "content-length": 0 });
          res.end();
          return;
        }
        sendJson(res, 200, { ok: true, event });
        return;
      }
      if (req.method === "POST" && url.pathname === "/note") {
        const body = await readJsonBody(req);
        if (typeof body.text !== "string") {
          sendJson(res, 400, { ok: false, error: "text must be a string" });
          return;
        }
        const event = await journal.append("journal/note", { text: body.text });
        sendJson(res, 200, { ok: true, event });
        return;
      }
      if (req.method === "POST" && url.pathname === "/flow") {
        const body = await readJsonBody(req);
        if (body.confirm !== true) {
          sendJson(res, 403, { ok: false, error: "confirm required" });
          return;
        }
        let bin;
        try {
          bin = resolveFlowBin(process.env);
        } catch (err) {
          sendJson(res, 403, { ok: false, error: err.message });
          return;
        }
        let flowArgs = ["status"];
        if (Array.isArray(body.args) && body.args.length > 0) {
          flowArgs = body.args.map(String);
        }
        await journal.append("flow/invoke", {
          bin,
          args: flowArgs,
          cwd: workspace,
        });
        const result = await invokeFlow({
          workspace,
          env: process.env,
          args: flowArgs,
        });
        const event = await journal.append("flow/result", {
          bin: result.bin,
          args: result.args,
          cwd: result.cwd,
          code: result.code,
          signal: result.signal,
          stdout: result.stdout,
          stderr: result.stderr,
          error: result.error,
        });
        sendJson(res, 200, { ok: true, event });
        return;
      }
      send(res, 404, "not found\n", { "content-type": "text/plain; charset=utf-8" });
    } catch (err) {
      const status = err.statusCode || 500;
      sendJson(res, status, { ok: false, error: err.message });
    }
  };
}

export async function startServer({
  workspace,
  host = HOST,
  port = PORT,
} = {}) {
  assertWorkspace(workspace);
  assertLoopbackHost(host);
  const journal = openJournal(workspace);
  const workplace = createWorkplace({ workspace });
  const server = createServer(createHandler(workspace, journal, workplace));

  return new Promise((resolve, reject) => {
    const onError = (err) => {
      reject(err);
    };
    server.once("error", onError);
    server.listen(port, host, async () => {
      server.off("error", onError);
      try {
        await journal.append("session/open", { workspace, host, port: server.address().port });
      } catch (err) {
        server.close();
        reject(err);
        return;
      }
      const addr = server.address();
      resolve({
        server,
        journal,
        workplace,
        host: addr.address,
        port: addr.port,
        url: `http://${addr.address}:${addr.port}`,
        close() {
          workplace.shutdown();
          return new Promise((res, rej) => {
            server.close((e) => (e ? rej(e) : res()));
          });
        },
      });
    });
  });
}
