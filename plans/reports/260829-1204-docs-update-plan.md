---
type: docs-update-plan
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
contract: plans/reports/260829-1204-brainstorm-docs.md
approach: A
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
land_readme: git show HEAD:README.md
land_readme_blob: 5ac82b102be4e4f0c621d779b9c4a3bb9819afbd
leftover_readme: 68190a5ffa073c082aa318aad5ed032e13cc90e3
p_plan: plans/260829-0054-isolate-flow-prd-unlock
p_status: completed
p_progress: 3/3 phases, 5/5 tasks
recook_p: no
company_phase_5: unpaid
---

# Docs update plan — after P

Contract: [260829-1204-brainstorm-docs.md](260829-1204-brainstorm-docs.md). Approach **A**. Do not recook P. Do not claim company Phase 5.

Land README = `git show HEAD:README.md`. Working `README.md` is leftover 5.

## Contract (closed)

| Field | Closed |
|---|---|
| **Outcome** | Evergreen WHERE under `docs/`. CHARTER one WHERE link. Journal records P ship. `AGENTS.md` deny-list only. P plan frontmatter matches 100% phases. Untracked P receipts that already exist may join the named reports set. |
| **Constraints** | No product rust. No leftover 5 edit. Cite land README as `git show HEAD:README.md`. No default sock. No sit `t13`. No `herdr server stop`. Keep `wP`/`w15`/`w16`/`t13`. No recook P. No fill 03. Docs own WHY/WHERE only. |
| **Non-goals** | Company Phase 5. Default occupancy. Leftover fold. Walk 04–05. `card`. Semantic. Push. Rewrite leftover README. ADR farm. Recook P. Fill `flow/03-prd.md`. |
| **Audience** | People and AI. Cold start: CHARTER → `docs/README.md` → executable owners. |

## Evidence (this wave)

| Pin | Live |
|---|---|
| Paper HEAD | `049e304` `docs(plan): check isolate prd-unlock phases` |
| P feat | `f1c966c` `feat(isolate): fail-then-pass flow.sh prd` |
| Rust log | `git log -1 -- rust/` → `b544f5f` `fix(attach): do not auto-start server on sit` |
| Land README | `git show HEAD:README.md` blob `5ac82b10…` |
| WT leftover README | `git hash-object README.md` = `68190a5f…` mint. Porcelain ` M`. Do not fold. |

Cite rust via `git show HEAD:rust/…`. Do not open leftover rust as land.

## P plan — completed; remainder unpaid

Confirm, do not recook.

`ak plan status ./plans/260829-0054-isolate-flow-prd-unlock --json`:

| Field | Value |
|---|---|
| `status` | `completed` |
| `phases_total` / `phases_done` | 3 / 3 |
| `total_tasks` / `done_tasks` | 5 / 5 |
| `progress_pct` | 100 |

Working `plans/260829-0054-isolate-flow-prd-unlock/plan.md`: frontmatter `status: completed`; phases table Done/Done/Done.

**No unpaid product hunk in 0054.** Next product cook is not this wave.

Remainder (named unpaid — still the list, still unpaid):

- Default occupancy
- `--wait` / leftover fold / skill taxi paper
- Fill 03 / walk 04–05 / `card` / semantic / company Phase 5

HEAD `049e304` still has this plan `status: pending` and phases table Pending. Working-tree paper already matches 100% phases. That delta is docs-wave paper on `plan.md` only. Phase `success criteria` checkboxes on HEAD are already `[x]` (5/5). Plan-level Success Criteria boxes on `plan.md` may stay unchecked — they are not the five tasks `ak` counts. Do not recook isolate to tick them.

## Approaches

| # | Approach | Verdict |
|---|---|---|
| **A** | New `docs/` + CHARTER pointer + journal + thin `AGENTS.md`. Do not touch leftover README. | **Chọn.** |
| B | Copy-aside leftover README, land-edit HEAD "Xong tới đâu". | Reject this wave. Leftover mint miss = fold. |
| C | Recook isolate / fill 03 because "kế hoạch còn". | Reject. P is 100%. Remainder is unpaid company. |

## Authority (smallest surfaces)

| Surface | Owns | Not |
|---|---|---|
| [CHARTER.md](../../CHARTER.md) | WHY: two chairs, kill, hình B. One WHERE link. | Cook SHAs, isolate taxi how-to |
| [docs/README.md](../../docs/README.md) | WHERE: paid isolate owners + named unpaid + leftover door | Command inventories, SHA tables, test names |
| [CAPACITY-FREEZE.md](../../CAPACITY-FREEZE.md) | Paid AOE 0; company Phase 5 freeze | Isolate receipts |
| Land `README` | Operator desk how-to | Working leftover README |
| `AGENTS.md` | Costly-action deny list | Product overview |
| `plans/` + `plans/reports/` + `plans/journals/` | Stateful evidence | Evergreen law |
| `scripts/dory-isolate-*.sh` | Isolate taxi contracts | Company occupancy |

Code owns WHAT/HOW. Point; do not paraphrase scripts.

## Reconcile (prune, do not refresh)

### Create / keep

| Path | Job |
|---|---|
| `docs/README.md` | Evergreen WHERE. Route CHARTER + paid isolate owners + unpaid remainder + leftover door. Land README cited as `git show HEAD:README.md`. P unlock-3 paid. Not company Phase 5. Not fill-the-PRD. |
| `AGENTS.md` | Deny-list only. Process memory. WHY → CHARTER. WHERE → `docs/README.md`. |
| `plans/journals/2026-08-29-p-unlock-3-shipped-docs-route-opened*.md` | P ship journal. File is source of truth if `ak journal list` store is empty. |
| Existing `plans/reports/260829-ensure-aoe5-flow-prd-*` | May join named reports set. Do not rewrite as evergreen. |

Paid isolate owners (point, do not copy bodies):

| Rung | Owner |
|---|---|
| AOE5 `gate` | `scripts/dory-isolate-aoe5-flow-judge.sh` |
| N unlock-1 | `scripts/dory-isolate-aoe5-flow-next.sh` |
| O unlock-2 | `scripts/dory-isolate-aoe5-flow-scope.sh` |
| P unlock-3 | `scripts/dory-isolate-aoe5-flow-prd.sh` |

Discover current paper with `git log --oneline -- scripts/dory-isolate-aoe5-flow-prd.sh`.

### Modify (one pointer)

| Path | Job |
|---|---|
| `CHARTER.md` | Insert **one** WHERE link to `docs/README.md` after the cold-read line, before `## Bốn hộp`. Kill conditions unchanged. |
| `plans/260829-0054-isolate-flow-prd-unlock/plan.md` | Paper only: `status: completed` + phases table Done iff `ak` already 3/3 5/5. Do not delete Remainder. Do not recook. |

### Do not touch

Leftover 5 (must stay ` M` mint):

- `README.md` (`68190a5f…`)
- `rust/src/attach.rs`
- `rust/src/main.rs`
- `rust/src/server.rs`
- `rust/tests/p5_attach.rs`

Also: no product rust, no leftover cargo, no `git add -A`, no `ak:git`, no default `dory.sock`, no sit `t13`/`p2R`, no `herdr server stop`, no close `wP`/`w15`/`w16`/`t13`, no fill `flow/03-prd.md`, no walk 04–05.

## Factory doors

Herdr is the factory chair. New `w13` tabs, `--no-focus`, cwd dory. Do not split `t13`. Stop isolate only with the 2357 compound **inside isolate scripts** — not this wave.

## Observed vs remaining (1204)

Sibling receipts already exist. This file is the update plan, not a recook trigger.

| Hunk | Observed |
|---|---|
| `docs/README.md` | Present. Route + paid owners + unpaid + leftover door. |
| CHARTER WHERE | One link. Kill block byte-identical to HEAD. |
| `AGENTS.md` | Deny-list only. |
| P journal | `2026-08-29-p-unlock-3-shipped-docs-route-opened-3` validates `ok:true`. |
| 0054 | `completed`, 3/3, 5/5. Remainder unpaid. |
| Leftover 5 | Still ` M` mint. Cached leftover empty. |
| Red-team | [260829-1204-redteam.md](260829-1204-redteam.md) `REVIEW_ACCEPT` critical 0. |

Remaining this wave: named pathspec executed by `/ak-cook` remainder (`260829-1331-cook-remainder.md`). Do not push.

## Acceptance

- [x] `docs/README.md` routes CHARTER + paid isolate owners + unpaid remainder
- [x] CHARTER has **one** WHERE link
- [x] `ak journal` entry for P (file path is authority)
- [x] `AGENTS.md` deny-list only
- [x] Red-team critical 0 on stale Phase-5 / leftover-fold / t13-sit claims
- [x] Leftover 5 still `M`; README hash-object still `68190a5f…`
- [x] Rust log `b544f5f`
- [x] 0054 remains `completed` 3/3 5/5; remainder still unpaid
- [x] No leftover 5 in index; no `git add -A`

## Ship (named pathspec — cook remainder 13:31)

Allow: `docs/README.md`, `CHARTER.md`, `AGENTS.md`, this wave's `plans/reports/260829-1204-*`, harvest `260829-1219-docs-harvest.md`, P journal files `opened*.md`, 0054 `plan.md` + phase paper, late P receipts `ensure-aoe5-flow-prd-{test-sit,hold-rust,review,review-left,review-next}.md`, cook remainder receipt.

Deny: leftover 5, `rust/`, `ak:git`, `git add -A`, push.

## Named unpaid (not this wave)

- Default occupancy / sit default (dead sock ≠ sâu)
- Fill `03-prd.md` / walk 04–05 / `card` / semantic `gate-rules.md`
- Company Phase 5 (a real project completed *inside* Dory)
- Leftover 5 fold / PATH retarget / leftover ELF rm
- Founder `prompt --wait` / skill taxi paper
