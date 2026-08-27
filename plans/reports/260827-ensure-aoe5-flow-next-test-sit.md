---
type: test-sit
date: 2026-08-27
time: 17:31
plan: 260827-1657-isolate-flow-next-unlock
phase: 02
role: ts_sit
watcher_pane: w13:p7R
watcher_tab: w13:t2K
sit_pane: w13:p82
sit_tab: w13:t2P
sit_label: dory-aoe5n-testsit
iso: aoe5n.X9Ll4T
needles: ["Flow 1. next", "Flow 0. next"]
sat_t13: no
invoked_dory: no
factory_connectable: 0
journal_cwd: aoe5n.X9Ll4T
journal_weak: no
verdict: SIT_PASS
land_rule: sit hit + weak journal = FAIL
---

# TEST sit — independent next unlock (phase 02)

**Verdict: SIT_PASS.** After run. Sit = `w13:t2P` / `w13:p82`, not `w13:t13`. Needles `Flow 1. next` then `Flow 0. next`. **Sit hit + weak journal = FAIL** (trap 30). This journal is not weak.

This pane = roster `ts_sit` (`w13:p7R` / `w13:t2K` `dory-aoe5n-test`). Not `ts_run` / `ts_jrnl` / `ts_left` / `ts_path`. Did not sit `w13:t13`. Did not `send-text` / `attach` / `agent start` on sit or factory. Did not invoke factory `dory`. Cook receipt unused as proof.

## Identity

| Role | Pane | Tab | Label | Agent |
|---|---|---|---|---|
| ts_sit (this) | `w13:p7R` | `w13:t2K` | `dory-aoe5n-test` | omp working |
| ts_run | `w13:p7F` | `w13:t2K` | `dory-aoe5n-test` | omp |
| sit (this run) | `w13:p82` | `w13:t2P` | `dory-aoe5n-testsit` | none |
| sit (cook; not this run) | `w13:p7D` | `w13:t2H` | `dory-aoe5n-sit` | none |
| factory | `w13:p2R` | `w13:t13` | `1` | cursor working; **not sat** |

`herdr pane get w13:p82` → `tab_id=w13:t2P` `pane_id=w13:p82` `agent_status=unknown` (no `.agent`). After wipe: cwd=`/home/manhquy/.cache/dory-isolates/aoe5n.X9Ll4T (deleted)`.

`HERDR_PANE_ID=w13:p7R` `HERDR_TAB_ID=w13:t2K` ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. Sit pane ≠ `w13:p2R` / `*wP:*`. Sit tab ≠ `w13:t13`.

## Needles (next, not gate)

After-run ground truth: **`Flow 1. next` / `Flow 0. next`**.

Watcher visible `--source visible` on `w13:p82`:

| When | `Flow 1. next` | `Flow 0. next` | `Flow *. gate` |
|---|---|---|---|
| live isolate (desk still `aoe5n.X9Ll4T`) | 1 | 0 | 0 |
| after wipe (`X9Ll4T (deleted)`) | 0 | 1 | 0 |

Chrome is `Flow {n}. {arg0}` (`desk.rs:3450-3458`). Not `Flow 1. gate` / `Flow 0. gate`.

Sit necessary, not sufficient. `Flow 0. next` is shared by empty-tree PASS and unlock PASS. Land = copied journal stdout + 01 sha. **Sit hit + weak journal = FAIL.**

## Journal (this sit ISO — not cook)

Copied `plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl` mtime `17:30:18 +0700`. cwd all four rows = `…/dory-isolates/aoe5n.X9Ll4T` (not cook `aoe5n.nOWHtI`). Independent parse of the copy; isolate wiped; path not re-read.

| # | type | args | code | land |
|---|---|---|---|---|
| 1 | `flow/result` | `["next"]` | 1 | `FAIL: gate for stage 00-idea is not clean.` |
| 2 | `flow/result` | `["next"]` | 0 | `unlocked stage 1 (flow/01-research.md)` |

Exactly two `flow/result`. `bin` both = abs `/home/manhquy/.claude/skills/flow/runner/flow.sh`. Not `/bin/true`. Not `["gate","00-idea"]`. Reject `GATE stage` / `already exists` / `unlocked stage 00`. Taxi2 land is unlock-1, not substring `clean`.

`01.sha256` same iso: digest `69429bc3…` == `_templates/01-research.md`. Body not copied.

Journal **not weak** → sit chrome may stand. If this copy had been cook `nOWHtI` / `GATE` / `/bin/true` / missing unlock-1, sit hit would still be **FAIL**.

## Factory doors (this pane)

| Door | After run |
|---|---|
| PATH `dory` | empty (`type: dory not found`) |
| default sock `/run/user/1000/dory/default/dory.sock` | `FileNotFoundError`, connectable=0 |
| `$XDG_RUNTIME_DIR/dory` / `dory/default` | absent |
| factory `flow/` | absent |
| repo `.dory/` | ABSENT |
| leftover 5 `git hash-object` | mint MATCH, porcelain ` M` ×5 unstaged |
| `desk.rs` | `4c788562…` == HEAD |
| `git log -1 -- rust/` | `b544f5f` |
| leftover ELF sha | `3ba0e3bc…` unchanged (stat only) |
| `SIT_DORY` land sha | `2ef20730…` (not exec'd from this pane) |
| cache `aoe5n.*` after wipe | none |

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT` UNSET. `HOME=/home/manhquy`. `~/.local/bin/dory` absent.

## This pane did not

- sit `w13:t13` / `w13:p2R` / `wP`
- `send-text` / `send-keys` / `pane run` / `agent start` on sit or factory
- start `/run/user/1000/dory/default`
- `mkdir` factory `dory/` or `dory/default`
- invoke factory `dory` / leftover ELF / isolate ELF on factory XDG
- `dory server stop` default
- `herdr server stop`
- cargo leftover / fold leftover 5
- write factory `flow/`
- recook / fill `01-research.md`
- cite cook `nOWHtI` journal as this sit's proof

## Result

`SIT_PASS`. Sit was `w13:t2P`/`w13:p82`, not `t13`. Needles `Flow 1. next` / `Flow 0. next`. Sit hit + weak journal = FAIL; this journal is `aoe5n.X9Ll4T` `[1,0]` `args=["next"]` unlock-1, so not that fail. Factory sock connectable=0. `t13` not sat.
