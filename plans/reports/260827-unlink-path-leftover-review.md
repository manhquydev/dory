REVIEW_ACCEPT

# Independent review — leftover unlink A

REVIEW_TAB=w13:t1Y
REVIEW_PANE=w13:p5M
role=ur_fold (lead; merged 4 doors)
COOK_TAB=w13:t1W
COOK_PANE=w13:p5C
TEST_TAB=w13:t1X
TEST_PANE=w13:p5G
cwd=/home/manhquy/Downloads/flow/dory
when=2026-08-27
phase=3 hostile review (paper only; no ship)
HEAD=`5a6095367f905a42ff1c38886ebffa0f0840977d`
critical_count=0

REVIEW_TAB ≠ COOK_TAB ≠ TEST_TAB ≠ t13.
REVIEW_PANE ≠ COOK_PANE ≠ TEST_PANE.

Sibling witness files `ur_sock` / `ur_retarg` / `ur_spawn` absent on disk.
Lead self-checked fold + sock + retarget + spawn (and ELF keep).

## Verdict

critical_count **0**. Spec A holds. Traps 1–16 did not fire.
PATH name `dory` gone. Leftover 5 mint, unstaged, uncached. Leftover ELF kept at cook sha. Default sock not connectable.

Land attach cited from `git show HEAD:rust/src/attach.rs:332-370` (auto-spawn unpaid C). Not leftover working tree.

## Score card

| Field | Value |
|---|---|
| score | 10/10 |
| critical_count | 0 |
| verdict | ACCEPT |

## Spec compliance

| # | Requirement | Status | Evidence |
|---|---|---|---|
| 1 | `~/.local/bin/dory` gone | PASS | live `test ! -e` / `test ! -L` exit 0; lexists=False |
| 2 | `hash -r`; `type -a dory` empty | PASS | `type: dory not found` TYPE_EXIT=1; PATH walk COUNT=0 HITS=[] |
| 3 | Leftover ELF kept; sha/mtime unchanged | PASS | exists; sha256/mtime/size/inode MATCH cook `3ba0e3bc…` / `1787716801` / `18568240` / `2490742` |
| 4 | Sock not connectable on `$XDG_RUNTIME_DIR` | PASS | `/run/user/1000/dory/default/dory.sock` connectable=False FileNotFoundError 1s |
| 5 | Leftover 5 = researcher-02 mint | PASS | live `git hash-object` MATCH all 5 |
| 6 | No `dory` / leftover ELF / isolate ELF / cargo leftover / rust/`scripts/` edit | PASS | receipts + live `/proc` exe NONE; porcelain leftover 5 still ` M`; `?? scripts/` pre-existing |
| 7 | Distinct cook/test/review tabs; not `t13` | PASS | t1W / t1X / t1Y |

No missing A. No unjustified extra (no retarget, no hop/sit cook, no rust hunk).

## Fold (live)

`git status --porcelain` leftover 5 still unstaged ` M` (space-M):

```
 M README.md
 M rust/src/attach.rs
 M rust/src/main.rs
 M rust/src/server.rs
 M rust/tests/p5_attach.rs
?? scripts/
```

`git diff --cached --name-only` = **empty** (whole index). Leftover 5 **not staged**.

Live `git hash-object` vs researcher-02 mint (`plans/260827-0940-unlink-leftover-path-dory/research/researcher-02-leftover.md`):

| File | hash-object | mint |
|---|---|---|
| README.md | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| rust/src/attach.rs | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| rust/src/main.rs | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| rust/src/server.rs | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| rust/tests/p5_attach.rs | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

No `scripts/` mutation this increment: still untracked dir of the five pre-existing flock scripts; none added. Rust drift vs pre-rm snapshot = leftover 5 ` M` only (allowed). Cached empty.

Observe-only: `desk.rs` hash-object `4c788562e4fdda10c8edd2878ed1fdd46050c218`. HEAD `5a60953` (not an A gate).

## Four doors (live)

### sock

| field | value |
|---|---|
| DORY_SOCKET / DORY_ENV / DORY_RECYCLE / PI_CODING_AGENT_DIR | UNSET |
| DORY_* | NONE |
| XDG_RUNTIME_DIR | `/run/user/1000` |
| path | `/run/user/1000/dory/default/dory.sock` |
| lexists / exists | False |
| connectable | False |
| err | `FileNotFoundError: [Errno 2] No such file or directory` |
| dead_inode | no |
| action | continue (no `dory server stop`; no `iso()`; no `DORY_SOCKET=`) |

### retarget

`hash -r`; `type -a dory` empty. No `ln` to isolate `land-4b70f79`. PATH walk name `dory` COUNT=0.

### spawn

Cook + test receipts: no `dory` / leftover ELF / isolate ELF argv. Live `/proc/*/exe` leftover+isolate = NONE. Sock still not connectable → no new default.

### ELF keep

`/home/manhquy/Downloads/flow/dory/rust/target/debug/dory` exists; sha256/mtime/size/inode MATCH cook before-snapshot. Not exec'd this review.

Isolate debug exists (observe-only): size=18493816 inode=1063590. Not exec'd.

## Land attach (HEAD only)

`git show HEAD:rust/src/attach.rs:332-370` — `ensure_server` still auto-spawns `current_exe()` + `server` if ping fails. Trap 1 / unpaid C. Do not claim factory doors held.

```
332:pub fn ensure_server() -> Result<(), i32> {
333:    if ping() {
...
340:    let mut cmd = Command::new(exe);
341:    cmd.arg("server")
...
354:    cmd.spawn()
...
367:}
```

## Traps 1–16

| # | Trap | Fired? |
|---|---|---|
| 1 | `ln -sfn` isolate | no |
| 2 | typed `dory` to verify | no (ls/type/python sock only) |
| 3 | `rm` leftover ELF | no |
| 4 | cargo leftover tree | no (mint + ELF sha held) |
| 5 | leftover attach as land | no (HEAD `:332-370` only) |
| 6 | exec hop / 1910 / 0043 / 0227 / 0242 | no |
| 7 | sit `t13` / close `wP`/`w15`/`t13` / `herdr server stop` / `dory server stop` | no |
| 8 | git add leftover 5 | no (cached empty) |
| 9 | remaining PATH `dory` | no (`type -a` empty) |
| 10 | unlink when target ≠ leftover | no (cook realpath leftover ELF) |
| 11 | `rm "$(readlink -f …)"` | no |
| 12 | cargo missed by leftover-5 hash | no (ELF sha/mtime/inode MATCH) |
| 13 | `exists` as spawn door / stop default | no (connectable on `$XDG_RUNTIME_DIR`) |
| 14 | `git diff --name-only` fold | no (porcelain + cached + mint) |
| 15 | hop PATH `dory` after unlink | known-broken paper; not cooked |
| 16 | cook mint self-ref / HEAD pin as A | no (researcher-02 before rm; HEAD not A gate) |

## Cook / test identity

| receipt | first line | tab/pane |
|---|---|---|
| `plans/reports/260827-unlink-path-leftover-cook.md` | COOK_PASS | w13:t1W / w13:p5C |
| `plans/reports/260827-unlink-path-leftover-before.txt` | COOK_PASS | same cook |
| `plans/reports/260827-unlink-path-leftover-test.md` | TEST_PASS | w13:t1X / w13:p5G |
| test sock/elf/hash witnesses | PASS | t1X p5J / p5K / p5H |

Cook: `test -L` then `rm` symlink path only. Mint gated **before** rm.

## Forbidden argv (this review)

No `dory`. No leftover ELF exec. No isolate ELF exec. No `ln`. No cargo leftover. No `git add` leftover 5. No sit `t13`. No `scripts/` mutation. No rust edit.

## Side-effect flags

| Flag | Value |
|---|---|
| `hop_exec` | false |
| `leftover_folded` | false |
| `leftover_staged` | false |
| `default_started` | false |
| `sit_t13` | false |
| `cargo_repo` | false |
| `reviewed_working_as_land` | false |
| `retarget_ln` | false |
| `scripts_mutated` | false |

## Known broken (not this increment)

Hop / USAGE / sit-child still PATH `dory` after unlink. `ensure_server` HEAD `:332-370` still auto-spawns. C unpaid.

## Ship note (not this tab)

ACCEPT paper only. Ship pathspec if later: `plans/260827-0940-unlink-leftover-path-dory/` + `plans/reports/260827-unlink-path-leftover-*` (+ optional `plans/reports/260827-0927-brainstorm-eval-team.md`). Refuse leftover 5, `README.md`, `rust/`, `scripts/`, `git add -u`, `git add rust/`. No push unless user asks.
