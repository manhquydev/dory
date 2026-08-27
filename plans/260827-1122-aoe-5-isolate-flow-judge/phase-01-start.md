---
phase: 1
title: "New isolate AOE 5 script"
status: pending
priority: P1
effort: "2h"
dependencies: []
---

# Phase 1: New isolate AOE 5 script

## Overview

Viết và chạy một lần `scripts/dory-isolate-aoe5-flow-judge.sh`: isolate + đàn nhỏ + taxi `gate 00-idea` fail rồi pass.

## Requirements

- Functional: script mới; mint FAIL idea; taxi 1 rc=1; occupant viết PASS; taxi 2 rc=0; sit `Flow 1. gate` rồi `Flow 0. gate`
- Non-functional: leftover 5 mint; factory sock connectable=0; không rust; stop compound only

## Architecture

Factory Herdr mở tab sit sạch + tab cook. Script mint ISO dưới `~/.cache/dory-isolates/aoe5.XXXXXX` (prefix mới, không `flock.6yaatuxg`). **`PROJECT=$ISO_REAL`** (không `hello/`): glance + taxi journal cùng `{ISO_REAL}/.dory/sessions/s1.jsonl` vì server `cd "$ISO_REAL"` → `world.cwd`. Mint `$ISO_REAL/flow/00-idea.md` từ template skill (FAIL). Isolate server: 0242 `:346-353` (`HOME="$ISO_REAL/home"`, `XDG_RUNTIME_DIR=$ISO_REAL`, `PI_CODING_AGENT_DIR` **chỉ** dòng server). Occupants: `coord` + `omptest`. Taxi helper + `HOME="$ISO_REAL/home"`. Attach = 1910 `:331`.

### Taxi helper (copy into script)

```
taxi() {
  (cd "$ISO_REAL" && \
    HOME="$ISO_REAL/home" DORY_SOCKET="$ISO_SOCK" DORY_ENV=1 \
    DORY_WORKSPACE_DIR="$ISO_REAL" FLOW_PROJECT_ROOT="$ISO_REAL" \
    FLOW_BIN="$FLOW_BIN" FLOW_LOG_DISABLE=1 DO_NOT_TRACK=1 \
    "$SIT_DORY" flow -- gate 00-idea)
}
```

### PASS file bytes (occupant writes; factory must not)

Exact bytes (probe /tmp 11:22, `gate 00-idea` rc=0):

```
# Stage 00 — Idea

## Gate — check ALL before `/flow next`
- [x] The pitch below is 3 sentences, no more
- [x] I can name at least ONE real person/group who has this pain (named below)
- [x] No FILL placeholders remain in this file

## Pitch (3 sentences: who, pain, what you'd build)

Operators sitting Dory cannot prove Flow judged a real project.
1910 /bin/true always exits 0 and never reads files.
This fixture is a one-stage idea so Flow can fail then pass.

## One real person/group with this pain

Founder sitting the Dory desk after isolate flock.
```

## Related Code Files

- Create: `scripts/dory-isolate-aoe5-flow-judge.sh`
- Create: `plans/reports/260827-ensure-aoe5-flow-judge-cook.md`
- Copy-law only (do not exec): `scripts/dory-isolate-flow-sit.sh` `scripts/dory-isolate-flock-prompt.sh`
- Do not modify: leftover 5, `rust/**`, paid isolate scripts

## Implementation Steps

1. Refuse: `HERDR_ENV!=1`; factory `DORY_*` / `PI_CODING_AGENT_DIR` / `FLOW_BIN` / `FLOW_PROJECT_ROOT` set; `HOME` != `FACTORY_HOME`; missing `SIT_PANE` `SIT_TAB` `SIT_DORY`; `SIT_DORY` = leftover ELF / `~/.local/bin/dory` / factory `rust/target`; `FLOW_BIN` pin abs skill `flow.sh` not `-x` or basename ≠ `flow.sh`; sit = `w13:t13` / `w13:p2R` / `*wP:*` (exact, plus `herdr pane get` tab_id).
2. Hash leftover 5 **vs mint table** (plan trap 23), not vs a fresh snap. `desk.rs` == HEAD. Snap repo `.dory/` ino/mtime (missing OK).
3. Mint ISO `aoe5.XXXXXX`. `PROJECT=$ISO_REAL`. Copy skill template → `$ISO_REAL/flow/00-idea.md` (FAIL). `case $0` + self-`rg` refuse source/exec paid scripts. Do **not** copy 1910 `:308-309`.
4. Start isolate server **0242 `:346-353` verbatim** (`setsid` + `XDG_RUNTIME_DIR="$ISO_REAL"` + `HOME="$ISO_REAL/home"` + PI only on that line). Wait isolate sock connectable + `workspace list`. `factory_must_dead`.
5. Start `coord` + `omptest` on isolate (omp). `agent report --state idle` trên cả hai.
6. Taxi 1 (helper). Assert rc=1 **và** new `flow/result` `bin`=$FLOW_BIN `code`=1 stdout `GATE stage 00-idea`. File vẫn FAIL (`- [ ]` hoặc `[FILL`). Missing journal = FAIL (không phải án).
7. Attach **1910 `:331` verbatim**. `wait-output` needle `Flow 1. gate` (pane id trước option). `factory_must_dead` sau attach.
8. `coord` `prompt` (no `--wait`; `--timeout` OK): omptest Writes **exact PASS bytes** to `$ISO_REAL/flow/00-idea.md` then report idle. Inner text = 0242 ban `herdr` / `server stop` / `--wait` / `~/.omp` / `agent.db` / `PI_CODING_AGENT_DIR`.
9. **Poll** file until MATCH PASS bytes (timeout 180s). Factory không Write. Rồi taxi 2. Assert rc=0, journal **exactly two** `flow/result`, codes `[1,0]`, same `bin`, pass stdout `clean`.
10. `wait-output` needle `Flow 0. gate` (chrome phụ; journal `bin` là bằng).
11. Leftover 5 = mint table. `desk.rs` == HEAD. Repo `.dory/` unchanged. Factory sock connectable=0.
12. Copy journal → `/tmp` or receipt **before** `wipe_iso`. Stop = 1910 `compound_stop`. Write cook receipt. Do not close tabs here.

## Success Criteria

- [ ] Script path exists and does not `exec`/`source` paid scripts
- [ ] COOK_PASS receipt with journal codes 1 then 0, bin = abs flow.sh
- [ ] Sit needles both seen
- [ ] Leftover 5 mint; sock connectable=0

## Risk Assessment

- Occupant không Write file → poll timeout → script FAIL (đúng). Signal: file still FAIL. Response: re-prompt once; still fail → STOP, không factory-write.
- Factory `FLOW_*` inherit. Signal: journal `cwd` not under `ISO_REAL`. Response: refuse at entry; pin taxi env.
- Isolate ELF missing. Signal: `SIT_DORY` not executable. Response: FAIL. Do not cargo leftover. Do not isolate `reset --hard`.
