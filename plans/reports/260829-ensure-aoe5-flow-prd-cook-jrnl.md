---
type: cook-jrnl
date: 2026-08-29
plan: 260829-0054-isolate-flow-prd-unlock
phase: 03
role: pk_jrnl
writer: scripts/dory-isolate-aoe5-flow-prd.sh
verdict: JRNL_PASS
dory: not invoked
---

# Cook journal — AOE5p prd (stage 03)

**Verdict: JRNL_PASS**

After COOK_PASS (`plans/reports/260829-ensure-aoe5-flow-prd-cook.md`), read `plans/reports/260829-ensure-aoe5-flow-prd-journal.jsonl`. Static read of `journal_taxi1` `:350-384` and `journal_taxi2` `:386-428` vs paid AOE5 `scripts/dory-isolate-aoe5-flow-judge.sh` `:280-334`. Did not `source`/`exec` paid scripts. Did not invoke `dory`. Did not sit `t13`. Did not run the isolate script.

Live jsonl cwd is `…/aoe5p.azx4PH` (post-`copy_prd_sha` recopy). Cook named `aoe5p.eGZMMi`. Same contract on both copies observed.

## Receipt jsonl

Exactly two `flow/result` (four lines total: invoke/result/invoke/result).

| # | bin | args | code | stdout |
|---|---|---|---|---|
| 1 | `/home/manhquy/.claude/skills/flow/runner/flow.sh` | `["next"]` | 1 | `FAIL: gate for stage 02-scope is not clean.` |
| 2 | same | `["next"]` | 0 | `unlocked stage 3 (flow/03-prd.md)` |

Both bins absolute `flow.sh` (not `/bin/true`). Both cwd isolate. Taxi1: no `unlocked stage`, no `GATE stage`. Taxi2: no `already exists`, no `unlocked stage 2 ` / `unlocked stage 1 ` / `unlocked stage 00`, no `GATE stage`. Taxi2 also has flow-lock reclaim NOTE; allowed because the land needle is `unlocked stage 3 (flow/03-prd.md)`, not substring `clean`.

## Not AOE5 GATE/clean copies

| Helper | prd sha256[:16] | AOE5 sha256[:16] | identical |
|---|---|---|---|
| `journal_taxi1` | `b413757846b33c92` | `a06ac8f9d82a3e61` | no |
| `journal_taxi2` | `123712d38ee26bde` | `2c95891e8be03240` | no |

AOE5 taxi1 **requires** `"GATE stage 00-idea" in stdout` and never reads `args`. AOE5 taxi2 **requires** `"clean" in stdout` and never reads `args`. PRD helpers do neither.

Taxi argv is `"$SIT_DORY" flow -- next` (`:336`). Contiguous `flow -- gate` absent.

## Contract

| Check | Land | Result |
|---|---|---|
| taxi1 `args == ["next"]` | `:368-369` exact `!= ["next"]` | PASS |
| taxi2 both rows `args == ["next"]` | `:407-408` loop | PASS |
| taxi1 `code==1`, one `flow/result`, `bin==FLOW_BIN` | `:363-371` | PASS |
| taxi1 stdout `FAIL: gate for stage 02-scope is not clean` | `:377-378` | PASS |
| taxi1 reject `unlocked stage` / `GATE stage` | `:379-382` | PASS |
| taxi2 exactly two `flow/result`, codes `[1, 0]`, same `bin` | `:399-406` | PASS |
| taxi2 stdout `unlocked stage 3 (flow/03-prd.md)` | `:414-416` | PASS |
| taxi2 reject `already exists` | `:417-418` | PASS |
| taxi2 reject `unlocked stage 2 ` | `:425-426` | PASS |
| taxi2 reject `unlocked stage 1 ` / `unlocked stage 00` / `GATE stage` | `:419-424` | PASS |
| taxi2 pass predicate is **not** `"clean" in stdout` | AOE5 `:332`; prd has no such test | PASS |
| live jsonl matches helpers | receipt after COOK_PASS | PASS |

Bare-`clean` launder is closed by requiring unlock-3 instead of substring `clean`. Unlock PASS from `flow.sh` still contains `gate clean`; that is allowed because the needle is `unlocked stage 3 (flow/03-prd.md)`.

## Closed false-pass doors

| Fake | Why it dies |
|---|---|
| Recook `gate 02-scope` | `args` not `["next"]`; stdout `GATE stage` rejected |
| Already-exists `next` | taxi2 `"already exists" in stdout` |
| Scope unlock (`unlocked stage 2`) | taxi2 `"unlocked stage 2 "` |
| Empty-tree taxi1 (`unlocked stage`, rc=0) | taxi1 `code!=1` + `"unlocked stage"` |
| Empty-tree / research unlock taxi2 | missing unlock-3 needle + `"unlocked stage 00"` / `"unlocked stage 1 "` |
| AOE5 `clean` copy | taxi2 does not accept substring `clean` |

Doors held: no dory invoke; no `t13`; paid `:280-334` not copied; `args==["next"]` on both rows.

AgentWiki publish skipped.
