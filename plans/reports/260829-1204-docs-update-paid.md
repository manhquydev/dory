---
type: docs-update
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
pane: du_paid
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
result: PASS
---

# Docs update — paid isolate owners

Contract: `plans/reports/260829-1204-brainstorm-docs.md`. This pane: confirm `docs/README.md` paid table owners exist on HEAD. No product rust. No leftover 5 edit. No sit `t13`. No default sock. No recook P. Copy-table only — scripts not sourced/exec'd.

Land README = `git show HEAD:README.md` (blob `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd`). Working `README.md` leftover mint `68190a5f`, still ` M`.

## Pins

| Signal | Value |
|---|---|
| Paper HEAD | `049e304` `docs(plan): check isolate prd-unlock phases` |
| P feat | `f1c966c` `feat(isolate): fail-then-pass flow.sh prd` |
| Rust land | `b544f5f` `fix(attach): do not auto-start server on sit`; `git diff --stat b544f5f HEAD -- rust/` empty |
| `git ls-files scripts/` | exactly the four paid owners below |
| Leftover 5 | ` M` ×5; README `68190a5f` |

`docs/README.md` is worktree `??` (not a HEAD blob). CHARTER WHERE already points there. Table owners are HEAD paths.

## Paid table vs HEAD

`docs/README.md` § Paid isolate taxi. P unlock-3 is paid. Not company Phase 5. Not fill-the-PRD.

| Rung | Owner | HEAD mode/blob | Landing | WT `hash-object` |
|---|---|---|---|---|
| AOE5 `gate` | `scripts/dory-isolate-aoe5-flow-judge.sh` | `100755` `e706e0c4472c8b6a1a6b9d6059cd065094c0395f` | `0475e6b` `flow.sh gate` | == HEAD |
| N unlock-1 | `scripts/dory-isolate-aoe5-flow-next.sh` | `100755` `647fec4011478d2efbb5041c9d97e0fbd0117cae` | `bcf7c72` `flow.sh next` | == HEAD |
| O unlock-2 | `scripts/dory-isolate-aoe5-flow-scope.sh` | `100755` `0cddb0c1e3275d2670283f5f76678bb08bf166a0` | `bcab751` `flow.sh scope` | == HEAD |
| P unlock-3 | `scripts/dory-isolate-aoe5-flow-prd.sh` | `100755` `56045085b966a2c9cb65a102b56a7cac35a0d59b` | `f1c966c` `flow.sh prd` | == HEAD |

Same four blobs on `f1c966c`. Discover P paper: `git log --oneline -- scripts/dory-isolate-aoe5-flow-prd.sh`.

## Not in this table

Worktree isolate scripts that are not HEAD (`dory-isolate-flow-sit.sh`, flock `roster`/`report`/`prompt`, `dory-flock-hop.sh`) are not paid-table owners. Default occupancy is unpaid. Company Phase 5 is unpaid.

## Do-not

- Recook P / fill `03-prd.md` / walk 04–05 / `card`
- Claim isolate unlock N/O/P is company Phase 5
- Edit leftover 5 / fold README / cite working README as land
- Sit `w13:t13` / start default sock / `herdr server stop` / close `wP` `w15` `w16` `t13`
- `source`/`exec` the four isolate scripts from this sibling
