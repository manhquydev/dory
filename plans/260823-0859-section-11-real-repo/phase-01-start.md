---
phase: 1
title: "Fixture + test"
status: pending
priority: P1
effort: "3-6h"
dependencies: []
---

# Phase 1: Fixture + test

## Overview

Add `occ_s11` and `p5_s11.rs` per `260822-p5-s11-cook-brief.md`. No `rust/src`.

## Requirements

- Functional: named driver opens SKILL.md, types the skill loop, taxis Flow on a spec-kit clone.
- Non-functional: `--offline`; no `omp`; poll `pane read` (server is single-threaded).

## Related Code Files

- Create: `rust/tests/fixtures/occ_s11`
- Create: `rust/tests/p5_s11.rs`
- Read: `skills/dory/SKILL.md`, `p5_skill_occ.rs`, `p5_real_repo.rs`

## Implementation Steps

1. Fixture argv: `/bin/bash occ_s11 <clone> <SKILL.md> <FLOW_BIN>`. Print `DORY_OCC_READY` before any `$DORY_BIN` so start can return.
2. Test clones spec-kit with `git clone --no-hardlinks`. `agent start` the driver only.
3. Poll driver pane for `S11_FINISH`. Assert verbs, env, journal, foreign paths.

## Todo

- [x] `occ_s11` opens SKILL.md and issues listed verbs
- [x] `p5_s11` starts the driver; no `pane_run` loop
- [x] Clone + journal + no original `.dory`

## Success Criteria

- [x] New test exists and is the §11 candidate
- [x] No rust/src edit

## Risk Assessment

Deadlock if the test blocks `wait-output` while the driver calls `$DORY_BIN`. Response: poll `pane read` only.
