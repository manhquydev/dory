import { appendFile, mkdir, readFile } from "node:fs/promises";
import { join } from "node:path";

export function journalPath(workspace) {
  return join(workspace, ".dory", "journal.jsonl");
}

export function parseJournalLines(bytes) {
  const text = Buffer.isBuffer(bytes) ? bytes.toString("utf8") : String(bytes ?? "");
  const rows = [];
  for (const line of text.split("\n")) {
    if (line.trim() === "") continue;
    try {
      const value = JSON.parse(line);
      if (value !== null && typeof value === "object" && !Array.isArray(value)) {
        rows.push(value);
      } else {
        rows.push({ type: "_broken", raw: line });
      }
    } catch {
      rows.push({ type: "_broken", raw: line });
    }
  }
  return rows;
}

export function sessionJournalPath(workspace, sessionId) {
  return join(workspace, ".dory", "sessions", `${sessionId}.jsonl`);
}

export function openJournal(workspace) {
  const file = journalPath(workspace);
  const dir = join(workspace, ".dory");
  let queue = Promise.resolve();

  async function readBytes() {
    try {
      return await readFile(file);
    } catch (err) {
      if (err && err.code === "ENOENT") return Buffer.alloc(0);
      throw err;
    }
  }

  function append(type, fields = {}) {
    const record = { ts: new Date().toISOString(), type, ...fields };
    const job = queue.then(async () => {
      await mkdir(dir, { recursive: true });
      await appendFile(file, `${JSON.stringify(record)}\n`, "utf8");
      return record;
    });
    queue = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  }

  return { file, readBytes, append };
}

export function openSessionJournal(workspace, sessionId) {
  const file = sessionJournalPath(workspace, sessionId);
  const dir = join(workspace, ".dory", "sessions");
  let queue = Promise.resolve();

  async function readBytes() {
    try {
      return await readFile(file);
    } catch (err) {
      if (err && err.code === "ENOENT") return Buffer.alloc(0);
      throw err;
    }
  }

  function append(type, fields = {}) {
    const record = { ts: new Date().toISOString(), type, ...fields };
    const job = queue.then(async () => {
      await mkdir(dir, { recursive: true });
      await appendFile(file, `${JSON.stringify(record)}\n`, "utf8");
      return record;
    });
    queue = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  }

  return { file, sessionId, readBytes, append };
}
