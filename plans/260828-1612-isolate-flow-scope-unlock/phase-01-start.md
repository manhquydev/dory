---
phase: 1
title: "New isolate scope-unlock script"
status: pending
priority: P1
effort: "2h"
dependencies: []
---

# Phase 1: New isolate scope-unlock script

## Overview

Viết và chạy một lần `scripts/dory-isolate-aoe5-flow-scope.sh`: isolate + đàn nhỏ + taxi `flow -- next` fail rồi pass. World-state `flow/02-scope.md`.

## Requirements

- Functional: script mới; mint 00 PASS + 01 template; taxi 1 rc=1; occupant viết PASS 01; taxi 2 rc=0; sit `Flow 1. next` rồi `Flow 0. next`; `02-scope.md` exists
- Non-functional: leftover 5 mint; factory sock connectable=0; không rust; `FLOW_HARNESS_DISABLE=1`; stop compound only

## Architecture

Factory Herdr mở tab sit sạch + tab cook. Script mint ISO dưới `~/.cache/dory-isolates/aoe5s.XXXXXX`. **`PROJECT=$ISO_REAL`**. Mint `$ISO_REAL/flow/00-idea.md` = PASS_00 (factory) + `$ISO_REAL/flow/01-research.md` từ template skill (FAIL). Isolate server: 0242 **`:340-353`**. Occupants: `coord` + `omptest` `--no-session --no-skills --no-rules --no-extensions`. Attach = 1910 `:331`. Copy-table only. Do **not** `source`/`.`/`exec` N / judge / 1910 / 0043 / 0227 / 0242 / hop.

### Taxi helper

```
taxi() {
  # env -u every factory FLOW_* first, then pin:
  (cd "$ISO_REAL" && \
    HOME="$ISO_REAL/home" DORY_SOCKET="$ISO_SOCK" DORY_ENV=1 \
    DORY_WORKSPACE_DIR="$ISO_REAL" FLOW_PROJECT_ROOT="$ISO_REAL" \
    FLOW_BIN="$FLOW_BIN" FLOW_LOG_DISABLE=1 FLOW_HARNESS_DISABLE=1 \
    DO_NOT_TRACK=1 \
    "$SIT_DORY" flow -- next)
}
```

### Journal helpers (do **not** copy N `:343-416` verbatim)

- taxi1: `code==1`; one `flow/result`; `bin`=abs flow.sh; `args == ["next"]`; stdout contains `FAIL: gate for stage 01-research is not clean`; reject `unlocked stage` / `GATE stage`
- After taxi1: 01 still FAIL; `! -f $ISO_REAL/flow/02-scope.md`; 00 still == PASS_00
- taxi2: `code==0`; exactly two `flow/result`; codes `[1,0]`; same `bin`; both `args == ["next"]`; stdout contains `unlocked stage 2 (flow/02-scope.md)`; reject `already exists` / `GATE stage` / `unlocked stage 1` / `unlocked stage 00` / bare `clean`
- After taxi2: `-f 02-scope.md` and sha == skill `_templates/02-scope.md` (still `[FILL]`). 00 still PASS. Do not fill 02.
- Taxi2 IFF `cmp -s` PASS 01 after occupant prompt.

### PASS 00 (factory mint — required)

Same fixture N occupant wrote for idea. Factory writes this **before** taxi1.

### PASS 01 (occupant writes; factory must not)

Mechanical: all gate boxes `[x]`; zero `[FILL`. Not semantic `gate-rules.md`.

## Related Code Files

- Create: `scripts/dory-isolate-aoe5-flow-scope.sh`
- Create: `plans/reports/260828-ensure-aoe5-flow-scope-cook.md`
- Copy-law only: N / judge / 1910 / 0242
- Do not modify: leftover 5, `rust/**`, paid isolate scripts

## Implementation Steps

1. Refuse: `HERDR_ENV!=1`; factory `DORY_*` / `PI_CODING_AGENT_DIR` / **any** `FLOW_*`; `HOME` != `FACTORY_HOME`; missing sit pins; `SIT_DORY` sha ≠ land; sit = `t13`/`p2R`/`wP`. Sit `type -a dory` empty.
2. Hash leftover 5 vs mint table. `desk.rs` == HEAD. Snap repo `.dory/`.
3. Mint ISO `aoe5s.XXXXXX`. Write PASS_00 to `00-idea.md`. `cp` 01 template. Refuse if 02 exists. Refuse if 01 == PASS_01. Refuse if 00 ≠ PASS_00.
4. Server 0242 `:340-353`. Occupants `--no-skills`. Taxi1 then attach sit `Flow 1. next`. Occupant PASS 01. Taxi2. Sit `Flow 0. next`.
5. Copy journal + 02 sha before wipe. Teardown. Write cook receipt.

## Success Criteria

- [ ] Script exists, executable, self-refuses N
- [ ] COOK_PASS once: codes `[1,0]`, unlock-2, leftover mint, sock dead

## Risk Assessment

- Recook N. Signal: taxi1 FAIL `00-idea` or taxi2 `unlocked stage 1`. Response: FAIL.
- Harness seed. Signal: taxi stdout intake / missing DISABLE. Response: pin + self-rg.
- Occupant stall. Signal: poll timeout. Response: one re-prompt; then FAIL. No `--wait`.
