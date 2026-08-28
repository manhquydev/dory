---
title: "Isolate flow next unlock"
description: "New isolate script: occupant PASS idea; taxi dory flow -- next fail then pass; world-state flow/01-research.md. No rust. No default. Leftover 5 mint."
status: pending
priority: P1
effort: 3h
branch: main
tags: [dory, isolate, aoe5, flow, next, unlock]
blockedBy: []
blocks: []
created: 2026-08-27
---

# Isolate flow next unlock

## Contract

| Field | Closed |
|---|---|
| Outcome | Isolate: occupant viết `flow/00-idea.md` sạch; taxi `dory flow -- next` với **FLOW_BIN = abs flow.sh**: lần 1 exit **1**, lần 2 exit **0**. Journal `bin` = flow.sh, `args=["next"]`. Sit `Flow 1. next` rồi `Flow 0. next`. **`$ISO_REAL/flow/01-research.md` exists**. Factory sock connectable=0. Leftover 5 mint. |
| Constraints | Script **mới**. Không exec/source 1910/0043/0227/0242/hop/**AOE5 judge**. Không rust. Không cargo leftover. Không start default. Không sit `t13`/`p2R`/`wP`. Không nút Flow. Factory không viết PASS. Stop = `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`. |
| Non-goals | Default occupancy. `prompt --wait` / `occ.report=Working`. Recook 1910/0043/0227/0242/C/AOE5. Fold leftover 5. Fill `01-research.md`. Walk 02–05. `card`. Semantic `gate-rules.md`. Company Phase 5. Retarget PATH. `herdr server stop`. Close `wP`/`w15`/`t13`. |
| Acceptance | Script exit 0 hai lần độc lập (cook + test). Journal: đúng hai `flow/result`, `bin` = abs `~/.claude/skills/flow/runner/flow.sh`, `args=["next"]`, codes `[1,0]`. Taxi1 stdout `FAIL` + `00-idea`. Taxi2 stdout `unlocked stage 1`. Sit needles `Flow 1. next` rồi `Flow 0. next`. `01-research.md` exists (do not fill). Leftover 5 mint. Sock connectable=0. PATH `dory` empty. Repo `.dory/` không đổi. Review critical 0. |

Nguồn: [1656 research-01](../reports/260827-1656-research-01-flow-next.md) · [1656 research-02](../reports/260827-1656-research-02-isolate-next.md) · [1638 eval-next](../reports/260827-1638-eval-next.md) · [1638 eval-synth](../reports/260827-1638-eval-synth.md) · [1650 accept](../reports/260827-1650-brainstorm-eval-accept.md) · AOE5 paid [1122](../260827-1122-aoe-5-isolate-flow-judge/plan.md)

## Scope Challenge

```
- Existing: AOE5 isolate gate 00-idea [1,0] paid; C no-spawn b544f5f; leftover 5 mint; sock dead; PATH gone; desk glance paints arg0
- Requested: implement the filtered next cook N — isolate taxi flow -- next unlock 01-research.md
- Complexity: 1 new script (copy AOE5 law + taxi/journal/needle delta). 0 rust. 3 phases
- Mode: HOLD (no --yagni). Company Phase 5 / 6-stage / default sit / p5 lock / skill taxi paper = named unpaid, not this cook
```

## Approaches

| # | Approach | Verdict |
|---|---|---|
| **A** | New isolate script. Same FAIL idea + occupant PASS. Taxi `flow -- next`. World-state `01-research.md`. | **Chọn.** |
| B | Recook `gate 00-idea` | Không unlock. Paid rồi. |
| C | p5 lock trap 10 / skill taxi paper | Đúng lỗ, sai độ cao. Residual. |
| D | Nút `dory flow next` / rust `next` | **Cấm** CHARTER. Taxi needs `--`. |
| E | Fill 01 + walk 02–05 / `card` | Company Phase 5. Unpaid. |
| F | Default sit | Unpaid. Cấm wave này. |

## Scout 16:56

- Unlock = `flow.sh` `cmd_next` (`:950-1030`). Dirty idea → `FAIL: gate for stage 00-idea is not clean.` (`:966`). Clean + missing 01 → `cp` `_templates/01-research.md` (`:1024`) + `PASS: … unlocked stage 1 (flow/01-research.md)` (`:1026`).
- `/tmp` probe (no dory): FAIL 58ms / PASS 57ms ≪ 15s (`HEAD:rust/src/flow.rs:14`). `00-idea.md` sha unchanged by `next`.
- Empty-tree first `next` copies `00-idea` rc=0 (`:953-962`) — **not** the án. Isolate must mint FAIL idea first.
- Dory has no `next` (`HEAD:rust/src/flow.rs:3`). Glance `Flow {n}. {arg0}` (`desk.rs:3450-3458`) → needles `Flow 1. next` / `Flow 0. next`.
- AOE5 taxi2 `clean` is gate-verb only. Word `clean` appears in **both** next FAIL (`not clean`) and PASS (`gate clean`). Journal taxi2 must require `unlocked stage 1`, not `clean`.
- Trap 13 of 1122 forbade `next` **inside that cook**. Unlock is remainder’s first world-state.

## Bẫy

1. Recook AOE5 `gate 00-idea` or keep journal `GATE`/`clean` / sit `Flow *. gate` — **cấm**. Án = `next` fail **rồi** pass + `01-research.md`.
2. Factory viết file PASS rồi taxi 2 — **cấm**. Occupant mới được lật `00-idea.md`.
3. Exec/source 1910/0043/0227/0242/hop/**judge** — **cấm**. Copy law, file mới `scripts/dory-isolate-aoe5-flow-next.sh`. Self-refuse regex **adds** `dory-isolate-aoe5-flow-judge`.
4. `PROJECT=$ISO_REAL`. Refuse **any** factory `FLOW_*` lúc vào (not only `FLOW_BIN`/`FLOW_PROJECT_ROOT`). Taxi + setsid: `env -u` the class, then pin only `FLOW_BIN` + `FLOW_PROJECT_ROOT` + `FLOW_LOG_DISABLE` + `DO_NOT_TRACK`. Không FAIL-hard `~/.claude/flow` mtime.
5. Pin `FLOW_BIN` after `realpath`; basename `flow.sh`; `-x`; refuse `/bin/true`. Journal `args` must be `["next"]` (prevents gate taxi claiming N).
6. Sit pane/tab **exact**: ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. `herdr pane get` `tab_id` khớp `SIT_TAB`. Close only wave tabs. Không `herdr server stop`.
7. Stop = 1910 `compound_stop` (`:69-100`): identity + sock real / not symlink / under `ISO_REAL` **rồi** `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`. Cấm `iso()` / `DORY_SOCKET=` on stop.
8. Factory `dory` / leftover ELF / isolate ELF trên factory XDG. Mọi dory = `"$SIT_DORY"` với isolate env.
9. Cargo `/home/manhquy/Downloads/flow/dory`. Fold leftover 5. `git add -u` / `commit -a` / `git add -A`.
10. `prompt --wait` / `occ.report = Working` (0242 trap 10).
11. `herdr pane run` attach (TUI không exit). Attach = `send-text` + `enter` + `wait-output`. Pane id **trước** option (Herdr 0.7.5).
12. Claim “company Phase 5” / default trống “đã sâu”. Slice này = isolate unlock. Default + 6-stage + semantic vẫn unpaid.
13. `assess` / `card` / PASS-path `check` / fill `01-research.md` / walk 02–05 — không gọi. 1122 trap 13 was cook-scoped; this cook’s án is **one** `next` only.
14. Repo `/home/manhquy/Downloads/flow/dory/.dory` create/mtime/ino đổi = FAIL.
15. Factory `DORY_*` / `PI_CODING_AGENT_DIR` set lúc vào = refuse. Isolate **server only** được `PI_CODING_AGENT_DIR=$FACTORY_HOME/.omp/agent`.
16. Không `export HOME` script-scope. Entry: `HOME` == `FACTORY_HOME`. Server + taxi prefix: `HOME="$ISO_REAL/home"`.
17. Không rust hunk. Không isolate `reset --hard`. Không cargo leftover **và không** cargo isolate. ELF missing = FAIL. Không rebuild. Pin leftover ELF sha `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` (stat only). Pin `SIT_DORY` sha256 `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3` (land-4b70f79 inode). Không hardcode path string `land-4b70f79` in script — hash the env pin.
18. Sock probe = AF_UNIX `…/dory/default/dory.sock`. Không probe thư mục session. Existence alone ≠ FAIL (only connectable).
19. Attach = 1910 `:331` verbatim: `cd "$ISO_REAL" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE "$SIT_DORY" attach`.
20. Server start = 0242 **`:340-353`** (`mkdir`/`ln -sfn` ISO/bin **rồi** `setsid`). `realpath` ISO/bin/dory == `SIT_DORY` == land sha. `PI_CODING_AGENT_DIR` **chỉ** dòng setsid. Isolate occupants: AOE5 `start_omp` `--no-session --no-skills --no-rules --no-extensions`. Không FAIL-hard `~/.omp` mtime (wP đang omp).
21. Taxi 2 **không** chạy ngay sau `agent prompt`. Poll `$ISO_REAL/flow/00-idea.md` khớp PASS bytes (~180s). Rồi mới taxi 2. Cấm `--wait`.
22. Journal đúng **hai** `flow/result`, codes `[1,0]`, cùng `bin`, **cả hai** `args == ["next"]`. `copy_journal || fail` **và** copy `01-research.md` (sha vs `_templates/01-research.md`) **trước** mọi `wipe_iso` (kể cả `fail`/`EXIT`). Receipt = journal + `ls`/`sha` of 01. Ban copy filled 01 body vào reports.
23. Leftover mint **bảng path+sha** (không snap-then-MATCH; không checkout/restore leftover):
    `README.md` `68190a5ffa073c082aa318aad5ed032e13cc90e3`
    `rust/src/attach.rs` `602479094e84d31ad6f017775a3d55aeb485c644`
    `rust/src/main.rs` `373d688636ff7315ccd665f450069d8284eb47ff`
    `rust/src/server.rs` `4de1554ad56e248cdcf42f02111b7389b08dae82`
    `rust/tests/p5_attach.rs` `9c28fc3e0f3666498a8952411242d5301f7911de`
    `desk.rs` worktree == `HEAD:rust/src/desk.rs` `4c788562e4fdda10c8edd2878ed1fdd46050c218`
24. Copy-law **cấm** 1910 taxi `:308-309` (`FLOW_BIN=/bin/true`). Script `case $0` refuse paid names **including judge**. `rg` self for `source`/`exec` those scripts.
25. Ship = `git add --` pathspec. **Không** `git add -A` / `ak:git` nếu nó add -A. `rv_left` ≠ “worktree rust clean”. `git log -1 -- rust/` vẫn `b544f5f`.
26. Commit subject không pretends company Phase 5. Dùng `feat(isolate): fail-then-pass flow.sh next`.
27. Journal taxi2 must **not** accept substring `clean` alone (`not clean` is FAIL). Require exact-class stdout `unlocked stage 1 (flow/01-research.md)`. Reject `already exists` / `GATE stage` / `unlocked stage 00` / `flow -- gate`.
28. Empty-tree `next` (no `00-idea.md`) rc=0 is **not** taxi1 **nor** taxi2. Mint FAIL idea before taxi1. Taxi1: `code==1`, stdout `FAIL: gate for stage 00-idea is not clean`, no `unlocked stage`. After taxi1: `! -f 01-research.md` + `idea_still_fail`. Do not trust `idea_still_fail` alone.
29. Taxi2 IFF `cmp -s` PASS file. Self-`rg` `--wait` and `flow -- gate`. Idle/prompt-rc is not the gate. Refuse PASS bytes before occupant prompt.
30. Sit needles necessary, not sufficient. Land = journal stdout + 01 sha. `Flow 0. next` is shared by empty-tree PASS and unlock PASS. Sit hit + weak journal = TEST_FAIL.
31. `self_refuse_paid` + `$0` case include judge **on first paste**. Phase-01: do not `source`/`.`/`exec` judge. Copy-table only.
32. Success Criteria rust = `git log -1 -- rust/` = `b544f5f`. Worktree rust dirty leftover = **pass**. Never `git checkout`/`restore` leftover. Never `git add -A`. Never `ak:git`. Named files only — no glob. Deny `260827-1638-eval-*` from this ship.

## Herdr

Sau gate sạch: tab mới trên `w13`, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Mỗi tab 4–6 OMP. Không split `t13`. Không ngồi factory. Close only wave tabs (kể cả sit tab script mint).

| Tab | Job |
|---|---|
| sit | Shell sạch. `SIT_PANE`/`SIT_TAB`. **Không** `agent start`. |
| cook | Script + chạy 1 lần + cook receipt |
| test | Chạy lại độc lập + test receipt |
| review | Judge-next / leftover / sit-door / fold |
| ship | Pathspec script+plan+reports. Never leftover 5. Không push. |

Factory cook/test/review/ship OMP: skills ON so they may run `ak:cook` / `ak:test` / `ak:code-review`. Ban factory cwd `/flow next` and factory `flow/` mint. Isolate occupants: `--no-session --no-skills --no-rules --no-extensions`. Ship = named-file `git add --` only. **Ban `ak:git`** (`git add -A`). Ban glob.

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [New isolate next-unlock script](./phase-01-start.md) | Pending |
| 2 | [Independent next unlock test](./phase-02-independent-next-unlock-test.md) | Pending |
| 3 | [Review and ship](./phase-03-review-and-ship.md) | Pending |

## Success Criteria

- [ ] New script `scripts/dory-isolate-aoe5-flow-next.sh` exit 0; does not exec/source paid scripts
- [ ] Taxi 1: exit 1, journal `bin` = abs flow.sh, `args=["next"]`, stdout `FAIL` + `00-idea`
- [ ] Occupant (not factory) writes PASS `00-idea.md`
- [ ] Taxi 2: exit 0, same `bin`, stdout `unlocked stage 1`, `$ISO_REAL/flow/01-research.md` exists
- [ ] Sit visible `Flow 1. next` then `Flow 0. next`
- [ ] Leftover 5 `git hash-object` = mint table (path+sha); sock connectable=0; PATH `dory` empty
- [ ] `git log -1 -- rust/` = `b544f5f`; leftover 5 still ` M`; no leftover/isolate cargo; no factory `dory` argv
- [ ] Repo `.dory/` unchanged; factory `flow/` not created

## Remainder (named unpaid — not this cook)

N is **paid** (`bcf7c72`). Sibling cook **O**: [260828-1612-isolate-flow-scope-unlock](../260828-1612-isolate-flow-scope-unlock/plan.md) (isolate fill 01 + `flow -- next` → `02-scope.md`).

- Default occupancy / sit default
- Founder `prompt --wait` / five states `working`/`blocked` / p5 trap-10 lock
- Skill taxi paper (empty `--` → `status`; 15s SIGTERM)
- Full 6-stage + semantic `gate-rules.md` + `card` + fill 02 / walk 03–05 (O takes one `next` only)
- Leftover 5 fold / PATH retarget / leftover ELF rm

## Red Team Review

### Round 1 — 2026-08-27 specialized (security / failure / fold)

| # | Finding | Sev | Disposition |
|---|---|---|---|
| S1 | SIT_DORY path-prefix only; leftover ELF spawn | Critical | **Accept** — hash-pin land sha `2ef20730…`; leftover ELF sha held |
| S2 | PI inherit factory `~/.omp` | Critical | **Accept paper** — isolate occupants AOE5 `--no-session --no-skills…`. **Reject** FAIL-hard `~/.omp` mtime (wP omp; 1122 S8) |
| S3 | Sit pane PATH unmeasured | High | **Accept paper** — sit `type -a dory` empty before attach |
| S4 | ISO/bin without hash | High | **Accept** — realpath + land sha |
| S5 | Refuse only two FLOW_* names | High | **Accept paper** — refuse any `FLOW_*`; pin four. **Reject** FAIL `~/.claude/flow` mtime |
| S6 | Phase 2 sit-t13 slogans | High | **Accept** — exact `w13:t13` / `p2R` / `wP` |
| S7 | rv_sit cannot fail S1–S5 | High | **Accept paper** — land sha + sit PATH. **Reject** `/proc` gold-plate + `~/.omp` FAIL |
| S8 | Stop one-liner vs compound_stop | Medium | **Accept** — paste `:69-100` |
| F1 | 01 existence optional then wipe | Critical | **Accept** — required sha vs template before wipe |
| F2 | taxi2 on idle / `--wait` | Critical | **Accept** — taxi2 IFF `cmp` PASS; `rg --wait` |
| F3 | journal `clean` launder | High | **Accept** — replace AOE5 journal helpers |
| F4 | empty-tree fake taxi1 | High | **Accept** — taxi1 `code==1` + FAIL line |
| F5 | empty-tree fake taxi2 | High | **Accept** — reject `unlocked stage 00` |
| F6 | sit cannot prove unlock | High | **Accept** — sit necessary; land = stdout + file |
| F7 | args not in AOE5 helpers | High | **Accept** — both rows `args == ["next"]` |
| F8 | `copy_journal \|\| true` | Medium | **Accept** — `\|\| fail` + 01 before wipe |
| F9 | self_refuse omits judge | Medium | **Accept** — first paste includes judge |
| L1 | Success Criteria rust-clean | Critical | **Accept** — `git log -1 -- rust/` only |
| L2 | `git add -A` / glob / 1638 mountain | Critical | **Accept** — named files; ban `ak:git`; deny 1638 |
| L3 | leftover table bare names | High | **Accept** — path+sha |
| L4 | cargo-if-missing door | High | **Accept** — ELF missing = FAIL |
| L5 | 01 exists without template | High | **Accept** — `cmp`/sha vs template |
| L6 | skills ON factory `/flow next` | High | **Accept split** — factory OMP may use `ak:*`; isolate occupants `--no-skills`; ban factory `flow/` |
| L7 | source omitted in phase-01 | High | **Accept** — no `source`/`.`/`exec` |
| L8 | 0242 `:346-353` misses mkdir/ln | High | **Accept** — cite `:340-353` |
| L9 | subject pretends Phase 5 | Medium | **Accept** — subject `feat(isolate): fail-then-pass flow.sh next` |

Rejected as cook: FAIL-on-`~/.omp` mtime; FAIL-on-`~/.claude/flow` mtime; `/proc` ELF scan gold-plate; all-panes `--no-skills` (factory needs `ak:cook`).

### Whole-Plan Consistency Sweep

- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas: land sha pin; journal helpers replaced; 01 sha required; 0242 `:340-353`; rust success = log not diff; named ship; isolate occupants `--no-skills`; taxi2 = poll only
- Unresolved contradictions: 0
- Reconciled stale: `Optionally` 01 copy; `No rust diff vs b544f5f`; `:346-353` alone; `do not exec` without source; `ak:git if`

## Validation Log

### Session 1 — 2026-08-27
**Trigger:** User ordered plan → red-team → validate → cook for hunk N. Settled by 1656 research + 1638/1650 + R1.
**Questions asked:** 0 live (no material fork left)

#### Questions & Answers

1. **[Architecture]** Unlock verb?
   - Options: taxi `dory flow -- next` fail-then-pass (Recommended) | recook `gate` | rust `next` button
   - **Answer:** taxi `dory flow -- next`
   - **Rationale:** research-01; CHARTER taxi; 1122 trap 13 cook-scoped

2. **[Scope]** Company Phase 5 vs isolate unlock?
   - Options: isolate unlock + `01-research.md` template (Recommended) | fill 01 + 6-stage now
   - **Answer:** isolate unlock only
   - **Rationale:** HOLD. Remainder named unpaid. Subject must not claim company.

3. **[Risks]** Taxi 2 timing + 01 proof?
   - Options: poll PASS + required 01 sha before wipe (Recommended) | `prompt --wait` | factory Write + receipt “existed”
   - **Answer:** poll + required 01 sha
   - **Rationale:** F1/F2/F7. `--wait` = trap 10. Optional existence = wipe lie.

#### Confirmed Decisions
- A = new script, `PROJECT=$ISO_REAL`, real skill `flow.sh`, occupant PASS, journal `[1,0]` `args=["next"]`, 01 sha == template
- Copy 1910 attach+stop, 0242 `:340-353`; never source judge; never 1910 taxi true
- No rust, no default, leftover mint path+sha, land ELF hash-pin
- Ship named files only; subject `feat(isolate): fail-then-pass flow.sh next`

#### Action Items
- None remaining on the plan

#### Verification Results
- **Tier:** Standard (3 phases)
- **Claims checked:** 10
- **Verified:** 10 | **Failed:** 0 | **Unverified:** 0
- `flow.sh:966` FAIL line — VERIFIED
- `flow.sh:1024-1026` unlock + `cp` template — VERIFIED
- `flow.sh:1019-1022` already-exists — VERIFIED
- `flow.sh:953-962` empty-tree rc=0 — VERIFIED
- `HEAD:rust/src/flow.rs:3` no `next`/`card`/`check`; `:14` 15s — VERIFIED
- `HEAD:rust/src/desk.rs:3450-3458` `Flow {n}. {arg0}` — VERIFIED
- `HEAD:rust/src/attach.rs:326-332` no spawn — VERIFIED
- 0242 `:340-353` mkdir/ln + setsid — VERIFIED
- 1910 attach `:331` + `compound_stop` `:69-100` — VERIFIED
- Land ELF sha `2ef20730…`; leftover ELF `3ba0e3bc…`; leftover 5 mint MATCH; rust log `b544f5f`; `_templates/01-research.md` exists — VERIFIED

#### Whole-Plan Consistency Sweep
- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas checked: 12
- Reconciled stale references: rust-clean checkbox; optional 01; `:346-353` without mkdir
- Unresolved contradictions: 0

<!-- slug: isolate-flow-next-unlock -->
