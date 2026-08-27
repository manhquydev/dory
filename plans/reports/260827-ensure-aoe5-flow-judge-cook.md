---
type: cook
date: 2026-08-27
plan: 260827-1122-aoe-5-isolate-flow-judge
phase: 01
writer: scripts/dory-isolate-aoe5-flow-judge.sh
verdict: COOK_PASS
command: SIT_PANE=w13:p6G SIT_TAB=w13:t27 SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory bash scripts/dory-isolate-aoe5-flow-judge.sh
exit: 0
aoe5: PASS
commit: none
elapsed: 45.37s
---

# Cook receipt — AOE 5 isolate flow judge (phase 01)

**Verdict: COOK_PASS**

One writer: `scripts/dory-isolate-aoe5-flow-judge.sh`. Did not `source`/`exec` 1910/0043/0227/0242/hop. Did not copy 1910 taxi `FLOW_BIN=/bin/true`. No rust hunk. No cargo leftover tree. No factory `dory` argv. No leftover 5 fold. No commit.

## Files touched

| Path | Role |
|---|---|
| `scripts/dory-isolate-aoe5-flow-judge.sh` | isolate + occupants + fail-then-pass taxi |
| `plans/reports/260827-ensure-aoe5-flow-judge-journal.jsonl` | journal copy before wipe |
| `plans/reports/260827-ensure-aoe5-flow-judge-cook.md` | this receipt |

No edit to leftover: `README.md`, `rust/src/attach.rs`, `rust/src/main.rs`, `rust/src/server.rs`, `rust/tests/p5_attach.rs`, `rust/src/desk.rs`.

## Sit / isolate

```
SIT_PANE=w13:p6G
SIT_TAB=w13:t27
SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory
FLOW_BIN=/home/manhquy/.claude/skills/flow/runner/flow.sh
WS_ID=w1
COORD_PANE=w1:p1
TEST_PANE=w1:p2
ISO=aoe5.gaLeT4 (wiped)
```

`herdr pane get w13:p6G` → `tab_id=w13:t27` ≠ `w13:t13`. Sit pane empty shell. No `agent start` on sit. Occupants `coord` + `omptest` on isolate only. Report idle. Prompt `--timeout` not `--wait`.

Attach: `herdr pane send-text` then `send-keys enter`. Not `pane run`. Command = 1910 `:331` verbatim.

Wait pane-id-first:

```
herdr pane wait-output "$SIT_PANE" --match "Flow 1. gate" --source visible --timeout 20000
herdr pane wait-output "$SIT_PANE" --match "Flow 0. gate" --source visible --timeout 20000
```

matched_line: `" Flow 1. gate"` then `" Flow 0. gate"`. `pane_id=w13:p6G` `tab_id=w13:t27`.

Stop = 1910 `compound_stop`. Server start = 0242 `:346-353` `setsid` + isolate `XDG_RUNTIME_DIR` + `HOME="$ISO_REAL/home"` + `PI_CODING_AGENT_DIR` only on that line.

## Judge

Copied journal `plans/reports/260827-ensure-aoe5-flow-judge-journal.jsonl`:

| # | bin | code | stdout |
|---|---|---|---|
| 1 | `/home/manhquy/.claude/skills/flow/runner/flow.sh` | 1 | `GATE stage 00-idea` + unchecked/FILL |
| 2 | same | 0 | `GATE stage 00-idea:` + `clean` |

Exactly two `flow/result`. cwd both under `…/dory-isolates/aoe5.gaLeT4`. args `gate` `00-idea`. Not `/bin/true`. Not bare `flow.sh`. Factory did not Write PASS; occupant poll MATCH then taxi 2.

`TAXI1_RC=1` `TAXI2_RC=0` `JOURNAL_CODES=1,0` `VISIBLE_MATCH=1` `aoe5=PASS`.

## Factory / repo

| Check | Result |
|---|---|
| Factory sock `/run/user/1000/dory/default/dory.sock` | FileNotFound, connectable=0 |
| `dory server stop default` / `herdr server stop` | not run |
| Repo `.dory/` | ABSENT before/after |
| PATH `dory` | empty |
| Leftover 5 `git hash-object` | mint table MATCH |
| `desk.rs` worktree | `4c788562e4fdda10c8edd2878ed1fdd46050c218` == HEAD |
| `git log -1 -- rust/` | `b544f5f` (not recooked) |
| Isolate `aoe5.*` after wipe | none |
| Hop / 1910 / 0242 | comment+self-rg refuse; not sourced/exec'd |
| Cargo leftover tree | not run |
| Sit `w13:t13` / `w13:p2R` / `wP` / cook pane `w13:p6H` | not sat |
| Commit | none |

## Doors held

| Door | Held |
|---|---|
| 1910 taxi `/bin/true` | not copied |
| Paid isolate scripts | not exec'd |
| Default sock | not started |
| `desk.rs` recook | not touched |
| Leftover rust fold | not touched |
| `prompt --wait` / `occ.report=Working` | not used |
| `herdr server stop` | not run |
| Factory Write PASS | not done |
| Company Phase 5 claim | not claimed |
| Isolate ELF missing → cargo | ELF present; no cargo |

Transient `rm: …/aoe5.gaLeT4/home: Directory not empty` during wipe retry; final wipe left no `aoe5.*`.
