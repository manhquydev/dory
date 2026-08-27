---
type: cook-jrnl
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 01
role: ck_jrnl
writer: scripts/dory-isolate-aoe5-flow-next.sh
verdict: JRNL_PASS
dory: not invoked
---

# Cook journal helpers — AOE5n next (phase 01)

**Verdict: JRNL_PASS**

Gate: `scripts/dory-isolate-aoe5-flow-next.sh` exists (29307 bytes). Static read of `journal_taxi1` `:343-377` and `journal_taxi2` `:379-417`. Compared to paid AOE5 `scripts/dory-isolate-aoe5-flow-judge.sh` `:280-334`. Did not `source`/`exec` paid scripts. Did not invoke `dory`. Did not run the isolate script.

## Not AOE5 GATE/clean copies

| Helper | next sha256[:16] | AOE5 sha256[:16] | identical |
|---|---|---|---|
| `journal_taxi1` | `749cd67b01fbaf62` | `a06ac8f9d82a3e61` | no |
| `journal_taxi2` | `86450666a709c99a` | `2c95891e8be03240` | no |

AOE5 taxi1 **requires** `"GATE stage 00-idea" in stdout` and never reads `args`. AOE5 taxi2 **requires** `"clean" in stdout` and never reads `args`. Next helpers do neither.

Taxi argv is `"$SIT_DORY" flow -- next` (`:329`). Contiguous `flow -- gate` absent (self-refuse splits the string).

## Contract

| Check | Land | Result |
|---|---|---|
| taxi1 `args == ["next"]` | `:361-362` exact `!= ["next"]` | PASS |
| taxi2 both rows `args == ["next"]` | `:397-401` loop | PASS |
| taxi1 `code==1`, one `flow/result`, `bin==FLOW_BIN` | `:356-364` | PASS |
| taxi1 stdout `FAIL: gate for stage 00-idea is not clean` | `:370-371` | PASS |
| taxi1 reject `unlocked stage` / `GATE stage` | `:372-375` | PASS |
| taxi2 exactly two `flow/result`, codes `[1, 0]`, same `bin` | `:392-399` | PASS |
| taxi2 stdout `unlocked stage 1 (flow/01-research.md)` | `:407-409` | PASS |
| taxi2 reject `already exists` | `:410-411` | PASS |
| taxi2 reject `unlocked stage 00` | `:414-415` | PASS |
| taxi2 reject `GATE stage` | `:412-413` | PASS |
| taxi2 pass predicate is **not** `"clean" in stdout` | AOE5 `:332`; next has no such test | PASS |

Bare-`clean` launder (F3) is closed by requiring the unlock needle instead of substring `clean`. Unlock PASS from `flow.sh:1026` still contains `gate clean`; that is allowed because the needle is `unlocked stage 1 (flow/01-research.md)`.

## Closed false-pass doors

| Fake | Why it dies |
|---|---|
| Recook `gate 00-idea` | `args` not `["next"]`; stdout `GATE stage` rejected |
| Already-exists `next` (`flow.sh:1019-1022`) | taxi2 `"already exists" in stdout` |
| Empty-tree taxi1 (`unlocked stage 00`, rc=0) | taxi1 `code!=1` + `"unlocked stage"` |
| Empty-tree taxi2 | taxi2 `"unlocked stage 00"` + missing unlock-1 needle |
| AOE5 `clean` copy | taxi2 does not accept substring `clean` |

Doors held: no dory invoke; paid `:280-334` not copied; `args==["next"]` on both rows.
