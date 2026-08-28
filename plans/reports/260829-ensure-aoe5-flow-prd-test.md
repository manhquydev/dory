---
type: test
date: 2026-08-29
wave: dory-aoe5p
result: TEST_PASS
sit: w13:t31 / w13:p98
run: w13:p99
iso: aoe5p.azx4PH (wiped)
writer: pt_run
---

# TEST_PASS — independent isolate prd unlock

Second run, new sit. Did not wait-output `aoe5-prd=PASS` on `w13:p99` (stale cook footer was `SIT_TAB=w13:t20` / `aoe5p.eGZMMi`). This run's footer is `SIT_TAB=w13:t31` / `SIT_PANE=w13:p98`. Journal receipt overwritten by this run (cwd `aoe5p.azx4PH`). Cook ISO `eGZMMi` no longer on disk.

Did not sit `w13:t13`. Did not invoke `dory`. Did not re-prompt isolate occupants.

| Assert | Result |
|---|---|
| Footer `SIT_TAB=w13:t31` | yes |
| New `aoe5p.*` not `eGZMMi` | `aoe5p.azx4PH` |
| Two `flow/result` | yes |
| `bin` abs `flow.sh` | yes |
| `args=["next"]` both | yes |
| codes `[1,0]` | yes |
| taxi1 `FAIL: gate for stage 02-scope is not clean` | yes |
| taxi1 no `unlocked stage` | yes |
| taxi2 `unlocked stage 3 (flow/03-prd.md)` | yes |
| taxi2 no stage 1 / stage 2 / already exists | yes |
| 03 sha == template `219c9350…` | yes |
| Leftover 5 mint | MATCH |
| Rust log `b544f5f` | yes |
| Sock connectable=0 | yes |
| PATH `dory` empty | yes |
| Sit `Flow 1. next` / `Flow 0. next` | yes |
| factory `flow/` | absent |
| repo `.dory/` | ABSENT |
| `t13` focused | no |

Nit: wipe `rm: …/aoe5p.azx4PH/home: Directory not empty` then retry; lock-reclaim NOTE on taxi2 stdout. `copy_prd_sha` second line is PRD template (not SCOPE). After run: no `aoe5p.*` dirs. Sit cwd `aoe5p.azx4PH (deleted)`. Visible still `Flow 0. next`.
