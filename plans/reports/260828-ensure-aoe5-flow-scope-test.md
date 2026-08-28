---
type: test
date: 2026-08-28
wave: dory-aoe5s
result: TEST_PASS
sit: w13:t2V / w13:p8J
iso: aoe5s.SXcpJE (wiped)
---

# TEST_PASS — independent isolate scope unlock

Second run, new sit. Journal receipt overwritten by this run (cwd `aoe5s.SXcpJE`). Cook ISO `rX1G2E` no longer on disk.

| Assert | Result |
|---|---|
| Two `flow/result` | yes |
| `bin` abs `flow.sh` | yes |
| `args=["next"]` both | yes |
| codes `[1,0]` | yes |
| taxi1 `FAIL: gate for stage 01-research is not clean` | yes |
| taxi1 no `unlocked stage` | yes |
| taxi2 `unlocked stage 2 (flow/02-scope.md)` | yes |
| taxi2 no stage 1 / already exists | yes |
| 02 sha == template `0a34fa48…` | yes |
| Leftover 5 mint | MATCH |
| Rust log `b544f5f` | yes |
| Sock connectable=0 | yes |
| PATH `dory` empty | yes |
| Sit `Flow 1. next` / `Flow 0. next` | yes |
| factory `flow/` | absent |

Nit: wipe `home` not empty then retry; lock-reclaim NOTE on taxi2 stdout.
