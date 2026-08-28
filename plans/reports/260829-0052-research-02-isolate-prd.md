---
type: research
date: 2026-08-29
time: 00:52
scope: copy O isolate law for unlock-3
---

# Research Report: isolate copy-law for P

## Executive Summary

Copy O (`dory-isolate-aoe5-flow-scope.sh`), do not source/exec it. Delta: mint 00 PASS + 01 PASS + 02 template; occupant PASS **02**; taxi1 FAIL `02-scope`; taxi2 unlock `03-prd.md`; self-refuse adds **O** (`dory-isolate-aoe5-flow-scope`) and N; ISO prefix `aoe5p.`; receipts `260829-ensure-aoe5-flow-prd-*`.

## Research Methodology

- Sources: O script + plan `260828-1612`, N script, leftover mint table, land ELF pin
- Live pins 00:52 MATCH scout

## Key Findings

### Copy (unchanged)

HERDR refuse; leftover mint table; land sha `2ef20730…`; leftover ELF `3ba0e3bc…` stat-only; compound_stop 2357; attach 1910; server 0242 `:340-353`; occupants `--no-session --no-skills --no-rules --no-extensions`; taxi `flow -- next`; poll PASS (no `--wait`); copy journal + unlock sha before wipe; `FLOW_HARNESS_DISABLE=1`.

### Delta vs O

| O | P |
|---|---|
| Mint PASS 00 + template 01 | Mint PASS 00 + PASS 01 + template 02 |
| Occupant PASS 01 | Occupant PASS 02 |
| FAIL `01-research` / unlock stage 2 | FAIL `02-scope` / unlock stage 3 |
| Self-refuse N | Self-refuse N **+ O** |
| Prefix `aoe5s.` | Prefix `aoe5p.` |

Factory **may** write PASS 00 and PASS 01 (already-proved world-state). Factory **must not** write PASS 02.

## Implementation Recommendations

New file `scripts/dory-isolate-aoe5-flow-prd.sh`. Subject `feat(isolate): fail-then-pass flow.sh prd`. Not company Phase 5.

## Common Pitfalls

- Journal still `unlocked stage 2` → false PASS recook O
- Mint 02 missing → idx=1 recooks O
- Dirty 00/01 + clean 02 unlocks 03 dishonestly
- Source O script
- `ak:git` / leftover 5 / sit `t13` / default sock
