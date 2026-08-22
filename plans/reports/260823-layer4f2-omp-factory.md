---
type: factory
date: 2026-08-23
time: 00:28
status: pass
authority:
  - plans/260823-0011-close-coding-occupancy/phase-03-factory-reprove.md
  - plans/reports/260823-layer4f-omp-factory.md
---

# Layer 4f2 factory — CR submit after splash (không CI)

Chấm **pass** từ file bằng chứng: start `unknown`, occupant left compose, `dory agent report` ran, `get` = `done` (idle report + unseen).

## Verdict table

| Check | Result | Source |
|---|---|---|
| start state | `unknown` | `/tmp/dory-l4f2-evidence/start.json` |
| `--kind` absent | yes | `-- omp --no-session` |
| splash before prompt | yes | `/tmp/dory-l4f2-evidence/splash-ready` |
| prompt | `ok:true`, no stall | `/tmp/dory-l4f2-evidence/prompt.json` |
| turn submitted | yes — tool spinner `Run dory agent report idle` / `DORY_BIN` | `/tmp/dory-l4f2-evidence/read.txt` (ring 256KiB; splash scrolled off) |
| wait CLI | `timeout` at 180s | `/tmp/dory-l4f2-evidence/wait.err` |
| get after wait | `done` `seen=false` | `/tmp/dory-l4f2-evidence/get-final.json` |
| classifier leak | no (`done` only via report; `omp` not allowlisted) | `classify_word` + get-final |
| no cargo omp | yes | this journal |
| no flow-skill bytes | yes | this journal |

## Why wait timed out

Transcript tail is an omp tool/approval spinner (`⟦esc⟧`) running `dory agent report` / `DORY_BIN`. Report landed by the time `get` ran after wait returned. Occupancy is proven; 180s was tight for that spinner. Not a new product hole. Not §11.

## Contrast 4f

4f: prompt sat in compose; no turn; get stayed `unknown`. Cause `submit`. Phase 2 CR after BP wrap + wait-for-splash closed that.

## Not §11

No real-repo Flow verdict.
