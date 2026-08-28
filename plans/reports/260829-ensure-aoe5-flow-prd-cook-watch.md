---
type: cook-watch
date: 2026-08-29
time: 01:06
plan: 260829-0054-isolate-flow-prd-unlock
wave: dory-aoe5p
watcher: pk_watch
watcher_pane: w13:p9G
watcher_tab: w13:t32
watched_pane: w13:p99
watched_tab: w13:t32
sit_pane: w13:p97
sit_tab: w13:t20
aoe5_prd: PASS
iso_cook: aoe5p.eGZMMi
iso_test: aoe5p.azx4PH
iso_wiped: yes
sat_t13: no
invoked_dory: no
reprompted_isolate: no
---

# Watch — `w13:p99` aoe5-prd

**`aoe5-prd=PASS`.** ISO names noted and wiped. Watcher did not sit `w13:t13`. Watcher did not invoke `dory`. Watcher did not re-prompt isolate occupants.

## Identity

| Role | Pane | Tab | Agent |
|---|---|---|---|
| pk_watch (this) | `w13:p9G` | `w13:t32` | omp |
| cook runner | `w13:p99` | `w13:t32` | none (bash) |
| sit (roster) | `w13:p97` | `w13:t20` | none |
| testsit | `w13:p98` | `w13:t31` | none |
| factory | `w13:p2R` | `w13:t13` | cursor `working`; not sat |

`HERDR_PANE_ID=w13:p9G` `HERDR_TAB_ID=w13:t32` ≠ `w13:t13` ≠ `w13:p2R`. `herdr tab get w13:t13` → `focused=false` at T_end. No `tab focus` / `pane focus` / `agent focus` / `send-text` / `agent prompt` on `t13` / `p2R` / isolate coord / omptest.

## ISO

| Run on `p99` | Sit | ISO | Verdict | Wipe |
|---|---|---|---|---|
| cook `./scripts/dory-isolate-aoe5-flow-prd.sh` | `w13:p97` / `w13:t20` | `aoe5p.eGZMMi` | `aoe5-prd=PASS` | gone |
| `TEST_RUN_START_P` same script | `w13:p98` / `w13:t31` | `aoe5p.azx4PH` | `aoe5-prd=PASS` | gone |

Both teardowns printed `rm: cannot remove '…/home': Directory not empty`, then `wipe_iso` retries succeeded.

T_end `~/.cache/dory-isolates/aoe5p.*` = **NONE**. Watcher did not `rm` a live isolate. Did not `dory server stop`. Did not send keys/text to `p97` / `p98` / isolate occupants.

Leftover attach (not factory, not re-prompted):

- `p97` PID `338383` `land-4b70f79/.../dory attach` cwd `aoe5p.eGZMMi (deleted)`
- `p98` PID `388289` same ELF cwd `aoe5p.azx4PH (deleted)`

## `p99` T_end visible

```
TAXI1_RC=1
TAXI2_RC=0
JOURNAL_CODES=1,0
NEEDLE1=Flow 1. next
NEEDLE2=Flow 0. next
JOURNAL_COPY=.../260829-ensure-aoe5-flow-prd-journal.jsonl
PRD_SHA=219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4
TEMPLATE_SHA=219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4
PRD_SHA_COPY=.../260829-ensure-aoe5-flow-prd-03.sha256
VISIBLE_MATCH=1
aoe5-prd=PASS
```

`p99` foreground = `/bin/bash` cwd factory repo. Journal + `03.sha256` copies present. Factory `flow/` absent.

## Watcher did not

- sit `w13:t13` / `w13:p2R` / `wP`
- invoke factory or isolate `dory`
- re-prompt isolate `coord` / `omptest`
- wipe `land-4b70f79` / leftover ELF / `flock.6yaatuxg`
- `herdr server stop`

## Result

`aoe5-prd=PASS`. ISO `aoe5p.eGZMMi` and `aoe5p.azx4PH` noted and wiped. Cache `aoe5p.*` empty.
