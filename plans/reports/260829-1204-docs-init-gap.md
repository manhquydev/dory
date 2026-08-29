---
type: docs-init-gap
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
pane: di_gap
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
land_readme: 5ac82b10
leftover_readme: 68190a5f
result: NO_MISSING_ROUTE
second_docs_index: 0
adr_farm: 0
leftover_readme_rewrite: none
dory: not invoked
t13: not sat
---

# docs-init gap — missing route

Contract: [260829-1204-brainstorm-docs.md](260829-1204-brainstorm-docs.md). Approach **A**. Pane `di_gap`.

Land README = `git show HEAD:README.md` (blob `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd`). Working `README.md` is leftover 5. Did not rewrite it.

Evidence HEAD `049e304` `docs(plan): check isolate prd-unlock phases`. P feat `f1c966c` `feat(isolate): fail-then-pass flow.sh prd`. Rust log `git log -1 -- rust/` = `b544f5f`. Isolate unlock-3 ≠ company Phase 5.

## Verdict

**No missing evergreen route.** `docs/` is one file. CHARTER has one WHERE pointer. No second docs index. No ADR farm. No leftover README rewrite.

Pre-wave hole (journal text: no `docs/` route, no P journal) is closed. What remains is named unpaid / rejected-this-wave, not a second index.

## Closed (route exists)

| Need | Live |
|---|---|
| Evergreen WHERE | `docs/README.md` only (`??`). Audience + authority table + paid isolate owners + named unpaid + factory doors |
| CHARTER WHERE | one link: `[docs/README.md](docs/README.md)` after the cold-read line. String `WHERE` ×1. Kill bullets byte-identical to `HEAD:CHARTER.md` |
| Paid owners | four HEAD scripts; copy-table; not sourced |
| Unpaid remainder | named in `docs/README.md` (default sit, fill 03, walk 04–05, `card`, semantic, company Phase 5, leftover fold, `--wait`) |
| Agent deny | `AGENTS.md` deny-list only (`??`). Not a product overview. Not a second docs index |
| P journal | `plans/journals/2026-08-29-p-unlock-3-shipped-docs-route-opened-3.md` validates `ok:true`. File is source of truth |
| P paper 100% | working `260829-0054-isolate-flow-prd-unlock`: YAML `completed`; phases table Done ×3; success `[x]` ×6; phase files YAML `completed`. Not a recook |
| Leftover 5 | still unstaged ` M` ×5. README hash-object `68190a5f…` mint |
| Red-team | `260829-1204-redteam.md` critical 0 |

## Confirm — non-goals held

### No second docs index

`find docs -type f` → `docs/README.md` only. No `docs/index.md`. No extra `docs/*.md`. `AGENTS.md` is process memory (HOW-TO-BEHAVE), not WHERE. Land README is `git show HEAD:README.md`, not a docs index.

### No ADR farm

`find` `*adr*` under the repo (minus `.git` / `eval/` / `.claude/`) = **0**. Journal template footer still says “Prefer docs/specs/ADRs”; that is store boilerplate, not an ADR directory this wave.

### No leftover README rewrite

| Pin | Live |
|---|---|
| WT `git hash-object README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` MATCH mint `68190a5f` |
| `HEAD:README.md` | `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd` (different blob) |
| Porcelain | ` M README.md` unstaged; cached empty |
| This pane | did not edit `README.md` |

Leftover still carries `## Now (25 Aug 2026)`. Land still carries `## Xong tới đâu` with no isolate-taxi row. Dual tree. Fold remints leftover and unlands C.

## Not a missing route (named unpaid / rejected)

These are holes in *land leftover paper*, not a missing `docs/` file. Approach **B** (copy-aside leftover, land-edit HEAD `Xong tới đâu`) rejected this wave.

1. **`git show HEAD:README.md` `## Xong tới đâu`** — no isolate taxi N/O/P. Operator desk how-to. Path to WHERE is land → CHARTER → `docs/README.md`. Do not land-edit this wave.
2. **Working leftover `## Now`** — 25 Aug factory occupancy. Remint = leftover-README rewrite = non-goal.
3. **Company Phase 5 / fill `03-prd.md` / walk 04–05 / `card` / default occupancy** — named unpaid in CHARTER freeze + docs unpaid list + P remainder. Not this wave.
4. **Triple journal copies** — unsuffixed / `-2` / `-3` same md5 `369472c9…`. Not missing. Validate the `-3` slug.
5. **`ak journal list` store empty** — nit. File path is the owner (`260829-1204-docs-journal-val.md`).

## Leftover 5 hold (this pane)

| Path | WT `git hash-object` |
|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` |

Cite rust via `git show HEAD:rust/…`. Do not open leftover rust as land.

## This pane did not

- edit leftover 5 / rewrite leftover README / checkout-restore leftover
- create a second docs index / ADR / `docs/` sibling
- recook P / fill `03-prd.md` / claim company Phase 5
- sit `w13:t13` / `w13:p2R` / start default `dory.sock` / invoke `dory`
- `herdr server stop` / close `wP` `w15` `w16` `t13`
- `git add -A` / `ak:git`
