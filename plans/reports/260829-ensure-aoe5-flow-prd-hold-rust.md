---
type: hold
lens: leftover-rust
date: 2026-08-29
wave: dory-aoe5p
writer: ps_hold
plan: 260829-0054-isolate-flow-prd-unlock
result: HOLD
cached_leftover5: empty
cached_rust: empty
---

# HOLD — leftover 5 and rust/ stay out of the commit

**HOLD.** Named pathspec ship only. Leftover 5 and `rust/` **must not** be in the commit. Ban `ak:git` / `git add -A` / `git add -u` / `git add rust/` / `git add README.md`. Do not push. Do not restore leftover. Dirty leftover worktree = pass.

If cached names include leftover 5 or any `rust/` path: **ABORT.** Unstage. Do not commit.

## Leftover 5 — never stage

Porcelain now ` M` ×5, unstaged. `git diff --cached --name-only --` leftover 5 = empty.

| Path | live `git hash-object` | mint | porcelain |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH | ` M` unstaged |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH | ` M` unstaged |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH | ` M` unstaged |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH | ` M` unstaged |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH | ` M` unstaged |

After commit these five **must** still be ` M` mint. Index blobs stay HEAD (`README.md` `5ac82b10…`, `attach.rs` `62f09a95…`, `main.rs` `5fc70ad5…`, `server.rs` `dfca2ac5…`, `p5_attach.rs` `fa44bfbb…`).

## rust/ — never stage

`git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit`. Success rust = that log unchanged.

`git diff --cached --name-only -- rust/` = empty. `desk.rs` worktree == HEAD `4c788562e4fdda10c8edd2878ed1fdd46050c218`. Leftover ELF `rust/target/debug/dory` sha `3ba0e3bc…` kept, not exec'd, not added.

Deny every `rust/**` path: leftover 5 rust files **and** clean HEAD rust (`flow.rs`, `desk.rs`, `Cargo.toml`, tests, vendor). This wave is 0 rust.

## After commit must hold

| Check | Required |
|---|---|
| leftover 5 | still ` M` mint, not in `HEAD` |
| `git log -1 -- rust/` | `b544f5f` |
| cached leftover 5 / `rust/` | empty |
| subject | `feat(isolate): fail-then-pass flow.sh prd` |
| push | no |

HOLD
