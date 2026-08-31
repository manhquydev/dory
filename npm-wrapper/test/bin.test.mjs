import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

const bin = join(dirname(fileURLToPath(import.meta.url)), "..", "bin", "dory-serve.js");

test("dory-serve with a desk-shaped verb exits 2 and names the lamp", () => {
  const r = spawnSync(process.execPath, [bin, "desk"], { encoding: "utf8" });
  assert.equal(r.status, 2);
  assert.match(r.stderr, /dory-serve \[--workspace/);
  assert.match(r.stderr, /journal lamp, not desk/);
});

test("package bin is dory-serve, not dory", async () => {
  const { readFile } = await import("node:fs/promises");
  const pkg = JSON.parse(await readFile(new URL("../package.json", import.meta.url), "utf8"));
  assert.equal(pkg.name, "@manhquy/dory");
  assert.deepEqual(pkg.bin, { "dory-serve": "bin/dory-serve.js" });
  assert.equal(Object.hasOwn(pkg.bin, "dory"), false);
});
