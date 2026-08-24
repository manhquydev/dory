import { spawn } from "node:child_process";
import { basename } from "node:path";

const FORBIDDEN_BIN = /^(herdr|dsh)(\.exe)?$/i;
const DEFAULT_TIMEOUT_MS = 15_000;

export function resolveFlowBin(env = process.env) {
  const bin = env.FLOW_BIN || "flow.sh";
  const base = basename(bin);
  if (FORBIDDEN_BIN.test(base) || bin.includes("@deepseek-ai/dsh")) {
    const err = new Error(`dory: refusing to exec ${base}`);
    err.code = "DORY_FORBIDDEN_BIN";
    throw err;
  }
  return bin;
}

export function invokeFlow({
  workspace,
  env = process.env,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  args = ["status"],
}) {
  const bin = resolveFlowBin(env);
  const argv = Array.isArray(args) && args.length > 0 ? args : ["status"];
  return new Promise((resolve) => {
    let child;
    try {
      child = spawn(bin, argv, {
        cwd: workspace,
        env,
        stdio: ["ignore", "pipe", "pipe"],
      });
    } catch (err) {
      resolve({
        bin,
        args: argv,
        cwd: workspace,
        code: null,
        signal: null,
        stdout: "",
        stderr: "",
        error: err.message,
      });
      return;
    }

    let stdout = "";
    let stderr = "";
    let settled = false;

    const finish = (result) => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      resolve({ bin, args: argv, cwd: workspace, stdout, stderr, ...result });
    };

    const timer = setTimeout(() => {
      child.kill("SIGTERM");
      setTimeout(() => {
        if (!settled) child.kill("SIGKILL");
      }, 1000).unref();
      finish({
        code: null,
        signal: "SIGTERM",
        error: `timed out after ${timeoutMs}ms`,
      });
    }, timeoutMs);
    timer.unref();

    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", (err) => {
      finish({ code: null, signal: null, error: err.message });
    });
    child.on("close", (code, signal) => {
      finish({ code, signal, error: null });
    });
  });
}
