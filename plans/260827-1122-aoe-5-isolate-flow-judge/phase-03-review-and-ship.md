---
phase: 3
title: "Review and ship"
status: pending
priority: P1
effort: "45m"
dependencies: [2]
---

# Phase 3: Review and ship

## Overview

Bốn lens review rồi paper commit (script + plan + reports). Không push. Không leftover 5.

## Requirements

- Functional: REVIEW_ACCEPT critical 0; paper HEAD chứa script mới
- Non-functional: leftover 5 still ` M`; no push; rust HEAD still `b544f5f`

## Architecture

Review tab 4 panes (≤6). Ship tab 1 pane, ak:git paper only.

### Review lenses

| Name | Accept iff |
|---|---|
| `rv_judge` | Taxi used abs flow.sh; codes 1 then 0; not `/bin/true`/`status`/`doctor`; occupant wrote PASS |
| `rv_left` | Leftover 5 mint table MATCH; `desk.rs` == HEAD; `git log -1 -- rust/` = `b544f5f`; no leftover cargo. Worktree rust dirty leftover = **pass** |
| `rv_sit` | Sit ≠ t13; stop compound; factory sock dead; no factory `dory` argv |
| `rv_fold` | Cached ship pathspec ⊆ allowlist; leftover 5 never staged |

### Ship pathspec ONLY

```
scripts/dory-isolate-aoe5-flow-judge.sh
plans/260827-1122-aoe-5-isolate-flow-judge/
plans/reports/260827-1116-research-01-flow-judge.md
plans/reports/260827-1116-research-02-isolate-project.md
plans/reports/260827-1122-scout-aoe5.md
plans/reports/260827-ensure-aoe5-flow-judge-*
plans/reports/260827-1122-aoe5-roster.md
```

Ban `git add -u` / `git add -A` / leftover 5 / `rust/**`. Không dùng `ak:git` nếu nó `add -A`. `git add --` từng path.

Message:

```
feat(isolate): fail-then-pass flow.sh gate
```

Paper-only. Do not push. After commit: leftover 5 still ` M` mint; `git log -1 -- rust/` still `b544f5f`; sock connectable=0. Subject ≠ company Phase 5.

Write `plans/reports/260827-ensure-aoe5-flow-judge-ops.md`.

## Related Code Files

- Review: `plans/reports/260827-ensure-aoe5-flow-judge-review.md` (+ per-lens)
- Ship: ops receipt
- Do not modify leftover 5 / rust

## Implementation Steps

1. Four review agents. Critical 0 required.
2. `git diff --cached --name-only` ⊆ pathspec.
3. Commit. Confirm leftover porcelain ` M` ×5.
4. Close wave tabs only (`dory-aoe5-*` + sit tab script minted). Leave `t13` `wP` `w15`.

## Success Criteria

- [ ] REVIEW_ACCEPT critical 0
- [ ] Paper commit pathspec-only; leftover 5 unstaged mint
- [ ] No push
- [ ] Wave tabs closed; `w13:t13` remains

## Risk Assessment

- Ship folds leftover via `git add scripts/` plus dirty rust. Signal: cached includes leftover 5. Response: unstage; abort; pathspec files only.
- Review calls sit PATH-pin / rust timeout a must-fix. Signal: out-of-scope finding. Response: Reject as cook (named unpaid / non-goal).
