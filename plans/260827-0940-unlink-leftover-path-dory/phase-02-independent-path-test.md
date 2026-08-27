---
phase: 2
title: "Independent PATH test"
status: pending
priority: P1
effort: "15m"
dependencies: [1]
---

# Phase 2: Independent PATH test

## Overview

A second `w13` tab (id ≠ cook, ≠ `t13`) re-proves A. No unlink. No `dory` invoke. No exec leftover/isolate ELF.

## Requirements

- Functional: independent `TEST_PASS` iff A asserts hold.
- Non-functional: do not mutate PATH; do not cargo; do not sit `t13`.

## Architecture

Read-only re-measure vs cook receipt + researcher-02 mint.

## Related Code Files

- Read: cook receipt + leftover hashes
- Create: `plans/reports/260827-unlink-path-leftover-test.md`
- Do not modify rust, leftover 5, `scripts/`

## Implementation Steps

1. New `w13` tab. `--no-focus`. cwd dory repo. Record `TEST_TAB` / `TEST_PANE`. Must differ from `COOK_TAB`. Not a split of cook. Not `t13`.
2. Refuse if `DORY_SOCKET` / `DORY_ENV` / `DORY_RECYCLE` / `PI_CODING_AGENT_DIR` set. Snapshot `XDG_RUNTIME_DIR`.
3. Do **not** invoke `dory` / attach / server / leftover ELF / isolate ELF. `strings` / `sha256sum` / `cmp` on binaries **allowed**.
4. **A asserts** (all required for PASS):
   1. `test ! -e ~/.local/bin/dory` and `test ! -L ~/.local/bin/dory`
   2. `hash -r`; `type -a dory` empty; PATH walk finds no `dory` name
   3. Leftover 5 `git hash-object` == researcher-02 full SHAs
   4. `"$XDG_RUNTIME_DIR/dory/default/dory.sock"` not connectable
   5. Leftover ELF exists; sha matches cook before-snapshot
5. **Observe-only** (do not FAIL A): `desk.rs` still `4c788562…`; isolate `land-4b70f79` debug exists; `git rev-parse HEAD` (paper ship may move later).
6. Write test receipt `TEST_PASS` or `TEST_FAIL` with the five A asserts.

## Success Criteria

- [x] `TEST_TAB` ≠ `COOK_TAB` ≠ `t13`
- [x] Five A asserts true
- [x] Test receipt on disk
- [x] No factory `dory`/ELF argv; sock still not connectable

## Risk Assessment

| Risk | Signal | Response |
|---|---|---|
| Test retargets | `ln` / PATH `dory` | FAIL. |
| Test exec leftover | process leftover ELF | FAIL. |
| Count skip | missing assert 3 or 5 | FAIL. |

## Next Steps

Phase 3 review + ship.
