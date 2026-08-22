---
phase: 1
title: "Factory 4f"
status: completed
priority: P1
effort: "2-4h"
dependencies: []
---

# Phase 1: Factory 4f

## Overview

Re-run the isolated Dory factory **after** 4c + 4e, with `omp --no-session`. Produce a journal scored from files. Do not edit `rust/src` in this phase.

## Requirements

- Functional: start `unknown`; prompt via `$DORY agent prompt`; wait `idle|done` or a single named FAIL cause.
- Non-functional: isolated `XDG_RUNTIME_DIR`; teardown `dory server stop` only.

## Architecture

Harness (same-uid `DORY_ENV=1`) creates layout and starts argv. Occupancy proof is the **pane occupant**, not the harness. Classifier stays `sleep|cat|sh|bash|true|false`.

## Related Code Files

- Read: `skills/dory/SKILL.md`
- Read: `rust/src/server.rs` (`agent_prompt`, `live_bracketed_paste`, stall on `Idle|Done` only)
- Read: `plans/reports/260822-2010-layer4b-omp-factory-brief.md` (protocol)
- Create: `/tmp/dory-l4f-evidence/`
- Create: `plans/reports/260823-layer4f-omp-factory.md`

## Implementation Steps

1. Isolated `XDG_RUNTIME_DIR`; `dory` binary `rust/target/debug/dory`; cwd `/home/manhquy/Downloads/flow/dory`.
2. `dory server`; `workspace create`; parse `.result.root_pane.id` (never invent `w1`).
3. `dory agent start coder --pane <root> --timeout 45000 -- omp --no-session`
4. Assert start `unknown`. If `idle`/`done` without report → FAIL classifier leak; stop.
5. `dory agent prompt coder --timeout 180000 --` with env-gate + read `skills/dory/SKILL.md` + `dory agent report --current --state idle`.
6. Prompt must **not** be `agent_prompt_stalled` (4e). If it is, journal that as the cause; do not patch here.
7. `dory agent wait coder --timeout 180000`
8. Write evidence (`start.json`, `prompt.json`, `wait.json`, `get-final.json`, `read.txt`). Teardown server. Journal `pass` or `fail` with **one** cause: `submit` | `skill-load` | `occupant-refused` | `stall` | `classifier-leak`.

## Todo

- [x] Isolated server + start `-- omp --no-session` → `unknown`
- [x] Prompt is Dory CLI; no stall
- [x] Wait settle or one named cause
- [x] Journal `plans/reports/260823-layer4f-omp-factory.md`

## Success Criteria

- [x] Journal exists and quotes evidence paths (no secrets, no full environ)
- [x] One sentence: not §11
- [x] No rust/src edit in this phase
- [x] `cargo test --offline --locked` still green (unchanged tree)

## Risk Assessment

- **omp splash / paste not a turn** → cause `submit`. Phase 2 may add Enter after BP. Signal: prompt text in TUI, no model turn, no report.
- **Skill never opened** → cause `skill-load`. Phase 2 may add `--append-system-prompt @SKILL.md` on argv. Signal: turn ran, no phrase, no `dory agent report`.
- **Model ignores skill** → cause `occupant-refused`. Phase 2 may tighten skill text only. Signal: occupant replies but never runs report.
- Pre-decided: do not allowlist; do not `--kind`; do not cook 1a.
