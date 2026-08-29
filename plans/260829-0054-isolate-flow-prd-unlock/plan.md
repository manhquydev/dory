---
title: "Isolate flow prd unlock"
description: "New isolate script: factory mint O world-state (00+01 PASS + 02 template); occupant PASS scope; taxi dory flow -- next fail then pass; world-state flow/03-prd.md. No rust. No default. Leftover 5 mint."
status: completed
priority: P1
effort: 3h
branch: main
tags: [dory, isolate, aoe5, flow, next, prd, unlock]
blockedBy: [260828-1612-isolate-flow-scope-unlock]
blocks: []
created: 2026-08-29
---

# Isolate flow prd unlock

## Contract

| Field | Closed |
|---|---|
| Outcome | Isolate: factory mint `00-idea.md` PASS + `01-research.md` PASS + `02-scope.md` = template; taxi `dory flow -- next` + **FLOW_HARNESS_DISABLE=1**: lần 1 exit **1**, lần 2 exit **0**. Journal `bin` = abs flow.sh, `args=["next"]`. Sit `Flow 1. next` rồi `Flow 0. next`. **`$ISO_REAL/flow/03-prd.md` exists** sha == template. Factory sock connectable=0. Leftover 5 mint. |
| Constraints | Script **mới**. Không exec/source 1910/0043/0227/0242/hop/judge/N/**O**. Không rust. Không default sit. Không sit `t13`. Factory không viết PASS **02**. Stop 2357 compound. |
| Non-goals | Company Phase 5. Fill 03. Walk 04–05. `card`. Semantic. Leftover fold. `--wait`. Recook O/N/AOE5. |
| Acceptance | Cook + test exit 0. Journal two `flow/result`, codes `[1,0]`, taxi1 FAIL `02-scope`, taxi2 `unlocked stage 3 (flow/03-prd.md)`. 03 sha == template. Review critical 0. Pathspec ship. No push. |

Nguồn: [0052 scout](../reports/260829-0052-scout.md) · [research-01](../reports/260829-0052-research-01-flow-prd.md) · [research-02](../reports/260829-0052-research-02-isolate-prd.md) · [brainstorm](../reports/260829-0052-brainstorm-accept.md) · O paid [1612](../260828-1612-isolate-flow-scope-unlock/plan.md)

## Scope Challenge

```
- Existing: O unlock-2 paid (bcab751); leftover 5 mint; sock dead; PATH gone; rust b544f5f
- Requested: next cook P — isolate taxi flow -- next unlock 03-prd.md
- Complexity: 1 new script (copy O). 0 rust. 3 phases
- Mode: HOLD. Company blob / leftover fold / --wait = named unpaid
```

## Approaches

| # | Approach | Verdict |
|---|---|---|
| **A** | New isolate script. Mint O world-state. Occupant PASS 02. Unlock `03-prd.md`. | **Chọn.** |
| B | Recook O | Paid. |
| C | Paper / leftover fold / default sit | Sai độ cao / kill. |

## Bẫy

1. Recook O / journal `unlocked stage 2` — **cấm**. Án = FAIL `02-scope` rồi `unlocked stage 3`.
2. Factory viết PASS **02** — **cấm**. Factory **được** mint PASS 00 + PASS 01.
3. Exec/source O/N/judge — **cấm**. File `scripts/dory-isolate-aoe5-flow-prd.sh`. Self-refuse adds `dory-isolate-aoe5-flow-scope` + next + judge.
4. Refuse any factory `FLOW_*`. Taxi pin `FLOW_BIN` + `FLOW_PROJECT_ROOT` + `FLOW_LOG_DISABLE` + **`FLOW_HARNESS_DISABLE=1`** + `DO_NOT_TRACK`.
5. Sit ≠ `t13`/`p2R`/`wP`. Close only wave tabs. Không `herdr server stop`.
6. Stop compound 2357. Land ELF sha `2ef20730…`. Leftover ELF `3ba0e3bc…` stat-only.
7. No rust. No leftover cargo. No `ak:git` / `git add -A`. Leftover mint path+sha (same table as O).
8. Taxi2 IFF `cmp` PASS **02**. Poll ~180s. Cấm `--wait`.
9. After taxi1: no `03-prd.md`; 02 still FAIL; 00+01 still PASS.
10. After taxi2: 03 sha == `_templates/03-prd.md` still `[FILL]`. 00+01 still PASS. Do not fill 03.
11. Journal both `args=["next"]`. Reject `already exists` / stage 2 / stage 1 / stage 00 / bare `clean`.
12. Subject `feat(isolate): fail-then-pass flow.sh prd`. Deny ship `260827-1743-eval-*` / `260828-ensure-aoe5-flow-scope-*` except remainder pointer on 1612.
13. Sit needles necessary, not sufficient. Land = stdout + 03 sha.
14. Success rust = `git log -1 -- rust/` = `b544f5f`.

## Herdr

Tab mới `w13`, `--no-focus`, cwd dory. 4–6 pane. Không split `t13`.

| Tab | Job |
|---|---|
| sit / testsit | Shell sạch. Không agent. |
| cook | Chạy script + cook receipt |
| test | Chạy lại độc lập |
| review | next / leftover / sit / fold |
| ship | Named pathspec. Ban `ak:git`. Không push. |

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [New isolate prd-unlock script](./phase-01-start.md) | Done |
| 2 | [Independent prd unlock test](./phase-02-independent-prd-unlock-test.md) | Done |
| 3 | [Review and ship](./phase-03-review-and-ship.md) | Done |

## Success Criteria

- [x] `scripts/dory-isolate-aoe5-flow-prd.sh` exit 0; self-refuses O/N/judge
- [x] Taxi 1 FAIL `02-scope`; no `03-prd.md`
- [x] Occupant writes PASS `02-scope.md`
- [x] Taxi 2 `unlocked stage 3`; 03 sha == template
- [x] Sit `Flow 1. next` / `Flow 0. next`
- [x] Leftover 5 mint; sock dead; PATH empty; rust log `b544f5f`

## Remainder (named unpaid)

- Default occupancy
- `--wait` / leftover fold / skill taxi paper
- Fill 03 / walk 04–05 / `card` / semantic / company Phase 5

## Red Team Review

### Round 1 — 2026-08-29 specialized (security / failure / fold)

See `plans/reports/260829-0054-redteam-r1-{security,failure,fold}.md`.

| # | Finding | Sev | Disposition |
|---|---|---|---|
| P-S1 | Source/exec O | Critical | **Accept** — `$0` + regex add scope |
| P-S2 | Missing harness pin | High | **Accept** — keep O pin |
| P-F1 | Journal still stage 2 | Critical | **Accept** — require stage 3; reject 2 |
| P-F2 | Factory PASS 02 | Critical | **Accept** — factory PASS 00+01 only |
| P-F3 | Missing 02 → recook O | Critical | **Accept** — mint 02 template; idx conceptually 2 |
| P-F4 | Dirty 00/01 unlocks 03 | High | **Accept** — assert 00+01 PASS before taxi1 and after taxi2 |
| P-L1 | Ship O mountain / leftover 5 | High | **Accept** — named files; deny leftover + O script body |
| P-L2 | Subject pretends Phase 5 | Medium | **Accept** — `feat(isolate): fail-then-pass flow.sh prd` |

### Whole-Plan Consistency Sweep

- Files: plan.md + 3 phases
- Unresolved contradictions: 0

## Validation Log

### Session 1 — 2026-08-29
**Trigger:** `/goal` after O paid. Settled by 0052 scout/research/brainstorm.
**Questions:** 0 live

#### Confirmed Decisions
- A = new script, mint O world-state, occupant PASS 02, unlock-3
- Copy O law; never source O; leftover mint; land hash-pin
- Subject `feat(isolate): fail-then-pass flow.sh prd`

#### Verification Results
- **Tier:** Standard
- **Claims checked:** 8
- **Verified:** 8 | **Failed:** 0 | **Unverified:** 0
- `flow.sh:122` STAGES includes 03-prd — VERIFIED
- `flow.sh:136-153` idx contiguous — VERIFIED
- `flow.sh:966` FAIL current — VERIFIED
- `flow.sh:1024-1026` unlock + cp — VERIFIED
- `_templates/03-prd.md` exists — VERIFIED
- O paid `bcab751` on HEAD — VERIFIED
- Land/leftover ELF + leftover 5 mint 00:52 — VERIFIED
- `HEAD:rust/src/flow.rs:3` no next button — VERIFIED

#### Whole-Plan Consistency Sweep
- Unresolved contradictions: 0

<!-- slug: isolate-flow-prd-unlock -->
