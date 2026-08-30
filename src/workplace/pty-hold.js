import { spawn } from "node:child_process";
import { basename } from "node:path";

const FORBIDDEN = /^(herdr|dsh)(\.exe)?$/i;

function assertAllowedBin(bin) {
  const base = basename(bin);
  if (FORBIDDEN.test(base) || String(bin).includes("@deepseek-ai/dsh")) {
    const err = new Error(`dory: refusing to exec ${base}`);
    err.code = "DORY_FORBIDDEN_BIN";
    throw err;
  }
}

export function isPidAlive(pid) {
  if (!Number.isInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

/**
 * Hold a live PTY. Detach (HTTP client gone) must not kill this process.
 * Server shutdown must kill it. This is not a restore-image.
 */
export function holdPty({ cwd, env = process.env }) {
  const bin = "script";
  assertAllowedBin(bin);
  const child = spawn(
    bin,
    ["-qefc", "bash --norc --noprofile", "/dev/null"],
    {
      cwd,
      env: { ...env, TERM: "xterm", PS1: "" },
      stdio: ["pipe", "pipe", "pipe"],
    },
  );

  let output = "";
  const onChunk = (chunk) => {
    output += chunk;
    if (output.length > 200_000) output = output.slice(-100_000);
  };
  child.stdout.setEncoding("utf8");
  child.stderr.setEncoding("utf8");
  child.stdout.on("data", onChunk);
  child.stderr.on("data", onChunk);

  return {
    pid: child.pid,
    child,
    write(data) {
      child.stdin.write(data);
    },
    read() {
      return output;
    },
    alive() {
      return child.exitCode === null && !child.killed && isPidAlive(child.pid);
    },
    kill() {
      if (child.exitCode !== null) return;
      child.kill("SIGTERM");
      setTimeout(() => {
        if (child.exitCode === null) child.kill("SIGKILL");
      }, 500).unref();
    },
  };
}
