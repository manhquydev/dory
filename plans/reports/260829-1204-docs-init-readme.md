---
type: docs-init
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
pane: di_readme
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
land_readme: 5ac82b10
leftover_readme: 68190a5f
did_not: edit README.md; checkout/restore leftover; git add leftover 5; edit HEAD; recook P; sit t13; invoke dory; herdr server stop; claim company Phase 5
---

# di_readme — land vs leftover README

Contract: [260829-1204-brainstorm-docs.md](260829-1204-brainstorm-docs.md). Approach **A** (do not touch leftover README). Approach **B** (copy-aside leftover, land-edit HEAD `Xong tới đâu`) rejected this wave.

Land README = `git show HEAD:README.md`. Working `README.md` is leftover 5. Dual tree. Fold remints leftover and unlands C.

## Pins (this wave)

| Surface | Live |
|---|---|
| Paper HEAD | `049e304` `docs(plan): check isolate prd-unlock phases` |
| P feat | `f1c966c` `feat(isolate): fail-then-pass flow.sh prd` |
| Rust land | `b544f5f` `fix(attach): do not auto-start server on sit` (`git show HEAD:rust/…`) |
| Land README blob | `HEAD:README.md` = `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd` |
| Land README sha256 | `365a3ac0a197f93e3cf69ea55de859c26d6884407c4d5db1fe64ef5427081c4c` |
| WT README hash-object | `68190a5ffa073c082aa318aad5ed032e13cc90e3` MATCH mint `68190a5f` |
| WT README sha256 | `b9d151ada7d720fef088e07261d086515fad5b5071bf59baa211f3bc6034c289` |
| Porcelain README | unstaged ` M`; cached empty |
| HEAD edited? | **no** (`049e304` still HEAD) |

Leftover 5 still `M` (hash-object mint MATCH prior doors):

| File | WT `git hash-object` |
|---|---|
| `README.md` | `68190a5f…` |
| `rust/src/attach.rs` | `60247909…` |
| `rust/src/main.rs` | `373d6886…` |
| `rust/src/server.rs` | `4de1554a…` |
| `rust/tests/p5_attach.rs` | `9c28fc3e…` |

Cite rust only via `git show HEAD:rust/…`. Do not open leftover rust as land.

## Authority split

| Artifact | Role this wave |
|---|---|
| `git show HEAD:README.md` | Operator desk how-to (land) |
| Working `README.md` | Leftover 5 mint. Not the docs route. Do not rewrite `## Now`. |
| [docs/README.md](../../docs/README.md) | Evergreen WHERE (paid / unpaid / leftover door) |
| [CHARTER.md](../../CHARTER.md) | WHY + one WHERE link |

`git diff --numstat HEAD -- README.md` = `38 14 README.md`. That delta is leftover vs land, not a pending land edit.

## Why they must not collapse

Land sit (`git show HEAD:README.md`, heading `Mở`): `dory server` then `dory`. Matches land `ensure_server` fail-closed (`git show HEAD:rust/src/attach.rs`).

Leftover sit (working `README.md`, heading `Open`): first run starts `dory server`. That is leftover spawn-on-sit. Not land. Folding it onto HEAD unlands C.

Leftover also carries `## Now (25 Aug 2026)` factory occupancy notes and a dirty-tree list of leftover 5. Those are leftover desk paper, not evergreen WHERE. WHERE already points at isolate owners in `docs/README.md`. Do not copy that table here.

## This pane

- Compared `git show HEAD:README.md` vs working `README.md`.
- Did not write `README.md`. Did not `git add` leftover 5. Did not amend/reset HEAD.
- Working hash-object still `68190a5f`.

**HOLD fold.** Land stays `git show HEAD:README.md`. Working stays leftover mint.
