---
phase: 2
title: "Prove section 11 table"
status: pending
priority: P1
effort: "1h"
dependencies: [1]
---

# Phase 2: Prove section 11 table

## Overview

Score §11 from the new test’s files. Keep `p5_real_repo` as not-§11.

## Requirements

- Every scout clause PASS. Suite green.

## Implementation Steps

1. `cargo test --offline --locked`
2. Write `plans/reports/260823-s11-table.md` from the test source + a passing run
3. `rg -i dory` on flow-skill = 0

## Todo

- [x] Suite green
- [x] Table journal PASS
- [x] flow-skill untouched

## Success Criteria

- [x] Stranger can mark Driver / DORY_ENV / verbs / foreign verdict / real repo / not-header as PASS
- [x] 0847 phases still paper

## Risk Assessment

Missing spec-kit path fails the test. Accepted. Do not fall back to copying `flow-skill/flow/`.
