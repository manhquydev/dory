---
phase: 3
title: "Review and ship"
status: completed
priority: P1
effort: "45m"
dependencies: [2]
---

# Phase 3: Review and ship

## Overview

Bốn lens rồi pathspec commit. Không push. Không leftover 5.

## Review lenses

| Name | Accept iff |
|---|---|
| rv_next | FAIL `02-scope` then `unlocked stage 3`; 03 sha == template; harness pin; not O unlock-2 |
| rv_left | leftover 5 mint; rust log `b544f5f` |
| rv_sit | sit ≠ t13/p2R/wP; land sha `2ef20730…` |
| rv_fold | named files only; no leftover 5; no O/N script body |

## Ship named files ONLY

```
scripts/dory-isolate-aoe5-flow-prd.sh
plans/260829-0054-isolate-flow-prd-unlock/{plan,phase-01,phase-02,phase-03}.md
plans/260828-1612-isolate-flow-scope-unlock/plan.md
plans/reports/260829-0052-*
plans/reports/260829-0054-redteam-r1-*
plans/reports/260829-0054-aoe5p-roster.md
plans/reports/260829-ensure-aoe5-flow-prd-*
```

Message: `feat(isolate): fail-then-pass flow.sh prd`

Ban `ak:git`. Ban 1743 eval. Ban leftover 5.

## Success Criteria

- [x] REVIEW_ACCEPT critical 0
- [x] Paper commit pathspec-only

## Risk Assessment

- Cached includes leftover or O script. Abort.
