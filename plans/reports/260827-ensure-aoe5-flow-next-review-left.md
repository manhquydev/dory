---
type: review-left
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 03
lens: rv_left
writer: rv_left
verdict: LEFT_ACCEPT
critical: 0
cargo: none
checkout_leftover: none
---

# REVIEW — rv_left

**LEFT_ACCEPT** critical 0

Accept iff leftover 5 **path+sha** mint MATCH; leftover ELF sha `3ba0e3bc…` unchanged; `desk.rs` == HEAD; `git log -1 -- rust/` = `b544f5f`; no leftover cargo. Worktree rust dirty leftover = **pass**. Never checkout leftover.

This pane live-measured. Cook/test receipts not used as proof. Did not `git checkout` / `restore` leftover. Did not cargo leftover. Did not `git add` leftover 5. Did not exec leftover ELF. Did not recook `desk.rs`.

## Spec (phase-03 `rv_left`)

| # | Requirement | Status | Evidence |
|---|---|---|---|
| 1 | Leftover 5 path+sha mint MATCH | PASS | live `git hash-object` == trap 23 table |
| 2 | Leftover ELF sha `3ba0e3bc…` unchanged | PASS | sha256 MATCH; size/inode/mtime held |
| 3 | `desk.rs` == HEAD | PASS | worktree blob == `HEAD:rust/src/desk.rs` `4c788562…` |
| 4 | `git log -1 -- rust/` = `b544f5f` | PASS | `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2` |
| 5 | no leftover cargo | PASS | no `cargo` proc; ELF mtime 2026-08-26; `Cargo.lock` porcelain empty |
| 6 | Worktree rust dirty leftover = **pass** | PASS | porcelain ` M` ×5; HEAD blobs ≠ mint |
| 7 | Never checkout leftover | PASS | mint still dirty vs HEAD; this pane did not checkout/restore |

Stage 1 spec: PASS. Stage 2 leftover quality: no rust hunk, no remint, no fold. Out of lens: taxi/`rv_next`, sit/`rv_sit`, ship pathspec/`rv_fold`.

## Leftover 5 path+sha mint (trap 23)

| Path | live `git hash-object` | mint | vs HEAD blob | |
|---|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd` | MATCH mint; dirty vs land |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `602479094e84d31ad6f017775a3d55aeb485c644` | `62f09a95e114ba0a66c02d5369559ccdf9da50a2` | MATCH mint; dirty vs land |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d688636ff7315ccd665f450069d8284eb47ff` | `5fc70ad53a63b1ee13682da2ecd23201fb2422f9` | MATCH mint; dirty vs land |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `dfca2ac5010e8b659e6e7e64889f06ff60ae2391` | MATCH mint; dirty vs land |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `fa44bfbb3bef53a7568607773d3b5d59c56dca51` | MATCH mint; dirty vs land |

Porcelain still ` M` ×5, unstaged:

```
 M README.md
 M rust/src/attach.rs
 M rust/src/main.rs
 M rust/src/server.rs
 M rust/tests/p5_attach.rs
```

`git diff --cached --name-only --` leftover 5 + `rust/` = empty. No leftover 5 staged.

`rv_left` ≠ worktree rust clean. Dirty leftover = **pass**. `git diff b544f5f -- rust/` is **not** this lens.

## `desk.rs` == HEAD

| Field | sha |
|---|---|
| worktree `git hash-object rust/src/desk.rs` | `4c788562e4fdda10c8edd2878ed1fdd46050c218` |
| `HEAD:rust/src/desk.rs` | `4c788562e4fdda10c8edd2878ed1fdd46050c218` |

Equal. Porcelain empty for `rust/src/desk.rs`. Not recooked.

## Leftover ELF (stat only; not exec'd)

path=`/home/manhquy/Downloads/flow/dory/rust/target/debug/dory`

| field | live | pin `3ba0e3bc…` |
|---|---|---|
| sha256 | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` | MATCH unchanged |
| size | `18568240` | held |
| inode | `2490742` | held |
| mtime_epoch | `1787716801` | held (2026-08-26 11:00:01 +0700) |

ELF present → cargo door not taken. `rust/Cargo.lock` mtime_epoch `1787465159` (2026-08-23); porcelain empty. `pgrep -a -f '[c]argo'` = none. No leftover cargo. No isolate cargo this pane.

## Rust log (not `git diff` clean)

`git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit`

full `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2`

Land commit held. Leftover 5 remain uncommitted mint.

## Findings

critical_count: 0

No leftover mint miss. No leftover ELF remint. No `desk.rs` recook. No leftover checkout. No leftover cargo. No leftover 5 staged.

## This pane did not

- cargo leftover tree / cargo isolate
- `git checkout` / `restore` leftover 5 or leftover ELF
- `git add` leftover 5 / `git add -A` / `ak:git`
- exec leftover ELF / isolate ELF / factory `dory`
- recook `desk.rs`

LEFT_ACCEPT
