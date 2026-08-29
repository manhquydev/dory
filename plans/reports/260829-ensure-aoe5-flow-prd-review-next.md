---
type: review
lens: next
date: 2026-08-29
wave: dory-aoe5p
role: pr_next
result: ACCEPT
critical: 0
---

# rv_next — unlock-3 only

Independent parse of `plans/reports/260829-ensure-aoe5-flow-prd-journal.jsonl` (test iso `aoe5p.azx4PH`), `*-test.md`, `*-cook.md`, `*-03.sha256`, live `_templates/03-prd.md`, and `scripts/dory-isolate-aoe5-flow-prd.sh`. Did not sit `t13`. Did not `git add -A`. Did not recook O.

| Check | Evidence | Result |
|---|---|---|
| Taxi1 FAIL `02-scope` | journal `flow/result` row0: `code=1`, `args=["next"]`, abs `…/flow/runner/flow.sh`, cwd `…/aoe5p.azx4PH`, stdout `FAIL: gate for stage 02-scope is not clean.` | PASS |
| Taxi1 no `unlocked stage` | same row: no `unlocked stage`, no `GATE stage` | PASS |
| Taxi2 `unlocked stage 3 (flow/03-prd.md)` | journal `flow/result` row1: `code=0`, same `bin`/`args`/`cwd`, stdout `PASS: stage 02-scope gate clean -> unlocked stage 3 (flow/03-prd.md)` | PASS |
| Taxi2 reject stage 2 / already exists | row1: no `unlocked stage 2 `, no `unlocked stage 1 `, no `unlocked stage 00`, no `already exists`, no `GATE stage` | PASS |
| codes `[1,0]` `args=["next"]` abs `flow.sh` | both result rows; also both invoke rows | PASS |
| 03 sha == template | live `sha256(_templates/03-prd.md)` = `219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4`; receipt both sides same digest (iso `03-prd.md` + template) | PASS |
| Harness pin | `scripts/dory-isolate-aoe5-flow-prd.sh` taxi `:334` `FLOW_HARNESS_DISABLE=1`; self-refuse requires the pin in file text | PASS |
| Not O unlock-2 | O land is FAIL `01-research` then `unlocked stage 2 (flow/02-scope.md)` (scope sha `0a34fa48…`). This land is FAIL `02-scope` then unlock-3; 03 sha `219c9350…` ≠ scope template. Script self-refuses `dory-isolate-aoe5-flow-scope`. | PASS |

Cook `COOK_PASS`. Independent test `TEST_PASS` (overwrote journal; cwd not cook `aoe5p.eGZMMi`).

Non-blocking: taxi2 stdout has flow-lock reclaim NOTE (land needle is unlock-3, not substring `clean`). Wipe `rm: …/home: Directory not empty` then retry.

Critical: 0
