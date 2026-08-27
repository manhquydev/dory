---
type: research
date: 2026-08-27
time: 10:27
kind: increment-C-ensure_server-no-spawn
cwd: /home/manhquy/Downloads/flow/dory
head_paper: 53f3cff
rust_land: 5a60953
scope: who calls ensure_server; how p5 starts daemon; what breaks if ping-miss returns Err without spawn
cite: git show HEAD:rust/... only for rust land
---

# Research — C: `ensure_server` ping-miss Err, no spawn

Read-only. Did not implement rust. Did not invoke `dory`. Did not cargo leftover tree `/home/manhquy/Downloads/flow/dory`. Did not start `/run/user/1000/dory/default`. Did not sit `w13:t13`. Did not fold leftover 5. Land rust = `git show HEAD:rust/...` (tree `20545c6b` == `5a60953:rust`). Leftover WT `attach.rs` `60247909` / leftover `p5_attach.rs` `9c28fc3e` = leftover-5, not land.

## Paid

Paper **A** closed. PATH name `dory` gone. Default sock absent (`lexists=0`). Rust land unchanged: `git diff --stat 5a60953 HEAD -- rust/` empty. Paper HEAD `53f3cff` (`docs(plan): record leftover PATH dory unlink`).

- Unlink A cook/test/review/ops paid. Leftover ELF kept. `plans/260827-0940-unlink-leftover-path-dory/plan.md`
- 1020 eval-team accepted C as later letter, not this wave cook. `plans/reports/260827-1020-brainstorm-eval-team.md:117-128`
- 0927: C only after PATH not leftover. A is done. C is eligible rust door. `plans/reports/260827-0927-brainstorm-eval-team.md:108-117`
- Explicit `dory server` is a **different function**. `git show HEAD:rust/src/main.rs:77-78` → `server::run_foreground()` `git show HEAD:rust/src/server.rs:87-91`. Does **not** call `ensure_server`. C does not break that verb.
- Isolate scripts already require isolate RPC before sit. They walk around the door. Not the product door.
- HEAD `p5_*` (47 of 48) start the daemon themselves via `Command::new(bin()).arg("server").spawn()`. Those stay green if attach stops auto-spawning.

HEAD rust blobs (land): `attach.rs` `cf00a2fa` · `main.rs` `2fd5b78b` · `server.rs` `dfca2ac5` · `desk.rs` `4c788562` · `p5_attach.rs` `76875706`.

Leftover-5 mint (dirty ` M`, do not fold): `attach.rs` `60247909` · `main.rs` `373d6886` · `server.rs` `4de1554a` · `p5_attach.rs` `9c28fc3e`. Same as 0940/1020.

## Gap

Product sit still auto-starts. One function, three CLI entries, same XDG default sock.

`ensure_server` (`git show HEAD:rust/src/attach.rs:332-367`): `ping()` miss → `current_exe()` + `arg("server")` + `DORY_SIT_SHELL=1` + `setsid` + `cmd.spawn()` `:354` + 5s poll. No `spawn_server` symbol on HEAD. Leftover WT names `spawn_server` at leftover `:429` and recycle at leftover `:379` — **leftover-5, not land**.

`ping()` (`:369-373`) = `server::rpc_line_quiet({"op":"ping"})` → `connect_control_quiet`: `DORY_SOCKET` if set, else `$XDG_RUNTIME_DIR/dory/default/dory.sock` (`git show HEAD:rust/src/server.rs:1817-1822` `:1838-1846` `git show HEAD:rust/src/socket.rs:90-96`). `DEFAULT_SESSION = "default"` (`git show HEAD:rust/src/server.rs:15`). Factory XDG + sock absent + land binary exec'd → mint `/run/user/1000/dory/default/dory.sock`.

After A, factory `type dory` is already command-not-found. C does **not** change today's factory keystroke. C changes the product when a land binary returns to PATH or is exec'd. Leftover ELF / isolate ELF still embed **their own** `ensure_server`. C on HEAD rust does not rewrite those ELFs.

## Callers (file:line from HEAD)

`git grep` on `HEAD -- rust` for `ensure_server` / `desk::run` / `run_with_pane`. Complete. No other callers. Tests do not call these symbols (they exec the bin).

### `ensure_server` — 1 def, 2 call sites

| Site | Role | Cite |
|---|---|---|
| `attach.rs:332` | def. ping-ok → `Ok`; else spawn + poll | `git show HEAD:rust/src/attach.rs:332-367` |
| `attach.rs:136` | `sit()` (plain attach) | `git show HEAD:rust/src/attach.rs:135-137` |
| `desk.rs:166` | `run_with_pane` (desk sit) | `git show HEAD:rust/src/desk.rs:165-167` |

### `desk::run` / `run_with_pane` — complete

| Site | Role | Cite |
|---|---|---|
| `desk.rs:161` | `pub fn run()` → `run_with_pane(None)` | `git show HEAD:rust/src/desk.rs:161-162` |
| `desk.rs:165` | `pub fn run_with_pane` → `ensure_server` then TTY/UI | `git show HEAD:rust/src/desk.rs:165-167` |
| `main.rs:69` | **only** `desk::run()` caller: empty argv → bare `dory` | `git show HEAD:rust/src/main.rs:68-69` |
| `attach.rs:102` | **only** external `run_with_pane` caller: `dory attach` without `--plain` | `git show HEAD:rust/src/attach.rs:99-102` |

### CLI graph (three entries, one hunk)

```
dispatch []                  → desk::run()            → run_with_pane(None) → ensure_server
dispatch ["attach"]          → attach::run
  plain                      → sit()                  → ensure_server
  else                       → desk::run_with_pane    → ensure_server
dispatch ["server"]          → server::run_foreground   (no ensure_server)
```

`sit` is private; only `attach::run` `:99-100` calls it. `attach::run` only from `main.rs:72`.

One hunk on `ensure_server` covers bare `dory` / `dory attach` / `dory attach --plain`. `dory server` stays `run_foreground`.

## Test start pattern

### Leftover-5 `p5_attach.rs` vs HEAD (do not treat leftover as land)

| | HEAD land `76875706` | Leftover-5 WT `9c28fc3e` |
|---|---|---|
| Tests | 5 | 6 (+ `desk_tree_workspace_and_tab_have_cwd_and_ping_has_abi`) |
| `bare_dory_without_tty_starts_server` | `:213` unchanged | same body (diff does not touch it) |
| `attach_help_and_usage_name_sit_down` | `--help` only; asserts `Bare \`dory\` opens the desk` | **extra** leftover asserts: `attach --help` has `workspace picker` + `next / prev tab` — HEAD attach help is `next / prev pane` and has no `workspace picker` (`git show HEAD:rust/src/attach.rs:106-130`). Leftover help text, leftover test. |
| Extra test | absent | leftover-5 `desk.tree` cwd + ping `abi=3`. Uses `start()` (explicit `dory server`). Would **not** fail from C. Depends on leftover `workspace_live_cwd` / ABI, not spawn. |

Leftover extra is leftover-5 mint. Do not fold. Do not cargo leftover. Do not cite leftover `attach.rs:379` `spawn_server` as land.

### HEAD `p5_*` that **would fail** if ping-miss returns Err without spawn

**Exactly one.**

`git show HEAD:rust/tests/p5_attach.rs:212-250` `bare_dory_without_tty_starts_server`

Does **not** start `dory server`. Temp `XDG_RUNTIME_DIR`. Bare bin (no args, stdin null):

```
let out = Command::new(bin())
    .env("XDG_RUNTIME_DIR", &xdg)
    ...
    .output()
```

Asserts exit `1` + stderr `needs a tty`. That string is the **post-spawn** TTY check (`git show HEAD:rust/src/desk.rs:169-171` `git show HEAD:rust/src/attach.rs:139-141` — “server is up”). Then `workspace list` on same XDG must succeed (daemon was auto-spawned). Then `server stop`.

If `ensure_server` returns `Err` on ping-miss:

1. Exit still 1, but stderr is **not** `needs a tty` (TTY check never runs). Today spawn-fail text is `dory: start server: …` / `dory: server did not come up` (`:355` `:365`). A silent `Err(1)` is empty stderr — test still fails the `needs a tty` assert.
2. `workspace list` fails (no daemon). Test fails the `"workspaces"` assert.
3. That is the contract this test encodes: **bare `dory` without TTY still births the server.**

No HEAD test covers `dory attach` / `dory attach --plain` auto-spawn. Same function; product breaks, p5 does not see it.

### HEAD `p5_*` that would **not** fail (they already start `dory server`)

Uniform helper. Quote HEAD `p5_attach` (same shape in every other `p5_*`):

```
fn start() -> Harness {                          // :49
    let mut server = Command::new(bin())
        .arg("server")                           // explicit
        .env("XDG_RUNTIME_DIR", &xdg)
        ...
        .spawn()
        .expect("spawn dory server");
    // poll UnixStream::connect(session_sock)
}
```

File comments say it: `git show HEAD:rust/tests/p5_discover.rs:3-4` — `temp XDG_RUNTIME_DIR`, `dory server`.

| File | Tests | Start |
|---|---|---|
| `p5_attach.rs` | `attach_handshake_writes_and_detach_leaves_pty:121` `neighbor_walks_split_panes:178` `desk_tree_lists_split_siblings:265` | `start()` `:49-59` |
| `p5_attach.rs` | `attach_help_and_usage_name_sit_down:254` | **no daemon**. `--help` only. Survives C if `Bare \`dory\` opens the desk` stays. Does **not** assert `Starts the server if needed`. |
| `p5_close.rs` | 7 tests `:152-268` | `start()` `:51-61` |
| `p5_discover.rs` | 6 tests `:265-422` | `start()` `:61-71` |
| `p5_inside.rs` | `p5_inside_slave_drives_split_start_prompt_wait_flow:475` | `start()` `:111-121` |
| `p5_layout.rs` | `desk_layout_right_split_abuts_and_divider_moves:100` `attach_default_still_focuses:196` `two_attach_streams_live_after_split_detach_leaves:262` | `start()` `:48-58`. “attach” here is RPC `pane.attach` after `start()`, not CLI auto-spawn. |
| `p5_live_loop.rs` | `p5_live_loop_occupant_then_flow_status:435` | `start()` `:92-102` |
| `p5_occupant.rs` | `p5_01`…`p5_13` (13) | `start()` `:92-102` |
| `p5_prompt_after_report.rs` | `:366` | `start()` `:73-83` |
| `p5_prompt_paste.rs` | 2 tests | `start()` `:79-89` |
| `p5_prompt_unknown.rs` | `:368` | `start()` `:75-85` |
| `p5_real_repo.rs` | `:596` | `start()` `:118-128` |
| `p5_report.rs` | 5 tests | `start()` `:74-84` |
| `p5_s11.rs` | `:392` | `start()` `:90-100` |
| `p5_skill_occ.rs` | `:381` | `start()` `:78-88` |

48 HEAD `p5_*` tests. **1 fails** (`bare_dory_without_tty_starts_server`). **1 help-only** (USAGE prefix). **46 explicit `dory server`**.

Non-p5 (`pane_io.rs`, `p3_*.rs`, `server.rs` unit spawn) also `arg("server")`. Out of `p5_*` ask. They do not depend on attach auto-spawn.

## USAGE strings

Land still sells auto-start. Same increment as the hunk (1020: not a second increment).

| Blob | Text |
|---|---|
| `git show HEAD:rust/src/main.rs:50` | `Bare \`dory\` opens the desk (sidebar + tiled live panes). Starts the server if needed.` |
| `git show HEAD:rust/src/attach.rs:3-4` | `Bare \`dory\` / \`dory attach\` starts the daemon if needed and sits at the desk` |
| `git show HEAD:rust/src/attach.rs:117` | `Starts \`dory server\` if needed. Default is the desk (sidebar + tiled live panes).` |
| `git show HEAD:rust/src/main.rs:51-54` | tests stay `bash --norc`; `dory server stop` then `dory` if old daemon |

Paper README (not rust land; same public contract): `git show HEAD:README.md:15` — `Lần đầu sẽ bật \`dory server\` rồi mở **desk**`.

HEAD `attach_help_and_usage_name_sit_down` asserts `Bare \`dory\` opens the desk`, `dory attach`, `--plain`. Does **not** lock `Starts the server if needed`. Dropping the auto-start clause keeps that test green. Rewriting the Bare-dory prefix fails it.

Leftover-5 attach-help extras (`workspace picker` / `next / prev tab`) are leftover help, leftover test. Not land. Do not fold.

## What must change

Ranked. One increment. Cargo **isolate worktree only**. Copy-aside leftover `attach.rs` `60247909`; restore mint after commit.

1. **`ensure_server` ping-miss → `Err`, no `Command::new(exe).arg("server").spawn()`.** `git show HEAD:rust/src/attach.rs:332-367`. Shared by `desk.rs:166` and `sit` `:136`. One hunk = three CLI entries. Prefer `eprintln` + nonzero (e.g. `dory: server not running; start with \`dory server\``) so fail-closed is readable. Silent `Err(1)` is worse UX than today's spawn.
2. **USAGE / module / attach help** in the same increment: `main.rs:50`, `attach.rs:3`, `:117`. First sit = `dory server` then `dory`.
3. **Paper README `:15`** in the same increment (or it lies the day a land binary is on PATH again).
4. **Rewrite HEAD `bare_dory_without_tty_starts_server`.** Invert: bare `dory`, no prior server → fail-closed (new stderr, not `needs a tty`); `workspace list` must **fail**. Optional second case: `start()` then bare `dory` without TTY still prints `needs a tty` (server already up). Do not keep “starts_server” in the name.
5. **`dory server` / `run_foreground` — no change.**

Do not touch leftover-5 `p5_attach` extras. Do not retarget PATH. Do not rewrite leftover ELF.

## Do-not

- Do not implement rust this research. Do not cargo leftover `/home/manhquy/Downloads/flow/dory`. Do not fold leftover 5. Do not `git add` leftover `attach.rs` / `server.rs` / `p5_attach.rs`.
- Do not cite leftover WT `attach.rs` (`60247909`, recycle/`spawn_server` `:379+`) or leftover `server.rs` (`4de1554a`) as land. Land spawn = `git show HEAD:rust/src/attach.rs:332-367`.
- Do not treat leftover `desk_tree_workspace_and_tab_have_cwd_and_ping_has_abi` or leftover attach-help extras as land tests.
- Do not invoke `dory` / leftover ELF / isolate ELF on factory XDG. Do not start `/run/user/1000/dory/default`. Do not `dory server stop` on factory XDG. Do not sit `w13:t13` / `w13:p2R`. Do not close `wP` / `w15`. Do not `herdr server stop`.
- Do not recook 1910 / 0043 / 0227 / 0242 / 0940. Do not retarget PATH. Do not `ln` isolate onto `~/.local/bin/dory`. Do not `rm` leftover ELF.
- Do not B (`occ.report = Working` / isolate `prompt --wait`). Do not hop → `SIT_DORY` (factory snap would mint default).
- Do not claim “factory doors held.” PATH name gone; rust spawn unpaid; leftover ELF still embeds spawn.

## Evidence

- Paper HEAD `53f3cff052e03c61ca96c4c5ea99c7134c4e476e`. Rust tree `HEAD:rust` == `5a60953:rust` `20545c6b`. `git diff --stat 5a60953 HEAD -- rust/` empty. Default sock `lexists=0`. Did not connect-as-start. Did not cargo. Did not exec `dory`.
- Spawn: `git show HEAD:rust/src/attach.rs:332-367` (`arg("server")` `:341` `cmd.spawn()` `:354`). `ping` `:369-373`.
- Callers: `git show HEAD:rust/src/desk.rs:161-167` `git show HEAD:rust/src/attach.rs:99-102` `:135-137`. Dispatch `git show HEAD:rust/src/main.rs:68-78`. `git grep ensure_server|run_with_pane|desk::run HEAD -- rust` = those lines only.
- Ping sock: `git show HEAD:rust/src/server.rs:15` `:87-91` `:1817-1846` `git show HEAD:rust/src/socket.rs:46` `:90-96`.
- USAGE: `git show HEAD:rust/src/main.rs:50` `git show HEAD:rust/src/attach.rs:3` `:117`.
- Fail-if-C test: `git show HEAD:rust/src/tests` no — `git show HEAD:rust/tests/p5_attach.rs:212-250`. Helper start `:49-59`.
- Other p5 start: `git show HEAD:rust/tests/p5_close.rs:51-61` `p5_discover.rs:3-4` `:61-71` `p5_layout.rs:48-58` `:196-210` (RPC attach) + same `.arg("server")` in remaining `p5_*` `start()`.
- Leftover-5 contrast (not land): `git hash-object` `60247909` / `9c28fc3e`. `git diff HEAD -- rust/tests/p5_attach.rs` = +help extras + cwd/abi test. Leftover `ensure_server` leftover `:379` + `spawn_server` leftover `:429` — do not cite as land.
- Prior (cite, do not recook): `plans/reports/260827-1012-eval-spawn.md` `plans/reports/260827-1020-brainstorm-eval-team.md:104` `:119-128` `plans/reports/260827-0927-brainstorm-eval-team.md:108-117`.

## Recommendation

**C is one HEAD hunk + USAGE/README + rewrite one p5 test.** 46/48 `p5_*` already `dory server`. Product door is `ensure_server`, not the harness. Cargo isolate only. Copy-aside leftover `60247909`.

Flip to hold if founder still wants “bare `dory` starts the server if needed” as shipped UX.

## Unresolved

- Founder has not signed sit-without-spawn as public contract (1020: assumption medium).
- Exact fail-closed stderr string (empty `Err(1)` vs named “start `dory server`”).
- Whether paper README `:15` ships in the same commit or a paper-only follow. Same lie if split.
- Leftover ELF `3ba0e3bc…` / isolate ELF still auto-spawn on factory XDG after C. Exec-do-not, not a second rust hunk.

HEAD `53f3cff`. Rust land `5a60953`. C unpaid. No rust this pane.
