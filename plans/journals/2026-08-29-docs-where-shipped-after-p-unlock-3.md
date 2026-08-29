---
title: Docs WHERE shipped after P unlock-3
date: 2026-08-29
summary: Named-pathspec paper fc8cb6c; leftover 5 still mint; not Phase 5
---

# Docs WHERE shipped after P unlock-3

## What happened

`/ak-cook` remainder executed the Ship section of `plans/reports/260829-1204-docs-update-plan.md`. Named commit `fc8cb6c` `docs: route WHERE after isolate prd-unlock` (39 files). Parent `049e304`.

Landed: `docs/README.md`, CHARTER one WHERE pointer, `AGENTS.md` deny-list, 0054 paper `completed` 3/3, 1204 receipts, harvest, P journals, late P sit/hold/review receipts.

Tester 10/10 PASS (`260829-1331-cook-test.md`). Reviewer REVIEW_ACCEPT 9/10 critical 0 (`260829-1331-cook-review.md`). PM SYNC_OK (`260829-1331-cook-progress.md`).

## Decision

Docs paper only. Leftover 5 stay `M` mint `68190a5f…`. Rust log `b544f5f`. No `git add -A`. No push. Isolate P (`f1c966c`) is not company Phase 5.

## Next steps

Named unpaid still unpaid: default sit, fill 03, company Phase 5, leftover fold, `--wait`. Do not recook 0054. Do not fold leftover.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
