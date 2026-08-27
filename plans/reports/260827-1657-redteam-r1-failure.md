---
type: redteam
date: 2026-08-27
time: 16:57
round: r1
role: Flow Tracer
lens: Hostile Failure Mode Analyst + journal/taxi/needle
plan: /home/manhquy/Downloads/flow/dory/plans/260827-1657-isolate-flow-next-unlock/
status: FAIL — plan names several traps, then leaves copy-AOE5 holes that re-open them
---

# Redteam r1 — isolate flow next unlock (plan, not code)

Verification: `flow.sh` `cmd_next`/`scan_gate`, `git show HEAD` glance + taxi timeout, AOE5 `taxi`/`journal_taxi*`/`poll_pass_file`/`wipe_iso`, `plans/reports/260827-1656-research-01-flow-next.md`. No paid-script exec. No dory. No default sock.

## Verdict

The plan knows the right án (`flow -- next`, codes `[1,0]`, `args=["next"]`, `unlocked stage 1`, file exists). It still licenses a cook to copy AOE5 journal/idle/wipe law and claim N. Eight holes below are enough to false-pass without a real unlock.

---

### 1. CRITICAL — `01-research.md` existence is optional, then wiped

**Plan:** `phase-01-start.md:88` — copy journal before wipe; **“Optionally”** copy `01-research.md` proof. `phase-02-independent-next-unlock-test.md:35,42` — do not read wiped path; accept a receipt that *records* the file existed. `phase-03-review-and-ship.md:29` — `rv_next` past-tense “existed”. `plan.md:20,23,79` — án is `$ISO_REAL/flow/01-research.md` exists.

**Land:** `scripts/dory-isolate-aoe5-flow-judge.sh:136-150` `wipe_iso` is `rm -rf -- "$ISO_REAL"`. `copy_journal` `:253-258` copies only `s1.jsonl`, and is a no-op if `JOURNAL` is missing. `teardown` `:398-407` does `copy_journal || true` then wipe.

**Fail:** World-state dies with the ISO. Test/review then grade a sentence in a receipt. Factory can pre-create `flow/01-research.md`; `cmd_next` takes the already-exists branch (`flow.sh:1019-1022`) — rc=0, **no** `cp`, stdout is `already exists`, not unlock. Existence after taxi2 is then true and meaningless. Plan never names `:1019-1022`.

**Fix (must):** Before wipe: assert `! -f 01-research.md` after taxi1; after taxi2 assert `-f` + sha == skill `_templates/01-research.md`; `cp` that file (or `ls -l` + sha) into the receipt as a **required** artifact. `copy_journal || fail`. Journal taxi2 must reject `already exists`.

---

### 2. CRITICAL — taxi2-on-idle / `--wait` on-ramp (poll is not the only signal)

**Plan:** `plan.md:67,78` bans `--wait` and says poll PASS bytes then taxi2. `phase-01-start.md:81` — `agent report --state idle` on both **before taxi1**. `:84` — prompt “Writes exact PASS bytes … **then report idle**”; “`--timeout` OK”. `:85` — poll file, then taxi2.

**Land:** AOE5 does **not** taxi on idle. `report_idle` is only the pre-taxi1 mark (`dory-isolate-aoe5-flow-judge.sh:668-672`). Taxi2 gate is `poll_pass_file` `:385-396` + `:713-730` (`cmp -s` vs `PASS_WANT`, 180s). Factory prompt is `--timeout 180000` **without** `--wait` (`:359`). Occupant text still says report idle (`:378-379`). `dory agent prompt` implements `--wait` (`rust/src/agent.rs:182-185`).

**Fail:** Step 5 already leaves both occupants idle. A cook who waits for idle after prompt returns immediately and taxis on a still-FAIL idea — or, after a factory write, on a clean idea the occupant never touched. `poll_pass_file` cannot tell factory bytes from occupant bytes (`cmp` only). `--timeout OK` + “then report idle” is the 0242 Working-lock on-ramp the non-goal list claims to forbid (`plan.md:22`). Phase-02/03 never `rg --wait` the new script and never require a poll receipt (timestamps / `cmp` log).

**Fix (must):** Taxi2 IFF `cmp -s "$ISO_REAL/flow/00-idea.md" "$PASS_WANT"`. Refuse `--wait` in `$0` (`rg` self). Do not treat idle/prompt-rc as the gate. After taxi1 `idea_still_fail`; refuse if PASS bytes appear before the occupant prompt.

---

### 3. HIGH — journal `clean` is a next-FAIL / already-exists / gate-PASS launder

**Plan:** `plan.md:53,90` and `phase-01-start.md:40,100` say do not accept bare `clean`; require `unlocked stage 1`. `phase-02-independent-next-unlock-test.md:41,59` same.

**Land:** AOE5 `journal_taxi2` is only `"clean" in stdout` (`dory-isolate-aoe5-flow-judge.sh:329-333`) and **does not read `args`**. `cmd_next` FAIL is `FAIL: gate for stage $cur is not clean.` (`flow.sh:966`) — contains `clean`. Unlock PASS is `… gate clean -> unlocked stage $nidx …` (`:1026`) — also `clean`. Already-exists PASS is `… gate clean. Stage $nidx ($nxt.md) already exists` (`:1020`) — `clean`, rc=0, **no** unlock string. `cmd_gate` PASS is `  clean` (`:1427`).

**Fail:** Copy AOE5 `journal_taxi2` and only change `taxi()` to `next`. Taxi2 on already-exists or on `gate 00-idea` after a clean idea: codes `[1,0]`, substring `clean` hits, `01-research.md` “exists”. Research already measured this (`260827-1656-research-01-flow-next.md:31,53,108`). The plan’s prose fix is not a required function body. Phase-01 journal bullets are a list, not a snippet that asserts `r["args"]==["next"]` and `"unlocked stage 1" in rows[1]["stdout"]` and `"already exists" not in stdout` and `"not clean" not in rows[1]`.

**Fix (must):** Replace AOE5 `journal_taxi1/2` entirely. Taxi2 stdout must contain `unlocked stage 1 (flow/01-research.md)` and must not contain `already exists` / `GATE stage` / `unlocked stage 00`. Both rows `args == ["next"]`.

---

### 4. HIGH — empty-tree `next` rc=0 is a fake taxi1; `idea_still_fail` still passes

**Plan:** `plan.md:51,91` trap 28 — empty-tree is not taxi1; mint FAIL idea first.

**Land:** `current_stage_idx` < 0 → `cmd_next` copies `_templates/00-idea.md`, prints `PASS: unlocked stage 00 -> flow/00-idea.md`, **return 0** (`flow.sh:953-962`). That template is still mechanically dirty (`_templates/00-idea.md:4-16` unchecked + `[FILL]`). `idea_still_fail` (`dory-isolate-aoe5-flow-judge.sh:269-277`) is true after empty-tree. Glance is `Flow 0. next` (`desk.rs:3450-3458`, arg0=`next`, code=0) — **same chrome as taxi2 success**.

**Fail:** Skip mint. Taxi1 = empty-tree: rc=0, stdout has `00-idea` (phase-01:39’s second needle), file still FAIL, `idea_still_fail` green. Sit wait for `Flow 1. next` misses unless cook skips it or attaches later. Occupant (or factory) then writes PASS; taxi2 unlocks. Journal becomes `[0,0]` — caught only if codes are enforced. Trap 28’s own after-taxi1 check (`idea_still_fail`) does **not** detect empty-tree. Phase-01 `journal_taxi1` never rejects `unlocked stage 00` or requires `code==1`.

**Fix (must):** Mint FAIL idea before any taxi. Taxi1: `code==1`, stdout has `FAIL: gate for stage 00-idea is not clean`, no `unlocked stage`. Sit `Flow 1. next` is mandatory, not chrome.

---

### 5. HIGH — empty-tree as fake **taxi2** (delete `00-idea` after fail)

**Plan:** Trap 28 is taxi1-only. Taxi2 spec (`phase-01-start.md:40`) is rc=0 + `unlocked stage 1` + file exists.

**Land:** After a real fail `next`, delete `$ISO_REAL/flow/00-idea.md`. Second `next` is empty-tree again (`flow.sh:953-962`): rc=0, `args=["next"]`, codes `[1,0]`, stdout `unlocked stage 00 -> flow/00-idea.md`. **No** `01-research.md`. Glance `Flow 0. next` (`desk.rs:3458`).

**Fail:** Sit acceptance (`plan.md:23,121`; `phase-02-independent-next-unlock-test.md:18,58`) lights green. Journal `bin`+`args` match (`phase-01-start.md:86` calls that the bằng). If taxi2 stdout check is `00-idea` / `PASS` / `unlocked` / AOE5 `clean` (empty-tree has none of `clean` — unless they only check rc+sit+args), the cook ships without stage-1 unlock. Phase-02:41 saves this **only if** `unlocked stage 1` is actually coded.

**Fix (must):** Taxi2 stdout exact unlock line. Reject `unlocked stage 00`. Required on-disk `01-research.md` (finding 1) before wipe.

---

### 6. HIGH — glance paints arg0, not stdout; sit cannot prove unlock

**Plan:** `plan.md:52` — needles `Flow 1. next` / `Flow 0. next` from glance. `phase-01-start.md:86` — sit is “chrome phụ; journal `bin`+`args` là bằng”. Acceptance still requires both sit needles (`plan.md:23`; `phase-02-independent-next-unlock-test.md:58` TEST_FAIL on sit miss even if journal is right).

**Land:** `FlowGlance.arg0 = top_json_first_arg` (`desk.rs:2321-2358`). Footer `Flow {n}. {payload}` (`:3450-3458`). Stdout is not read. Nested stdout is ignored (`:5023-5027`). Timeout paints `Flow lỗi. timed out after 15000ms` (`:5036-5042`; `flow.rs:14,201-211`). Research already said this (`260827-1656-research-01-flow-next.md:118-123`).

**Fail:** Sit proves `args[0]=="next"` and codes 1 then 0. It does **not** prove FAIL vs unlock vs empty-tree vs already-exists vs `flow -- next extra`. `phase-01-start.md:86` then tells the cook that `bin`+`args` is enough for taxi2 — which is exactly how a gate recook with a later `next` extra-arg, or empty-tree taxi2, survives a sit-first review. Inverse of phase-02:58 is missing: sit hit + weak journal must be TEST_FAIL.

**Fix (must):** Sit is necessary and not sufficient. Land = journal stdout + file. Document that `Flow 0. next` is shared by empty-tree PASS and unlock PASS.

---

### 7. HIGH — `args=["next"]` is not in AOE5 journal helpers; gate taxi can still claim N

**Plan:** `plan.md:62,79` — journal `args` must be `["next"]` so a gate taxi cannot claim N. `phase-02-independent-next-unlock-test.md:39`.

**Land:** Paid taxi is `"$SIT_DORY" flow -- gate 00-idea` (`dory-isolate-aoe5-flow-judge.sh:261-266`) → `args=["gate","00-idea"]`. `journal_taxi1` / `journal_taxi2` (`:280-334`) never read `args`. Empty `--` becomes `status` (`flow.rs:34-37`). No `--` is usage rc=2 and **no** `flow/result` (`:26-28`). Dory has no `next` (`flow.rs:3`).

**Fail:** Copy-law says change `taxi()` and “see §2” for journal. A cook who copies `:280-334` and only swaps the argv still has no `args` assert. Combined with finding 3 (`clean`) + finding 1 (receipt “existed”): recook `gate 00-idea`, factory-drop `01-research.md`, sit needles stay `Flow *. gate` unless they also skip sit (`phase-01-start.md:86` chrome). Phase-03 `rv_next` lists `not gate` but will read a receipt, not the live journal, if wipe already happened.

**Fix (must):** Journal helpers must `== ["next"]` on **both** rows (not “contains next”). Self-refuse must `rg` the new file for `flow -- gate` and for `GATE stage`.

---

### 8. MEDIUM — `copy_journal || true` then wipe; fail path drops both proofs

**Plan:** `plan.md:79` / `phase-01-start.md:88` — copy journal before `wipe_iso`.

**Land:** Success AOE5 does `copy_journal || fail` then `teardown` (`dory-isolate-aoe5-flow-judge.sh:759-760`). `fail` → `teardown` (`:410-416`). `teardown` copies with `|| true` then wipes (`:403-407`). `copy_journal` (`:253-258`) does not fail if the file is already gone.

**Fail:** Reorder success path to leftover-check → wipe → copy (easy when copying `teardown` as the only copy site). EXIT trap wipes after a leftover fail **after** taxi2: journal may copy, `01-research.md` does not (optional). Phase-02 then has no independent 01 and maybe no journal.

**Fix (must):** `copy_journal` + copy `01-research.md` must `fail` on miss, and must run before any `wipe_iso`, including `fail()`/`EXIT`.

---

### 9. MEDIUM — `self_refuse_paid` copied verbatim still allows exec of the paid judge

**Plan:** `plan.md:60,87` — refuse regex **adds** `dory-isolate-aoe5-flow-judge`.

**Land:** AOE5 `self_refuse_paid` (`dory-isolate-aoe5-flow-judge.sh:223-236`) lists sit/prompt/roster/report/hop only. It does **not** list itself. `$0` case (`:38-43`) same.

**Fail:** “Copy AOE5 law” (`plan.md:32`; `phase-01-start.md:23`) plus a later bullet to add judge is two edits. One skipped edit → `source`/`exec` the paid gate script (trap 3). That recooks `gate 00-idea` under a next filename.

**Fix (must):** New `$0` identity + refuse list must include the judge name in the **first** paste of `self_refuse_paid`, not as a follow-up.

---

## Scout (edge cases the plan does not show)

| Path | What the plan implies | What the runner actually does |
|---|---|---|
| Dirty idea `next` | FAIL + not clean | `flow.sh:965-974` rc=1; stdout contains **both** `FAIL` and `clean` |
| Clean + missing 01 | unlock + cp | `flow.sh:1024-1026` |
| Clean + 01 already there | (unnamed) | `flow.sh:1019-1022` rc=0, no cp, `already exists` |
| No `00-idea.md` | “not the án” | `flow.sh:953-962` rc=0, `unlocked stage 00`; glance `Flow 0. next` |
| `gate 00-idea` | recook | `flow.sh:1426-1428`; journal `args=["gate","00-idea"]`; glance `Flow n. gate` |
| Taxi >15s | not discussed | `flow.rs:14,201-211` SIGTERM; glance `Flow lỗi. timed out…` (`desk.rs:5036-5042`) |

## What the plan got right (calibration only)

`args=["next"]` vs gate, `unlocked stage 1` vs bare `clean`, mint-FAIL-before-taxi1, poll-vs-`--wait` as named traps, glance needle change `gate`→`next`. Those are the correct needles. They are not wired as required helpers, and wipe/`Optionally`/idle/`idea_still_fail` undo them.

## Required plan deltas before cook

1. Required receipt artifacts: copied journal **and** `01-research.md` (sha vs template) **before** wipe; `copy_* || fail`.
2. Journal functions: do not copy AOE5 `:280-334`; assert `args`, codes, unlock line, reject `already exists` / `unlocked stage 00` / `GATE stage` / bare `clean`.
3. Taxi2 gate = `poll_pass_file` only; `rg --wait`; idle after prompt is not a gate.
4. Taxi1: mint FAIL first; reject empty-tree (`unlocked stage 00`, `code!=1`); do not trust `idea_still_fail` alone.
5. Sit = necessary chrome; land = stdout + file. Phase-02: sit hit + weak journal = TEST_FAIL.
6. `self_refuse_paid` includes judge on first paste.

## Metrics

- Type / tests / lint: N/A (plan review; no build)
- Findings: 9 (2 critical, 5 high, 2 medium)
- Plan traps that do not close the hole: 21, 22, 27, 28
