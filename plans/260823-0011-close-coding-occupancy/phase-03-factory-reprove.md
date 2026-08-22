---
phase: 3
title: "Factory reprove"
status: completed
priority: P1
effort: "2-4h"
dependencies: [1, 2]
---

# Phase 3: Factory reprove

## Overview

If phase 1 passed, skip (this plan is already done). If phase 2 cooked, re-run the factory protocol once. This plan accepts only on a `pass` journal, not on green cargo tests alone.

## Requirements

- Functional: same world-state as phase 1 success criteria, using the binary/skill/argv that phase 2 produced.
- Non-functional: isolated XDG; no Herdr as Dory occupant control plane.

## Architecture

Same loop as phase 1. Argv must still be `-- omp --no-session` plus any **phase-2-allowed** extra omp flags (e.g. `--append-system-prompt`). No `--kind`.

## Related Code Files

- Create: `/tmp/dory-l4f2-evidence/`
- Create: `plans/reports/260823-layer4f2-omp-factory.md` (or dated equivalent)
- Must not touch: `flow-skill/`, classifier, `0847` phases

## Implementation Steps

1. If phase 1 journal is `pass` and phase 2 was N/A: mark this phase N/A and close the plan.
2. Otherwise repeat phase 1 protocol against the post-patch tree.
3. Start must stay `unknown`. Wait must settle `idle|done`.
4. Journal `pass` or `fail`. A second fail with a **new** cause is a replan, not another silent hole.

## Todo

- [x] Re-run or N/A documented
- [x] Journal pass (or N/A because phase 1 already passed)
- [x] Lead suite still green

## Success Criteria

- [x] Factory `status: pass` exists for the tree that will be cooked from
- [x] Start `unknown`; wait CLI timed out at 180s; `get-final` `done` after report; skill/report visible
- [x] Still not §11 (no real-repo Flow verdict required)

## Risk Assessment

- **Pass in CI fixtures, fail on omp** → do not score this plan complete from `p5_skill_occ` / `p5_prompt_unknown`.
- **New cause after patch** → stop; new brainstorm, do not stack patches.
- Pre-decided response: keep 1a HOLD; keep allowlist frozen.
