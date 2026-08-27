---
phase: 1
title: "New isolate next-unlock script"
status: pending
priority: P1
effort: "2h"
dependencies: []
---

# Phase 1: New isolate next-unlock script

## Overview

Viết và chạy một lần `scripts/dory-isolate-aoe5-flow-next.sh`: isolate + đàn nhỏ + taxi `flow -- next` fail rồi pass. World-state `flow/01-research.md`.

## Requirements

- Functional: script mới; mint FAIL idea; taxi 1 rc=1; occupant viết PASS; taxi 2 rc=0; sit `Flow 1. next` rồi `Flow 0. next`; `01-research.md` exists
- Non-functional: leftover 5 mint; factory sock connectable=0; không rust; stop compound only

## Architecture

Factory Herdr mở tab sit sạch + tab cook. Script mint ISO dưới `~/.cache/dory-isolates/aoe5n.XXXXXX` (prefix mới, không `aoe5.` / `flock.6yaatuxg`). **`PROJECT=$ISO_REAL`**. Mint `$ISO_REAL/flow/00-idea.md` từ template skill (FAIL). Isolate server: 0242 **`:340-353`** (`mkdir`/`ln` rồi `setsid`). Occupants: `coord` + `omptest` with AOE5 `start_omp` `--no-session --no-skills --no-rules --no-extensions`. Attach = 1910 `:331`. Copy-table only. Do **not** `source`/`.`/`exec` judge / 1910 / 0043 / 0227 / 0242 / hop.

### Taxi helper (copy into script)

```
taxi() {
  (cd "$ISO_REAL" && \
    HOME="$ISO_REAL/home" DORY_SOCKET="$ISO_SOCK" DORY_ENV=1 \
    DORY_WORKSPACE_DIR="$ISO_REAL" FLOW_PROJECT_ROOT="$ISO_REAL" \
    FLOW_BIN="$FLOW_BIN" FLOW_LOG_DISABLE=1 DO_NOT_TRACK=1 \
    "$SIT_DORY" flow -- next)
}
```

### Journal helpers (do **not** copy AOE5 `:280-334`)

- taxi1: `code==1`; one `flow/result`; `bin`=abs flow.sh; `args == ["next"]`; stdout contains `FAIL: gate for stage 00-idea is not clean`; reject `unlocked stage` / `GATE stage`
- After taxi1: `idea_still_fail`; `! -f $ISO_REAL/flow/01-research.md`; file still ≠ PASS_WANT
- taxi2: `code==0`; exactly two `flow/result`; codes `[1,0]`; same `bin`; both `args == ["next"]`; stdout contains `unlocked stage 1 (flow/01-research.md)`; reject `already exists` / `GATE stage` / `unlocked stage 00` / bare `clean`
- After taxi2: `-f 01-research.md` and `sha256` == skill `_templates/01-research.md` (still `[FILL]`). Do not fill.
- Taxi2 IFF `cmp -s` PASS file after occupant prompt. Self-`rg` `--wait` and `flow -- gate`.

### PASS file bytes (occupant writes; factory must not)

Same fixture as AOE5 (scan_gate on `00-idea.md` is identical):

```
# Stage 00 — Idea

## Gate — check ALL before `/flow next`
- [x] The pitch below is 3 sentences, no more
- [x] I can name at least ONE real person/group who has this pain (named below)
- [x] No FILL placeholders remain in this file

## Pitch (3 sentences: who, pain, what you'd build)

Operators sitting Dory cannot prove Flow unlocked research after a real idea gate.
AOE5 paid gate 00-idea; next must mint flow/01-research.md.
This fixture is a one-stage idea so Flow can fail then unlock.

## One real person/group with this pain

Founder sitting the Dory desk after isolate AOE5 gate.
```

Do **not** fill `01-research.md` after unlock.

## Related Code Files

- Create: `scripts/dory-isolate-aoe5-flow-next.sh`
- Create: `plans/reports/260827-ensure-aoe5-flow-next-cook.md`
- Copy-law only (do not `source`/`.`/`exec`): `scripts/dory-isolate-aoe5-flow-judge.sh` `scripts/dory-isolate-flow-sit.sh` `scripts/dory-isolate-flock-prompt.sh`
- Do not modify: leftover 5, `rust/**`, paid isolate scripts

## Implementation Steps

1. Refuse: `HERDR_ENV!=1`; factory `DORY_*` / `PI_CODING_AGENT_DIR` / **any** `FLOW_*` set; `HOME` != `FACTORY_HOME`; missing `SIT_PANE` `SIT_TAB` `SIT_DORY`; `SIT_DORY` sha ≠ land `2ef20730…` or path leftover ELF / `~/.local/bin/dory` / factory `rust/target` / leftover isolate / factory XDG ELF; `FLOW_BIN` pin abs skill `flow.sh`; sit = `w13:t13` / `w13:p2R` / `*wP:*` (`herdr pane get` `tab_id`). Sit pane before attach: `type -a dory` empty.
2. Hash leftover 5 **vs mint table** (plan trap 23). `desk.rs` == HEAD. Snap repo `.dory/` ino/mtime (missing OK).
3. Mint ISO `aoe5n.XXXXXX`. `PROJECT=$ISO_REAL`. Copy skill template → `$ISO_REAL/flow/00-idea.md` (FAIL). `$0` case + `self_refuse_paid` **first paste** includes judge. Self-`rg` `source`/`exec` paid names + `--wait` + `flow -- gate`. Do **not** copy 1910 `:308-309`.
4. ISO/bin `mkdir`/`ln -sfn` + start isolate server **0242 `:340-353`**. `realpath` bin/dory == `SIT_DORY` == land sha. Wait isolate sock connectable + `workspace list`. `factory_must_dead`.
5. Start `coord` + `omptest` on isolate with AOE5 `start_omp` `--no-session --no-skills --no-rules --no-extensions`. `agent report --state idle` trên cả hai **before taxi1 only** — idle is not the taxi2 gate.
6. Taxi 1 (helper). Assert rc=1 **và** new `flow/result` `bin`=$FLOW_BIN `args=["next"]` `code`=1 stdout `FAIL`+`00-idea`. File vẫn FAIL. Missing journal = FAIL.
7. Attach **1910 `:331` verbatim**. `wait-output` needle `Flow 1. next` (pane id trước option). `factory_must_dead` sau attach.
8. `coord` `prompt` (no `--wait`; `--timeout` OK): omptest Writes **exact PASS bytes** to `$ISO_REAL/flow/00-idea.md` then report idle. Inner text = 0242 ban `herdr` / `server stop` / `--wait` / `~/.omp` / `agent.db` / `PI_CODING_AGENT_DIR`.
9. **Poll** file until MATCH PASS bytes (timeout 180s). Factory không Write. Taxi2 IFF `cmp -s`. Assert rc=0, journal **exactly two** `flow/result`, codes `[1,0]`, same `bin`, both `args == ["next"]`, stdout `unlocked stage 1 (flow/01-research.md)` (reject `already exists` / `unlocked stage 00` / `GATE stage`). Assert `01-research.md` sha == `_templates/01-research.md`. Do not fill it.
10. `wait-output` needle `Flow 0. next` (necessary chrome; land = journal stdout + 01 sha). Sit hit + weak journal = FAIL.
11. Leftover 5 = mint path+sha table. `desk.rs` == HEAD. Repo `.dory/` unchanged. Factory sock connectable=0. Factory `flow/` absent.
12. `copy_journal || fail` + copy 01 sha/stat **before** any `wipe_iso` (including `fail`/`EXIT`). Stop = paste 1910 `compound_stop` `:69-100` (identity gates, not the one-liner alone). Same land-hash `SIT_DORY` for stop. Write cook receipt. Do not close tabs here.

## Success Criteria

- [ ] Script path exists and does not `exec`/`source` paid scripts (judge included)
- [ ] COOK_PASS receipt with journal codes 1 then 0, bin = abs flow.sh, both args `["next"]`, copied `01-research.md` sha == template
- [ ] Sit needles both seen (`Flow 1. next` / `Flow 0. next`)
- [ ] Leftover 5 mint; sock connectable=0

## Risk Assessment

- Occupant không Write file → poll timeout → script FAIL (đúng). Signal: file still FAIL. Response: re-prompt once; still fail → STOP, không factory-write.
- Journal taxi2 accepts `clean` → FAIL next (`not clean`) can false-pass. Signal: taxi1 stdout has `clean`. Response: require `unlocked stage 1`.
- Factory `FLOW_*` inherit. Signal: journal `cwd` not under `ISO_REAL` or `args` ≠ `["next"]`. Response: refuse at entry; pin taxi env.
- Isolate ELF missing. Signal: `SIT_DORY` not executable. Response: FAIL. Do not cargo leftover.
