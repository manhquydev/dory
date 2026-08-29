---
phase: 2
title: "Independent prd unlock test"
status: completed
priority: P1
effort: "45m"
dependencies: [1]
---

# Phase 2: Independent prd unlock test

## Overview

Chạy lại script trên sit mới. Đọc copied journal.

## Requirements

- Functional: exit 0 lần 2; journal `[1,0]`; unlock-3
- Non-functional: leftover mint; sock dead

## Implementation Steps

1. New testsit ≠ t13. Same SIT_DORY land sha.
2. Run script. Assert journal: FAIL `02-scope`; `unlocked stage 3 (flow/03-prd.md)`; reject stage 2; 03 sha == template.
3. TEST_PASS receipt.

## Success Criteria

- [x] TEST_PASS independent journal

## Risk Assessment

- Wait-output hits cook PASS leftover in scrollback. Response: require SIT_TAB testsit + new ISO name in footer.
