---
type: brainstorm
date: 2026-08-27
time: 09:27
status: accepted
advice: kongming GO
feeds: exploration-only
authority:
  - CHARTER.md
  - CAPACITY-FREEZE.md
  - plans/reports/260825-2105-brainstorm-herdr-depth-founder.md
  - plans/260827-0242-clear-report-on-prompt/plan.md
sources:
  - plans/reports/260827-0918-eval-roster.md
  - plans/reports/260827-0918-eval-charter.md
  - plans/reports/260827-0918-eval-aoe.md
  - plans/reports/260827-0918-eval-desk.md
  - plans/reports/260827-0918-eval-occup.md
  - plans/reports/260827-0918-eval-coord.md
  - plans/reports/260827-0918-eval-isol.md
  - plans/reports/260827-0918-eval-left.md
  - plans/reports/260827-0918-eval-taxi.md
  - plans/reports/260827-0918-eval-test.md
  - plans/reports/260827-0918-eval-sec.md
  - plans/reports/260827-0918-eval-docs.md
  - plans/reports/260827-0918-eval-next.md
live:
  head: 5a6095367f905a42ff1c38886ebffa0f0840977d
  factory: w13:t13 / w13:p2R
  eval_tabs: [w13:t1T, w13:t1V]
  default_sock: absent
---

# Brainstorm — 09:27 eval-team acceptance

Twelve Herdr OMP panes wrote scoped reports. Factory re-read every file,
re-checked git / leftover hashes / sock / PATH, then asked kongming
(`--advise`). This note is the durable picture. It is **not** a cook.

## Contract

| Field | Closed |
|---|---|
| **Outcome** | One accepted Dory status map at HEAD `5a60953`. Wave tabs `w13:t1T` / `w13:t1V` closed after nghiệm thu. Factory stays `t13`. Default sock stays absent. Leftover 5 stays mint. |
| **Constraints** | Two chairs. Do not sit `t13` / `p2R`. Do not start `/run/user/1000/dory/default`. Do not fold leftover 5. Do not cargo leftover tree. Do not close `wP` / `w15` / `t13`. Do not `herdr server stop`. Cite `git show HEAD`, never leftover `server.rs`. |
| **Non-goals** | Rust this wave. Recook 1910 / 0043 / 0227 / 0242. `occ.report = Working` (0242 trap 10). Isolate `prompt --wait` as the next hunk. Paint `idle`. AOE 5. Default occupancy. Unlink PATH in this close (recommend only). Rewrite leftover README Now in this close (remints leftover hash). |
| **Acceptance** | All 12 reports on disk. Live hashes match mint. HEAD has `occ.report = None` at `server.rs:1501` and no `workspace_live_cwd`. Sock absent. Kongming GO. Tabs `t1T`/`t1V` closed; `t13`/`wP`/`w15` still live. |

0240/0242 contract is already paid. Do not reopen it.

## Live 09:27 (factory, not leftover)

| Signal | Value |
|---|---|
| HEAD | `5a60953` `fix(server): clear occupant report on agent prompt` |
| HEAD `occ.report = None` | `git show HEAD:rust/src/server.rs:1501` |
| HEAD `workspace_live_cwd` | absent |
| Leftover 5 WT | `68190a5f` `60247909` `373d6886` `4de1554a` `9c28fc3e` (mint) |
| Leftover `server.rs` | still has `workspace_live_cwd`; **no** `occ.report = None` |
| `desk.rs` | clean `4c788562` == HEAD |
| Default sock | absent |
| PATH `dory` | `~/.local/bin/dory` → leftover `rust/target/debug/dory` |
| `SIT_DORY` | `land-4b70f79` debug exists |
| Factory | `w13:t13` / `w13:p2R`. Eval tabs still open at write time. |

## Paid picture (12 panes agree)

Identity: Session OS + Workplace OS. Flow foreign judge. Hình B. Kill list intact. Two chairs (factory Herdr / shipped Dory). CHARTER + FREEZE untouched.

| AOE | Git | Isolate | Default |
|---|---|---|---|
| 0 freeze | paper paid | same | same |
| 2 desk | overlay + glance + Agents + idle-word hide + `desk_tree` `world.cwd` (`f6614d9`) | 1910 sit + taxi chrome | unpaid (sock absent) |
| 3 occupancy | classify + report + `done` from idle+!seen | 0043 names; 0227 `done` words; 0242 same | unpaid |
| 4 coord | `5a60953` clear report before stall; `p5_prompt_after_report` | 0043 sibling on unknown; 0242 sibling **after** report | unpaid |
| 5 operator | unpaid | 1910 `FLOW_BIN=/bin/true` ≠ judge | unpaid |

Taxi `dory flow --` + footer last `flow/result` paid on git. `--kind` refused. Isolate scripts pin `SIT_DORY`, 2357 stop, connectable-only factory FAIL.

Founder 2105 (1) on isolate: nhìn đàn **và** giao việc (prompt, no `--wait`). Desk chiếu Flow chrome. Default trống ≠ sâu.

## Gaps that stay gaps

- After clear, omp is `unknown` (not allowlisted). Stall exits. `wait_hit` is only idle\|done\|blocked → `prompt --wait` after report cannot settle until a new report.
- `agent_send_keys` does not clear `occ.report` (0242 non-goal).
- PATH leftover binary embeds `workspace_live_cwd` and lacks report-clear. One factory `dory` / `dory attach` can mint default **and** paint leftover cwd.
- `ensure_server` still auto-spawns on ping miss (HEAD and leftover).
- Sit script has no isolate `/bin` PATH pin (flock does).
- `dory-isolate-flow-sit.sh` untracked.
- Leftover README `## Now` sells 25 Aug flock / HEAD `65a1d8a` as current.
- p5 never reports `working` / `blocked`.
- Five states: `working` / `blocked` never isolate-gated.

No status report is false vs live git/sock/PATH. **Next-fix lines are pane opinion, not a cook order.**

## Approaches (later increment only)

**A — Unlink leftover PATH `dory`.** Delete `~/.local/bin/dory`. Do not retarget to `land-4b70f79` (that puts a “correct” binary on factory PATH; bare `dory` still hits `ensure_server` → default sock). Sit PATH-pin and landing the sit script are the same family, after PATH is gone. README Now rewrite remints leftover `68190a5f` — not silent, not this close.

- Assumes: nobody needs PATH `dory` in factory today.
- Fails first: someone reinstalls leftover `dory`, or types the full leftover path.

**B — `occ.report = Working` + isolate `prompt --wait`.** Occup + aoe Next-fix.

- Assumes: năm từ may be painted by the OS, and omp will report again.
- Fails first: 0242 trap 10 (`plan.md:38`); wait hangs if no second report. **Reject.**

**C — `ensure_server` returns Err, no `spawn_server`.** Sec Next-fix. New sit contract: `dory server` then `dory`.

- Assumes: founder wants `dory` back on PATH later without auto-spawn.
- Fails first: leftover PATH still folds live-cwd if A is skipped. Not the rust version of A.

## Recommendation

**Accept the picture. Do not cook this wave.** 0242 is closed.

If a later increment is asked: **A = unlink PATH leftover `dory`**, not retarget, not B. **C only after PATH is not leftover**, and only if sit-without-spawn is the new public contract.

Kongming (`--advise`): **GO.** Next increment letter **A (unlink)**. Next risk: PATH leftover `dory`. Reject occup/aoe Next-fix as the cook, not as the gap picture.

## Close list

Close only wave tabs minted for this eval:

- `w13:t1T` dory-eval-core
- `w13:t1V` dory-eval-gap

Leave `w13:t13`, `wP`, `w15`. Do not unlink PATH in this close.

## Unresolved (not blocking accept)

- Founder has not asked to unlink PATH yet. Recommendation only.
- Whether C becomes a later rust contract is unknown until A is done or refused.
- p5 `working`/`blocked` is a real later test card, not this close.
