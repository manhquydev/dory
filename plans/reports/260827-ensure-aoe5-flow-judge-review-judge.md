# REVIEW — rv_judge

Accept iff taxi used abs skill `flow.sh`; journal exactly two `flow/result` codes `[1,0]`; not `/bin/true`/`status`/`doctor`; occupant wrote PASS (factory did not).

Read: plan, `scripts/dory-isolate-aoe5-flow-judge.sh`, cook+test receipts, `plans/reports/260827-ensure-aoe5-flow-judge-journal.jsonl` if present.

Do not recook. Do not invoke dory. Do not sit t13.

Write `plans/reports/260827-ensure-aoe5-flow-judge-review-judge.md` (this file: append verdict).
Reply `JUDGE_ACCEPT` or `JUDGE_REJECT`.

## Verdict

**JUDGE_ACCEPT**

Did not recook. Did not invoke dory. Did not sit t13. Evidence = plan + writer script + cook/test receipts + on-disk journal (test overwrite of cook copy).

| Gate | Evidence |
|---|---|
| taxi abs skill `flow.sh` | Script pins `FLOW_SKILL=/home/manhquy/.claude/skills/flow/runner/flow.sh` → `FLOW_BIN=$(realpath …)`; basename must be `flow.sh`; refuse `/bin/true`. `taxi()` sets `FLOW_BIN="$FLOW_BIN"` and `"$SIT_DORY" flow -- gate 00-idea`. Journal both `bin` = that abs path. Cook/test receipts same. |
| exactly two `flow/result` codes `[1,0]` | `plans/reports/260827-ensure-aoe5-flow-judge-journal.jsonl`: two `flow/result` (`code` 1 then 0). Two `flow/invoke`. args both `["gate","00-idea"]`. cwd both `…/dory-isolates/aoe5.mSvfhl` (test run). Script `journal_taxi2` requires `count==2` and `codes==[1,0]`. Test: count 2, codes `[1, 0]`. Cook claimed same shape on wiped `aoe5.gaLeT4`. |
| not `/bin/true` / `status` / `doctor` | Journal args `gate` `00-idea` only. Bin is abs `flow.sh`, not `/bin/true`. Fail stdout `GATE stage 00-idea` + unchecked/FILL; pass stdout `clean`. Plan trap 1 / approach C rejected. |
| occupant wrote PASS (factory did not) | Factory mints FAIL template; refuses if dest already PASS. Taxi 1 then `idea_still_fail`. Factory writes sidecar `$PASS_WANT` only. `coord_prompt_write_pass` → isolate coord/omptest overwrite `00-idea.md`. `poll_pass_file` MATCH then taxi 2. Cook: factory did not Write PASS. |

Cook `COOK_PASS` exit 0 `TAXI1_RC=1` `TAXI2_RC=0`. Independent test `TEST_PASS` exit 0, same journal contract; cook receipt not used as proof.
