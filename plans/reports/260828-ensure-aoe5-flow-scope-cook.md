---
type: cook
date: 2026-08-28
wave: dory-aoe5s
result: COOK_PASS
sit: w13:t2T / w13:p8H
run: w13:p8K
iso: aoe5s.rX1G2E (wiped)
---

# COOK_PASS — isolate flow.sh scope

Script `scripts/dory-isolate-aoe5-flow-scope.sh` exit 0.

| Signal | Value |
|---|---|
| Taxi | `"$SIT_DORY" flow -- next` codes `[1,0]` |
| Journal | abs `flow.sh`, `args=["next"]`, taxi1 `FAIL: gate for stage 01-research is not clean`, taxi2 `unlocked stage 2 (flow/02-scope.md)` |
| Sit | `Flow 1. next` then `Flow 0. next` |
| 02 sha | `0a34fa4864ca0861bff831d52f0f20b3932b4d159e152c6890e961ca6d8119c2` == `_templates/02-scope.md` |
| Land ELF | `2ef20730…` |
| Leftover 5 | mint MATCH |
| Sock | connectable=0 |
| PATH `dory` | gone |
| Rust | `b544f5f` |
| Factory `flow/` | absent |

Nit: wipe `rm: …/home: Directory not empty` then retry; no leftover `aoe5s.*`. Taxi2 stdout also has flow-lock reclaim NOTE (same class as N).

Did not recook N. Did not fill 02. Did not sit `t13`.
