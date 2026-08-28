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

Review tab 4 panes (≤6). Named-file `git add --` only. **Ban `ak:git`**. Ban glob.

### Review lenses

| Name | Accept iff |
|---|---|
| `rv_next` | Taxi abs flow.sh `flow -- next`; codes 1 then 0; both `args == ["next"]`; stdout FAIL `01-research` then `unlocked stage 2 (flow/02-scope.md)`; occupant wrote PASS 01; copied 02 sha == template; `FLOW_HARNESS_DISABLE=1`; not `/bin/true`/`gate`/N unlock-1 |
| `rv_left` | Leftover 5 path+sha mint; leftover ELF `3ba0e3bc…`; `desk.rs` == HEAD; rust log `b544f5f`; no leftover cargo |
| `rv_sit` | sit ≠ `t13`/`p2R`/`wP`; `SIT_DORY` sha `2ef20730…`; sit PATH empty; compound_stop; factory sock dead |
| `rv_fold` | Cached ⊆ named allowlist; leftover 5 + rust + N script + `260827-1638-eval-*` + `260827-1743-eval-*` never staged |

### Ship named files ONLY (no glob)

```
scripts/dory-isolate-aoe5-flow-scope.sh
plans/260828-1612-isolate-flow-scope-unlock/plan.md
plans/260828-1612-isolate-flow-scope-unlock/phase-01-start.md
plans/260828-1612-isolate-flow-scope-unlock/phase-02-independent-scope-unlock-test.md
plans/260828-1612-isolate-flow-scope-unlock/phase-03-review-and-ship.md
plans/260827-1657-isolate-flow-next-unlock/plan.md
plans/reports/260828-1612-research-01-flow-scope.md
plans/reports/260828-1612-research-02-isolate-scope.md
plans/reports/260828-1612-redteam-r1-security.md
plans/reports/260828-1612-redteam-r1-failure.md
plans/reports/260828-1612-redteam-r1-fold.md
plans/reports/260828-1612-aoe5s-roster.md
plans/reports/260828-ensure-aoe5-flow-scope-*
```

Ban leftover 5 / rust / N script body / judge / 1638 / 1743 eval. Message:

```
feat(isolate): fail-then-pass flow.sh scope
```

Do not push. After commit: leftover 5 still ` M`; rust log `b544f5f`.

## Related Code Files

- Review: `plans/reports/260828-ensure-aoe5-flow-scope-review.md`
- Ship: ops receipt
- Update pointer only: `plans/260827-1657-isolate-flow-next-unlock/plan.md` remainder

## Implementation Steps

1. Four review agents. Critical 0 required.
2. `git diff --cached --name-only` ⊆ pathspec.
3. Commit. Confirm leftover porcelain ` M` ×5.
4. Close wave tabs only (`dory-aoe5s-*`). Leave `t13` `wP` `w15`.

## Success Criteria

- [ ] REVIEW_ACCEPT critical 0
- [ ] Paper commit pathspec-only; leftover 5 unstaged mint
- [ ] No push

## Risk Assessment

- Ship folds leftover or N script. Signal: cached includes leftover 5 or next.sh. Response: abort.
- Subject says Phase 5. Response: use `feat(isolate): fail-then-pass flow.sh scope`.
