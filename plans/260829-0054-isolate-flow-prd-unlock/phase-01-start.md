---
phase: 1
title: "New isolate prd-unlock script"
status: pending
priority: P1
effort: "2h"
dependencies: []
---

# Phase 1: New isolate prd-unlock script

## Overview

Viết và chạy một lần `scripts/dory-isolate-aoe5-flow-prd.sh`. World-state `flow/03-prd.md`.

## Requirements

- Functional: mint 00+01 PASS + 02 template; taxi1 rc=1; occupant PASS 02; taxi2 rc=0; sit needles; 03 exists
- Non-functional: leftover mint; sock dead; `FLOW_HARNESS_DISABLE=1`; không rust

## Architecture

ISO `aoe5p.XXXXXX`. Copy-table from O. Do not source O. Occupants `--no-skills`. Taxi `flow -- next`.

Journal: taxi1 FAIL `02-scope`; taxi2 `unlocked stage 3 (flow/03-prd.md)`. 03 sha == `_templates/03-prd.md`.

## Related Code Files

- Create: `scripts/dory-isolate-aoe5-flow-prd.sh`
- Copy-law: `scripts/dory-isolate-aoe5-flow-scope.sh`
- Do not modify leftover 5 / rust / O/N/judge

## Implementation Steps

1. Refuse HERDR / factory FLOW_* / DORY_* / sit t13 / land sha miss.
2. Mint ISO. Write PASS_00 + PASS_01. `cp` 02 template. Refuse 03 exists. Refuse 02 == PASS_02.
3. Server 0242. Occupants. Taxi1. Attach sit. Occupant PASS 02. Taxi2. Copy journal + 03 sha. Wipe.
4. COOK_PASS receipt.

## Success Criteria

- [ ] Script executable, self-refuses O
- [ ] COOK_PASS once

## Risk Assessment

- Recook O. Signal: taxi1 FAIL `01-research` or taxi2 stage 2. Response: FAIL.
- Occupant stall. One re-prompt; then FAIL. No `--wait`.
