---
type: test-jrnl
date: 2026-08-29
plan: 260829-0054-isolate-flow-prd-unlock
phase: 02
role: pt_jrnl
source: plans/reports/260829-ensure-aoe5-flow-prd-journal.jsonl
iso: aoe5p.azx4PH
verdict: JRNL_PASS
dory: not invoked
cook_journal_cited: no
---

# Test journal — AOE5p prd (phase 02)

**Verdict: JRNL_PASS**

Waited for independent test overwrite of `plans/reports/260829-ensure-aoe5-flow-prd-journal.jsonl`. Copied jsonl mtime 01:05:35 (+0700), after cook receipt 01:04:39. cwd left cook `aoe5p.eGZMMi` for `aoe5p.azx4PH`. Test receipt `260829-ensure-aoe5-flow-prd-test.md` was absent at parse time; notes land here. Did not invoke `dory`. Did not run the isolate script. Did not cite cook receipt as proof. Independent python parse of the copied jsonl only.

## Rows

4 lines: `flow/invoke` `flow/result` `flow/invoke` `flow/result`. No other types.

| # | ts (UTC) | type | args | code |
|---|---|---|---|---|
| 1 | 2026-08-28T18:04:48.145Z | flow/invoke | `["next"]` | — |
| 2 | 2026-08-28T18:04:48.227Z | flow/result | `["next"]` | 1 |
| 3 | 2026-08-28T18:05:35.145Z | flow/invoke | `["next"]` | — |
| 4 | 2026-08-28T18:05:35.218Z | flow/result | `["next"]` | 0 |

Exactly **two** `flow/result`. codes **`[1, 0]`**. Both result rows and both invoke rows `args == ["next"]`. Not `["gate","02-scope"]`. `bin` all four = `/home/manhquy/.claude/skills/flow/runner/flow.sh`. Not `/bin/true`. Not bare `flow.sh`. cwd all four = `/home/manhquy/.cache/dory-isolates/aoe5p.azx4PH`. Not `aoe5p.eGZMMi`. `stderr=""`. `error=null`. `signal=null`. Isolate dirs wiped (`aoe5p.azx4PH` and `aoe5p.eGZMMi` absent).

## Taxi1 (code 1)

stdout starts `FAIL: gate for stage 02-scope is not clean.`

| Reject | Present |
|---|---|
| `GATE stage` | no |
| `unlocked stage` | no |
| `already exists` | no |
| `unlocked stage 2` | no |
| `unlocked stage 00` | no |

Contains `not clean` (FAIL). That is not AOE5 `GATE stage 02-scope` / `clean`.

## Taxi2 (code 0)

stdout:

```
NOTE: reclaiming a flow lock from a dead session [ppid:manhquy-Legion-5-15ACH6:388104] (pid 388105 no longer alive).
PASS: stage 02-scope gate clean -> unlocked stage 3 (flow/03-prd.md)
  tip: '/flow recall' surfaces prior debt/retro/friction before you fill this stage.
```

Land needle **present**: `unlocked stage 3 (flow/03-prd.md)`.

| Reject | Present |
|---|---|
| `already exists` | no |
| `GATE stage` | no |
| `unlocked stage 00` | no |
| `unlocked stage 1 ` | no |
| `unlocked stage 2` | no |

Word `clean` appears inside `gate clean -> unlocked stage 3`. Pass predicate is the unlock-3 needle, **not** substring `clean`. `not clean` is taxi1 only. Stage 2 rejected: no `unlocked stage 2`.

## Closed false-pass doors (this jsonl)

| Fake | Why this copy dies |
|---|---|
| Recook `gate 02-scope` | `args` are `["next"]`; stdout has no `GATE stage` |
| Already-exists `next` | taxi2 has no `already exists` |
| Empty-tree taxi1 (`unlocked stage 00`, rc=0) | taxi1 `code==1` + no `unlocked stage` |
| Empty-tree taxi2 | no `unlocked stage 00`; unlock-3 needle present |
| Scope unlock-2 copy | taxi2 has no `unlocked stage 2`; land is unlock-3 |
| Next unlock-1 copy | taxi2 has no `unlocked stage 1 ` |
| AOE5 `clean` copy | land is unlock-3, not bare `clean` |
| Cook journal reuse | cwd `aoe5p.azx4PH`, ts 18:04/18:05 — not `eGZMMi` 18:01/18:02 |

## 03 sha (receipt, not body)

`plans/reports/260829-ensure-aoe5-flow-prd-03.sha256` rewritten with this iso:

```
219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4  …/aoe5p.azx4PH/flow/03-prd.md
219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4  …/flow/_templates/03-prd.md
```

Same digest as live template (`219c9350…`). Not scope template `0a34fa48…`. Body not copied. Isolate path not read (wiped).

Doors held: no dory invoke; cook receipt unused; `args==["next"]` on both `flow/result`; reject `GATE stage` / `already exists` / `unlocked stage 2` / `unlocked stage 00`.
