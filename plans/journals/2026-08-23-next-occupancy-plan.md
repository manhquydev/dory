---
title: Next occupancy plan
date: 2026-08-23
summary: Locked next cook to factory 4f then one hole; new plan 260823-0011; 0847 phases not flipped.
---

# Next occupancy plan

## What happened

Brainstorm reused the occupant-lock contract. Next slice is coding occupancy, not §11 and not 1a.

Factory 4b/4d FAIL still stands. 4c (bracketed-paste) and 4e (no stall on unknown) shipped after 4d, so 4f must re-run before any rust.

## Decision

New plan `plans/260823-0011-close-coding-occupancy/` (3 phases). Do not flip `260822-0847-workplace-skill-mux` from paper. Allowed post-fail patches are submit / skill-load argv / skill text only. Classifier frozen.

## Next steps

Cook phase 1 factory 4f, or validate the plan first.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
