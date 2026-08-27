---
phase: 2
title: "Independent AOE 5 test"
status: pending
priority: P1
effort: "45m"
dependencies: [1]
---

# Phase 2: Independent AOE 5 test

## Overview

Chạy lại script trên tab test, không tin cook. Đo journal + leftover + sock độc lập.

## Requirements

- Functional: script exit 0 lần 2; fail-then-pass journal; sit needles
- Non-functional: leftover mint; no factory `dory`; no leftover cargo

## Architecture

Tab `dory-aoe5-test` mint sit pane riêng (`SIT_PANE`/`SIT_TAB` mới). Cùng `SIT_DORY`. Không reuse ISO của cook (script tự mint + wipe). Đọc journal bằng python trên `$PROJECT/.dory/sessions/s1.jsonl` — không cite cook receipt làm bằng.

## Related Code Files

- Exec: `scripts/dory-isolate-aoe5-flow-judge.sh`
- Create: `plans/reports/260827-ensure-aoe5-flow-judge-test.md`
- Do not modify rust / leftover 5 / cook receipt

## Implementation Steps

1. Refuse factory `DORY_*` / leftover-or-isolate ELF argv on factory XDG / leftover cargo.
2. Mint test sit tab (`--no-focus`). Pass `SIT_PANE` `SIT_TAB` `SIT_DORY`.
3. Run script. Record rc. Read **copied** journal from receipt (script wipes ISO; do not read wiped path).
4. Independent asserts:
   - Copied journal has **exactly two** `flow/result`
   - both `bin` == abs skill `flow.sh` (not `/bin/true`, not bare `flow.sh`, not `flow-skill` copy)
   - codes `[1, 0]` in order
   - fail stdout has `GATE stage 00-idea`; pass stdout has `clean`
   - leftover 5 = mint table; `desk.rs` == HEAD
   - sock `dory.sock` connectable=0
   - `type -a dory` empty
   - `git log -1 -- rust/` = `b544f5f` (worktree rust **được** dirty leftover)
5. Write TEST_PASS or TEST_FAIL. Do not recook.

## Success Criteria

- [ ] TEST_PASS
- [ ] Independent journal measure (not copy-paste cook)
- [ ] Leftover 5 mint; sock connectable=0

## Risk Assessment

- Cook ISO chưa wipe / sock isolate sống. Signal: second mint collide or factory XDG polluted. Response: script wipe + identity check; test FAIL if factory sock connectable.
- Sit needle timeout nhưng journal đúng. Signal: wait-output miss. Response: TEST_FAIL (acceptance includes sit). Read visible; do not recook `desk.rs`.
