---
type: research
date: 2026-08-27
time: 16:56
researcher: researcher-01
kind: flow-next-unlock-stage-1
repo: /home/manhquy/Downloads/flow/dory
judge: /home/manhquy/.claude/skills/flow/runner/flow.sh
probe: /tmp/flow-next-unlock-260827-1656
scope: how flow.sh next unlocks stage 1; no implement; no dory; no factory sock; no sit w13:t13
---

# Research — `flow.sh next` unlocks stage 1

Land = `git show HEAD:path:line`. Not leftover working rust. Desk glance from HEAD blob. Judge = skill `flow.sh`. Taxi cite HEAD `flow.rs`. Paid AOE5 script **read only**, not exec.

This pane: `flow.sh` + `git show` + `/tmp` probe. No factory `dory`. No `/run/user/1000/dory/default`. No w13:t13.

## 1. One sentence

**Unlock stage 1 = occupant makes `flow/00-idea.md` mechanically clean, then taxi `dory flow -- next` (not `gate`).** `flow.sh` copies `_templates/01-research.md` → `flow/01-research.md` and prints `unlocked stage 1`. Dory does not know `next`. Dory execs `FLOW_BIN` argv after `--`.

## 2. Exact argv

Paid isolate taxi (`scripts/dory-isolate-aoe5-flow-judge.sh:261-266`):

```
"$SIT_DORY" flow -- gate 00-idea
```

Child argv to `FLOW_BIN`: `["gate","00-idea"]`. `cmd_gate` is **read-only** (`flow.sh:1403-1428`). No template copy. Stdout `GATE stage 00-idea:` then findings or `  clean`.

Next cook taxi (same envelope, one token change — eval `260827-1638-eval-next.md:61-65`):

```
"$SIT_DORY" flow -- next
```

Child argv: `["next"]`. Dispatch `flow.sh:3906` → `cmd_next` (`:950-1030`).

`--` is the land. Missing `--` → `dory: usage: dory flow -- <args>` rc=2 (`HEAD:rust/src/flow.rs:26-29`). That is the forbidden button `dory flow next`. Empty after `--` defaults `status` (`flow.rs:34-37`). Dory has **no** `next`/`card`/`check` (`flow.rs:3`).

## 3. Mechanical unlock (file:line)

`STAGES="00-idea 01-research …"` (`flow.sh:122`). `current_stage_idx` = highest contiguous file from 00 (`:136-152`). With only `00-idea.md`, idx=0.

`cmd_next` (`:950-1030`):

1. `lock_acquire next` (`:951`, `:496`) — writes `flow/.lock`.
2. `scan_gate flow/00-idea.md` (`:157-176`, called `:965`). Unchecked `- [ ]` **or** leftover `[FILL` → rc=1. **No unlock.**
3. If clean and `01-research.md` missing: `cp "$TEMPLATE_DIR/01-research.md" "$FLOW_DIR/01-research.md"` (`:1024`). `TEMPLATE_DIR=$SCRIPT_DIR/../_templates` (`:20`).
4. Print `PASS: stage $cur gate clean -> unlocked stage $nidx (flow/$nxt.md)` (`:1026`). Here `$cur=00-idea` `$nidx=1` `$nxt=01-research`.
5. `gate_durable_hook 00-idea` is a **no-op** (only `01-research` / `04-adr` seed harness) (`:660-677`).
6. `_graph_stage_record` no-ops if graph off (`:339-346`). Probe created no harness files.

`gate` on the same clean file: `GATE stage 00-idea:` / `  clean` / rc=0. **Does not** mint `01-research.md`. Probe: no `02-scope.md` after pass-gate.

Empty tree first `next` copies `00-idea.md` and rc=0 (`:953-962`). That is **not** the fail-then-pass án. Isolate must mint FAIL idea first (`judge.sh:567-568`).

## 4. /tmp probe (`flow.sh` only, no dory)

Dir: `/tmp/flow-next-unlock-260827-1656`. Env: `FLOW_PROJECT_ROOT=$PROBE` `FLOW_LOG_DISABLE=1` `DO_NOT_TRACK=1`. `FLOW_BIN` unset. `command -v dory` = absent. Factory sock lexists=0 connectable=0.

Seed: `cp` skill `_templates/00-idea.md` → `flow/00-idea.md` (unchecked + four `[FILL`).

| Step | argv | rc | ms | stdout must-have | 01-research.md |
|---|---|---|---|---|---|
| gate FAIL | `gate 00-idea` | 1 | 24 | `GATE stage 00-idea:` | no |
| next FAIL | `next` | 1 | 58 | `FAIL: gate for stage 00-idea is not clean` | no |
| next PASS | `next` | 0 | 57 | `PASS: stage 00-idea gate clean -> unlocked stage 1 (flow/01-research.md)` | **yes** |
| gate PASS | `gate 00-idea` | 0 | 22 | `GATE stage 00-idea:` + `  clean` | unchanged; no 02 |

FAIL full first line + scan (`flow.sh:966-967`):

```
FAIL: gate for stage 00-idea is not clean.
  [x] unchecked gate boxes:
      L4:- [ ] …
  [x] unfilled [FILL] placeholders:
      L10:[FILL: sentence 1 — who has the problem]
      …
```

PASS exact line (`flow.sh:1026`):

```
PASS: stage 00-idea gate clean -> unlocked stage 1 (flow/01-research.md)
```

Then tip `'/flow recall' …` (`:1029`).

**4. Does PASS create `flow/01-research.md`?** Yes. Byte-identical to `/home/manhquy/.claude/skills/flow/_templates/01-research.md` (sha256 match). First line `# Stage 01 — Research (inspect first)`. Still `[FILL]` + unchecked boxes — **do not fill**.

**5. Does `next` mutate `00-idea.md` besides unlocking?** No. sha256(seed)==sha256(after FAIL next). sha256(PASS write)==sha256(after PASS next). Unlock = new sibling file, not rewrite pitch/boxes. Side write this probe: `flow/.lock` only. No `.flow/` (log disabled). No `MODE` / law copies (those are first-unlock / missing-00 path `:953-962`).

## 5. Env

| Var | `flow.sh` (this probe) | Dory taxi (isolate) |
|---|---|---|
| `FLOW_PROJECT_ROOT` | Wins ROOT (`flow.sh:30-31`). Pin isolate or `cd` project. Else ancestor walk (`:34-46`). | **Pin on taxi line** `$ISO_REAL`. AOE5: `judge.sh:52-54` refuse factory set; `:264` pin. |
| `FLOW_BIN` | **Unused.** Judge is the script itself. | **Required pin** abs `…/flow/runner/flow.sh` (`flow.rs:39`, `judge.sh:456-468`). Absent → `"flow.sh"` on PATH (`flow.rs:121-125`). Refuse `/bin/true`. |
| `FLOW_LOG_DISABLE` / `DO_NOT_TRACK` | Optional. `_log_disabled` (`flow.sh:3760-3761`). Unlock works without. | AOE5 pins both (`:265`). Hygiene: no `.flow/events.jsonl`. Not an án. |
| `DORY_ENV=1` | N/A | Required or taxi dies before exec (`require_skill_env`, cited 1116 `:68`). |
| `DORY_WORKSPACE_DIR` | N/A | Journal cwd (`flow.rs:105-118`). Same as project. |

Probe: `FLOW_BIN` unset, unlock still happened. Taxi without `FLOW_BIN` is chrome risk (1910 `/bin/true`).

## 6. Time vs taxi 15s

`HEAD:rust/src/flow.rs:14` `DEFAULT_TIMEOUT = 15_000` ms. Then SIGTERM / 1s grace / SIGKILL (`:15`, `:201-211`).

Probe `next` FAIL 58ms / PASS 57ms. ≪ 15s. No rust timeout bump. Same class as AOE5 tiny fixture.

## 7. Journal + glance

Taxi journals `{cwd}/.dory/sessions/s1.jsonl` (`flow.rs:16,66-100,115-118`). `args` = argv after `--`.

Glance reads last `flow/result`, `arg0` = first `args[]` string (`HEAD:rust/src/desk.rs:2321-2358`). Footer `Flow {n}. {arg0}` (`:3450-3458`). **Not stdout.**

| Taxi | journal `args` | `code` | sit needle |
|---|---|---|---|
| paid `gate 00-idea` FAIL/PASS | `["gate","00-idea"]` | 1 then 0 | `Flow 1. gate` / `Flow 0. gate` (`judge.sh:700,745`) |
| next FAIL/PASS | `["next"]` | 1 then 0 | **`Flow 1. next` / `Flow 0. next`** |

`bin` both rows = abs `flow.sh`. Exactly two `flow/result`, codes `[1,0]` (eval-next `:71-74`).

## 8. Ranked choice

| Option | Does it unlock 01? | Complexity | Adoption risk | Fit |
|---|---|---|---|---|
| **A. Isolate taxi `dory flow -- next` fail-then-pass** | Yes. World-state `flow/01-research.md` | Copy AOE5 law; change argv + stdout needles | Low. Judge already paid. Taxi already land | **Pick.** One increment. |
| B. Recook `gate 00-idea` | No | Zero | Wastes the paid slice | Reject |
| C. Dory button / rust `next` | Lie. Moves gate into Dory | High | Founder ban (`2105:28-31,39`; `CHARTER.md:30-32`; `flow.rs:3`) | Reject |
| D. Fill 01 + walk 02–05 / `card` | Company AOE 5, not this cook | High | Trap 13 harness mutate; 1122 remainder | Reject |

**Recommendation:** A. New script `scripts/dory-isolate-aoe5-flow-next.sh`. Do not source/exec paid judge script. Same FAIL mint + occupant PASS. Taxi `"$SIT_DORY" flow -- next`. Pin `FLOW_BIN` + `FLOW_PROJECT_ROOT` + `FLOW_LOG_DISABLE=1`. Án = codes `[1,0]` + FAIL/PASS substrings above + `01-research.md` exists under `$ISO_REAL` + glance `Flow 1. next` then `Flow 0. next`.

## 9. Forbidden this cook

- `card`. Fill `01-research.md`. Walk 02–05. Semantic `gate-rules.md` as unlock.
- Dory button `dory flow next` (no `--`). Grow `next`/`card`/`check` in rust.
- Exec paid `dory-isolate-aoe5-flow-judge.sh` / 1910/0043/0227/0242/hop.
- Factory `dory`. Start `/run/user/1000/dory/default`. Sit `w13:t13`. `herdr server stop`.
- `prompt --wait`. Factory-write PASS idea. `FLOW_BIN=/bin/true`.

## Limitations

No live Dory taxi (banned this pane). Glance needles are HEAD-deduced + AOE5 gate analog, not a sit this turn. Harness/graph off in `/tmp` — production isolate with harness on still must not rewrite `00-idea.md` (`gate_durable_hook` 00 is empty). Did not re-read leftover `desk.rs` as land.

## Unresolved

None for the mechanical unlock. Occupant PASS bytes stay isolate-local (not this report). Script authoring is the next cook, not this file.

---

**paid-this-cook** isolate AOE5 `gate 00-idea` + this `/tmp` `flow.sh next` fail-then-pass (rc `[1,0]`, 01 minted from template).
**unpaid** isolate taxi `flow -- next`; company 6-stage; default sit; leftover fold; rust `next`.
**do-not** `card`; fill 01; walk 02–05; `dory flow next` without `--`; factory `dory`; start default sock; sit w13:t13; exec paid isolate; implement here.
**evidence** `flow.sh:20,122,136-176,496,660-677,950-1030,1403-1428,3760-3761,3906`; `_templates/00-idea.md`; `_templates/01-research.md`; `HEAD:rust/src/flow.rs:3,14,26-39,101-102,121-125`; `HEAD:rust/src/desk.rs:2321-2358,3450-3458`; `scripts/dory-isolate-aoe5-flow-judge.sh:52-54,261-266,456-468,567-568,700,745`; `CHARTER.md:30-32`; `plans/reports/260825-2105-brainstorm-herdr-depth-founder.md:39`; `plans/reports/260827-1638-eval-next.md:51-74`; probe `/tmp/flow-next-unlock-260827-1656` FAIL 58ms / PASS 57ms.
