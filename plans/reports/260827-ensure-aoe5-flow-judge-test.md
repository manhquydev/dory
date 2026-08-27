---
type: test
date: 2026-08-27
plan: 260827-1122-aoe-5-isolate-flow-judge
phase: 01
writer: aoe5_test
verdict: TEST_PASS
command: SIT_PANE=w13:p6R SIT_TAB=w13:t2C SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory scripts/dory-isolate-aoe5-flow-judge.sh
exit: 0
aoe5: PASS
elapsed: 39.80s
---

# TEST — independent AOE 5 re-run

**Verdict: TEST_PASS**

Skills OFF. Did not recook. Did not edit the script. Did not sit `w13:t13`. Did not invoke factory `dory`. Did not cargo leftover. Did not fold leftover 5.

Sit: `herdr pane get w13:p6R` → `tab_id=w13:t2C`, no agent, empty shell (revision 1) before run.

## Independent asserts

| Check | Result |
|---|---|
| Script exit | 0 |
| Copied journal `plans/reports/260827-ensure-aoe5-flow-judge-journal.jsonl` `flow/result` count | 2 |
| both `bin` | `/home/manhquy/.claude/skills/flow/runner/flow.sh` |
| codes | `[1, 0]` |
| fail stdout | contains `GATE stage 00-idea` |
| pass stdout | contains `clean` (`GATE stage 00-idea:\n  clean\n`) |
| leftover README.md | `68190a5ffa073c082aa318aad5ed032e13cc90e3` |
| leftover rust/src/attach.rs | `602479094e84d31ad6f017775a3d55aeb485c644` |
| leftover rust/src/main.rs | `373d688636ff7315ccd665f450069d8284eb47ff` |
| leftover rust/src/server.rs | `4de1554ad56e248cdcf42f02111b7389b08dae82` |
| leftover rust/tests/p5_attach.rs | `9c28fc3e0f3666498a8952411242d5301f7911de` |
| `desk.rs` worktree | `4c788562e4fdda10c8edd2878ed1fdd46050c218` == HEAD |
| sock `/run/user/1000/dory/default/dory.sock` | absent, connectable=0 |
| `type -a dory` | empty (rc 1, not found) |
| `git log -1 -- rust/` | `b544f5f` |

Journal cwd both under `…/dory-isolates/aoe5.mSvfhl`. args both `gate` `00-idea`. Not `/bin/true`.

Script banner: `TAXI1_RC=1` `TAXI2_RC=0` `JOURNAL_CODES=1,0` `VISIBLE_MATCH=1` `aoe5=PASS` `FACTORY_CONNECTABLE=0` `REPO_DORY_STAT=ABSENT`.

Wait-output (this run, not cook pane): matched `" Flow 1. gate"` then `" Flow 0. gate"` on `pane_id=w13:p6R` `tab_id=w13:t2C`.

Transient wipe: `rm: cannot remove '…/aoe5.mSvfhl/home': Directory not empty`. Script still exit 0.

Cook receipt not used as proof.
