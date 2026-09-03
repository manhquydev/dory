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

Cite land rust with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Occupant neighbor-then-focus compose is `dory pane neighbor` (land `desk.neighbor`, already paid) then `dory pane focus --pane <id>` (land `pane.focus`). `dory pane focus` stays id-only. Do not claim `--direction` on focus. There is no `pane.neighbor` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane focus` `73aacfa` | `git show HEAD:rust/src/main.rs` |
| Skill `pane focus` `2c924df` | `git show HEAD:skills/dory/SKILL.md` |
| Skill neighbor-then-focus | `git show HEAD:skills/dory/SKILL.md` (`c816657`) |

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

Cite land rust with `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs`. Working-tree leftover `main.rs` / `server.rs` are not the owner. Land wrap stays `desk.tree`. Pane-row `cwd` via existing `proc_cwd`. Workspace-row cwd still `world.cwd`. No `pid` on the tree row. This is Dory tree, not `agent list`. Do not claim `agent.list`. Do not claim `pane.process-info`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `tree` `dd60fd3` | `git show HEAD:rust/src/main.rs` |
| Occupant `tree` pane cwd | `git show HEAD:rust/src/server.rs` |
| Skill `tree` `3d03cac` | `git show HEAD:skills/dory/SKILL.md` |
| Skill tree pane cwd | `git show HEAD:skills/dory/SKILL.md` (`442876b`) |

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

## Paid occupant agent read --current

Cite land rust with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.read`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`, or keeps `name`. Named and `--pane` stay inspect. `--current` needs `DORY_ENV=1`. Keep `--source`. `agent read` does not mark seen. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `agent read --current` `e93878f` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent read `--current` `3bfc7b7` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent focus --current

Cite land rust with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.focus`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`, or keeps `name`. Every arm mutating (`DORY_ENV=1`). Focus marks seen. `agent read` / `pane read` do not. Keep `dory pane focus` as a different verb. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `agent focus --current` `eed1dff` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent focus `--current` `9b421be` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent send-keys --current

Cite land rust with `git show HEAD:rust/src/agent.rs` and `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `agent.send-keys`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`, or keeps `name`. Every arm mutating (`DORY_ENV=1`). `<key>` required. Allowlist `enter` / `esc` / `ctrl+c` only. Named JSON keeps `name` + `key`. Pane arms send `pane` + `key` and omit `name`. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `agent send-keys --current` `ebfe500` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill agent send-keys `--current` `a953635` | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane run --current

Cite land with `git show HEAD:rust/src/main.rs` and `git show HEAD:rust/src/agent.rs`. Working-tree leftover `main.rs` is not the owner. Land RPCs stay `pane.write` / `pane.read` / `pane.wait` / `agent.report`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`. Report has no `name`. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane run` / `read` / `wait-output` `--current` | `git show HEAD:rust/src/main.rs` |
| Occupant `agent report --current` | `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/main.rs` |
| Skill pane `--current` + report contract | `git show HEAD:skills/dory/SKILL.md` (`34be95a`) |

## Paid occupant pane close --current

Cite land with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land RPC stays `pane.close`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`. Closing the last live pane is refused. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane close --current` | `git show HEAD:rust/src/main.rs` |
| Skill pane close `--current` | `git show HEAD:skills/dory/SKILL.md` (`1f8a2c2`) |

## Paid occupant read --lines

Cite land with `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/server.rs`. Working-tree leftover `main.rs` / `server.rs` are not the owner. Land ops stay `pane.read` / `agent.read`. `--lines` tails the land snapshot (`tail_lines`). There is no `agent.list` RPC. Do not claim `pane.zoom`. Do not claim `detection` / `--format`.

| Landing | Owner |
|---|---|
| Occupant `pane read` / `agent read` `--lines` | `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/agent.rs` + `git show HEAD:rust/src/server.rs` |
| Skill read `--lines` | `git show HEAD:skills/dory/SKILL.md` (`eea19f1`) |

## Paid occupant pane send-keys

Cite land with `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs`. Working-tree leftover `main.rs` / `server.rs` are not the owner. Land wrap stays `pane.write` + `"raw":true`. Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`. `<key>` required. Allowlist `enter` / `esc` / `ctrl+c` only. There is no `pane.send-keys` RPC. Keep `dory pane run` as text + Enter. Keep `dory agent send-keys` as the occupant-named verb (`locate_agent`). There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane send-keys` | `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs` |
| Skill pane send-keys | `git show HEAD:skills/dory/SKILL.md` (`e21b722`) |

## Paid occupant pane send-text

Cite land with `git show HEAD:rust/src/main.rs`. Working-tree leftover `main.rs` is not the owner. Land wrap stays `pane.write` + `"raw":true` (no extra newline). Occupant fills `pane` from `--pane` or `--current`→`DORY_PANE_ID`. `<text>` required. There is no `pane.send-text` RPC. Keep `dory pane run` as text + Enter. Keep `dory pane send-keys` as allowlist keys. Keep `dory agent send-keys` as the occupant-named verb (`locate_agent`). There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane send-text` | `git show HEAD:rust/src/main.rs` |
| Skill pane send-text | `git show HEAD:skills/dory/SKILL.md` (`68f0c25`) |

## Paid occupant pane wait-output --source --lines

Cite land with `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs`. Working-tree leftover `main.rs` / `server.rs` are not the owner. Land wrap stays `pane.wait`. Occupant optional `--source visible|recent|recent-unwrapped` and `--lines N`. Omit source → land still matches `recent_unwrapped` (not `pane read` default `recent`). `--lines N` tails that snapshot. There is no new wait RPC. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane wait-output` `--source` `--lines` | `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs` |
| Skill wait-output `--source` `--lines` | `git show HEAD:skills/dory/SKILL.md` (`fe85855`) |

## Paid occupant pane split --ratio

Cite land with `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs`. Working-tree leftover `main.rs` / `server.rs` are not the owner. Land wrap stays `pane.split`. Occupant optional `--ratio F`. Omit → land `split_leaf` default `0.5`. Present → land `set_ratio` (clamp `[0.05, 0.95]`). CLI does not re-clamp. There is no new split RPC. Pane split still has no `--cwd`. Keep `dory pane divider` for an existing pair. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane split` `--ratio` | `git show HEAD:rust/src/main.rs` + `git show HEAD:rust/src/server.rs` |
| Skill pane split `--ratio` | `git show HEAD:skills/dory/SKILL.md` (`e0a243c`) |

## Paid occupant pane get pid cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.get`. Occupant parse `.result.pid` and `.result.cwd`. cwd via existing `proc_cwd` (`/proc/{pid}/cwd` with `world.cwd` fallback). There is no new RPC. There is no `pane.process-info`. CLI USAGE stays `dory pane get [--current | --pane <id>]`. Keep `dory pane current` as the same `pane.get` RPC. Do not claim argv / cmdline / foreground. No `--format`. No `--kind`. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane get` pid/cwd | `git show HEAD:rust/src/server.rs` |
| Skill pane get pid/cwd | `git show HEAD:skills/dory/SKILL.md` (`cc99a27`) |

## Paid occupant pane list pid cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.list`. Occupant parse `.result.panes[].pid` and `.result.panes[].cwd`. cwd via existing `proc_cwd` (`/proc/{pid}/cwd` with `world.cwd` fallback). There is no new RPC. There is no `pane.process-info`. CLI USAGE stays `dory pane list [--workspace <id> | --current]`. Keep `dory tree` for the live occupant roster. Keep `dory pane get` for a single pane. Do not claim argv / cmdline / foreground. No `--format`. No `--kind`. There is no `agent.list` RPC. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane list` pid/cwd | `git show HEAD:rust/src/server.rs` |
| Skill pane list pid/cwd | `git show HEAD:skills/dory/SKILL.md` (`a29b752`) |

## Paid occupant tab list pane_count

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.list`. Occupant parse `.result.tabs[].id`, `.result.tabs[].occupant`, `.result.tabs[].pane_count`. `pane_count` via existing `tab.panes.len()`. Keep `id` / `occupant`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab list [--workspace <id> | --current]`. Keep `dory pane list` / `dory tree` as they are. No `--label`. No `--format`. No `--kind`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `tab list` pane_count | `git show HEAD:rust/src/server.rs` |
| Skill tab list pane_count | `git show HEAD:skills/dory/SKILL.md` (`11ec87a`) |

## Paid occupant workspace list/get counts

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].tab_count` / `.result.workspaces[].pane_count` and get `.result.tab_count` / `.result.pane_count`. `tab_count` via existing `ws.tabs.len()`. `pane_count` via sum of `tab.panes.len()`. Keep `workspace.id` and `tabs[]`. Do not add `pane_count` on nested tab objects. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `workspace.focus`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `workspace list`/`get` counts | `git show HEAD:rust/src/server.rs` |
| Skill workspace list/get counts | `git show HEAD:skills/dory/SKILL.md` (`348251c`) |

## Paid occupant tab list focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.list`. Occupant parse `.result.tabs[].focused`. Land `focused` is JSON boolean: tab contains the pane whose id is `world.focused`. Keep `id` / `occupant` / `pane_count`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab list [--workspace <id> | --current]`. No `--label`. No `tab.focus`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `tab list` focused | `git show HEAD:rust/src/server.rs` |
| Skill tab list focused | `git show HEAD:skills/dory/SKILL.md` (`08c6f2c`) |

## Paid occupant pane list focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.list`. Occupant parse `.result.panes[].focused`. Land `focused` is JSON boolean: `pane.id == world.focused`. Keep `id` / `pid` / `cwd` / `occupant`. There is no new RPC. There is no `pane.process-info`. CLI USAGE stays `dory pane list [--workspace <id> | --current]`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `pane list` focused | `git show HEAD:rust/src/server.rs` |
| Skill pane list focused | `git show HEAD:skills/dory/SKILL.md` (`3b1b928`) |

## Paid occupant workspace list/get focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].focused` and get `.result.focused`. Land `focused` is JSON boolean: workspace contains the pane whose id is `world.focused`. Keep `workspace.id` / `tab_count` / `pane_count` / `tabs[]`. Do not add `focused` on nested tab objects. There is no new RPC. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `workspace.focus`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `workspace list`/`get` focused | `git show HEAD:rust/src/server.rs` |
| Skill workspace list/get focused | `git show HEAD:skills/dory/SKILL.md` (`99303c3`) |

## Paid occupant workspace nested tabs focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].tabs[].focused` and get `.result.tabs[].focused`. Land nested `focused` is JSON boolean: that tab contains the pane whose id is `world.focused`. Keep workspace `id` / `tab_count` / `pane_count` / workspace-level `focused` / `tabs[]` `id` / `root_pane` / `occupant`. Do not add `pane_count` on nested tabs. Do not add `active_tab_id`. There is no new RPC. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `workspace.focus`. No `tab.get`. No `tab.focus`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `workspace list`/`get` nested tabs focused | `git show HEAD:rust/src/server.rs` |
| Skill workspace nested tabs focused | `git show HEAD:skills/dory/SKILL.md` (`4239be4`) |

## Paid occupant workspace nested tabs pane_count

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].tabs[].pane_count` and get `.result.tabs[].pane_count`. Land nested `pane_count` is `tab.panes.len()` as a JSON number. Keep workspace `id` / `tab_count` / workspace-level `pane_count` / `focused` / nested `id` / `root_pane` / `occupant` / `focused`. Do not add `active_tab_id`. There is no new RPC. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `workspace.focus`. No `tab.get`. No `tab.focus`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant `workspace list`/`get` nested tabs pane_count | `git show HEAD:rust/src/server.rs` |
| Skill workspace nested tabs pane_count | `git show HEAD:skills/dory/SKILL.md` (`133293b`) |

## Paid occupant tree item focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse `.result.items[].focused`. Land item `focused` is JSON boolean: pane row is `pane.id == world.focused`; tab row is that tab contains that pane; workspace row is that workspace contains that pane. Root `.result.focused` stays the pane-id string. Keep pane `cwd` / `occ` / `st`. Keep workspace `cwd`. No `pid` on tree rows. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory tree`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant tree item focused | `git show HEAD:rust/src/server.rs` |
| Skill tree item focused | `git show HEAD:skills/dory/SKILL.md` (`94b5a9e`) |

## Paid occupant tree tab pane_count

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse tab-row `.pane_count`. Land `pane_count` is `tab.panes.len()` as a JSON number. Root `.result.focused` stays the pane-id string. Keep item `focused`. Keep pane `cwd` / `occ` / `st`. Keep workspace `cwd`. No `pid` on tree rows. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory tree`. Do not add `focused` on `pane.get`. No `tab.get`. Do not add workspace tree `pane_count`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant tree tab pane_count | `git show HEAD:rust/src/server.rs` |
| Skill tree tab pane_count | `git show HEAD:skills/dory/SKILL.md` (`8da9162`) |

## Paid Darwin CI XDG_RUNTIME_DIR

Cite land with `git show HEAD:.github/workflows/ci.yml`. Darwin GHA rust-desk writes `XDG_RUNTIME_DIR=/tmp` beside `TMPDIR=/tmp` so occupant inspect (`--tab` / `--pane` without `--current`) hits a missing sock (exit 1) instead of `MissingRuntimeDir` exit 2. Product still refuses unset XDG with exit 2 (`git show HEAD:rust/src/socket.rs`). Do not claim Darwin occupant `done`/`idle` is paid. Do not claim Darwin `/proc` units run. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Darwin CI `XDG_RUNTIME_DIR` `85d87d4` | `git show HEAD:.github/workflows/ci.yml` |

## Paid occupant tree workspace pane_count

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse workspace-row `.pane_count`. Land `pane_count` is sum of `tab.panes.len()` as a JSON number. Root `.result.focused` stays the pane-id string. Keep item `focused`. Keep pane `cwd` / `occ` / `st`. Keep workspace `cwd`. Keep tab `pane_count`. No `pid` on tree rows. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory tree`. Do not add `focused` on `pane.get`. No `tab.get`. Do not add workspace tree `tab_count`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree workspace pane_count | `git show HEAD:rust/src/server.rs` |
| Skill tree workspace pane_count | `git show HEAD:skills/dory/SKILL.md` (`2e39122`) |

## Paid occupant tree workspace tab_count

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse workspace-row `.tab_count`. Land `tab_count` is `ws.tabs.len()` as a JSON number. Keep workspace `pane_count`. Root `.result.focused` stays the pane-id string. Keep item `focused`. Keep pane `cwd` / `occ` / `st`. Keep workspace `cwd`. Keep tab `pane_count`. No `pid` on tree rows. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory tree`. Do not add `focused` on `pane.get`. No `tab.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree workspace tab_count | `git show HEAD:rust/src/server.rs` |
| Skill tree workspace tab_count | `git show HEAD:skills/dory/SKILL.md` (`de3eb2a`) |

## Paid occupant pane list tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.list`. Occupant parse `.result.panes[].tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `id` / `pid` / `cwd` / `occupant` / `focused`. No `workspace_id` this slice. There is no new RPC. There is no `tab.get`. There is no `pane.process-info`. CLI USAGE stays `dory pane list [--workspace <id> | --current]`. Do not add `tab_id` on `pane.get`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant `pane list` tab_id | `git show HEAD:rust/src/server.rs` |
| Skill pane list tab_id | `git show HEAD:skills/dory/SKILL.md` (`99f6103`) |

## Paid occupant pane list workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.list`. Occupant parse `.result.panes[].workspace_id`. Land `workspace_id` is `ws.id`. Keep `id` / `pid` / `cwd` / `occupant` / `focused` / `tab_id`. There is no new RPC. There is no `tab.get`. There is no `pane.process-info`. CLI USAGE stays `dory pane list [--workspace <id> | --current]`. Do not add `workspace_id` on `pane.get`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant `pane list` workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill pane list workspace_id | `git show HEAD:skills/dory/SKILL.md` (`c1e7232`) |

## Paid occupant tab list workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.list`. Occupant parse `.result.tabs[].workspace_id`. Land `workspace_id` is `ws.id`. Keep `id` / `occupant` / `pane_count` / `focused`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab list [--workspace <id> | --current]`. No `--label`. No `tab.focus`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant `tab list` workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill tab list workspace_id | `git show HEAD:skills/dory/SKILL.md` (`490fcf2`) |

## Paid occupant tree tab workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse tab-row `.workspace_id`. Land `workspace_id` is `ws.id`. Keep tab `k` / `id` / `focused` / `pane_count`. Keep workspace `pane_count` / `tab_count`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. Do not add `workspace_id` on tree pane rows. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree tab workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill tree tab workspace_id | `git show HEAD:skills/dory/SKILL.md` (`c846b94`) |

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
