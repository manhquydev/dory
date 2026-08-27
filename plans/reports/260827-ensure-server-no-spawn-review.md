---
type: review
date: 2026-08-27
plan: 260827-1032-ensure-server-no-auto-spawn
verdict: REVIEW_ACCEPT
critical: 0
head: b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
---

# Review — increment C no auto-spawn

Four OMP panes on `w13:t24`. Factory did not sit `t13`.

| Pane | Name | Verdict | Critical |
|---|---|---|---|
| `w13:p6A` | `rv_spawn` | SPAWN_ACCEPT | 0 |
| `w13:p6D` | `rv_left` | LEFT_ACCEPT | 0 |
| `w13:p6C` | `rv_usage` | USAGE_ACCEPT | 0 |
| `w13:p6E` | `rv_fold` | FOLD_ACCEPT | 0 |

HEAD `ensure_server` fail-closed. Leftover 5 mint. USAGE/README sit = server then dory. PATH empty. `dory.sock` connectable=0. Isolate HEAD == `b544f5f`. Leftover ELF still embeds spawn (kept). Do not claim factory doors held.

REVIEW_ACCEPT
