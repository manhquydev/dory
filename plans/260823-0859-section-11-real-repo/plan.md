---
title: "Section 11 real repo"
description: "Named inside occupant reads skills/dory and taxis Flow on a spec-kit clone. Not a flow-skill copy."
status: completed
priority: P1
effort: "4-8h"
tags: [dory, s11, flow-taxi]
blockedBy: []
blocks: []
created: 2026-08-23
authority:
  - CHARTER.md
  - plans/reports/260822-skill-cli-socket-contract.md
  - plans/reports/260822-p5-s11-cook-brief.md
  - plans/reports/260823-0856-brainstorm-s11.md
---

# Section 11 real repo

## Overview

Close contract §11. Occupancy is already proven. Unhold 1a: a `/bin/bash` fixture that **opens** `skills/dory/SKILL.md` drives split / start / prompt / wait / `flow -- status` on a temp clone of `/home/manhquy/Downloads/spec-kit`.

`p5_real_repo` stays history (copied `flow-skill/flow/` + harness `pane_run`).

## Goals

| # | Goal | Priority |
|---|------|----------|
| 1 | `p5_s11` world-state fills the §11 table | P1 |
| 2 | Suite `cargo test --offline --locked` green; no `omp` | P1 |

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Fixture + test](./phase-01-start.md) | Pending |
| 2 | [Prove table](./phase-02-prove-section-11-table.md) | Pending |

## Success Criteria

- [ ] Driver is the named occupant; test does not `pane_run` the loop
- [ ] `printenv DORY_ENV` → `1` on that pane
- [ ] split / start / prompt / wait / flow issued via `$DORY_BIN` from skill verbs
- [ ] Journal `{clone}/.dory/sessions/s1.jsonl` has `flow/invoke` + `flow/result`
- [ ] Clone is git from spec-kit, not `flow-skill/flow/` copy, not `eval/phase5-project`
- [ ] Original spec-kit has no `.dory`; `rg -i dory` on flow-skill is 0
- [ ] No rust/src; classifier frozen; no `--kind`

## Refuse

`--kind`. Allowlist `omp`. `omp` in cargo. Xia `--copy`. Flip `0847` paper. Copy `flow-skill` into the clone. Factory omp this slice.

<!-- slug: section-11-real-repo -->
