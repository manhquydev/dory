---
type: redteam
lens: path
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
result: REVIEW_ACCEPT
critical: 0
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
readme_mint: 68190a5f
land_readme: git show HEAD:README.md
---

# Red-team — broken paths (docs route)

Surfaces: `docs/README.md`, `CHARTER.md`, `AGENTS.md`.
Land README cited as `git show HEAD:README.md` (not working-tree leftover).
Not company Phase 5. Leftover 5 not edited.

## Evidence

| Probe | Result |
|---|---|
| `git rev-parse HEAD` | `049e30460d9afabcd851ada1611370420e6169a9` |
| `f1c966c` ancestor of HEAD | yes (`feat(isolate): fail-then-pass flow.sh prd`) |
| `b544f5f` ancestor of HEAD | yes (`fix(attach): do not auto-start server on sit`) |
| `git hash-object README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` (mint) |
| leftover 5 `git status --short` | still `M` (`README.md`, `rust/src/attach.rs`, `rust/src/main.rs`, `rust/src/server.rs`, `rust/tests/p5_attach.rs`) |
| `git show HEAD:README.md` | ok, 6236 bytes |
| `git show HEAD:rust/src/flow.rs` | ok, 13888 bytes (docs land-taxi cite) |
| `git show HEAD:rust/src/{attach,main,server}.rs` + `HEAD:rust/tests/p5_attach.rs` | ok |
| `/run/user/1000/dory/default/dory.sock` | absent (factory default not started) |
| `git log --oneline -- scripts/dory-isolate-aoe5-flow-prd.sh` | `f1c966c` |

## Markdown hrefs (must resolve)

Resolved from the file that owns the link.

| # | Owner | Href | Repo path | |
|---|---|---|---|---|
| P1 | `docs/README.md` | `../CHARTER.md` | `CHARTER.md` | OK |
| P2 | `docs/README.md` | `../CAPACITY-FREEZE.md` | `CAPACITY-FREEZE.md` | OK |
| P3 | `docs/README.md` | `../plans/reports/260822-north-star-aoe.md` | same | OK |
| P4 | `CHARTER.md` | `plans/reports/260822-north-star-aoe.md` | same | OK |
| P5 | `CHARTER.md` | `CAPACITY-FREEZE.md` (×2) | same | OK |
| P6 | `CHARTER.md` | `docs/README.md` | same | OK (sole WHERE href) |
| P7 | `AGENTS.md` | — | no markdown hrefs | — |

CHARTER WHERE count: 1 (`docs/README.md`).

## Backtick / command path cites

| # | Owner | Cite | |
|---|---|---|---|
| P8 | `docs/README.md` | `AGENTS.md` | OK (untracked, present) |
| P9 | `docs/README.md` | `plans/` + `plans/reports/` | OK |
| P10 | `docs/README.md` | `scripts/dory-isolate-aoe5-flow-judge.sh` | OK |
| P11 | `docs/README.md` | `scripts/dory-isolate-aoe5-flow-next.sh` | OK |
| P12 | `docs/README.md` | `scripts/dory-isolate-aoe5-flow-scope.sh` | OK |
| P13 | `docs/README.md` | `scripts/dory-isolate-aoe5-flow-prd.sh` | OK |
| P14 | `docs/README.md` | `git show HEAD:README.md` | OK |
| P15 | `docs/README.md` | `git show HEAD:rust/src/flow.rs` | OK |
| P16 | `docs/README.md` | `git log --oneline -- scripts/dory-isolate-aoe5-flow-prd.sh` | OK → `f1c966c` |
| P17 | `docs/README.md` | `/run/user/$UID/dory/default/dory.sock` | pattern; live `UID=1000` absent |
| P18 | `CHARTER.md` | `rust/` + `skills/dory` | OK |
| P19 | `CHARTER.md` | `plans/reports/260821-1416-xia-compare-deepseek-harness.md` | OK |
| P20 | `CHARTER.md` | `plans/reports/260821-1436-xia-compare-herdr.md` | OK |
| P21 | `AGENTS.md` | `CHARTER.md`, `docs/README.md` | OK |
| P22 | `AGENTS.md` | leftover 5 five paths | OK, still `M` |
| P23 | `AGENTS.md` | `git show HEAD:rust/...` | OK |

## Expected-absent (unpaid / former — not nav 404)

| # | Cite | In this repo | Disposition |
|---|---|---|---|
| P24 | `docs/README.md` unpaid `03-prd.md` | miss | Isolate world-state, not evergreen. Named unpaid. |
| P25 | `AGENTS.md` deny `flow/03-prd.md` | miss | Same hunk; prefix `flow/` is the isolate path. |
| P26 | `docs/README.md` unpaid `gate-rules.md` | miss | Semantic unpaid; no repo owner. |
| P27 | `CHARTER.md` formerly `HIEN-PHAP.md` | miss | Rename. No href. "Citations of that path still bind" → `CHARTER.md`. |

## Stale-claim gate (acceptance)

| Claim class | docs | CHARTER | AGENTS | Sev |
|---|---|---|---|---|
| isolate unlock = company Phase 5 | denies (`P unlock-3 is paid. Not company Phase 5`) | no isolate=P5 claim; Phase 1 = engine mile | deny N/O/P = Phase 5 | 0 |
| leftover 5 fold | `mint, do not fold` | no fold instruction | deny edit leftover 5 | 0 |
| sit factory `t13` | `Do not sit factory w13:t13` | no sit | deny sit `w13:t13` / `w13:p2R`; keep `t13` | 0 |

## Findings

| ID | Sev | Note |
|---|---|---|
| D-P1 | Nit | Unpaid remainder spelling: docs `03-prd.md` vs AGENTS `flow/03-prd.md`. Executable owner is isolate `$ISO_REAL/flow/03-prd.md`. Not a broken href. |
| D-P2 | Nit | `HIEN-PHAP.md` has no stub. CHARTER + land README name it as former only. |
| D-P3 | — | `scripts/dory-isolate-*.sh` glob also matches flock/sit scripts. Table still names the four paid rungs. Not a miss. |

Critical: 0

## Land README (cite only)

`git show HEAD:README.md` hrefs sampled (`CHARTER.md`, freeze, north-star, stack, skill-cli-socket, desk grid, 0847 plan, occupant-lock, s11 table, p5-accept-s11, `skills/dory/SKILL.md`, `bin/dory.js`, `eval/phase5-project`, `rust/src/{desk,layout,attach,server}.rs`, `rust/tests/p5_s11.rs`) all exist. Working `README.md` left mint. No leftover-5 edit.

## Non-actions

No product rust. No leftover 5 edit. No default sock. No sit `t13`. No `herdr server stop`. No recook P. No fill 03. No company Phase 5 claim.
