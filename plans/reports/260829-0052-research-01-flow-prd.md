---
type: research
date: 2026-08-29
time: 00:52
scope: flow.sh cmd_next 02 → 03
---

# Research Report: flow.sh next unlocks PRD

## Executive Summary

`current_stage_idx` is contiguous files from `00-idea`. With `00`+`01`+`02` present, idx = **2**. `cmd_next` `scan_gate`s **`02-scope.md`**. Dirty 02 → `FAIL: gate for stage 02-scope is not clean.` rc=1. Clean + missing 03 → `cp` `_templates/03-prd.md` + `PASS: … unlocked stage 3 (flow/03-prd.md)`.

This is the same mechanical án as N and O, one stage later. Not semantic `gate-rules.md`. Not `card`. `gate_durable_hook` after unlock runs on **02-scope** (just-passed). Hook cases are only `01-research` and `04-adr` (`flow.sh:660-671`) — 02 is a no-op even if harness is on. Still pin `FLOW_HARNESS_DISABLE=1` (copy-law, refuse factory `FLOW_*`).

## Research Methodology

- Sources: `flow.sh` runner, `_templates/02-scope.md`, `_templates/03-prd.md`, `HEAD:rust/src/flow.rs`, O journal `260828-ensure-aoe5-flow-scope-journal.jsonl`
- Date: 2026-08-29. In-repo authority. No external web (project-local taxi law).

## Key Findings

### 1. Technology Overview

`STAGES="00-idea 01-research 02-scope 03-prd 04-adr 05-contract"` `LAST_STAGE_IDX=5` (`flow.sh:122-123`). Unlock is `cp` template + stdout needle. Dory rust has no `next` verb (`git show HEAD:rust/src/flow.rs:3`). Taxi remains `dory flow -- next`.

### 2. Current State

O paid unlock-2: 02 sha == template `[FILL]`. Factory has no `flow/`. Next world-state after O is **template `03-prd.md`**, not a filled PRD.

### 3. Best Practices (this repo)

Mint honest prior stages: 00 PASS + 01 PASS + 02 template. Assert 00/01 still PASS after both taxis (idx=2 only scans 02; dirty 00 would still unlock 03).

### 4. Security

Refuse factory `FLOW_*`. Pin `FLOW_HARNESS_DISABLE=1`. No factory ELF. No default sock.

### 5. Performance

`/tmp`-class `next` ≪ 15s taxi timeout. Occupant poll ~180s is the long pole.

## Comparative Analysis

| Approach | Verdict |
|---|---|
| Isolate taxi `next` fail-then-pass → `03-prd.md` | **P** — project advance, same rung as O |
| Recook O | Paid. Ban. |
| Fill 02 semantically + `card` | Company. Unpaid blob. |
| Skill taxi paper | Wrong altitude. |

## Implementation Recommendations

Taxi argv unchanged: `"$SIT_DORY" flow -- next`. Journal taxi1 FAIL `02-scope`. Taxi2 `unlocked stage 3 (flow/03-prd.md)`. Reject `unlocked stage 2` / `already exists`. 03 sha == template. Do not fill 03.

## Resources

- `flow.sh:136-153` idx; `:157-175` scan_gate; `:950-1030` cmd_next; `:1019-1026` already-exists / unlock
- `_templates/03-prd.md` exists (boxes + `[FILL`)
- O script copy-law: `scripts/dory-isolate-aoe5-flow-scope.sh`
