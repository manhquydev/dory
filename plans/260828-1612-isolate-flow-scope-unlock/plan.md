---
title: "Isolate flow scope unlock"
description: "New isolate script: factory mint N world-state (00 PASS + 01 template); occupant PASS research; taxi dory flow -- next fail then pass; world-state flow/02-scope.md. No rust. No default. Leftover 5 mint."
status: pending
priority: P1
effort: 3h
branch: main
tags: [dory, isolate, aoe5, flow, next, scope, unlock]
blockedBy: [260827-1657-isolate-flow-next-unlock]
blocks: []
created: 2026-08-28
---

# Isolate flow scope unlock

## Contract

| Field | Closed |
|---|---|
| Outcome | Isolate: factory mint `00-idea.md` PASS + `01-research.md` = template; taxi `dory flow -- next` với **FLOW_BIN = abs flow.sh** + **FLOW_HARNESS_DISABLE=1**: lần 1 exit **1**, lần 2 exit **0**. Journal `bin` = flow.sh, `args=["next"]`. Sit `Flow 1. next` rồi `Flow 0. next`. **`$ISO_REAL/flow/02-scope.md` exists** sha == template. Factory sock connectable=0. Leftover 5 mint. |
| Constraints | Script **mới**. Không exec/source 1910/0043/0227/0242/hop/**AOE5 judge**/**N**. Không rust. Không cargo leftover. Không start default. Không sit `t13`/`p2R`/`wP`. Không nút Flow. Factory không viết PASS **01**. Stop = `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`. |
| Non-goals | Default occupancy. `prompt --wait` / `occ.report=Working`. Recook 1910/0043/0227/0242/C/AOE5/**N**. Fold leftover 5. Fill `02-scope.md`. Walk 03–05. `card`. Semantic `gate-rules.md`. Company Phase 5. Retarget PATH. `herdr server stop`. Close `wP`/`w15`/`t13`. |
| Acceptance | Script exit 0 hai lần độc lập (cook + test). Journal: đúng hai `flow/result`, `bin` = abs `~/.claude/skills/flow/runner/flow.sh`, `args=["next"]`, codes `[1,0]`. Taxi1 stdout `FAIL` + `01-research`. Taxi2 stdout `unlocked stage 2`. Sit needles `Flow 1. next` rồi `Flow 0. next`. `02-scope.md` exists (do not fill). Leftover 5 mint. Sock connectable=0. PATH `dory` empty. Repo `.dory/` không đổi. Review critical 0. |

Nguồn: [1612 research-01](../reports/260828-1612-research-01-flow-scope.md) · [1612 research-02](../reports/260828-1612-research-02-isolate-scope.md) · [1743 eval-next](../reports/260827-1743-eval-next.md) · [1743 eval-synth](../reports/260827-1743-eval-synth.md) · N paid [1657](../260827-1657-isolate-flow-next-unlock/plan.md)

## Scope Challenge

```
- Existing: N isolate next → 01 template paid (bcf7c72); C no-spawn b544f5f; leftover 5 mint; sock dead; PATH gone
- Requested: next cook O — isolate taxi flow -- next unlock 02-scope.md after occupant PASS 01
- Complexity: 1 new script (copy N law + taxi/journal/needle delta). 0 rust. 3 phases
- Mode: HOLD (no --yagni). Company Phase 5 / 6-stage / semantic / default sit / leftover fold / --wait = named unpaid
```

## Approaches

| # | Approach | Verdict |
|---|---|---|
| **A** | New isolate script. Mint N world-state. Taxi `flow -- next`. Occupant PASS 01. World-state `02-scope.md`. | **Chọn.** |
| B | Recook N `00→01` | Paid rồi. |
| C | Skill taxi paper / p5 `--wait` | Sai độ cao. |
| D | Nút `dory flow next` / rust `next` | **Cấm** CHARTER. |
| E | Semantic fill 01 + `gate-rules.md` + `card` | Company. Unpaid. |
| F | Default sit | Unpaid. Cấm wave này. |

## Scout 16:12

- Unlock-2 = `flow.sh` `cmd_next` on idx 1 (`:136-153`, `:950-1030`). Dirty 01 → `FAIL: gate for stage 01-research is not clean.` (`:966`). Clean + missing 02 → `cp` template + `PASS: … unlocked stage 2 (flow/02-scope.md)` (`:1024-1026`).
- `gate_durable_hook 01-research` (`:1027`) seeds harness unless `FLOW_HARNESS_DISABLE` (`:289-294`).
- Sit needles same as N (`Flow *. next`). Land = stdout `unlocked stage 2` + 02 sha.
- N self-refuse omits itself. O must add `dory-isolate-aoe5-flow-next`.

## Bẫy

1. Recook N taxi 00→01 / journal `unlocked stage 1` / sit-only land — **cấm**. Án = `next` fail **rồi** pass + `02-scope.md`.
2. Factory viết PASS **01** rồi taxi 2 — **cấm**. Occupant mới được lật `01-research.md`. Factory **được** mint PASS **00** (N world-state).
3. Exec/source 1910/0043/0227/0242/hop/judge/**N** — **cấm**. File mới `scripts/dory-isolate-aoe5-flow-scope.sh`. Self-refuse regex **adds** `dory-isolate-aoe5-flow-next`.
4. `PROJECT=$ISO_REAL`. Refuse **any** factory `FLOW_*` lúc vào. Taxi + setsid: `env -u` the class, then pin `FLOW_BIN` + `FLOW_PROJECT_ROOT` + `FLOW_LOG_DISABLE` + **`FLOW_HARNESS_DISABLE=1`** + `DO_NOT_TRACK`.
5. Pin `FLOW_BIN` after `realpath`; basename `flow.sh`; `-x`; refuse `/bin/true`. Journal `args` must be `["next"]`.
6. Sit pane/tab **exact**: ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. Close only wave tabs. Không `herdr server stop`.
7. Stop = 1910 `compound_stop` (`:69-100`): identity + sock real / not symlink / under `ISO_REAL` **rồi** `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`.
8. Factory `dory` / leftover ELF / isolate ELF trên factory XDG. Mọi dory = `"$SIT_DORY"` với isolate env.
9. Cargo leftover tree. Fold leftover 5. `git add -u` / `commit -a` / `git add -A`.
10. `prompt --wait` / `occ.report = Working`.
11. `herdr pane run` attach. Attach = `send-text` + `enter` + `wait-output`. Pane id **trước** option.
12. Claim company Phase 5 / O = semantic research. Mechanical boxes only.
13. `assess` / `card` / fill `02-scope.md` / walk 03–05 — không gọi.
14. Repo `/home/manhquy/Downloads/flow/dory/.dory` đổi = FAIL.
15. Factory `DORY_*` / `PI_CODING_AGENT_DIR` set lúc vào = refuse. Isolate **server only** được `PI_CODING_AGENT_DIR=$FACTORY_HOME/.omp/agent`.
16. Không `export HOME` script-scope. Server + taxi: `HOME="$ISO_REAL/home"`.
17. Không rust. ELF missing = FAIL. Pin leftover ELF sha `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14`. Pin `SIT_DORY` sha256 `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3`.
18. Sock probe = AF_UNIX `…/dory/default/dory.sock`. Existence alone ≠ FAIL (only connectable).
19. Attach = 1910 `:331` verbatim.
20. Server start = 0242 **`:340-353`**. Isolate occupants: `--no-session --no-skills --no-rules --no-extensions`.
21. Taxi 2 **không** chạy ngay sau `agent prompt`. Poll `$ISO_REAL/flow/01-research.md` khớp PASS bytes (~180s). Cấm `--wait`.
22. Journal đúng **hai** `flow/result`, codes `[1,0]`, cùng `bin`, **cả hai** `args == ["next"]`. `copy_journal || fail` **và** copy `02-scope.md` sha vs `_templates/02-scope.md` **trước** wipe. Ban copy filled 02 body vào reports.
23. Leftover mint **bảng path+sha** (same as N trap 23). `desk.rs` == HEAD `4c788562`.
24. Copy-law **cấm** 1910 taxi `/bin/true`. `case $0` refuse paid names **including N**.
25. Ship = `git add --` pathspec. **Không** `ak:git`. `git log -1 -- rust/` vẫn `b544f5f`.
26. Subject `feat(isolate): fail-then-pass flow.sh scope`. Không pretends company Phase 5.
27. Journal taxi2 require `unlocked stage 2 (flow/02-scope.md)`. Reject `already exists` / `GATE stage` / `unlocked stage 1` / `unlocked stage 00` / bare `clean`.
28. After taxi1: `! -f 02-scope.md` + 01 still FAIL + 00 still PASS. Taxi1 stdout `FAIL: gate for stage 01-research is not clean`.
29. Taxi2 IFF `cmp -s` PASS **01**. Self-`rg` `--wait` and `flow -- gate` and `dory-isolate-aoe5-flow-next` source/exec.
30. Sit needles necessary, not sufficient. Land = journal stdout + 02 sha.
31. Before taxi1: 00 == PASS_00; 01 sha == template (`[FILL]`); 02 absent; idx conceptually 1.
32. Success rust = `git log -1 -- rust/` = `b544f5f`. Worktree leftover dirty = pass. Deny `260827-1638-eval-*` and `260827-1743-eval-*` from this ship (except this plan's own 1612 research/redteam).

## Herdr

Sau gate sạch: tab mới trên `w13`, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Mỗi tab 4–6 OMP. Không split `t13`. Close only this-wave tabs. Close leftover eval `t2Q`/`t2R`/`t2S` before mint (they are prior wave).

| Tab | Job |
|---|---|
| sit | Shell sạch. **Không** `agent start`. |
| cook | Script + chạy 1 lần + cook receipt |
| test | Chạy lại độc lập + test receipt |
| review | Judge-scope / leftover / sit-door / fold |
| ship | Pathspec script+plan+reports. Never leftover 5. Không push. |

Factory OMP: skills ON. Isolate occupants: `--no-skills`. Ban factory `flow/` mint. Ban `ak:git`.

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [New isolate scope-unlock script](./phase-01-start.md) | Pending |
| 2 | [Independent scope unlock test](./phase-02-independent-scope-unlock-test.md) | Pending |
| 3 | [Review and ship](./phase-03-review-and-ship.md) | Pending |

## Success Criteria

- [ ] New script `scripts/dory-isolate-aoe5-flow-scope.sh` exit 0; does not exec/source paid scripts including N
- [ ] Taxi 1: exit 1, journal `bin` = abs flow.sh, `args=["next"]`, stdout `FAIL` + `01-research`; no `02-scope.md`
- [ ] Occupant (not factory) writes PASS `01-research.md`
- [ ] Taxi 2: exit 0, same `bin`, stdout `unlocked stage 2`, `$ISO_REAL/flow/02-scope.md` sha == template
- [ ] Sit visible `Flow 1. next` then `Flow 0. next`
- [ ] Leftover 5 mint; sock connectable=0; PATH `dory` empty
- [ ] `git log -1 -- rust/` = `b544f5f`; leftover 5 still ` M`
- [ ] Repo `.dory/` unchanged; factory `flow/` not created

## Remainder (named unpaid — not this cook)

- Default occupancy / sit default
- Founder `prompt --wait` / five states / p5 trap-10 lock
- Skill taxi paper
- Full 6-stage + semantic `gate-rules.md` + `card` + fill 02 / walk 03–05
- Leftover 5 fold / PATH retarget / leftover ELF rm

## Red Team Review

### Round 1 — 2026-08-28 specialized (security / failure / fold)

Baked into traps before first paste (N R1 + O deltas). See `plans/reports/260828-1612-redteam-r1-{security,failure,fold}.md`.

| # | Finding | Sev | Disposition |
|---|---|---|---|
| S1–S8 | N land-hash / PI / sit PATH / ISO/bin / FLOW_* / sit ids / stop compound | — | **Keep** from N |
| O-S1 | Missing `FLOW_HARNESS_DISABLE` → harness seed on unlock-2 | Critical | **Accept** — pin on taxi; self-rg |
| O-S2 | Source/exec N | Critical | **Accept** — `$0` + regex add next |
| F1–F9 | N journal / wipe / `--wait` / empty-tree | — | **Keep**, retarget 01/02 |
| O-F1 | Journal still `unlocked stage 1` | Critical | **Accept** — require stage 2 |
| O-F2 | Factory PASS 01 | Critical | **Accept** — factory PASS 00 only |
| O-F3 | Dirty 00 + clean 01 unlocks 02 | High | **Accept** — assert 00 == PASS_00 before taxi1 and after taxi2 |
| O-F4 | Pre-mint 02 → `already exists` | High | **Accept** — refuse 02 before taxi1; reject already-exists |
| L1–L9 | N leftover / `ak:git` / rust log | — | **Keep** |
| O-L1 | Ship 1743-eval mountain | High | **Accept** — deny `260827-1743-eval-*` |
| O-L2 | Subject pretends Phase 5 | Medium | **Accept** — `feat(isolate): fail-then-pass flow.sh scope` |

### Whole-Plan Consistency Sweep

- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas: harness pin; factory mint 00 PASS + 01 template; occupant PASS 01; unlock-2; self-refuse N; deny 1743 ship
- Unresolved contradictions: 0

## Validation Log

### Session 1 — 2026-08-28
**Trigger:** User `continue next cook` after 1743 synth named O. Settled by 1612 research + 1743 next/synth.
**Questions asked:** 0 live (no material fork)

#### Confirmed Decisions
- A = new script, mint N world-state, taxi `flow -- next`, occupant PASS 01, journal `[1,0]` `args=["next"]`, 02 sha == template
- `FLOW_HARNESS_DISABLE=1`; never source N; never fill 02
- No rust, no default, leftover mint, land ELF hash-pin
- Ship named files; subject `feat(isolate): fail-then-pass flow.sh scope`

#### Verification Results
- **Tier:** Standard (3 phases)
- **Claims checked:** 10
- **Verified:** 10 | **Failed:** 0 | **Unverified:** 0
- `flow.sh:136-153` idx contiguous — VERIFIED
- `flow.sh:966` FAIL current stage — VERIFIED
- `flow.sh:1024-1026` unlock + cp — VERIFIED
- `flow.sh:1019-1022` already-exists — VERIFIED
- `flow.sh:289-294` harness_available + DISABLE — VERIFIED
- `flow.sh:660-671` hook 01-research — VERIFIED
- `_templates/02-scope.md` exists — VERIFIED
- `HEAD:rust/src/flow.rs:3` no next button — VERIFIED
- N script self-refuse omits itself `:44-46` `:263-276` — VERIFIED
- Land/leftover ELF + leftover 5 mint 16:12 — VERIFIED

#### Whole-Plan Consistency Sweep
- Unresolved contradictions: 0

<!-- slug: isolate-flow-scope-unlock -->
