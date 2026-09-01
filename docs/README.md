# Docs route

Audience: people and AI collaborators. This folder is WHERE. [CHARTER.md](../CHARTER.md) is WHY the product exists.

Code owns WHAT and HOW. Do not copy command inventories, SHA tables, or test names here. Point at the owner.

## Authority

| Surface | Owns | Not |
|---|---|---|
| [CHARTER.md](../CHARTER.md) | Two chairs, kill conditions, hình B | Current cook SHAs |
| [CAPACITY-FREEZE.md](../CAPACITY-FREEZE.md) | Paid AOE 0; company Phase 5 freeze | Isolate taxi receipts |
| [north star](../plans/reports/260822-north-star-aoe.md) | Session OS + Workplace OS + Flow as foreign judge | Claim that isolate unlock = Phase 5 |
| Land `README` | Public English project face (`git show HEAD:README.md`) | Working-tree leftover README |
| `README.vi.md` | Public Vietnamese face | Leftover WT README |
| `AGENTS.md` | Costly-action deny list | Product overview |
| `plans/` + `plans/reports/` | Stateful evidence | Evergreen law |
| `scripts/dory-isolate-*.sh` | Isolate taxi contracts | Company occupancy |

Land README is `git show HEAD:README.md`. The working `README.md` is leftover 5 — mint, do not fold.

## Paid isolate taxi (executable owners)

Mechanical `dory flow -- next` on an isolate. Flow is the foreign judge. Not a Dory `next` button — land taxi is `git show HEAD:rust/src/flow.rs`. P unlock-3 is paid. Not company Phase 5. Not fill-the-PRD.

| Rung | Owner |
|---|---|
| AOE5 `gate` | `scripts/dory-isolate-aoe5-flow-judge.sh` |
| N unlock-1 | `scripts/dory-isolate-aoe5-flow-next.sh` |
| O unlock-2 | `scripts/dory-isolate-aoe5-flow-scope.sh` |
| P unlock-3 | `scripts/dory-isolate-aoe5-flow-prd.sh` |

Discover current paper with `git log --oneline -- scripts/dory-isolate-aoe5-flow-prd.sh`.

## Paid flow taxi paper

| Landing | Owner |
|---|---|
| Flow taxi paper `a87a12c` | `git show HEAD:skills/dory/SKILL.md` |

## Paid desk chrome and parked waits

Plan `260831-2119-desk-en-chrome-rpc-accept` is paid land. Cite land rust with `git show HEAD:rust/...`. Working-tree leftover `server.rs` is not the owner.

| Landing | Owner |
|---|---|
| English operator chrome (`ec69608`) | `git show HEAD:rust/src/desk.rs` |
| Park wait RPCs off accept (`d8b8fb2`) | `git show HEAD:rust/src/server.rs` + `git show HEAD:rust/tests/p_accept_wait.rs` |

## Paid CI `p_accept_wait`

| Landing | Owner |
|---|---|
| Linux CI `p_accept_wait` `e3defcf` | `git show HEAD:.github/workflows/ci.yml` |
| Darwin CI `p_accept_wait` `16e266b` | `git show HEAD:.github/workflows/ci.yml` |

## Paid `wait_dead` without `/proc`

| Landing | Owner |
|---|---|
| `wait_dead` without `/proc` `eb135be` | `git show HEAD:rust/tests/p_accept_wait.rs` |

## Paid occupant pane resize

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner.

| Landing | Owner |
|---|---|
| Occupant `pane resize` `6c4bfb8` | `git show HEAD:rust/src/main.rs` |
| Skill `pane resize` `e591fe9` | `git show HEAD:skills/dory/SKILL.md` |

## Named unpaid

- Default occupancy / sit default (dead sock ≠ sâu)
- Fill `03-prd.md` / walk 04–05 / `card` / semantic `gate-rules.md`
- Company Phase 5 (a real project completed *inside* Dory)
- Leftover 5 fold / PATH retarget / leftover ELF rm
- Founder `prompt --wait`
- Desk GitHub release / cargo-dist (`curl | sh` ELF) — lamp doctor is not that

## Factory doors

The factory chair is not a Dory occupant. Do not sit the factory default pane. Do not start `/run/user/$UID/dory/default/dory.sock` from the factory. Stop isolate only with the 2357 compound in the isolate scripts.

Public contributor docs (`README.md` land, `README.vi.md`, `npm-wrapper/README*.md`, `CONTRIBUTING*.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`) must not name the factory workplace product.

## Ship loop (lamp + land CI)

Two engines stay two engines. Green CI on `main` is the land/lamp compile-and-test door. npm Trusted Publishing is the lamp ship door. Owners:

| Door | Owner |
|---|---|
| What CI runs / skips | `.github/workflows/ci.yml` |
| How the lamp ships | `.github/workflows/publish-npm-wrapper.yml` + [npm-wrapper/RELEASE_CHECKLIST.md](../npm-wrapper/RELEASE_CHECKLIST.md) |
| How a user installs the lamp | [npm-wrapper/README.md](../npm-wrapper/README.md) + `scripts/dory-lamp-doctor.sh` |
| Costly actions | [AGENTS.md](../AGENTS.md) |

Rejected: laptop `--provenance`, `NPM_TOKEN` in git, publish `version` = environment name, folding leftover 5 to make Darwin `/proc` tests pass, `ps`/libproc inside the desk server. Dist-tag promote is laptop-only (OIDC E401) — token never in chat. First live E404 after Sigstore is “package does not exist / TP not on the package page” — see [RELEASE_CHECKLIST.md](../npm-wrapper/RELEASE_CHECKLIST.md) § First package.
