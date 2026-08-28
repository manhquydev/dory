---
type: research
date: 2026-08-28
time: 16:12
wave: dory-cook-o
scope: flow.sh cmd_next 01 → 02
---

# Research 01 — flow.sh next unlocks scope

Cite runner `/home/manhquy/.claude/skills/flow/runner/flow.sh`. No dory. No recook N.

## current_stage_idx

`flow.sh:136-153`: highest contiguous file from `00-idea`. Both `00-idea.md` + `01-research.md` exist → **idx = 1**. `cmd_next` then `scan_gate` **01**, not 00.

## Taxi 1 (01 dirty)

`flow.sh:965-974`: dirty current → `FAIL: gate for stage 01-research is not clean.` rc=1. Does **not** `cp` 02.

`scan_gate` (`:157-175`) = unchecked `- [ ]` **or** `[FILL`. Template `_templates/01-research.md` fails both.

## Taxi 2 (01 clean, 02 missing)

`flow.sh:1024-1026`: `cp` `_templates/02-scope.md` + `PASS: … unlocked stage 2 (flow/02-scope.md)`.

Already-exists `:1019-1022` — reject as taxi2.

Empty-tree first next `:953-962` copies `00-idea` rc=0 — **not** this án.

## Harness trap

After unlock, `gate_durable_hook 01-research` (`:1027`, `:660-671`) seeds intake **if** `harness_available` (`:289-294`). `FLOW_HARNESS_DISABLE` set → hook no-op. Pin on taxi.

## Word `clean`

FAIL has `not clean`. PASS has `gate clean`. Journal taxi2 must require `unlocked stage 2 (flow/02-scope.md)`, not bare `clean`.

## Dory taxi

`git show HEAD:rust/src/flow.rs:3` — no `next`/`card`/`check`. Glance `Flow {n}. {arg0}` → sit `Flow 1. next` / `Flow 0. next` (shared with N). Land = stdout + 02 sha.

## Verdict

Án = mint N world-state (00 PASS + 01 template) then taxi `flow -- next` fail-then-pass → `02-scope.md` == template. Not company Phase 5. Not semantic `gate-rules.md`.
