# COOK — increment C (read this whole file)

You are `c_cook`. Skills ON (ak:cook, ak:git). Implement **only** phase 1 of:

`/home/manhquy/Downloads/flow/dory/plans/260827-1032-ensure-server-no-auto-spawn/plan.md`
`/home/manhquy/Downloads/flow/dory/plans/260827-1032-ensure-server-no-auto-spawn/phase-01-start.md`

Follow the phase **exactly**. Cite `git show HEAD` rust, never leftover `attach.rs:379`.

## STOP if

- `DORY_SOCKET` / `DORY_ENV` / `DORY_RECYCLE` / `PI_CODING_AGENT_DIR` set
- leftover 5 `git hash-object` ≠ mint:
  - README `68190a5ffa073c082aa318aad5ed032e13cc90e3`
  - attach `602479094e84d31ad6f017775a3d55aeb485c644`
  - main `373d688636ff7315ccd665f450069d8284eb47ff`
  - server `4de1554ad56e248cdcf42f02111b7389b08dae82`
  - p5_attach `9c28fc3e0f3666498a8952411242d5301f7911de`
- You would exec leftover ELF `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory`
- You would exec isolate ELF `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` on factory XDG
- You would `cargo` in `/home/manhquy/Downloads/flow/dory`
- You would `git add -u` / `git commit -a` / add leftover `server.rs`
- You would sit `w13:t13` / start default sock / `herdr server stop`

## Land (0130 method)

KEEP=`/home/manhquy/.cache/dory-isolates/leftover-keep-0130`

1. Copy **four** leftover files to KEEP **only if** WT already mint. After checkout: refuse WT→KEEP.
2. `git checkout HEAD -- rust/src/attach.rs rust/src/main.rs rust/tests/p5_attach.rs README.md`
   - Crash abort: four named `cp` KEEP→those paths. **Never** restore `server.rs`. **Never** `git restore` leftover 5.
3. `ensure_server`: if `ping()` { Ok } else `eprintln!("dory: server not running; start with `dory server`"); Err(1)`.
   Delete spawn/setsid/5s poll. Keep `ping()`.
4. Drop auto-start in `main.rs` USAGE, `attach.rs` module docs + help. Keep `Bare \`dory\` opens the desk`.
5. HEAD `README.md` sit sentence (`:15` on HEAD blob): `dory server` then `dory`. Leave `git show HEAD:README.md:31` `22↔4↔0`.
6. Rewrite `bare_dory_without_tty_starts_server` → `bare_dory_without_server_fails_closed`:
   - every Command `env_remove("DORY_SOCKET")`
   - no daemon → exit 1 + new stderr; `workspace list` fail; temp XDG sock connectable=0
   - then `start()` + bare dory no TTY still `needs a tty`
   - help test: `assert!(!body.contains("Starts the server if needed"))` + attach-help equivalents
7. `git add` **exactly** those four paths. Cached names must equal those four.
8. Commit: `fix(attach): do not auto-start server on sit`
9. **After commit:** four named KEEP→WT cps. Leftover 5 must mint again (server untouched `4de1554a`).
10. `NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)`
    `git -C /home/manhquy/.cache/dory-isolates/land-4b70f79 reset --hard "$NEWHEAD"`
    Isolate HEAD must == `$NEWHEAD` and ≠ `5a60953`. No factory `reset --hard`.
11. `env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml p5_attach`
    then same three `-u` on `cargo build` same manifest.
12. Leftover ELF sha still `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14`.
    Isolate ELF strings: **no** `dory: start server:` / `server did not come up`.
    Factory `…/default/dory.sock` connectable=0. `type -a dory` empty.

## Receipt

Write `/home/manhquy/Downloads/flow/dory/plans/reports/260827-ensure-server-no-spawn-cook.md`

Must include: NEWHEAD, leftover 5 hashes after restore, leftover ELF sha, isolate HEAD, isolate ELF strings check, `p5_attach` cargo result, sock connectable, PATH empty, COOK_TAB/COOK_PANE, `COOK_PASS` or `COOK_FAIL`.

Do not push. Do not recook 1910/0043/0227/0242/0940. Do not hop/sit-pin/B/retarget.

When done, reply only: `COOK_PASS` or `COOK_FAIL` plus the receipt path.
