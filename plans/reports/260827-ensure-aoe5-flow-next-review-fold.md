---
type: review
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 03
lens: rv_fold
writer: rv_fold
verdict: FOLD_ACCEPT
critical: 0
cached: empty
leftover_staged: 0
eval_1638_staged: 0
git_add: none
---

# REVIEW — rv_fold

**Verdict: FOLD_ACCEPT**

Cached empty until ship. Leftover 5 still ` M` unstaged mint. Eleven `260827-1638-eval-*.md` `??` untracked, not staged, `ls-files` 0. Script does not `source` / `.` / `exec` judge. Paid judge unmodified. This pane did not `git add`.

## Spec (phase-03 `rv_fold`)

Accept iff: cached ⊆ named allowlist; leftover 5 + `README.md` + `rust/**` + `260827-1638-eval-*` never staged; paid judge not modified.

User pin this pane: cached **empty** until ship; leftover 5 + 1638-eval not staged; script does not source judge; do not `git add`.

| # | Requirement | Status | Evidence |
|---|---|---|---|
| 1 | Cached empty until ship | PASS | `git diff --cached --name-only` → 0 paths; `--stat` empty |
| 2 | Cached ⊆ named allowlist | PASS | empty set |
| 3 | Leftover 5 never staged | PASS | porcelain ` M` ×5; `git diff --cached --name-only --` leftover 5 = empty |
| 4 | `README.md` + `rust/**` never staged | PASS | same; `desk.rs` not dirty; cached `rust/` empty |
| 5 | `260827-1638-eval-*` never staged | PASS | 11× `??`; cached empty; `git ls-files` count 0 |
| 6 | Paid judge not modified | PASS | `scripts/dory-isolate-aoe5-flow-judge.sh` porcelain empty |
| 7 | Script does not source/exec judge | PASS | no `^\s*(source\|\.\|exec)\s+` in next script; judge name only in `$0` refuse, `self_refuse_paid` regex, header comment |
| 8 | No `git add` this pane | PASS | index still 0 after write |

## Cached

```
git diff --cached --name-only
# (empty)
count=0
```

Allowlist not staged. Ship later = named `git add --` only. Ban `git add -A` / `-u` / `ak:git` / glob.

## Leftover 5 — unstaged mint (not folded)

Porcelain:

```
 M README.md
 M rust/src/attach.rs
 M rust/src/main.rs
 M rust/src/server.rs
 M rust/tests/p5_attach.rs
```

` M` = worktree dirty, index clean. Not `M ` / `MM` / `A `.

| Path | live `git hash-object` | mint | |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

`desk.rs` worktree `4c788562e4fdda10c8edd2878ed1fdd46050c218` == `HEAD:rust/src/desk.rs`. Not leftover 5.

`git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit`.

HEAD blobs ≠ mint (tracked land). Dirty vs `b544f5f` is HOLD, not a fold.

## 1638-eval — not staged

11 untracked, 0 tracked, 0 cached:

```
?? plans/reports/260827-1638-eval-aoe.md
?? plans/reports/260827-1638-eval-charter.md
?? plans/reports/260827-1638-eval-desk.md
?? plans/reports/260827-1638-eval-iso.md
?? plans/reports/260827-1638-eval-left.md
?? plans/reports/260827-1638-eval-next.md
?? plans/reports/260827-1638-eval-occ.md
?? plans/reports/260827-1638-eval-roster.md
?? plans/reports/260827-1638-eval-skill.md
?? plans/reports/260827-1638-eval-synth.md
?? plans/reports/260827-1638-eval-wait.md
```

Deny-list for ship (plan L2 / trap 32). Nguồn citations only.

## Script does not source judge

File: `scripts/dory-isolate-aoe5-flow-next.sh` porcelain `??` (new, **not staged**).

`git add` in script: **0**.

Judge token `dory-isolate-aoe5-flow-judge` only:

| Line | Role |
|---|---|
| `:5` | comment: do not source or exec … or judge |
| `:44` | `$0` case refuse if this file is the judge name |
| `:266` | `self_refuse_paid` regex rejects `source` / `.` / `exec` of judge |

`self_refuse_paid` `:259-277` runs against `$0` text. A real `source`/`exec` of judge would fail that gate.

Other `source`/`exec` tokens are **not** bash source of paid scripts:

- `herdr … --source visible` / `--source recent-unwrapped` (Herdr flag)
- `/bin/bash -c 'cd "$0" && exec "$1" server' "$ISO_REAL" "$SIT_DORY"` (`:725`) — isolate land ELF `server`, not judge

No line matches `^\s*(source|\.|exec)\s+`.

Paid judge `scripts/dory-isolate-aoe5-flow-judge.sh`: porcelain empty (not modified, not staged).

## Residual (not a reject)

`git add -A` / `ak:git` / `git add plans/reports/` **would** scoop leftover 5 + the 1638 mountain. Live index does not. Ship must `git add --` named files in `phase-03-review-and-ship.md`. This pane left cached empty.

## This pane did not

- `git add` / `git add -A` / `-u` / `ak:git`
- stage leftover 5 / `README.md` / `rust/**` / `260827-1638-eval-*` / judge / next script
- `git checkout` / `restore` leftover
- cargo leftover
- exec judge / source judge

FOLD_ACCEPT
