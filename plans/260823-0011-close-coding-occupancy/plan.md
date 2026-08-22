---
title: "Close coding occupancy"
description: "Factory-prove a PATH omp occupant reports via skills/dory; patch only the hole that journal names. Not contract §11."
status: completed
priority: P1
effort: "1-2d"
tags: [dory, occupancy, factory]
blockedBy: []
blocks: []
created: 2026-08-23
authority:
  - CHARTER.md
  - plans/reports/260822-skill-cli-socket-contract.md
  - plans/reports/260823-0009-brainstorm-next-occupancy-scope.md
  - plans/reports/260822-1942-brainstorm-occupant-lock.md
---

# Close coding occupancy

## Overview

Close the **coding occupant** hole the 4b/4d factory proved: `omp` on PATH starts (`unknown`), but `wait` never settles. Layers 4c (live bracketed-paste) and 4e (no stall on `unknown`) shipped in CI **after** 4d. This plan factory-retries first, then cooks **at most one** cause-aligned hole, then factory-reproves.

Does **not** replace `plans/260822-0847-workplace-skill-mux` (leave its phase files `todo` on paper). Does **not** unhold 1a spec-kit. Does **not** claim contract §11.

```text
isolated dory server
  → workspace create (harness DORY_ENV=1)
  → agent start coder --pane <id> -- omp --no-session
  → start state unknown
  → dory agent prompt (BP wrap if CSI ? 2004 h)
  → occupant opens skills/dory + report --current --state idle
  → agent wait → idle|done
```

## Goals

| # | Goal | Priority |
|---|------|----------|
| 1 | Factory journal `status: pass`: start `unknown`, wait `idle\|done`, skill phrase or report command on transcript | P1 |
| 2 | If 4f fails: one named cause, one patch, no classifier farm | P1 |
| 3 | `cargo test --offline --locked` stays green; no `omp` in cargo tests | P1 |

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Factory 4f](./phase-01-start.md) | Completed |
| 2 | [Cause-aligned hole](./phase-02-cause-aligned-hole.md) | Completed |
| 3 | [Factory reprove](./phase-03-factory-reprove.md) | Completed |

Phase 1 journal was `fail` (`submit`). Phase 2 + 3 cooked/reproved; not skipped.

## Success Criteria

Evidence: `260823-layer4f-omp-factory.md` (FAIL `submit`); `260823-layer4-submit-cr.md` (CR after BP); `260823-layer4f2-omp-factory.md` (PASS).

- [x] Isolated factory session; not the Herdr socket; no `herdr server stop`
- [x] Start argv is `-- omp --no-session` (no `--kind`)
- [x] Start state `unknown` (classifier leak if `idle`/`done` without report)
- [x] Prompt CLI is `dory agent prompt`, not Herdr
- [x] Wait CLI timed out at 180s; `get-final` was `done` after occupant `report` (4f2). Occupancy proven; 180s tight for omp tool spinner. Not a new hole. Not wait-CLI settle.
- [x] Transcript shows skill first-action phrase or the report example from `skills/dory/SKILL.md`
- [x] `rg -i dory` on `flow-skill` still 0; no `.dory` written into `flow-skill`
- [x] Full suite green without spawning `omp`

## Refuse

`--kind`. Allowlist `omp`. Xia `--copy`. `dory flow` / spec-kit clone. Flip `0847` phase markdown from paper. Node `/workplace`. Binary coding-agent Dory.

## Related

- Brainstorm: `plans/reports/260823-0009-brainstorm-next-occupancy-scope.md`
- Prior factory FAIL: `plans/reports/260822-2011-layer4b-omp-factory.md`, `260822-2031-layer4d-omp-factory.md`
- This-slice factory: `plans/reports/260823-layer4f-omp-factory.md` (FAIL submit), `260823-layer4f2-omp-factory.md` (PASS get=done)
- Historical mux plan (do not flip): `plans/260822-0847-workplace-skill-mux/`
- Next plan **after** this one accepts: contract §11 on a real external repo. Not 1a unhold as a substitute.

<!-- slug: close-coding-occupancy -->
