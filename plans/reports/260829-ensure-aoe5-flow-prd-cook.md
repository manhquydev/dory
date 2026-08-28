---
type: cook
date: 2026-08-29
wave: dory-aoe5p
result: COOK_PASS
sit: w13:t20 / w13:p97
run: w13:p99
iso: aoe5p.eGZMMi (wiped)
---

# COOK_PASS — isolate flow.sh prd

Script `scripts/dory-isolate-aoe5-flow-prd.sh` exit 0.

| Signal | Value |
|---|---|
| Taxi | `"$SIT_DORY" flow -- next` codes `[1,0]` |
| Journal | abs `flow.sh`, `args=["next"]`, taxi1 `FAIL: gate for stage 02-scope is not clean`, taxi2 `unlocked stage 3 (flow/03-prd.md)` |
| Sit | `Flow 1. next` then `Flow 0. next` |
| 03 sha | `219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4` == `_templates/03-prd.md` |
| Land ELF | `2ef20730…` |
| Leftover 5 | mint MATCH |
| Sock | connectable=0 |
| PATH `dory` | gone |
| Rust | `b544f5f` |
| Factory `flow/` | absent |

Nit: wipe `rm: …/home: Directory not empty` then retry; no leftover `aoe5p.*`. Taxi2 stdout also has flow-lock reclaim NOTE (same class as O). Cook receipt `*-03.sha256` second line was SCOPE_TEMPLATE (script bug); land `prd_sha_ok` still compared 03 to PRD_TEMPLATE and passed. Fixed `copy_prd_sha` to hash `$PRD_TEMPLATE` before independent test.

Did not recook O. Did not fill 03. Did not sit `t13`.
