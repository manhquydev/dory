---
phase: 2
title: "Cause-aligned hole"
status: completed
priority: P1
effort: "2-6h"
dependencies: [1]
---

# Phase 2: Cause-aligned hole

## Overview

Cook **only** if phase 1 journal is `fail` with one cause. Skip entirely if phase 1 is `pass`. One hole. No classifier table change.

## Requirements

- Functional: the named cause from `260823-layer4f-omp-factory.md` is addressed by exactly one of the allowed patches below.
- Non-functional: new CI must not exec `omp`; full suite stays green.

## Architecture

Decision table (do not invent a fourth class):

| Cause | Allowed patch | Forbidden |
|---|---|---|
| `submit` | After BP wrap, Enter as `\r` or extra `send-keys enter`; keep 4c detect | Allowlist; `--kind` |
| `skill-load` | Factory/start argv: `omp --no-session --append-system-prompt @<abs/skills/dory/SKILL.md>`. Inject `DORY_SKILL` at PTY spawn **only if** argv path already failed once | Classifier; copy Xia |
| `occupant-refused` | Skill text only: first action after gate remains `report --current --state idle` | New Dory agent binary |
| `stall` | Re-open 4e (should already skip `unknown`) | Treat as occupancy |
| `classifier-leak` | Stop. Do not “fix” by expanding allowlist | Any comm add |

## Related Code Files

- Modify (only the chosen row): `rust/src/server.rs` **or** `skills/dory/SKILL.md` **or** factory argv in the phase 3 brief
- Tests if rust: follow existing harness (`p5_prompt_paste.rs` / `p5_prompt_unknown.rs` / `p5_skill_occ.rs`)
- Must not touch: `flow-skill/`, `comm_allowlisted`, `0847` phase files

## Implementation Steps

1. Read the phase 1 journal cause. If `pass`, write `N/A` on this file’s todos and stop.
2. Implement the single allowed patch for that cause.
3. Isolated test for that patch (no `omp`).
4. Lead: `cargo test --offline --locked`

## Todo

- [x] Cause copied verbatim from phase 1 journal (or N/A)
- [x] One allowed patch landed or explicitly skipped
- [x] Suite green; allowlist unchanged

## Success Criteria

- [x] Diff matches one row of the table
- [x] No `--kind`; no `omp` in cargo tests
- [x] Phase 3 is unblocked (or N/A because phase 1 passed)

## Risk Assessment

- **Journal names two causes** → stop and re-run a narrower 4f; do not dual-patch.
- **Cook wants `DORY_SKILL` first** → reject unless argv `--append-system-prompt` already failed in evidence.
- Signal it broke: suite red or factory still `unknown` after wait with the same transcript shape. Response: revert the hole, do not add a second hole in the same cook.
