---
type: cook
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 01
writer: scripts/dory-isolate-aoe5-flow-next.sh
verdict: COOK_PASS
command: SIT_PANE=w13:p7D SIT_TAB=w13:t2H SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory bash scripts/dory-isolate-aoe5-flow-next.sh
exit: 0
aoe5-next: PASS
commit: none
elapsed: 41.49s
---

# Cook receipt — isolate flow next unlock (phase 01)

**Verdict: COOK_PASS**

One writer: `scripts/dory-isolate-aoe5-flow-next.sh`. Copy-table only. Did not `source`/`exec` judge/1910/0043/0227/0242/hop. Did not copy 1910 taxi `FLOW_BIN=/bin/true`. No rust hunk. No cargo leftover tree. No factory `dory` argv. No leftover 5 fold. No commit. Tabs left open.

## Files touched

| Path | Role |
|---|---|
| `scripts/dory-isolate-aoe5-flow-next.sh` | isolate + occupants + fail-then-unlock taxi `flow -- next` |
| `plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl` | journal copy before wipe |
| `plans/reports/260827-ensure-aoe5-flow-next-01.sha256` | 01 sha/stat before wipe (no body) |
| `plans/reports/260827-ensure-aoe5-flow-next-cook.md` | this receipt |

No edit to leftover: `README.md`, `rust/src/attach.rs`, `rust/src/main.rs`, `rust/src/server.rs`, `rust/tests/p5_attach.rs`, `rust/src/desk.rs`.

## Sit / isolate

```
SIT_PANE=w13:p7D
SIT_TAB=w13:t2H
SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory
SIT_DORY_SHA=2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3
FLOW_BIN=/home/manhquy/.claude/skills/flow/runner/flow.sh
WS_ID=w1
COORD_PANE=w1:p1
TEST_PANE=w1:p2
ISO=aoe5n.nOWHtI (wiped)
```

`herdr pane get w13:p7D` → `tab_id=w13:t2H` ≠ `w13:t13`. Sit pane empty shell. No `agent start` on sit. Occupants `coord` + `omptest` on isolate only with `--no-session --no-skills --no-rules --no-extensions`. Report idle before taxi1 only. Prompt `--timeout` not `--wait`. Taxi2 IFF `cmp -s` PASS file.

Attach: `herdr pane send-text` then `send-keys enter`. Not `pane run`. Command = 1910 `:331` verbatim.

Wait pane-id-first:

```
herdr pane wait-output "$SIT_PANE" --match "Flow 1. next" --source visible --timeout 20000
herdr pane wait-output "$SIT_PANE" --match "Flow 0. next" --source visible --timeout 20000
```

matched_line: `" Flow 1. next"` then `" Flow 0. next"`. `pane_id=w13:p7D` `tab_id=w13:t2H`.

Stop = 1910 `compound_stop` `:69-100`. Server start = 0242 `:340-353` `mkdir`/`ln` then `setsid` + isolate `XDG_RUNTIME_DIR` + `HOME="$ISO_REAL/home"` + `PI_CODING_AGENT_DIR` only on that line. ISO/bin `realpath` == `SIT_DORY` == land sha.

Sit pane PATH: `type -a dory` rc≠0 via isolate probe file (not wait-output on typed echo).

## Next unlock

Copied journal `plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl`:

| # | bin | args | code | stdout |
|---|---|---|---|---|
| 1 | `/home/manhquy/.claude/skills/flow/runner/flow.sh` | `["next"]` | 1 | `FAIL: gate for stage 00-idea is not clean.` |
| 2 | same | `["next"]` | 0 | `unlocked stage 1 (flow/01-research.md)` |

Exactly two `flow/result`. cwd both `…/dory-isolates/aoe5n.nOWHtI`. Not `/bin/true`. Not `["gate","00-idea"]`. Not `GATE stage`. Not `unlocked stage 00`. Not `already exists`. Taxi2 land is `unlocked stage 1`, not bare `clean`.

Factory did not Write PASS; occupant poll MATCH then taxi 2. After taxi1: no `01-research.md`. After taxi2: `01` sha `69429bc3e11f467c1dbcad4694055078cda4192dab447bf86832c2d17b1264aa` == `_templates/01-research.md`. Still `[FILL]`. Body not copied into reports.

`TAXI1_RC=1` `TAXI2_RC=0` `JOURNAL_CODES=1,0` `VISIBLE_MATCH=1` `aoe5-next=PASS`.

## Factory / repo

| Check | Result |
|---|---|
| Factory sock `/run/user/1000/dory/default/dory.sock` | FileNotFound, connectable=0 |
| `dory server stop default` / `herdr server stop` | not run |
| Repo `.dory/` | ABSENT before/after |
| Factory `flow/` | absent |
| PATH `dory` | empty |
| Leftover 5 `git hash-object` | mint path+sha table MATCH |
| `desk.rs` worktree | `4c788562e4fdda10c8edd2878ed1fdd46050c218` == HEAD |
| `git log -1 -- rust/` | `b544f5f` (not recooked) |
| Leftover ELF sha (stat only) | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| Isolate `aoe5n.*` after wipe | none |
| Judge / hop / 1910 / 0242 | `$0`+self-rg refuse includes judge; not sourced/exec'd |
| Cargo leftover tree | not run |
| Sit `w13:t13` / `w13:p2R` / `wP` / cook pane `w13:p7E` | not sat |
| Commit | none |

## Doors held

| Door | Held |
|---|---|
| 1910 taxi `/bin/true` | not copied |
| Paid isolate scripts including judge | not exec'd/sourced |
| Default sock | not started |
| `desk.rs` recook | not touched |
| Leftover rust fold | not touched |
| `prompt --wait` / `occ.report=Working` | not used |
| `herdr server stop` | not run |
| Factory Write PASS | not done |
| Factory `FLOW_*` inherit | refused at entry; taxi `env -u` class then pin four |
| Company Phase 5 claim | not claimed |
| Isolate ELF missing → cargo | ELF present; hash-pinned; no cargo |
| Hardcode `land-4b70f79` in script | not present; env pin hashed |

Transient `rm: …/aoe5n.nOWHtI/home: Directory not empty` during wipe retry; final wipe left no `aoe5n.*`.
