import { parseArgs } from "node:util";
import { HOST, PORT, startServer } from "./serve.js";

export function usage() {
  return "usage: dory serve --workspace <abs-dir>\nopens http://127.0.0.1:7380/ (journal lamp, not desk)";
}

export async function main(argv) {
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
    console.error(usage());
    console.error(err.message);
    return 2;
  }

  if (positionals.length !== 1 || positionals[0] !== "serve") {
    console.error(usage());
    return 2;
  }

  if (values.workspace === undefined) {
    console.error("dory: missing --workspace <abs-dir>");
    return 2;
  }

  try {
    const started = await startServer({
      workspace: values.workspace,
      host: values.host,
      port: PORT,
    });
    console.error(
      `dory: journal projection on http://${HOST}:${started.port}/ workspace=${values.workspace} (not a workplace)`,
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
