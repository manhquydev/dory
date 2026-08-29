---
type: redteam
lens: fold
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
result: REVIEW_ACCEPT
critical: 0
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
did_not: sit t13/p2R; start default dory.sock; herdr server stop; close wP/w15/w16/t13; edit leftover 5; git add -A; recook P; cargo leftover; claim company Phase 5
---

# Red-team — leftover fold

Factory evidence this pane. Critical this wave = any edit to leftover 5.

Land README = `git show HEAD:README.md` (blob `5ac82b102be4e4f0c621d779b9c4a3bb9819afbd`). Working `README.md` is leftover, not land.

## Live mint (hash-object, not snap)

| path | WT sha1 | vs judge `leftover_mint_ok` | mtime |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH | 2026-08-27 10:55 |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH | 2026-08-27 10:55 |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH | 2026-08-27 10:55 |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH | 2026-08-25 19:36 |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH | 2026-08-27 10:55 |

HEAD blobs ≠ mint (`5ac82b10` / `62f09a95` / `5fc70ad5` / `dfca2ac5` / `fa44bfbb`). Porcelain ` M` ×5, unstaged. Cached leftover empty. `git log -1 -- rust/` = `b544f5f` `fix(attach): do not auto-start server on sit`. `desk.rs` WT == `HEAD:` `4c788562e4fdda10c8edd2878ed1fdd46050c218`. Leftover ELF `3ba0e3bc…` stat-only (2026-08-26 11:00). Default sock absent. Factory `flow/` absent.

Docs-wave mtimes (CHARTER / AGENTS / `docs/README.md` / 1204 reports) are 2026-08-29 12:06–12:14. Leftover 5 mtimes did not move.

## Spec

| # | Requirement | Status | Evidence |
|---|---|---|---|
| 1 | No leftover 5 edit this wave | PASS | hash-object MATCH mint; mtime 08-25/08-27 |
| 2 | Working README stays mint `68190a5f` | PASS | full sha `68190a5ffa073c…` |
| 3 | Cite land README as `git show HEAD:README.md` | PASS | `docs/README.md`; HEAD `# Dory` desk how-to |
| 4 | Leftover 5 still `M` | PASS | porcelain ` M` ×5 |
| 5 | Rust log `b544f5f` | PASS | `git log -1 -- rust/` |
| 6 | No `git add -A` of leftover | PASS | `git diff --cached` leftover empty |
| 7 | No recook P | PASS | `scripts/dory-isolate-aoe5-flow-prd.sh` mtime 01:04; P paper only |
| 8 | No company Phase 5 claim | PASS | docs/CHARTER/AGENTS/journal: isolate ≠ Phase 5 |

## Personas

| # | Persona | Vector | Sev | Disposition |
|---|---|---|---|---|
| D-L1 | Security adversary | Rewrite leftover README as land / docs route | — | No edit. Approach B rejected. WT README still mint. |
| D-L2 | Supply chain | `git add -A` / `ak:git` stages tracked leftover 5 | — | Cached leftover empty. `AGENTS.md` denies `-A`. |
| D-L3 | Insider | `checkout`/`restore` leftover to “clean rust” vs `b544f5f` | — | Still dirty mint, not HEAD blobs. 1122 L2 class not executed. |
| D-L4 | Infrastructure | leftover cargo remints ELF; default `dory.sock` | — | ELF sha/mtime hold. `DEFAULT_SOCK=no`. |

STRIDE Tampering on leftover 5: none this wave.

## Not leftover fold

- CHARTER `M`: one WHERE line → `docs/README.md`.
- `AGENTS.md` / `docs/` untracked: deny-list + WHERE. Not leftover 5.
- `260829-0054-isolate-flow-prd-unlock/*` `M`: frontmatter/phases/checkboxes → 100% Done. Contract allowed. Isolate script not rewritten.

## Residual door (not this-wave Critical)

Leftover 5 remain tracked ` M`. A later `git add -A` would fold them into land and leave `b544f5f`. Deny list already names that. Do not “fix” by checkout.

Critical: 0
