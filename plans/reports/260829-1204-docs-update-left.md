---
type: docs-update-left
date: 2026-08-29
wave: dory-docs-1204
writer: du_left
verdict: LEFT_PASS
critical: 0
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
cargo: none
checkout_leftover: none
---

# Docs-update leftover — independent mint

**Verdict: LEFT_PASS**

Live this pane. Cook / red-team receipts not used as proof. Did not fold leftover. Did not recook P. Did not claim company Phase 5.

Land README is `git show HEAD:README.md` (blob `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd`, size 6236). Working `README.md` is leftover 5 — different blob, mint, do not fold.

Evidence: `HEAD` `049e304` `docs(plan): check isolate prd-unlock phases`. `f1c966c` `feat(isolate): fail-then-pass flow.sh prd`.

## Leftover 5 path+sha mint

| Path | live `git hash-object` | mint | vs HEAD |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH `68190a5f` | dirty `5ac82b10…` |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH `60247909` | dirty `62f09a95…` |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH `373d6886` | dirty `5fc70ad5…` |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH `4de1554a` | dirty `dfca2ac5…` |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH `9c28fc3e` | dirty `fa44bfbb…` |

Porcelain leftover 5 still unstaged ` M` ×5:

```
 M README.md
 M rust/src/attach.rs
 M rust/src/main.rs
 M rust/src/server.rs
 M rust/tests/p5_attach.rs
```

`git diff --cached --name-only --` leftover 5 + `rust/` = empty. No leftover 5 staged.

`CHARTER.md` is also ` M` (WHERE pointer). That is not leftover 5.

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
| mtime_epoch | `1787716801` | held |

No leftover cargo. `pgrep cargo` empty this pane. Did not invoke factory `dory`.

## Rust log (not `git diff` clean)

`git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit` (`b544f5ff75a3c085ae6ab51ccafb3b58fa551db2`).

Worktree rust dirty leftover = **pass**.

## This pane did not

- edit leftover 5 / rewrite leftover README
- cargo leftover tree / cargo isolate
- `git checkout` / `restore` leftover 5 or leftover ELF
- `git add` leftover 5 / `git add -A` / `ak:git`
- exec leftover ELF / isolate ELF / factory `dory`
- recook P / fill `03-prd.md` / sit `t13` / `herdr server stop`
- claim isolate unlock = company Phase 5

Critical: 0
