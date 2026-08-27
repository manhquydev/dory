---
type: ops
date: 2026-08-27
plan: 260827-1122-aoe-5-isolate-flow-judge
phase: paper-ship
verdict: SHIP_PASS
rust_head: b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
paper_head: 0475e6be1124785d38acb94514807e0e3f65fa17
pushed: no
---

# Ops — paper commit AOE 5 isolate flow judge

**Verdict: SHIP_PASS**

Paper only. `git add --` pathspec. Did not push. Did not `git add -u` / `git add -A`. Did not add leftover 5. Did not invoke factory `dory` / leftover ELF / isolate ELF. Did not `herdr server stop`. Did not sit `t13`.

```
0475e6b feat(isolate): fail-then-pass flow.sh gate
```

Rust land remains `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2`.

REVIEW_ACCEPT critical 0 (review.md). Subject is isolate slice, not company Phase 5.

## Cached before commit (⊆ pathspec)

23 files, all paper. `git diff --cached --name-only` extra=[], leftover 5 not in cached. Secret scan: 0 credential hits (17 `token` prose false positives in research/rt).

```
plans/260827-1122-aoe-5-isolate-flow-judge/phase-01-start.md
plans/260827-1122-aoe-5-isolate-flow-judge/phase-02-independent-aoe-5-test.md
plans/260827-1122-aoe-5-isolate-flow-judge/phase-03-review-and-ship.md
plans/260827-1122-aoe-5-isolate-flow-judge/plan.md
plans/reports/260827-1116-research-01-flow-judge.md
plans/reports/260827-1116-research-02-isolate-project.md
plans/reports/260827-1122-aoe5-roster.md
plans/reports/260827-1122-rt-failure.md
plans/reports/260827-1122-rt-fold.md
plans/reports/260827-1122-rt-security.md
plans/reports/260827-1122-scout-aoe5.md
plans/reports/260827-ensure-aoe5-flow-judge-cook-prompt.md
plans/reports/260827-ensure-aoe5-flow-judge-cook.md
plans/reports/260827-ensure-aoe5-flow-judge-journal.jsonl
plans/reports/260827-ensure-aoe5-flow-judge-review-fold.md
plans/reports/260827-ensure-aoe5-flow-judge-review-judge.md
plans/reports/260827-ensure-aoe5-flow-judge-review-left.md
plans/reports/260827-ensure-aoe5-flow-judge-review-sit.md
plans/reports/260827-ensure-aoe5-flow-judge-review.md
plans/reports/260827-ensure-aoe5-flow-judge-ship-prompt.md
plans/reports/260827-ensure-aoe5-flow-judge-test-prompt.md
plans/reports/260827-ensure-aoe5-flow-judge-test.md
scripts/dory-isolate-aoe5-flow-judge.sh
```

## After commit

| check | result |
|---|---|
| paper HEAD | `0475e6be1124785d38acb94514807e0e3f65fa17` |
| subject | `feat(isolate): fail-then-pass flow.sh gate` |
| `git show HEAD:scripts/dory-isolate-aoe5-flow-judge.sh` | present |
| leftover 5 porcelain | ` M` unstaged (all five) |
| leftover cached | empty |
| `git log -1 -- rust/` | `b544f5f fix(attach): do not auto-start server on sit` |
| push | no |

### Leftover 5 mint

| path | `git hash-object` | mint | |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5f…` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `60247909…` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d6886…` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554a…` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e…` | MATCH |

`desk.rs` worktree == `HEAD:rust/src/desk.rs` `4c788562e4fdda10c8edd2878ed1fdd46050c218`. Leftover ELF sha `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` held (not exec'd).

### PATH

`hash -r`; `type -a dory` → `type: dory not found` TYPE_EXIT=1.

### sock

AF_UNIX `UnixStream.connect` on `$XDG_RUNTIME_DIR/dory/default/dory.sock` = `/run/user/1000/dory/default/dory.sock` (not the session dir).

exists=False. connectable=0. `FileNotFoundError: [Errno 2] No such file or directory`. timeout 1s. Did not start default.

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` UNSET.

SHIP_PASS
