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

## Paid occupant pane focus

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner.

| Landing | Owner |
|---|---|
| Occupant `pane focus` `73aacfa` | `git show HEAD:rust/src/main.rs` |
| Skill `pane focus` `2c924df` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant create `--cwd`

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner.

| Landing | Owner |
|---|---|
| Occupant create `--cwd` `8b026ba` | `git show HEAD:rust/src/main.rs` |
| Skill create `--cwd` `8afd97e` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane current

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner.

| Landing | Owner |
|---|---|
| Occupant `pane current` `363894d` | `git show HEAD:rust/src/main.rs` |
| Skill `pane current` `0e16964` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant tree

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC is `desk.tree`. This is Dory tree, not `agent list`. Do not claim `agent.list`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `tree` `dd60fd3` | `git show HEAD:rust/src/main.rs` |
| Skill `tree` `3d03cac` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane neighbor

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC is `desk.neighbor`. This is Dory neighbor, not `pane focus --direction`. Do not claim `pane.neighbor` RPC or `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane neighbor` `e4f51f9` | `git show HEAD:rust/src/main.rs` |
| Skill `pane neighbor` `6d38e96` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant list --current

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPCs stay `pane.list` / `tab.list`. This is Dory list-current, not implicit focused list. Do not claim a new list RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane`/`tab` list `--current` `b6999fe` | `git show HEAD:rust/src/main.rs` |
| Skill list `--current` `72edb12` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant tab/workspace close --current

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPCs stay `tab.close` / `workspace.close`. This is Dory close-current, not implicit focused close. Do not claim a new close RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `tab`/`workspace` close `--current` `2899ab4` | `git show HEAD:rust/src/main.rs` |
| Skill close `--current` `f6c7dff` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant tab create --current

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `tab.create` with `workspace`. `--current` fills `DORY_WORKSPACE_ID`. This is Dory create-current, not Herdr `--label` / `--no-focus`. Do not claim a new create RPC. Do not claim `pane.split --cwd`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `tab create --current` `a58db64` | `git show HEAD:rust/src/main.rs` |
| Skill tab create `--current` `a60a968` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant workspace get --current

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `workspace.get` with `workspace`. Positional `<id>` stays inspect (no env). `--current` fills `DORY_WORKSPACE_ID` after `DORY_ENV=1`. This is Dory get-current, not implicit focused get. Do not claim a new get RPC. Do not claim `tab.get`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `workspace get --current` `f1b1008` | `git show HEAD:rust/src/main.rs` |
| Skill workspace get `--current` `ce56706` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start --current

Cite land with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.start` with `pane`. `--current` fills `DORY_PANE_ID`. Keep `--pane <id>`. Start still never splits. No `--kind`. No `agent.list`. No `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `agent start --current` `d48483f` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent start `--current` `3ce24c0` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane layout

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `desk.layout` with `tab` + `cols` + `rows`. Occupant verb wraps `desk.layout`. There is no `pane.layout` RPC. `--tab` inspects; `--current` fills `DORY_TAB_ID`. Do not claim `pane.zoom`. Do not claim `desk.divider`.

| Landing | Owner |
|---|---|
| Occupant `pane layout` `b994985` | `git show HEAD:rust/src/main.rs` |
| Skill pane layout `cc8733f` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane divider

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `desk.divider` with `a` + `b` + `ratio`. Occupant verb wraps `desk.divider`. There is no `pane.divider` RPC. `--a` inspects an id after `DORY_ENV=1`. `--current` fills `DORY_PANE_ID` for the first pane. Keep `--b` and `--ratio`. Do not claim `pane.zoom`. Do not claim `--direction` / `--amount`.

| Landing | Owner |
|---|---|
| Occupant `pane divider` `018ebd6` | `git show HEAD:rust/src/main.rs` |
| Skill pane divider `69fe3c9` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane neighbor prev/next

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `desk.neighbor`. Occupant verb wraps `desk.neighbor`. There is no `pane.neighbor` RPC. Spatial `left|right|up|down` still needs `cols`+`rows`. Ring `prev|next` omits cols/rows. `--pane` inspects; `--current` fills `DORY_PANE_ID`. This is the land global pane ring, not desk chrome tab n/p. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane neighbor` prev/next `bd0bf35` | `git show HEAD:rust/src/main.rs` |
| Skill pane neighbor prev/next `7573d53` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt --current

Cite land rust with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.prompt`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`, or keeps `name`. There is no `agent.list` RPC. `--wait` / `--timeout` stay as paid. Do not recook founder `--wait`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `agent prompt --current` `951d89d` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent prompt `--current` `be058f0` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent get --current

Cite land rust with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.get`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`, or keeps `name`. Named and `--pane` stay inspect. `--current` needs `DORY_ENV=1`. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `agent get --current` `2df9c74` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent get `--current` `9c047a3` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent wait --current

Cite land rust with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.wait`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`, or keeps `name`. Every arm mutating (`DORY_ENV=1`). `--until` / `--timeout` stay as paid. There is no `agent.list` RPC. Do not claim `pane.zoom`. Do not recook founder `--wait`.

| Landing | Owner |
|---|---|
| Occupant `agent wait --current` `a71fd58` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent wait `--current` `eab2cce` | `git show HEAD:skills/dory/SKILL.md` |

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
