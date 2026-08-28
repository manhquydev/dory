---
type: review
lens: fold
date: 2026-08-29
wave: dory-aoe5p
result: ACCEPT
critical: 0
---

# rv_fold — named ship only

Allow:

- `scripts/dory-isolate-aoe5-flow-prd.sh`
- `plans/260829-0054-isolate-flow-prd-unlock/*`
- `plans/260828-1612-isolate-flow-scope-unlock/plan.md` (pointer only, +3/−1)
- `plans/reports/260829-0052-*`
- `plans/reports/260829-0054-redteam-r1-*`
- `plans/reports/260829-0054-aoe5p-roster.md`
- `plans/reports/260829-ensure-aoe5-flow-prd-*`

Deny: leftover 5, rust/, O/N/judge script bodies, `260827-1743-eval-*`, `260827-1638-eval-*`, `.claude/`, `eval/`.

Critical: 0
