---
type: test-sit
date: 2026-08-29
time: 01:08
plan: 260829-0054-isolate-flow-prd-unlock
phase: 02
role: pt_sit
watcher_pane: w13:p9K
watcher_tab: w13:t33
sit_pane: w13:p98
sit_tab: w13:t31
sit_label: dory-aoe5p-testsit
iso: aoe5p.azx4PH
needles: ["Flow 1. next", "Flow 0. next"]
sat_t13: no
invoked_dory: no
factory_connectable: 0
journal_cwd: aoe5p.azx4PH
journal_weak: no
verdict: SIT_PASS
land_rule: sit hit + weak journal = FAIL
---

# TEST sit — independent prd unlock (phase 02)

**Verdict: SIT_PASS.** After run. Sit = `w13:t31` / `w13:p98`, not `w13:t13` / `w13:p2R` / `*wP:*`. Needles `Flow 1. next` then `Flow 0. next`. **Sit hit + weak journal = FAIL** (trap 13). This journal is not weak.

This pane = roster `pt_sit` (`w13:p9K` / `w13:t33` `dory-aoe5p-test`). Not `pt_run` / `pt_jrnl` / `pt_left` / `pt_path`. Did not sit `w13:t13`. Did not `send-text` / `send-keys` / `pane run` / `agent start` on sit or factory. Did not invoke factory `dory`. Cook receipt `aoe5p.eGZMMi` unused as proof.

## Identity

| Role | Pane | Tab | Label | Agent |
|---|---|---|---|---|
| pt_sit (this) | `w13:p9K` | `w13:t33` | `dory-aoe5p-test` | omp working |
| pt_run | `w13:p9A` | `w13:t33` | `dory-aoe5p-test` | omp |
| sit (this run) | `w13:p98` | `w13:t31` | `dory-aoe5p-testsit` | none |
| sit (cook; not this run) | `w13:p97` | `w13:t20` | `dory-aoe5p-sit` | none |
| factory | `w13:p2R` | `w13:t13` | `1` | cursor working; **not sat** |

`herdr pane get w13:p98` → `tab_id=w13:t31` `pane_id=w13:p98` `agent_status=unknown` (no `.agent`). After wipe: cwd=`/home/manhquy/.cache/dory-isolates/aoe5p.azx4PH (deleted)`.

`herdr tab get w13:t31` → `label=dory-aoe5p-testsit` `pane_count=1`.

`HERDR_PANE_ID=w13:p9K` `HERDR_TAB_ID=w13:t33` ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. Sit pane ≠ `w13:p2R` / `*wP:*`. Sit tab ≠ `w13:t13`. Sit pane ≠ cook sit `w13:p97`.

## Needles (next, not gate)

After-run ground truth: **`Flow 1. next` / `Flow 0. next`**.

Watcher visible `--source visible` on `w13:p98` (read only; no attach this turn):

| When | `Flow 1. next` | `Flow 0. next` | `Flow *. gate` |
|---|---|---|---|
| after wipe (`azx4PH (deleted)`) | 0 | 1 | 0 |

Chrome footer `Flow 0. next`. Spaces `aoe5p.azx4PH`. Agents `coord p1` / `omptest unknown p2`. Not `Flow 1. gate` / `Flow 0. gate`.

Sit necessary, not sufficient. `Flow 0. next` is shared by empty-tree PASS and unlock PASS. Land = copied journal stdout + 03 sha. **Sit hit + weak journal = FAIL.**

## Journal (this sit ISO — not cook)

Copied `plans/reports/260829-ensure-aoe5-flow-prd-journal.jsonl`. cwd all four rows = `…/dory-isolates/aoe5p.azx4PH` (not cook `aoe5p.eGZMMi`). Independent parse of the copy; isolate wiped; path not re-read.

| # | type | args | code | land |
|---|---|---|---|---|
| 1 | `flow/result` | `["next"]` | 1 | `FAIL: gate for stage 02-scope is not clean.` |
| 2 | `flow/result` | `["next"]` | 0 | `unlocked stage 3 (flow/03-prd.md)` |

Exactly two `flow/result`. `bin` both = abs `/home/manhquy/.claude/skills/flow/runner/flow.sh`. Not `/bin/true`. Not `["gate","02-scope"]`. Taxi1 no `unlocked stage`. Taxi2 no `already exists` / `unlocked stage 2` / stage 1 / stage 00. Taxi2 stdout also has flow-lock reclaim NOTE (same class as O). Taxi2 land is unlock-3, not substring `clean`.

`03.sha256` same iso: digest `219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4` == live `_templates/03-prd.md`. Body not copied. Receipt second line is PRD template (not SCOPE_TEMPLATE).

Journal **not weak** → sit chrome may stand. If this copy had been cook `eGZMMi` / `GATE` / `/bin/true` / missing unlock-3, sit hit would still be **FAIL**.

## SIT_DORY land sha

Live `sha256sum` `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` (stat only; not exec'd from this pane):

`2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3`

Script `LAND_SHA` same. Prefix `2ef20730`.

## Factory doors (this pane)

| Door | After run |
|---|---|
| PATH `dory` | empty (`type: dory not found`) |
| default sock `/run/user/1000/dory/default/dory.sock` | `FileNotFoundError`, connectable=0 |
| `$XDG_RUNTIME_DIR/dory` / `dory/default` | absent |
| factory `flow/` | absent |
| repo `.dory/` | ABSENT |
| `SIT_DORY` land sha | `2ef20730…` (not exec'd from this pane) |
| `~/.local/bin/dory` | absent |

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT` UNSET. `HOME=/home/manhquy`. `XDG_RUNTIME_DIR=/run/user/1000`.

Factory `herdr pane get w13:p2R` → `tab_id=w13:t13` `agent=cursor` working. This pane did not target it.

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
- recook / fill `03-prd.md`
- cite cook `eGZMMi` journal as this sit's proof

## Result

`SIT_PASS`. Sit was `w13:t31`/`w13:p98`, not `t13`/`p2R`/`wP`. Needles `Flow 1. next` / `Flow 0. next`. Sit hit + weak journal = FAIL; this journal is `aoe5p.azx4PH` `[1,0]` `args=["next"]` unlock-3, so not that fail. Land sha `2ef20730…`. Factory sock connectable=0. `t13` not sat.
