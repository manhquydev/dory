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

Review tab 4 panes (≤6). Ship tab 1 pane. Named-file `git add --` only. **Ban `ak:git`**. Ban glob. Ban `git add -A`/`-u`.

### Review lenses

| Name | Accept iff |
|---|---|
| `rv_next` | Taxi used abs flow.sh `flow -- next`; codes 1 then 0; both `args == ["next"]`; stdout FAIL then `unlocked stage 1 (flow/01-research.md)`; occupant wrote PASS; copied 01 sha == template; not `/bin/true`/`gate`/`status`/`doctor`/`already exists` |
| `rv_left` | Leftover 5 **path+sha** mint MATCH; leftover ELF sha `3ba0e3bc…` unchanged; `desk.rs` == HEAD; `git log -1 -- rust/` = `b544f5f`; no leftover cargo. Worktree rust dirty leftover = **pass**. Never checkout leftover. |
| `rv_sit` | `herdr pane get` `tab_id` ≠ `w13:t13`; pane ≠ `w13:p2R`/`*wP:*`; `SIT_DORY` sha `2ef20730…`; sit pane `type -a dory` empty; stop = compound_stop `:69-100`; factory sock dead; no leftover/isolate ELF argv on factory XDG; attach 1910 `:331` |
| `rv_fold` | Cached ⊆ named allowlist; leftover 5 + `README.md` + `rust/**` + `260827-1638-eval-*` never staged; paid judge not modified |

### Ship named files ONLY (no glob)

```
scripts/dory-isolate-aoe5-flow-next.sh
plans/260827-1657-isolate-flow-next-unlock/plan.md
plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md
plans/260827-1657-isolate-flow-next-unlock/phase-02-independent-next-unlock-test.md
plans/260827-1657-isolate-flow-next-unlock/phase-03-review-and-ship.md
plans/reports/260827-1656-research-01-flow-next.md
plans/reports/260827-1656-research-02-isolate-next.md
plans/reports/260827-1657-redteam-r1-security.md
plans/reports/260827-1657-redteam-r1-failure.md
plans/reports/260827-1657-redteam-r1-fold.md
plans/reports/260827-ensure-aoe5-flow-next-cook.md
plans/reports/260827-ensure-aoe5-flow-next-test.md
plans/reports/260827-ensure-aoe5-flow-next-review.md
plans/reports/260827-ensure-aoe5-flow-next-ops.md
plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl
```

Ban `git add -u` / `git add -A` / leftover 5 / `README.md` / `rust/**` / paid judge / `260827-1638-eval-*` / `ak:git`. `git add --` từng path đã tồn tại. 1650 accept-paper không bắt buộc trên ship này.

Message:

```
feat(isolate): fail-then-pass flow.sh next
```

Paper-only. Do not push. After commit: leftover 5 still ` M` mint; `git log -1 -- rust/` still `b544f5f`; sock connectable=0. Subject ≠ company Phase 5.

Write `plans/reports/260827-ensure-aoe5-flow-next-ops.md`.

## Related Code Files

- Review: `plans/reports/260827-ensure-aoe5-flow-next-review.md` (+ per-lens)
- Ship: ops receipt
- Do not modify leftover 5 / rust / paid isolate scripts

## Implementation Steps

1. Four review agents. Critical 0 required.
2. `git diff --cached --name-only` ⊆ pathspec.
3. Commit. Confirm leftover porcelain ` M` ×5.
4. Close wave tabs only (`dory-aoe5n-*` + sit tab script minted). Leave `t13` `wP` `w15`.

## Success Criteria

- [ ] REVIEW_ACCEPT critical 0
- [ ] Paper commit pathspec-only; leftover 5 unstaged mint
- [ ] No push
- [ ] Wave tabs closed; `w13:t13` remains

## Risk Assessment

- Ship folds leftover via `git add scripts/` plus dirty rust. Signal: cached includes leftover 5. Response: unstage; abort; pathspec files only.
- Review calls sit PATH-pin / rust timeout / fill-01 a must-fix. Signal: out-of-scope finding. Response: Reject as cook (named unpaid / non-goal).
- Subject says AOE 5 / Phase 5 done. Signal: pretends company. Response: use `feat(isolate): fail-then-pass flow.sh next`.
