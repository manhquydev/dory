import { parseArgs } from "node:util";
import { HOST, PORT, startServer } from "./serve.js";

export function usage() {
  return "usage: dory-serve [--workspace <abs-dir>]\nopens http://127.0.0.1:7380/ on the current directory (journal lamp, not desk)";
}

export function parseCli(argv, { cwd = process.cwd() } = {}) {
  let values;
  let positionals;
  try {
    ({ values, positionals } = parseArgs({
      args: argv,
      allowPositionals: true,
      strict: true,
      options: {
        workspace: { type: "string" },
        host: { type: "string", default: HOST },
      },
    }));
  } catch (err) {
    return { ok: false, code: 2, error: `${usage()}\n${err.message}` };
  }

  if (positionals.length > 1) {
    return { ok: false, code: 2, error: usage() };
  }
  if (positionals.length === 1 && positionals[0] !== "serve") {
    return { ok: false, code: 2, error: usage() };
  }

  return {
    ok: true,
    workspace: values.workspace === undefined ? cwd : values.workspace,
    host: values.host,
  };
}

export async function main(argv) {
  const parsed = parseCli(argv);
  if (!parsed.ok) {
    console.error(parsed.error);
    return parsed.code;
  }

  try {
    const started = await startServer({
      workspace: parsed.workspace,
      host: parsed.host,
      port: PORT,
    });
    console.error(
      `dory: journal projection on http://${HOST}:${started.port}/ workspace=${parsed.workspace} (not a workplace)`,
    );
    await new Promise((resolve) => {
      const onStop = () => {
        started.close().finally(resolve);
      };
      process.once("SIGINT", onStop);
      process.once("SIGTERM", onStop);
    });
    return 0;
  } catch (err) {
    console.error(err.message);
    return err.exitCode || 1;
  }
}
