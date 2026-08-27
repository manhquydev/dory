# REVIEW — rv_next

Accept iff taxi used abs `flow.sh` `flow -- next`; codes 1 then 0; both `args == ["next"]`; stdout FAIL then `unlocked stage 1 (flow/01-research.md)`; occupant wrote PASS; copied 01 sha == template; not `/bin/true`/`gate`/`status`/`doctor`/`already exists`.

Read: `plans/260827-1657-isolate-flow-next-unlock/phase-03-review-and-ship.md`, `scripts/dory-isolate-aoe5-flow-next.sh`, COOK_PASS + TEST_PASS, `plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl`, `plans/reports/260827-ensure-aoe5-flow-next-01.sha256`.

Did not recook. Did not invoke dory. Did not sit t13.

## Verdict

**NEXT_ACCEPT**

`critical_count` **0**. Stage 1 (rv_next gates) **PASS**.

Evidence = plan + writer script + cook/test receipts + on-disk journal (test overwrite of cook copy) + live template sha. Independent python parse of the jsonl. Independent sha256 of `_templates/01-research.md`. `flow.sh:950-1030` read-only.

| Gate | Evidence |
|---|---|
| taxi abs `flow.sh` `flow -- next` | Script pins `FLOW_SKILL=/home/manhquy/.claude/skills/flow/runner/flow.sh` → `FLOW_BIN=$(realpath …)`; basename must be `flow.sh`; refuse `/bin/true`. `taxi()` `:316-329` `env -u` FLOW_* class then pin `FLOW_BIN` + `FLOW_PROJECT_ROOT` + `FLOW_LOG_DISABLE` + `DO_NOT_TRACK` and `"$SIT_DORY" flow -- next`. Live `flow.sh` exists, executable, realpath = that abs path. Journal all four rows `bin` = that path. Cook `COOK_PASS` / test `TEST_PASS` same. |
| codes `[1,0]` | Copied journal: exactly two `flow/result` (`code` 1 then 0). Two `flow/invoke`. Cook: `TAXI1_RC=1` `TAXI2_RC=0` `JOURNAL_CODES=1,0`. Test independent: codes `[1, 0]`, script exit 0. |
| both `args == ["next"]` | Journal invoke+result all `args=["next"]`. Not `["gate","00-idea"]`. `journal_taxi1` `:361-362` and `journal_taxi2` `:397-401` require exact `["next"]`. |
| FAIL then `unlocked stage 1` | Taxi1 stdout starts `FAIL: gate for stage 00-idea is not clean.` No `unlocked stage`. Matches `flow.sh:966` dirty-idea FAIL. Taxi2 stdout contains `unlocked stage 1 (flow/01-research.md)` on `PASS: stage 00-idea gate clean -> unlocked stage 1 (flow/01-research.md)`. Matches `flow.sh:1026`. Taxi2 land is the unlock-1 needle, not bare `clean` (F3). Taxi1 has `not clean`; that is FAIL. |
| occupant wrote PASS (factory did not) | Factory copies FAIL template; fails if dest already PASS (`:693-694`). Taxi1 then `idea_still_fail` + no `01-research.md` + idea ≠ PASS_WANT (`:817-823`). Factory writes sidecar `$PASS_WANT` only. `coord_prompt_write_pass` → isolate coord/omptest overwrite `00-idea.md`. `poll_pass_file` MATCH then taxi2 IFF `cmp -s` (`:877-891`). Cook: factory did not Write PASS. Journal gap 10:28:54 → 10:30:17 (~83s) is occupant+poll, not factory-immediate taxi2. |
| copied 01 sha == template | Receipt `260827-ensure-aoe5-flow-next-01.sha256`: both files `69429bc3e11f467c1dbcad4694055078cda4192dab447bf86832c2d17b1264aa`. Live template hash now = same digest; size 3241; still `[FILL]`. Body not copied into reports. Isolate path wiped (`aoe5n.X9Ll4T`). `research_sha_ok` requires sha match + `[FILL]` still present. |
| not `/bin/true` / `gate` / `status` / `doctor` / `already exists` | Bin is abs `flow.sh`, not `/bin/true`. Args never `gate`/`status`/`doctor`. Taxi2 stdout has no `already exists` / `GATE stage` / `unlocked stage 00`. Contiguous `flow -- gate` absent in writer (`self_refuse_paid` splits the string). Word `gate` in stdout is `flow.sh` next FAIL/PASS copy, not a gate taxi. |

Cook `COOK_PASS` exit 0 on wiped `aoe5n.nOWHtI`. Independent test `TEST_PASS` exit 0; cook receipt not used as proof. Live journal cwd `…/dory-isolates/aoe5n.X9Ll4T`.

## Closed false-pass doors

| Fake | Why it dies here |
|---|---|
| 1910 `FLOW_BIN=/bin/true` | Pin + refuse; journal bin is abs `flow.sh` |
| Recook AOE5 `gate 00-idea` | `args` are `["next"]`; stdout has no `GATE stage` |
| `status` / `doctor` taxi | args only `["next"]` |
| Already-exists `next` (`flow.sh:1019-1022`) | taxi2 has no `already exists`; unlock-1 needle present |
| Empty-tree taxi1 (`unlocked stage 00`, rc=0) | taxi1 `code==1` + no `unlocked stage` |
| Empty-tree taxi2 | no `unlocked stage 00`; unlock-1 present |
| AOE5 `clean` copy | taxi2 requires unlock-1, not substring `clean` |
| Factory Write PASS then taxi2 | factory mint PASS fails; taxi1 still FAIL; taxi2 IFF occupant `cmp` |
| Sit needle without land | land = journal stdout + 01 sha (this lens); sit is rv_sit |

## Nits (not Critical; do not reject)

- Taxi2 stdout prefixes a flow-lock reclaim NOTE from `lock_acquire` after taxi1's process died. Unlock-1 line is still present. Expected sequential taxis.
- Copied jsonl is the test overwrite. Cook journal not re-measurable on disk. Test cwd/ts differ from cook (`X9Ll4T` vs `nOWHtI`).

Doors held: no dory invoke; no sit t13; leftover/rust/fold out of this lens.
