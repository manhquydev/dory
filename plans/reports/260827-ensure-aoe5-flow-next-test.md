---
type: test
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 02
writer: aoe5n_test
verdict: TEST_PASS
command: SIT_PANE=w13:p82 SIT_TAB=w13:t2P SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory bash scripts/dory-isolate-aoe5-flow-next.sh
exit: 0
aoe5-next: PASS
elapsed: 88.24s
---

# TEST — independent next unlock re-run

**Verdict: TEST_PASS**

Did not recook. Did not fill `01-research.md`. Did not sit `w13:t13` / `w13:p2R` / `wP`. Did not git commit. Did not cargo leftover. Did not fold leftover 5. Cook receipt `260827-ensure-aoe5-flow-next-cook.md` not used as proof.

## Sit door (before run)

`herdr pane get w13:p82` → `tab_id=w13:t2P` ≠ `w13:t13`. `pane_id=w13:p82` ≠ `w13:p2R`. No `agent` field. `herdr agent list` p82/t2P empty. Visible: factory shell prompt only. `SIT_DORY` sha256 `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3`. Factory `DORY_*` / `FLOW_*` / `PI_CODING_AGENT_DIR` unset. `HOME=/home/manhquy`.

## Independent asserts (after overwrite)

Copied journal `plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl` mtime 1787826618 > cook 1787826242. cwd left cook `aoe5n.nOWHtI` for `aoe5n.X9Ll4T`.

| Check | Result |
|---|---|
| Script exit | 0 |
| `flow/result` count | 2 |
| both `bin` | `/home/manhquy/.claude/skills/flow/runner/flow.sh` (abs, not `/bin/true`, not bare `flow.sh`) |
| both `args` | `["next"]` (not `["gate","00-idea"]`) |
| codes | `[1, 0]` |
| taxi1 stdout | `FAIL: gate for stage 00-idea is not clean.` ; no `unlocked stage` ; no `GATE stage` |
| taxi2 stdout | `unlocked stage 1 (flow/01-research.md)` ; no `already exists` / `unlocked stage 00` / `GATE stage` / `flow -- gate` |
| taxi2 land | unlock-1 needle, not bare `clean` |
| copied 01 sha | `69429bc3e11f467c1dbcad4694055078cda4192dab447bf86832c2d17b1264aa` == `_templates/01-research.md` |
| leftover README.md | `68190a5ffa073c082aa318aad5ed032e13cc90e3` |
| leftover rust/src/attach.rs | `602479094e84d31ad6f017775a3d55aeb485c644` |
| leftover rust/src/main.rs | `373d688636ff7315ccd665f450069d8284eb47ff` |
| leftover rust/src/server.rs | `4de1554ad56e248cdcf42f02111b7389b08dae82` |
| leftover rust/tests/p5_attach.rs | `9c28fc3e0f3666498a8952411242d5301f7911de` |
| leftover 5 porcelain | still ` M` |
| `desk.rs` worktree | `4c788562e4fdda10c8edd2878ed1fdd46050c218` == HEAD |
| leftover ELF sha (stat only) | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| sock `/run/user/1000/dory/default/dory.sock` | absent, connectable=0 |
| `type -a dory` (test tab) | empty (rc 1, not found) |
| `git log -1 -- rust/` | `b544f5f` |
| repo `.dory/` | absent |
| factory `flow/` | absent |
| leftover/isolate cargo | 0 |

01 body not copied. Isolate path not re-read (wiped). Template still has `[FILL]`.

## Sit needles (this run, not cook pane)

Wait-output matched `" Flow 1. next"` then `" Flow 0. next"` on `pane_id=w13:p82` `tab_id=w13:t2P`. Independent `herdr pane read w13:p82 --source visible` after teardown still shows `Flow 0. next`. Sit + journal unlock-1 + 01 sha. Sit hit without those would have been TEST_FAIL.

Sit PATH: script probe before attach (rc≠0). Did not send `type -a dory` into post-attach TUI.

## Script banner (not proof)

`TAXI1_RC=1` `TAXI2_RC=0` `JOURNAL_CODES=1,0` `VISIBLE_MATCH=1` `aoe5-next=PASS` `FACTORY_CONNECTABLE=0` `REPO_DORY_STAT=ABSENT` `RESEARCH_SHA=TEMPLATE_SHA`.

Transient wipe: `rm: cannot remove '…/aoe5n.X9Ll4T/home': Directory not empty`. After run: no `aoe5n.*` dirs. Sit cwd `aoe5n.X9Ll4T (deleted)`.
