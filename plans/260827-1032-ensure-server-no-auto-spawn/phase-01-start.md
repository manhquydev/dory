---
phase: 1
title: "Land no-spawn"
status: pending
priority: P1
effort: "60m"
dependencies: []
---

# Phase 1: Land no-spawn

## Overview

HEAD `ensure_server` ping-miss fail-closed. USAGE + HEAD README cùng commit. Đảo một p5. Restore leftover 5. Cargo chỉ isolate `land-4b70f79`.

## Requirements

- Functional: `git show HEAD:rust/src/attach.rs` `ensure_server` — if `ping()` { Ok } else eprintln `dory: server not running; start with \`dory server\`` + `Err(1)`. Xóa `Command::new(exe).arg("server")` + `setsid` + `spawn` + 5s poll.
- Functional: `sit` `:136` và `desk.rs:166` vẫn gọi `ensure_server` (không sửa `desk.rs`).
- Functional: `dory server` vẫn `run_foreground` (`main.rs:77-78`).
- Functional: USAGE drop auto-start: `main.rs:50`, `attach.rs:3-4`, `attach.rs:117`. Giữ `Bare \`dory\` opens the desk`.
- Functional: HEAD README `:15` — ngồi = `dory server` rồi `dory`. Không sửa leftover `## Now`.
- Functional: rewrite HEAD `bare_dory_without_tty_starts_server` → `bare_dory_without_server_fails_closed`: every `Command` `env_remove("DORY_SOCKET")` + `DORY_ENV`; no daemon → exit 1 + new stderr; `workspace list` **fail**; temp XDG `dory.sock` connectable=0; then `start()` + bare `dory` no TTY still `needs a tty`.
- Non-functional: leftover 5 mint after restore. Cargo only isolate **absolute** manifest. No factory `dory` / leftover ELF / isolate ELF. Sock `…/default/dory.sock` connectable=0.

## Architecture

```
KEEP=~/.cache/dory-isolates/leftover-keep-0130
# copy KEEP only if WT hashes already mint. after checkout: refuse WT→KEEP
cp rust/src/attach.rs  $KEEP/attach.rs
cp rust/src/main.rs    $KEEP/main.rs
cp rust/tests/p5_attach.rs $KEEP/p5_attach.rs
cp README.md           $KEEP/README.md
# server.rs stays leftover; do not checkout; do not restore
git checkout HEAD -- rust/src/attach.rs rust/src/main.rs rust/tests/p5_attach.rs README.md
# hunk…
git add rust/src/attach.rs rust/src/main.rs rust/tests/p5_attach.rs README.md
# cached names MUST equal those four. ban git add -u / commit -a
git commit
NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)
cp $KEEP/attach.rs rust/src/attach.rs
cp $KEEP/main.rs rust/src/main.rs
cp $KEEP/p5_attach.rs rust/tests/p5_attach.rs
cp $KEEP/README.md README.md
# leftover server.rs untouched — hash still 4de1554a
git -C /home/manhquy/.cache/dory-isolates/land-4b70f79 reset --hard "$NEWHEAD"
env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \
  cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml p5_attach
env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \
  cargo build --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml
```

Land spawn cite (before hunk): `git show HEAD:rust/src/attach.rs:332-367`.
Callers: `attach.rs:136` `desk.rs:166`. Dispatch: `main.rs:68-78`.

## Related Code Files

- Modify (HEAD blobs only): `rust/src/attach.rs` `rust/src/main.rs` `rust/tests/p5_attach.rs` `README.md`
- Do not modify: leftover WT copies (restore after commit), `desk.rs`, `server.rs`, leftover ELF, `scripts/`
- Create: `plans/reports/260827-ensure-server-no-spawn-cook.md`

## Implementation Steps

1. New `w13` tab, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Not `t13`. Record `COOK_TAB` / `COOK_PANE`.
2. Refuse if `DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` set. **Also STOP** if argv would exec leftover ELF or isolate ELF on factory XDG. Snapshot `XDG_RUNTIME_DIR` (unset/empty → STOP). Snapshot leftover 5 `git hash-object` — must MATCH researcher-02. Mismatch → **STOP**.
3. Copy **four** leftover files into KEEP **only if** WT hashes already mint. `cmp` KEEP==WT for those four. After this step: **refuse** any WT→KEEP copy. `server.rs` leftover stays; do not copy-as-restore later.
4. `git checkout HEAD -- rust/src/attach.rs rust/src/main.rs rust/tests/p5_attach.rs README.md`. Crash → four named KEEP→WT `cp` (not `server.rs`, not `git restore`).
5. Rewrite `ensure_server`: ping ok → Ok; else eprintln + `Err(1)`. Delete spawn/setsid/5s poll. Keep `ping()` (DORY_SOCKET-first — paper, not a `server.rs` hunk).
6. Drop auto-start sentences in `main.rs:50` `attach.rs:3-4` `:117`. Keep Bare-dory desk prefix.
7. HEAD README `:15`: `dory server` then `dory`. Do not touch leftover Now. Do not change `git show HEAD:README.md:31` `22↔4↔0` (leftover `:21`/`:36` is `26` — smuggle).
8. In HEAD `p5_attach.rs`: rename + invert `bare_dory_without_tty_starts_server`. Every Command `env_remove("DORY_SOCKET")`. Help test: keep Bare-dory prefix; add `assert!(!body.contains("Starts the server if needed"))` and attach-help equivalents. Do not add leftover cwd/abi test.
9. `git add rust/src/attach.rs rust/src/main.rs rust/tests/p5_attach.rs README.md`. `git diff --cached --name-only` **exactly** those four. Any `server.rs` / leftover 5 → unstage STOP. Ban `git add -u` / `git commit -a`. Commit. Message: `fix(attach): do not auto-start server on sit`.
10. **After commit only:** four named `cp` KEEP→WT. Hash-object: attach `60247909` main `373d6886` p5 `9c28fc3e` README `68190a5f` server `4de1554a` (untouched). Cached empty.
11. `NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)`. `git -C /home/manhquy/.cache/dory-isolates/land-4b70f79 reset --hard "$NEWHEAD"`. Isolate `rev-parse HEAD` == `$NEWHEAD` ≠ `5a60953`. Cấm factory `reset --hard`.
12. `env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml p5_attach` then the same three `-u` on `cargo build`. Leftover ELF sha still `3ba0e3bc…`. Isolate ELF strings: **no** `dory: start server:` / `server did not come up`. Do not exec either ELF on factory XDG.
13. Factory: `UnixStream::connect("$XDG_RUNTIME_DIR/dory/default/dory.sock")` connectable=0; `type -a dory` empty; leftover ELF sha + spawn-strings unchanged. Write cook receipt.

## Success Criteria

- [ ] HEAD `ensure_server` no spawn; stderr locked
- [ ] USAGE + HEAD README sit sentence updated; leftover README hash `68190a5f` restored
- [ ] Isolate `p5_attach` PASS; leftover `p5_attach` hash `9c28fc3e` restored
- [ ] Leftover attach `60247909` / main `373d6886` / server `4de1554a` mint
- [ ] No factory `dory`/leftover/isolate ELF argv; `dory.sock` connectable=0; PATH empty
- [ ] Isolate ELF spawn-strings gone; leftover ELF sha+strings held
- [ ] `NEWHEAD` bound; isolate HEAD == `$NEWHEAD`

## Risk Assessment

| Risk | Signal | Response |
|---|---|---|
| Edit leftover attach in place | hash ≠ `60247909` | STOP. Restore KEEP. Checkout HEAD blob. |
| Cargo leftover tree | leftover ELF sha changed | FAIL. |
| Forget restore | leftover 5 ≠ mint | `cp` KEEP. Do not delete KEEP. |
| Isolate not reset | ELF still `dc0e867a` | `reset --hard "$NEWHEAD"` then cargo. |
| Factory invoke | sock connectable | FAIL. Do not `server stop` unless user intends kill. |

## Next Steps

Phase 2 on a **different** `w13` tab id, not a split of cook, not `t13`.
