---
phase: 2
title: "Independent scope unlock test"
status: pending
priority: P1
effort: "45m"
dependencies: [1]
---

# Phase 2: Independent scope unlock test

## Overview

Chạy lại script trên tab test, không tin cook. Đo journal + leftover + sock + `02-scope` độc lập.

## Requirements

- Functional: script exit 0 lần 2; fail-then-pass journal; sit needles; unlock file existed
- Non-functional: leftover mint; no factory `dory`; no leftover cargo

## Architecture

Tab `dory-aoe5s-test` mint sit pane riêng. Cùng `SIT_DORY`. Không reuse ISO cook. Đọc **copied** journal từ receipt.

## Related Code Files

- Exec: `scripts/dory-isolate-aoe5-flow-scope.sh`
- Create: `plans/reports/260828-ensure-aoe5-flow-scope-test.md`
- Do not modify rust / leftover 5 / cook receipt / N / judge

## Implementation Steps

1. Refuse factory `DORY_*` / ELF on factory XDG / leftover cargo / factory `flow/`. Sit ≠ `t13`/`p2R`/`wP`.
2. Mint test sit tab (`--no-focus`). Pass `SIT_PANE` `SIT_TAB` `SIT_DORY` (land sha `2ef20730…`).
3. Run script. Read **copied** journal **and** copied 02 sha (ISO wiped).
4. Independent asserts:
   - Exactly two `flow/result`
   - both `bin` == abs skill `flow.sh`
   - both `args` == `["next"]`
   - codes `[1, 0]`
   - fail stdout `FAIL: gate for stage 01-research is not clean`; pass stdout `unlocked stage 2 (flow/02-scope.md)`
   - reject `already exists` / `unlocked stage 1` / `unlocked stage 00` / `GATE stage` / bare `clean`
   - copied `02-scope.md` sha == `_templates/02-scope.md`
   - leftover 5 mint; `desk.rs` == HEAD; sock connectable=0; PATH empty
   - `git log -1 -- rust/` = `b544f5f`
   - Sit hit + weak journal = TEST_FAIL
5. Write TEST_PASS or TEST_FAIL. Do not fill `02-scope.md`.

## Success Criteria

- [ ] TEST_PASS
- [ ] Independent journal measure
- [ ] Leftover 5 mint; sock connectable=0

## Risk Assessment

- Cook ISO chưa wipe. Signal: factory sock connectable. Response: TEST_FAIL.
- Test cites N `unlocked stage 1`. Signal: false PASS. Response: require stage 2.
