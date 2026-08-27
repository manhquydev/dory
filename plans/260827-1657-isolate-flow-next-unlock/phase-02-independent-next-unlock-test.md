---
phase: 2
title: "Independent next unlock test"
status: pending
priority: P1
effort: "45m"
dependencies: [1]
---

# Phase 2: Independent next unlock test

## Overview

Chạy lại script trên tab test, không tin cook. Đo journal + leftover + sock + `01-research` độc lập.

## Requirements

- Functional: script exit 0 lần 2; fail-then-pass journal; sit needles; unlock file existed
- Non-functional: leftover mint; no factory `dory`; no leftover cargo

## Architecture

Tab `dory-aoe5n-test` mint sit pane riêng (`SIT_PANE`/`SIT_TAB` mới). Cùng `SIT_DORY`. Không reuse ISO của cook (script tự mint + wipe). Đọc **copied** journal từ receipt — không cite cook receipt làm bằng.

## Related Code Files

- Exec: `scripts/dory-isolate-aoe5-flow-next.sh`
- Create: `plans/reports/260827-ensure-aoe5-flow-next-test.md`
- Do not modify rust / leftover 5 / cook receipt / paid judge script

## Implementation Steps

1. Refuse factory `DORY_*` / leftover-or-isolate ELF argv on factory XDG / leftover cargo / factory `flow/` mint. Sit env: `herdr pane get` `tab_id` ≠ `w13:t13`, pane ≠ `w13:p2R`/`*wP:*`, no agent on sit.
2. Mint test sit tab (`--no-focus`). Pass `SIT_PANE` `SIT_TAB` `SIT_DORY` (land sha `2ef20730…`).
3. Run script. Record rc. Read **copied** journal **and** copied 01 sha from receipt (script wipes ISO; do not read wiped path).
4. Independent asserts:
   - Copied journal has **exactly two** `flow/result`
   - both `bin` == abs skill `flow.sh` (not `/bin/true`, not bare `flow.sh`)
   - both `args` == `["next"]` (not `["gate","00-idea"]`)
   - codes `[1, 0]` in order
   - fail stdout has `FAIL: gate for stage 00-idea is not clean`; pass stdout has `unlocked stage 1 (flow/01-research.md)` (reject `already exists` / `unlocked stage 00` / `GATE stage` / bare `clean`)
   - copied `01-research.md` sha == `_templates/01-research.md`
   - leftover 5 = mint **path+sha** table; `desk.rs` == HEAD
   - sock `dory.sock` connectable=0
   - `type -a dory` empty (test tab **and** sit pane probe)
   - `git log -1 -- rust/` = `b544f5f` (worktree rust **được** dirty leftover — not `git diff` clean)
   - Sit hit + weak journal = TEST_FAIL
5. Write TEST_PASS or TEST_FAIL. Do not recook. Do not fill `01-research.md`. Do not `git checkout` leftover.

## Success Criteria

- [x] TEST_PASS
- [x] Independent journal measure (not copy-paste cook)
- [x] Leftover 5 mint; sock connectable=0

## Risk Assessment

- Cook ISO chưa wipe / sock isolate sống. Signal: second mint collide or factory XDG polluted. Response: script wipe + identity check; test FAIL if factory sock connectable.
- Sit needle timeout nhưng journal đúng. Signal: wait-output miss. Response: TEST_FAIL (acceptance includes sit). Read visible; do not recook `desk.rs`.
- Test cites cook `GATE`/`clean` strings. Signal: false PASS on recooked gate taxi. Response: require `args=["next"]` + `unlocked stage 1`.
