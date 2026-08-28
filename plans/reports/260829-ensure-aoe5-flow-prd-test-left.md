---
type: test-left
date: 2026-08-29
plan: 260829-0054-isolate-flow-prd-unlock
phase: 02
writer: pt_left
wave: dory-aoe5p
verdict: LEFT_PASS
cargo: none
checkout_leftover: none
---

# TEST leftover — independent prd unlock (phase 02)

**Verdict: LEFT_PASS**

After independent run. Live `git hash-object` / `sha256sum` this pane. Cook receipt not used as proof. Did not cargo leftover. Did not `git checkout` / `restore` leftover. Did not `git add` leftover 5. Did not exec leftover ELF. Did not invoke factory `dory`. Did not recook `desk.rs`. Did not fold leftover.

Worktree rust dirty leftover = **pass**.

Independent journal cwd `aoe5p.azx4PH` ≠ cook ISO `aoe5p.eGZMMi`. Copied journal `plans/reports/260829-ensure-aoe5-flow-prd-journal.jsonl` mtime 1787940335 > cook 1787940279.

## Leftover 5 path+sha mint (trap 23)

| Path | live `git hash-object` | mint | |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

Porcelain still ` M` ×5, unstaged:

```
 M README.md
 M rust/src/attach.rs
 M rust/src/main.rs
 M rust/src/server.rs
 M rust/tests/p5_attach.rs
```

`git diff --cached --name-only --` leftover 5 = empty. No leftover 5 staged.

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

No leftover cargo. No isolate cargo. `pgrep cargo` empty this pane.

## Rust log (not `git diff` clean)

`git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit`

## This pane did not

- cargo leftover tree / cargo isolate
- `git checkout` / `restore` leftover 5 or leftover ELF
- `git add` leftover 5 / `git add -A` / `ak:git`
- exec leftover ELF / isolate ELF / factory `dory`
- recook `desk.rs`
- fold leftover

LEFT_PASS
