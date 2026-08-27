---
type: test-jrnl
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 02
role: ts_jrnl
source: plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl
iso: aoe5n.X9Ll4T
verdict: JRNL_PASS
dory: not invoked
cook_journal_cited: no
---

# Test journal — AOE5n next (phase 02)

**Verdict: JRNL_PASS**

Waited for a journal after ts_run. Copied jsonl mtime jumped 17:24:02 → 17:30:18. cwd left cook `aoe5n.nOWHtI` for `aoe5n.X9Ll4T`. Test receipt `260827-ensure-aoe5-flow-next-test.md` was absent at parse time; notes land here. Did not invoke `dory`. Did not run the isolate script. Did not cite cook receipt as proof. Independent python parse of the copied jsonl only.

## Rows

4 lines: `flow/invoke` `flow/result` `flow/invoke` `flow/result`. No other types.

| # | ts (UTC) | type | args | code |
|---|---|---|---|---|
| 1 | 2026-08-27T10:28:54.138Z | flow/invoke | `["next"]` | — |
| 2 | 2026-08-27T10:28:54.212Z | flow/result | `["next"]` | 1 |
| 3 | 2026-08-27T10:30:17.549Z | flow/invoke | `["next"]` | — |
| 4 | 2026-08-27T10:30:17.621Z | flow/result | `["next"]` | 0 |

Exactly **two** `flow/result`. codes **`[1, 0]`**. Both result rows and both invoke rows `args == ["next"]`. Not `["gate","00-idea"]`. `bin` all four = `/home/manhquy/.claude/skills/flow/runner/flow.sh`. Not `/bin/true`. Not bare `flow.sh`. cwd all four = `/home/manhquy/.cache/dory-isolates/aoe5n.X9Ll4T`. `stderr=""`. `error=null`. `signal=null`.

## Taxi1 (code 1)

stdout starts `FAIL: gate for stage 00-idea is not clean.`

| Reject | Present |
|---|---|
| `GATE stage` | no |
| `unlocked stage` | no |
| `already exists` | no |
| `unlocked stage 00` | no |

Contains `not clean` (FAIL). That is not AOE5 `GATE stage 00-idea` / `clean`.

## Taxi2 (code 0)

stdout:

```
NOTE: reclaiming a flow lock from a dead session [ppid:manhquy-Legion-5-15ACH6:2998434] (pid 2998435 no longer alive).
PASS: stage 00-idea gate clean -> unlocked stage 1 (flow/01-research.md)
  tip: '/flow recall' surfaces prior debt/retro/friction before you fill this stage.
```

Land needle **present**: `unlocked stage 1 (flow/01-research.md)`.

| Reject | Present |
|---|---|
| `already exists` | no |
| `GATE stage` | no |
| `unlocked stage 00` | no |

Word `clean` appears inside `gate clean -> unlocked stage 1`. Pass predicate is the unlock-1 needle, **not** substring `clean` (F3). `not clean` is taxi1 only.

## Closed false-pass doors (this jsonl)

| Fake | Why this copy dies |
|---|---|
| Recook `gate 00-idea` | `args` are `["next"]`; stdout has no `GATE stage` |
| Already-exists `next` (`flow.sh:1019-1022`) | taxi2 has no `already exists` |
| Empty-tree taxi1 (`unlocked stage 00`, rc=0) | taxi1 `code==1` + no `unlocked stage` |
| Empty-tree taxi2 | no `unlocked stage 00`; unlock-1 needle present |
| AOE5 `clean` copy | land is unlock-1, not bare `clean` |
| Cook journal reuse | cwd `aoe5n.X9Ll4T`, ts 10:28/10:30 — not `nOWHtI` 10:23/10:24 |

## 01 sha (receipt, not body)

`plans/reports/260827-ensure-aoe5-flow-next-01.sha256` rewritten with this iso:

```
69429bc3e11f467c1dbcad4694055078cda4192dab447bf86832c2d17b1264aa  …/aoe5n.X9Ll4T/flow/01-research.md
69429bc3e11f467c1dbcad4694055078cda4192dab447bf86832c2d17b1264aa  …/flow/_templates/01-research.md
```

Same digest as template. Body not copied. Isolate path not read (wiped).

Doors held: no dory invoke; cook receipt unused; `args==["next"]` on both `flow/result`; reject `GATE stage` / `already exists` / `unlocked stage 00`.
