---
title: Occupancy cook closed
date: 2026-08-23
summary: 4f FAIL submit; CR after BP; 4f2 get=done. Wait 180s timed out. Plan 260823-0011 completed. Not §11.
---

# Occupancy cook closed

## What happened

Factory 4f after 4c+4e: prompt ok, no stall; text sat in omp compose; wait timeout; get unknown. Cause submit.

Phase 2: agent_prompt live BP ends with CR. send-keys enter stays LF. Suite 98/98.

Factory 4f2: splash then prompt; occupant ran report; wait 180s timeout; get-final done.

## Decision

Score occupancy pass on get+report, not wait CLI. Classifier frozen. Next product plan is §11, not 1a substitute.

## Next steps

User commit choice. Then a new plan for contract §11 on a real external repo.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
