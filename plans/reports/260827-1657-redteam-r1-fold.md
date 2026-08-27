---
type: review
date: 2026-08-27
time: 16:57
round: r1
role: Scope Auditor
lens: Hostile Assumption Destroyer + leftover/fold/ship
plan: 260827-1657-isolate-flow-next-unlock
verdict: PLAN_REJECT
authority: plan files + live leftover mint + 1122 L2/L3 + 1638 untracked mountain
did_not: invoke dory; cargo leftover tree; git add leftover 5; invoke dory
---

# PLAN REVIEW — leftover/fold/ship (Scope Auditor)

**PLAN_REJECT.** Review is the plan, not code quality. Traps 23/25/26 copy the 1122 leftover/fold lesson, then Success Criteria and ship allowlist re-open the same leftover-destruct and `git add -A` mountain. Do not cook until the findings below are closed in the plan text.

Live pins this pane (`git hash-object` / `git log` / `git status` only):

| path | WT sha1 | vs judge `leftover_mint_ok` |
|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

HEAD blobs ≠ mint. Porcelain ` M` ×5, unstaged. `git diff --stat HEAD -- rust/` = attach/main/server/`p5_attach` (+414/−94). `git log -1 -- rust/` = `b544f5f` `fix(attach): do not auto-start server on sit`. `desk.rs` WT == `HEAD:rust/src/desk.rs` = `4c788562e4fdda10c8edd2878ed1fdd46050c218`. Leftover ELF sha `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` (stat only). `type -a dory` empty. Factory `flow/` absent.

## Scope
- Files: `plan.md` (134), `phase-01-start.md` (102), `phase-02-independent-next-unlock-test.md` (59), `phase-03-review-and-ship.md` (81). Total 376 LOC.
- Focus: `rv_left` vs worktree rust dirty; `git add -A`; subject pretends Phase 5; source AOE5; leftover cargo; PATH retarget; fill 01 gold-plate; p5/skill sneak; 1638 eval `git add -A` mountain
- Scout: leftover 5 **are** the rust+README dirty vs `b544f5f`; 11× `260827-1638-eval-*.md` untracked; `plans/reports/` 310 untracked; `ak:git` Tool 1 is `git add -A`

## Overall Assessment
The plan says leftover HOLD and pathspec-only in one column, then writes a Success Criterion that fails on the live mint and a ship allowlist that invites the 1638 mountain. Cook following `plan.md:123` + `ak:git` + Nguồn citations folds leftover 5 or gold-plates 01 / Phase 5. 1122 already Accepted these as L2/L3/L4/L7.

---

## Finding 1: Success Criteria `No rust diff vs b544f5f` is leftover-destruct — 1122 L2 reintroduced
- **Severity:** Critical
- **Location:** `plan.md:123`; vs `plan.md:88`; `phase-03-review-and-ship.md:19,30`; `phase-02-independent-next-unlock-test.md:46`
- **Flaw:** Leftover 5 **are** the rust+README worktree diff vs land `b544f5f`. Demanding “no rust diff vs `b544f5f`” is demanding leftover gone. `git log -1 -- rust/` still `b544f5f` is compatible with mint. `git diff b544f5f -- rust/` is not. `rv_left` at phase-03:30 correctly says dirty leftover = **pass**. The parent Success Criteria requires the opposite. 1122 L2 already Accepted this class: leftover 5 được dirty; land rust commit vẫn `b544f5f`. This plan dropped 1122’s parenthetical `(leftover 5 **là** dirty rust)` at trap 25, then put the dead phrase back on the checkbox.
- **Failure scenario:** Review sees `git diff --stat HEAD -- rust/` = attach/main/server/`p5_attach` dirty → Critical. Cook “fixes” with `git checkout HEAD -- rust/src/attach.rs rust/src/main.rs rust/src/server.rs rust/tests/p5_attach.rs README.md` or `git restore rust/`. Mint 68190a5/6024790/373d688/4de1554/9c28fc3 die. Trap 9/17 forbid this; Success Criteria requires it.
- **Evidence:** Live `git diff --stat HEAD -- rust/` is those four files. `plan.md:123` “No rust diff vs `b544f5f`”. `plan.md:88` “`rv_left` ≠ ‘worktree rust clean’”. `phase-03-review-and-ship.md:30` “Worktree rust dirty leftover = **pass**”. `phase-03-review-and-ship.md:19` “rust HEAD still `b544f5f`” (log vs diff conflated). `phase-02-independent-next-unlock-test.md:46` “worktree rust **được** dirty leftover”. `plans/260827-1122-aoe-5-isolate-flow-judge/plan.md:157` L2 Accept. `plans/reports/260827-1122-rt-fold.md:46-52` same kill.
- **Suggested fix:** Delete “No rust diff vs `b544f5f`”. Accept rust **commit** via `git log -1 -- rust/` = `b544f5f` only. Accept leftover 5 ` M` + exact mint SHAs. Write “rust worktree dirty = leftover; do not checkout/restore/reset.”

## Finding 2: Ship pathspec is a decoy — `git add -A` scoops leftover 5 + the 1638 eval mountain
- **Severity:** Critical
- **Location:** `phase-03-review-and-ship.md:23,36-45,79`; `plan.md:25,66,88,105`; `~/.cursor/skills/ak-git/SKILL.md:75`
- **Flaw:** Leftover 5 are tracked ` M`. Eleven `260827-1638-eval-*.md` are untracked. `plans/reports/` has 310 untracked paths. `git add -A` stages leftover 5 **and** the 1638 mountain. Plan cites 1638 eval-next/eval-synth as Nguồn (`plan.md:25`) but does not put them on the allowlist **or** the deny list. Glob `plans/reports/260827-ensure-aoe5-flow-next-*` is not a git pathspec unless the shell expands it; widening to `plans/reports/` to “make the glob work” ships the mountain. `ak:git` first tool is still `git add -A`. “Không dùng `ak:git` nếu nó `add -A`” is a hedge, not a ban — cook still opens the skill. Risk text still claims fold comes from `git add scripts/` plus dirty rust — `scripts/` cannot stage `README.md`. 1122 L3 already Accepted: pathspec `git add --` only.
- **Failure scenario:** Ship follows `ak:git` or `git add -A` or `git add plans/reports/`. Cached includes leftover 5 (mint becomes land; rust HEAD leaves `b544f5f`) **and** 11 eval reports that 1650 said were accept-paper, not this cook. Or they unstage leftover too late after a `feat`/`fix` split. `rv_fold` “cached ⊆ allowlist” only works if Tool 1 never runs.
- **Evidence:** This pane `git add -A -n` names `README.md` + four rust leftover paths + `plans/reports/260827-1638-eval-next.md`. `git ls-files plans/reports/260827-1638-eval-*.md` count 0. `phase-03-review-and-ship.md:42` glob. `phase-03-review-and-ship.md:45` bans leftover 5 / `rust/**` / paid judge — does **not** name `README.md` separately, does **not** name `260827-1638-eval-*`. `phase-03-review-and-ship.md:79` “Ship folds leftover via `git add scripts/` plus dirty rust” — false mechanism (`plans/reports/260827-1122-rt-fold.md:54-60`). `plans/reports/260827-1656-research-02-isolate-next.md:179` “named files, no glob-as-git”. Allowlist here is **wider** than research-02 (adds 1656-01 + 1650).
- **Suggested fix:** Named files only. No glob. No `ak:git`. Deny-list leftover 5 paths + `README.md` + `rust/**` + `plans/reports/260827-1638-eval-*`. Pre-commit: cached ⊆ allowlist **and** leftover `hash-object` == mint **and** porcelain ` M` ×5.

## Finding 3: Trap 23 leftover table omits paths — cook hashes the wrong `p5_attach` / `README`
- **Severity:** High
- **Location:** `plan.md:80-86`; vs `scripts/dory-isolate-aoe5-flow-judge.sh:173-178`; `plans/reports/260827-1656-research-02-isolate-next.md:167-171`
- **Flaw:** Trap 23 lists `README` / `attach` / `main` / `server` / `p5_attach` as bare names. Judge and research-02 pin **full** paths. `rust/src/p5_attach.rs` does not exist. Leftover-dory/ does not exist. Cook hashing `README` (no `.md`) or `rust/src/p5_attach.rs` fails or invents a snap. Phase-02:43 says “leftover 5 = mint table” pointing at those short names.
- **Failure scenario:** Cook copies 0242 six-file snap (includes `desk.rs`) or hashes `HEAD:` blobs (`5ac82b10` / `62f09a95` / …) instead of worktree. `leftover_mint_ok` green while mint is dead — or they “fix” a hash miss by checkout (Finding 1).
- **Evidence:** Live only `rust/tests/p5_attach.rs` exists. Judge table `scripts/dory-isolate-aoe5-flow-judge.sh:174-178`. 1122 S9/L1 Accepted “bảng mint” with paths; this plan kept SHAs and dropped paths.
- **Suggested fix:** Paste the five-row path+sha table from judge `:173-178` into plan + phase-01/02/03. Ban snap-then-MATCH.

## Finding 4: Trap 17 “cargo trừ ELF missing” reopens leftover-tree cargo
- **Severity:** High
- **Location:** `plan.md:66,74`; `phase-01-start.md:102`; `plans/reports/260827-1638-eval-iso.md:44`; `plans/reports/260827-1638-eval-left.md:44`
- **Flaw:** Trap 9 forbids `cargo /home/manhquy/Downloads/flow/dory`. Trap 17 then opens “không cargo trừ ELF missing (mặc định: FAIL, không rebuild)”. The exception is the door. Leftover ELF already exists (`3ba0e3bc…`, mtime 2026-08-26 11:00). `cargo` in leftover cwd remints that ELF from leftover 5 sources. 1122 L4 Accepted: ELF có thì không cargo. This plan re-opens the exception.
- **Failure scenario:** `SIT_DORY` missing or “stale.” Cook `cargo --manifest-path rust/Cargo.toml` “just to see.” Leftover ELF sha moves. Leftover 5 still ` M` but the binary C held is gone. Or isolate reset + cargo (no rust hunk in this cook).
- **Evidence:** `plan.md:74` “trừ ELF missing”. `phase-01-start.md:102` “Do not cargo leftover” then the parent exception. `plans/reports/260827-1638-eval-iso.md:44` leftover cargo remints `3ba0e3bc…`. `plans/260827-1122-aoe-5-isolate-flow-judge/plan.md:159` L4 Accept.
- **Suggested fix:** Close the exception. ELF missing = FAIL. Pin leftover ELF sha. `rv_left` = leftover ELF sha MATCH + no leftover `target` mtime jump.

## Finding 5: `01-research.md exists` with no template assert — fill-01 gold-plate still PASSes
- **Severity:** High
- **Location:** `plan.md:20,23,79,120`; `phase-01-start.md:66,85,88`; `phase-02-independent-next-unlock-test.md:42`; `~/.claude/skills/flow/_templates/01-research.md:23-25`
- **Flaw:** Non-goal and trap 13 say do not fill `01-research.md`. Acceptance is **existence only**. `cmd_next` copies the FILL template (`flow.sh:1024`). Unchecked boxes + `[FILL]` remain. If cook (or occupant, or a skills-ON pane) gold-plates research content, `exists` is still true. Optional “copy 01 into cook receipt” (`plan.md:79`, `phase-01-start.md:88`) turns a filled 01 into a shipped artifact.
- **Failure scenario:** Cook fills 01 so the receipt “looks like company Phase 5.” Or copies filled bytes into `plans/reports/`. Taxi2 already returned 0. Review `rv_next` “`01-research.md` existed” (`phase-03-review-and-ship.md:29`) accepts. Remainder `plan.md:131` (fill 01 unpaid) is laundered.
- **Evidence:** Template `_templates/01-research.md:23-25` is `[FILL]`. No phase asserts `cmp` vs template or leftover `[FILL]` / unchecked boxes. `phase-03-review-and-ship.md:80` tells review to **reject** fill-01 as a must-fix — so a cook who already filled it has no mechanical fail.
- **Suggested fix:** After taxi2, `cmp` ISO `01-research.md` to skill `_templates/01-research.md` (or assert `[FILL]` still present). Receipt = `ls`/`stat` only. Ban copying 01 body into `plans/reports/`.

## Finding 6: p5 / skill taxi sneak in via leftover `p5_attach` + Herdr `skills ON`
- **Severity:** High
- **Location:** `plan.md:33,105,129-130`; leftover path `rust/tests/p5_attach.rs`; `plans/reports/260827-1650-brainstorm-eval-accept.md:22`
- **Flaw:** Approach C names p5 lock / skill taxi paper as unpaid residual. Remainder repeats it. Then Herdr law turns skills ON for cook/test/review/ship unless a pane prompt says `--no-skills`. Factory has **no** `flow/`. A skills-ON agent that runs `/flow next` mints factory `flow/00-idea.md` then `flow/01-research.md` — fill-01 gold-plate on the **repo tree**, not ISO. Leftover 5 **includes** `rust/tests/p5_attach.rs`. `rv_left` hashing that file is HOLD leftover; `cargo test p5_attach` is leftover cargo (Finding 4).
- **Failure scenario:** Cook pane with skills ON drives flow-skill on factory cwd. Factory `flow/` appears. Or review “verifies p5” by cargo-testing leftover `p5_attach.rs`. Or skill-taxi empty `--` → `status` paper gets written because FLOW_BIN is already pinned. 1650: p5 lock and skill taxi stay unpaid.
- **Evidence:** `plan.md:105` “skills ON unless a pane prompt says `--no-skills`”. Factory `ls flow` → No such file. `plan.md:33` C unpaid. `CHARTER.md:9` lệnh không phải `flow`; `:30-32` Dory taxis stranger `flow.sh`; `:61` kill if hàng xuất calls `herdr`/`dsh` (factory Herdr is `:43` — product rust must not grow a Flow button).
- **Suggested fix:** Cook/test/review/ship panes `--no-skills` unless a prompt names a skill. Ban factory `flow/` create. Ban `cargo test` leftover `p5_attach`. Keep leftover hash as read-only HOLD.

## Finding 7: “Copy AOE5 law” + phase-01 “do not exec” (source omitted) is the source-AOE5 door
- **Severity:** High
- **Location:** `phase-01-start.md:23,72`; vs `plan.md:21,60`; `plans/reports/260827-1656-research-02-isolate-next.md:217-221,244`
- **Flaw:** Constraints forbid `exec`/`source` of the paid judge. Phase-01 Architecture leads with “Copy AOE5 law.” Related Code Files say “Copy-law only (**do not exec**)” and omit `source`. Cook under time pressure `source leftover_mint_ok` or `. dory-isolate-aoe5-flow-judge.sh` to reuse the 790-line body, then overrides `taxi()`. Judge main still taxis `flow -- gate 00-idea` and sit `Flow *. gate` — recook AOE5, false án.
- **Failure scenario:** New script is judge with a renamed file. Journal `args=["gate","00-idea"]` or sit `Flow 1. gate`. Taxi2 accepts `clean` (AOE5 verb). Unlock 01 never happens, or happens as a side effect after a gate recook. Self-`rg` refuse is skipped because they sourced instead of pasted.
- **Evidence:** `phase-01-start.md:72` “do not exec”. `plan.md:21` “Không exec/source … AOE5 judge”. `plan.md:60` trap 3. Research-02 trade-off: Source AOE5 = “hidden coupling / forbidden trap 3”. Judge taxi `scripts/dory-isolate-aoe5-flow-judge.sh:261-266` is `gate 00-idea`.
- **Suggested fix:** Phase-01 Related + step 3 must say do not `source`/`.`/`exec`. Copy-table from research-02 only (stop/attach/setsid/mint/poll/taxi env). Self-`rg` for `source`/`exec`/`dory-isolate-aoe5-flow-judge`.

## Finding 8: 0242 cite `:346-353` omits ISO/bin mkdir/ln — PATH retarget temptation
- **Severity:** High
- **Location:** `plan.md:77`; `phase-01-start.md:84`; vs `scripts/dory-isolate-flock-prompt.sh:340-353`; `plan.md:22,132`; `plans/reports/260827-1638-eval-iso.md:46`
- **Flaw:** Trap 20 and phase-01 step 4 say start server **0242 `:346-353` verbatim**. Those lines are `setsid` + `PATH="$ISO_REAL/bin:$PATH"`. `mkdir ISO/bin` + `ln -sfn "$SIT_DORY"` are `:340-344`. Verbatim `:346-353` pins PATH at a missing `bin/dory`. Isolate children CNF. Cook “fixes” factory PATH or `~/.local/bin/dory` — that is PATH retarget, named unpaid remainder and 1638 do-not. 1910 sit.bin recook is also do-not.
- **Failure scenario:** Occupant `dory` misses isolate ELF. Cook retargets factory PATH to leftover `rust/target/debug/dory` (`3ba0e3bc…`, spawn-strings kept) or isolate ELF on factory XDG. Default sock comes back. Leftover mint may still hash; PATH `dory` empty fails — or they retarget then unlink again and remint leftover ELF.
- **Evidence:** `scripts/dory-isolate-flock-prompt.sh:340-344` mkdir/ln; `:346-353` setsid. Research-02 copy-table `:340-353`. `plans/reports/260827-1638-eval-iso.md:46` 1910 sit.bin unpaid. `plan.md:132` PATH retarget unpaid. Live `type -a dory` empty — HOLD, do not retarget.
- **Suggested fix:** Cite 0242 `:340-353` (mkdir/ln + setsid). Assert factory PATH `dory` empty before **and** after. Ban `~/.local/bin/dory` create. Ban leftover `rust/target` on any PATH.

## Finding 9: Subject / script `aoe5` pretends company Phase 5
- **Severity:** Medium
- **Location:** `plan.md:26,69,89`; `phase-03-review-and-ship.md:50-53,81`; filename `scripts/dory-isolate-aoe5-flow-next.sh`
- **Flaw:** Trap 26 and phase-03 message `feat(isolate): fail-then-pass flow.sh next` are correct. The script name, ISO prefix `aoe5n.`, Nguồn “AOE5 paid”, and leftover `p5_attach` in the mint table train the cook to write `feat(isolate): AOE5 next` / “Phase 5 done.” 1122 L7 Accepted “đổi subject” for the same pretence. CHARTER/north-star Phase 5 = project completed inside Dory — unpaid (`plan.md:131`).
- **Failure scenario:** Paper HEAD claims AOE 5 / Phase 5. Remainder (6-stage, fill 01, semantic, default) looks paid. Next cook recooks this slice.
- **Evidence:** `phase-03-review-and-ship.md:81` “Subject says AOE 5 / Phase 5 done.” `plans/260827-1122-aoe-5-isolate-flow-judge/plan.md:162` L7. `CHARTER.md:9` lệnh không phải `flow`. `plans/reports/260827-1638-eval-synth.md:47,123` AOE5 ≠ company Phase 5.
- **Suggested fix:** Keep the isolate subject verbatim. Ban `AOE 5` / `Phase 5` / `aoe5` in the commit subject. Filename may stay as analog; subject must not.

---

## Edge Cases Found by Scout
- Leftover 5 WT hashes MATCH judge `:173-178` **and** plan trap 23 SHAs; HEAD blobs differ. Dirty rust vs `b544f5f` is HOLD, not drift.
- `git add -A -n` stages leftover 5 + untracked 1638 eval-next. 310 untracked under `plans/reports/`.
- `ak:git` `SKILL.md:75` / `workflow-commit.md:7` = `git add -A`. Rust leftover groups as `code:` → `feat|fix`.
- `_templates/01-research.md` is FILL-heavy; existence ≠ unfilled.
- Factory `flow/` absent — skills-ON `/flow next` would create it.
- 0242 ISO/bin mkdir/ln is `:340-344`, not inside `:346-353`.
- CHARTER `:43` factory Herdr OK; `:61` kill if **shipped** Dory calls `herdr`/`dsh`; `:9` no Flow command/button. Plan factory-herdr is in-bounds; a Dory `next` button is not.

## Recommended Actions
1. Delete `plan.md:123` “No rust diff vs `b544f5f`”. Align Success Criteria with `rv_left` dirty-pass + `git log -1 -- rust/` = `b544f5f`.
2. Named-file ship only. Deny leftover 5 + `README.md` + `rust/**` + `260827-1638-eval-*`. Ban `ak:git`. Ban glob.
3. Paste judge leftover path+sha table. Ban snap-then-MATCH. Ban checkout/restore/reset leftover.
4. Close trap 17 cargo exception. Pin leftover ELF sha. ELF missing = FAIL.
5. After taxi2, `cmp` 01 to template (or assert `[FILL]`). Receipt = ls/stat. Ban fill-01 body in reports.
6. Cook/test/review/ship `--no-skills` unless named. Ban factory `flow/`. Ban leftover `cargo test`.
7. Phase-01: do not `source`/`.`/`exec` judge. Copy-table only. Cite 0242 `:340-353`. Assert PATH `dory` empty; do not retarget.
8. Subject stays `feat(isolate): fail-then-pass flow.sh next`.

## Metrics
- Type Coverage: n/a (plan review)
- Test Coverage: n/a
- Linting Issues: n/a
- Plan LOC: 376
- Findings: 9 (Critical 2, High 6, Medium 1)
- Leftover 5 mint: MATCH this pane
- 1638 eval reports tracked: 0 / 11
- Live rust diff vs `b544f5f`: leftover 4 files + `README.md`

## Unresolved Questions
- Whether 1650 + 1656-01 belong on this cook’s paper allowlist (research-02 omitted them; phase-03 added them). Scope call for the lead — not a leftover fold by themselves.
- Isolate ELF pin (land `2ef20730…`) is not in this plan; only leftover ELF risk is in scope here.

## Plan task status (report only — do not mutate)
- Phase 1 / 2 / 3: still Pending. Do not start cook until Findings 1–2 are closed in plan text. Findings 3–8 are the same leftover/fold class that 1122 had to Accept before AOE5 cooked.
