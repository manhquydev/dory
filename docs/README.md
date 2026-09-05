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


## Paid occupant pane list pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.list`. Occupant parse `.result.panes[].pane_id`. Land `pane_id` is `pane.id` (same as `.result.panes[].id`). Keep `id` / `pid` / `cwd` / `occupant` / `focused` / `tab_id` / `workspace_id`. There is no new RPC. There is no `tab.get`. There is no `pane.process-info`. CLI USAGE stays `dory pane list [--workspace <id> | --current]`. Do not recook `pane.get` pane_id. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant `pane list` pane_id | `git show HEAD:rust/src/server.rs` |
| Skill pane list pane_id | `git show HEAD:skills/dory/SKILL.md` (`29209a3`) |

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

## Paid occupant tree pane workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse pane-row `.workspace_id`. Land `workspace_id` is `ws.id`. Keep pane `k` / `id` / `occ` / `st` / `cwd` / `focused`. Keep tab `workspace_id`. Keep workspace `pane_count` / `tab_count`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. Do not add `tab_id` on tree pane rows. Do not add `focused` on `pane.get`. Do not add `workspace_id` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree pane workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill tree pane workspace_id | `git show HEAD:skills/dory/SKILL.md` (`0ddfb49`) |

## Paid occupant tree pane tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse pane-row `.tab_id`. Land `tab_id` is `tab.id`. Keep pane `k` / `id` / `occ` / `st` / `cwd` / `focused` / `workspace_id`. Keep tab `workspace_id`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. Do not add `tab_id` on `pane.get`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree pane tab_id | `git show HEAD:rust/src/server.rs` |
| Skill tree pane tab_id | `git show HEAD:skills/dory/SKILL.md` (`eddf36c`) |

## Paid occupant tree pane pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse pane-row `.pane_id`. Land `pane_id` is `pane.id` (same as row `id`). Keep pane `k` / `id` / `occ` / `st` / `cwd` / `focused` / `workspace_id` / `tab_id`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. Do not add `pane_id` on tab/workspace tree rows this slice. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree pane pane_id | `git show HEAD:rust/src/server.rs` |
| Skill tree pane pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant tree tab occupant

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse tab-row `.occupant`. Land `occupant` is first-pane `pane_occupant_json` (`null` or `{name,state,seen}`). Keep tab `k` / `id` / `focused` / `pane_count` / `workspace_id`. Keep pane `occ` / `st` / `cwd` / `workspace_id` / `tab_id`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. Do not add `occ` / `st` on tab rows. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree tab occupant | `git show HEAD:rust/src/server.rs` |
| Skill tree tab occupant | `git show HEAD:skills/dory/SKILL.md` (`e196800`) |

## Paid occupant workspace nested tab workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].tabs[].workspace_id` and get `.result.tabs[].workspace_id`. Land nested `workspace_id` is `ws.id`. Keep nested `id` / `root_pane` / `occupant` / `pane_count` / `focused`. Keep workspace `id` / `tab_count` / `pane_count` / `focused`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace nested tab workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill workspace nested tab workspace_id | `git show HEAD:skills/dory/SKILL.md` (`9385947`) |

## Paid occupant workspace list/get workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].workspace_id` and get `.result.workspace_id`. Land `workspace_id` is `ws.id` (same as `workspace.id`). Keep `workspace.id` / `tab_count` / `pane_count` / `focused` / nested tab `workspace_id`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace list/get workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill workspace list/get workspace_id | `git show HEAD:skills/dory/SKILL.md` (`43f79f0`) |

## Paid occupant tree workspace workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse workspace-row `.workspace_id`. Land `workspace_id` is `ws.id` (same as row `id`). Keep workspace `k` / `id` / `cwd` / `focused` / `pane_count` / `tab_count`. Keep tab `workspace_id` / `occupant`. Keep pane `workspace_id` / `tab_id`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree workspace workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill tree workspace workspace_id | `git show HEAD:skills/dory/SKILL.md` (`fbda711`) |

## Paid occupant workspace list/get occupant

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].occupant` and get `.result.occupant`. Land `occupant` is first-tab first-pane `pane_occupant_json` (`null` or `{name,state,seen}`). Keep `workspace.id` / `tab_count` / `pane_count` / `focused` / `workspace_id` / nested tab `occupant`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `active_tab_id`. No `agent_status`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace list/get occupant | `git show HEAD:rust/src/server.rs` |
| Skill workspace list/get occupant | `git show HEAD:skills/dory/SKILL.md` (`e271b2c`) |

## Paid occupant tree workspace occupant

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse workspace-row `.occupant`. Land `occupant` is first-tab first-pane `pane_occupant_json` (`null` or `{name,state,seen}`). Keep workspace `k` / `id` / `cwd` / `focused` / `pane_count` / `tab_count` / `workspace_id`. Keep tab `occupant`. Keep pane `occ` / `st`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. No `--label`. No `active_tab_id`. No `agent_status`. Do not add `occ`/`st` on workspace rows. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree workspace occupant | `git show HEAD:rust/src/server.rs` |
| Skill tree workspace occupant | `git show HEAD:skills/dory/SKILL.md` (`182d3e5`) |

## Paid occupant tab list tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.list`. Occupant parse `.result.tabs[].tab_id`. Land `tab_id` is `tab.id` (same as `id`). Keep `id` / `occupant` / `pane_count` / `focused` / `workspace_id`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab list [--workspace <id> | --current]`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tab list tab_id | `git show HEAD:rust/src/server.rs` |
| Skill tab list tab_id | `git show HEAD:skills/dory/SKILL.md` (`d8734a4`) |

## Paid occupant workspace nested tab tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].tabs[].tab_id` and get `.result.tabs[].tab_id`. Land nested `tab_id` is `tab.id` (same as nested `id`). Keep nested `id` / `root_pane` / `occupant` / `pane_count` / `focused` / `workspace_id`. Keep workspace `id` / `tab_count` / `pane_count` / `focused` / `workspace_id` / `occupant`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace nested tab tab_id | `git show HEAD:rust/src/server.rs` |
| Skill workspace nested tab tab_id | `git show HEAD:skills/dory/SKILL.md` (`ea50153`) |

## Paid occupant tree tab tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.tree`. Occupant parse tab-row `.tab_id`. Land `tab_id` is `tab.id` (same as row `id`). Keep tab `k` / `id` / `focused` / `pane_count` / `workspace_id` / `occupant`. Keep pane `tab_id`. Root `.result.focused` stays the pane-id string. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tree`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tree tab tab_id | `git show HEAD:rust/src/server.rs` |
| Skill tree tab tab_id | `git show HEAD:skills/dory/SKILL.md` (`b6c92fd`) |

## Paid occupant agent get cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.get`. Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent get [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `tab_id` / `focused` / `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent get cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent get cwd | `git show HEAD:skills/dory/SKILL.md` (`47bfe14`) |

## Paid occupant agent get focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.get`. Occupant parse `.result.focused`. Land `focused` is JSON boolean: `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent get [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent get focused | `git show HEAD:rust/src/server.rs` |
| Skill agent get focused | `git show HEAD:skills/dory/SKILL.md` (`388d95f`) |

## Paid occupant agent get tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.get`. Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent get [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent get tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent get tab_id | `git show HEAD:skills/dory/SKILL.md` (`ffa0169`) |

## Paid occupant agent get workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.get`. Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent get [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `workspace_id` on `pane.get`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent get workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent get workspace_id | `git show HEAD:skills/dory/SKILL.md` (`ed4968c`) |

## Paid occupant agent read cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.read`. Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.source` / `.result.text`. `agent read` does not mark seen. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent read [<name> | --current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`. No `--label`. No `active_tab_id`. Do not add `focused` / `tab_id` / `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent read cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent read cwd | `git show HEAD:skills/dory/SKILL.md` (`2508876`) |

## Paid occupant agent read focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.read`. Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.source` / `.result.text`. `agent read` does not mark seen. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent read [<name> | --current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`. No `--label`. No `active_tab_id`. Do not add `focused` / `tab_id` / `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent read focused | `git show HEAD:rust/src/server.rs` |
| Skill agent read focused | `git show HEAD:skills/dory/SKILL.md` (`f95002c`) |

## Paid occupant agent read tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.read`. Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.source` / `.result.text`. `agent read` does not mark seen. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent read [<name> | --current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent read tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent read tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent read workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.read`. Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id` / `.result.source` / `.result.text`. `agent read` does not mark seen. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent read [<name> | --current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`. No `--label`. No `active_tab_id`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent read workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent read workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent focus cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.focus`. Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. Focus still marks seen. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent focus [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `focused` / `tab_id` / `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent focus cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent focus cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent focus focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.focus`. Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. Focus still marks seen. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent focus [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent focus focused | `git show HEAD:rust/src/server.rs` |
| Skill agent focus focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent focus tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.focus`. Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. Focus still marks seen. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent focus [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent focus tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent focus tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent focus workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.focus`. Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. Focus still marks seen. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent focus [<name> | --current | --pane <id>]`. No `--label`. No `active_tab_id`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent focus workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent focus workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent wait cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.wait` (settle via `tick_agent_wait`). Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. Wait still does not mark seen. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent wait [<name> | --current | --pane <id>] [--until idle|done|blocked|working|unknown] [--timeout MS]`. No `--label`. No `active_tab_id`. Do not add `focused` / `tab_id` / `workspace_id` on `pane.get` or on `agent.wait` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent wait cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent wait cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent wait focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.wait` (settle via `tick_agent_wait`). Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. Wait still does not mark seen. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent wait [<name> | --current | --pane <id>] [--until idle|done|blocked|working|unknown] [--timeout MS]`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not add `tab_id` / `workspace_id` on `agent.wait` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent wait focused | `git show HEAD:rust/src/server.rs` |
| Skill agent wait focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent wait tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.wait` (settle via `tick_agent_wait`). Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. Wait still does not mark seen. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent wait [<name> | --current | --pane <id>] [--until idle|done|blocked|working|unknown] [--timeout MS]`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not add `workspace_id` on `agent.wait` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent wait tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent wait tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent wait workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.wait` (settle via `tick_agent_wait`). Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. Wait still does not mark seen. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent wait [<name> | --current | --pane <id>] [--until idle|done|blocked|working|unknown] [--timeout MS]`. No `--label`. No `active_tab_id`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent wait workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent wait workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.start` immediate `LineReply::Msg` (non-allowlisted argv / already classified). Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. Pending classify is unchanged this slice. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `focused` / `tab_id` / `workspace_id` on `pane.get` or on `agent.start` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent start cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.start` immediate `LineReply::Msg` (non-allowlisted argv / already classified). Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. Pending classify is unchanged this slice. There is no new RPC. There is no `agent.list`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not add `tab_id` / `workspace_id` on `agent.start` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start focused | `git show HEAD:rust/src/server.rs` |
| Skill agent start focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.start` immediate `LineReply::Msg` (non-allowlisted argv / already classified). Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. Pending classify is unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not add `workspace_id` on `agent.start` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent start tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.start` immediate `LineReply::Msg` (non-allowlisted argv / already classified). Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. Pending classify is unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent start workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start classify cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_classify` settle (`classified || deadline`) after `agent.start` Pending classify (allowlisted argv). Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Immediate start siblings are unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `focused` / `tab_id` / `workspace_id` on classify settle this slice. Do not add `cwd` on `pane.get` (already paid). Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start classify cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent start classify cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start classify focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_classify` settle (`classified || deadline`) after `agent.start` Pending classify (allowlisted argv). Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. Immediate start siblings are unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `focused` on `pane.get`. Do not add `tab_id` / `workspace_id` on classify settle this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start classify focused | `git show HEAD:rust/src/server.rs` |
| Skill agent start classify focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start classify tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_classify` settle (`classified || deadline`) after `agent.start` Pending classify (allowlisted argv). Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. Immediate start siblings are unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `tab_id` on `pane.get`. Do not add `workspace_id` on classify settle this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start classify tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent start classify tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent start classify workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_classify` settle (`classified || deadline`) after `agent.start` Pending classify (allowlisted argv). Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. Immediate start siblings are unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent start <name> --pane <id> | --current -- <argv>`. No `--label`. No `active_tab_id`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent start classify workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent start classify workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.prompt` immediate `LineReply::Msg` (non-wait, non-stall). Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. Wait/stall settle (`tick_agent_prompt` / `agent_snapshot_reply`) is unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `focused` / `tab_id` / `workspace_id` on `agent.prompt` this slice. Do not add `cwd` on `pane.get` (already paid). Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt settle cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_prompt` settle via `agent_snapshot_reply` after `agent.prompt` wait/stall. Occupant parse `.result.cwd`. Land `cwd` is `proc_cwd` on the occupant pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep immediate prompt `cwd`. `agent_snapshot` is unchanged. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `focused` / `tab_id` / `workspace_id` on prompt settle this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt settle cwd | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt settle cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.prompt` immediate `LineReply::Msg` (non-wait, non-stall). Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. Wait/stall settle is unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `focused` on `pane.get`. Do not add `tab_id` / `workspace_id` on `agent.prompt` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt focused | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt settle focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_prompt` settle via `agent_snapshot_reply`. Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd`. Keep immediate prompt `focused`. `agent_snapshot` is unchanged. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `focused` on `pane.get`. Do not add `tab_id` / `workspace_id` on prompt settle this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt settle focused | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt settle focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.prompt` immediate `LineReply::Msg` (non-wait, non-stall). Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. Wait/stall settle is unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `tab_id` on `pane.get`. Do not add `workspace_id` on `agent.prompt` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt settle tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_prompt` settle via `agent_snapshot_reply`. Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused`. Keep immediate prompt `tab_id`. `agent_snapshot` is unchanged. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `tab_id` on `pane.get`. Do not add `workspace_id` on prompt settle this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt settle tab_id | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt settle tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent.prompt` immediate `LineReply::Msg` (non-wait, non-stall). Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. Wait/stall settle is unchanged this slice. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent prompt settle workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tick_agent_prompt` settle via `agent_snapshot_reply`. Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.agent` `name` / `pane` / `state` / `seen`. Keep `.result.cwd` / `.result.focused` / `.result.tab_id`. Keep immediate prompt `workspace_id`. `agent_snapshot` is unchanged. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`. No `--label`. No `active_tab_id`. Do not recook founder `--wait`. Do not add `workspace_id` on `pane.get`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent prompt settle workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill agent prompt settle workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane get focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.get`. Occupant parse `.result.focused`. Land `focused` is `pane.id == world.focused`. Keep `.result.pane.id` / `.result.pid` / `.result.cwd` / `.result.occupant`. `pane current` stays the same RPC. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory pane get [--current | --pane <id>]`. No `--label`. Do not add `tab_id` / `workspace_id` on `pane.get` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant pane get focused | `git show HEAD:rust/src/server.rs` |
| Skill pane get focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane get tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.get`. Occupant parse `.result.tab_id`. Land `tab_id` is the enclosing `tab.id`. Keep `.result.pane.id` / `.result.pid` / `.result.cwd` / `.result.occupant` / `.result.focused`. `pane current` stays the same RPC. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory pane get [--current | --pane <id>]`. No `--label`. Do not add `workspace_id` on `pane.get` this slice. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant pane get tab_id | `git show HEAD:rust/src/server.rs` |
| Skill pane get tab_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant pane get workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.get`. Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.pane.id` / `.result.pid` / `.result.cwd` / `.result.occupant` / `.result.focused` / `.result.tab_id`. `pane current` stays the same RPC. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory pane get [--current | --pane <id>]`. No `--label`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant pane get workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill pane get workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane get pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.get`. Occupant parse `.result.pane_id`. Land `pane_id` is `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id` / `.result.pid` / `.result.cwd` / `.result.occupant` / `.result.focused` / `.result.tab_id` / `.result.workspace_id`. `pane current` stays the same RPC. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory pane get [--current | --pane <id>]`. No `--label`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant pane get pane_id | `git show HEAD:rust/src/server.rs` |
| Skill pane get pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant pane get nested pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.get`. Occupant parse `.result.pane.pane_id`. Land nested `pane_id` is `pane.id` (same as `.result.pane.id` and root `.result.pane_id`). Keep root `.result.pane_id` / `.result.pane.id` / `.result.pid` / `.result.cwd` / `.result.occupant` / `.result.focused` / `.result.tab_id` / `.result.workspace_id`. `pane current` stays the same RPC. There is no new RPC. There is no `agent.list`. There is no `tab.get`. CLI USAGE stays `dory pane get [--current | --pane <id>]`. No `--label`. Do not recook root pane.get pane_id. Do not recook live `snapshot` occupant/cwd/id family. Do not recook `desk.snapshot`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant pane get nested pane_id | `git show HEAD:rust/src/server.rs` |
| Skill pane get nested pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant layout cell pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.layout`. Occupant parse `.result.cells[]` `.pane_id`. Land `pane_id` is `cell.id` (same as cell `id`). Keep `id` / `x` / `y` / `w` / `h` / `occ` / `st`. Keep `.result.tab` / `.result.focused` / `.result.cols` / `.result.rows`. There is no new RPC. There is no `pane.layout` RPC. CLI USAGE stays `dory pane layout [--current | --tab <id>] --cols N --rows N`. Do not add `zoomed`. Do not alias `width`/`height`. Do not recook pane.get/list/tree pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant layout cell pane_id | `git show HEAD:rust/src/server.rs` |
| Skill layout cell pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant layout tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.layout`. Occupant parse `.result.tab_id`. Land `tab_id` is `tab.id` (same as `.result.tab`). Keep `.result.tab` / `.result.focused` / `.result.cols` / `.result.rows`. Keep cells `id` / `x` / `y` / `w` / `h` / `occ` / `st` / `pane_id`. There is no new RPC. There is no `pane.layout` RPC. CLI USAGE stays `dory pane layout [--current | --tab <id>] --cols N --rows N`. Do not add `zoomed`. Do not alias `width`/`height`. Do not add `workspace_id` on layout root this slice. Do not recook pane.get/list/tree pane_id. Do not recook layout cell pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant layout tab_id | `git show HEAD:rust/src/server.rs` |
| Skill layout tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant layout workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.layout`. Occupant parse `.result.workspace_id`. Land `workspace_id` is `ws.id`. Keep `.result.tab` / `.result.tab_id` / `.result.focused` / `.result.cols` / `.result.rows`. Keep cells `id` / `x` / `y` / `w` / `h` / `occ` / `st` / `pane_id`. There is no new RPC. There is no `pane.layout` RPC. CLI USAGE stays `dory pane layout [--current | --tab <id>] --cols N --rows N`. Do not add `zoomed`. Do not alias `width`/`height`. Do not add `focused_pane_id` this slice. Do not recook layout tab_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant layout workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill layout workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant layout focused_pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.layout`. Occupant parse `.result.focused_pane_id`. Land `focused_pane_id` is `world.focused` (same as `.result.focused`). Keep `.result.tab` / `.result.tab_id` / `.result.workspace_id` / `.result.focused` / `.result.cols` / `.result.rows`. Keep cells `id` / `x` / `y` / `w` / `h` / `occ` / `st` / `pane_id`. There is no new RPC. There is no `pane.layout` RPC. CLI USAGE stays `dory pane layout [--current | --tab <id>] --cols N --rows N`. Do not add `zoomed`. Do not alias `width`/`height`. Do not add `focused` on cells. Do not recook layout tab_id. Do not recook layout workspace_id. Do not recook layout cell pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant layout focused_pane_id | `git show HEAD:rust/src/server.rs` |
| Skill layout focused_pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant layout cell focused

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.layout`. Occupant parse `.result.cells[]` `.focused`. Land `focused` is JSON boolean `cell.id == world.focused`. Keep `id` / `x` / `y` / `w` / `h` / `occ` / `st` / `pane_id`. Keep `.result.tab` / `.result.tab_id` / `.result.workspace_id` / `.result.focused` / `.result.focused_pane_id` / `.result.cols` / `.result.rows`. There is no new RPC. There is no `pane.layout` RPC. CLI USAGE stays `dory pane layout [--current | --tab <id>] --cols N --rows N`. Do not add `zoomed`. Do not alias `width`/`height` / `rect`. Do not recook layout focused_pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant layout cell focused | `git show HEAD:rust/src/server.rs` |
| Skill layout cell focused | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant neighbor pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.neighbor`. Occupant parse `.result.pane_id`. Land `pane_id` is neighbor `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id`. There is no new RPC. There is no `pane.neighbor` RPC. CLI USAGE stays `dory pane neighbor [--current | --pane <id>] --direction left|right|up|down|prev|next [--cols N --rows N]`. Occupant verb stays `dory pane neighbor`. Do not add `pane.split` `pane_id` this slice. Do not recook layout family. Do not add `zoomed`. Do not alias `width`/`height` / `rect`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant neighbor pane_id | `git show HEAD:rust/src/server.rs` |
| Skill neighbor pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant split pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.split`. Occupant parse `.result.pane.pane_id`. Land `pane_id` is the new `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id` / `.result.direction` / `.result.occupant` / optional `.result.ratio`. There is no new RPC. CLI USAGE stays `dory pane split [--current | --pane <id>] [--direction right|down] [--ratio F] [--no-focus]`. Occupant verb stays `dory pane split`. Do not add `--cwd`. Do not recook neighbor pane_id. Do not recook layout family. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant split pane_id | `git show HEAD:rust/src/server.rs` |
| Skill split pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant tab create root_pane pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.create`. Occupant parse `.result.root_pane.pane_id`. Land `pane_id` is the new root `pane.id` (same as nested `.result.root_pane.id`). Keep `.result.tab.id` / `.result.root_pane.id` / `.result.occupant`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab create [--workspace <id> | --current] [--cwd <path>]`. Occupant verb stays `dory tab create`. Do not add `pane.close` `pane_id` this slice. Do not recook neighbor pane_id. Do not recook split pane_id. Do not recook layout family. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tab create root_pane pane_id | `git show HEAD:rust/src/server.rs` |
| Skill tab create root_pane pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant close pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.close`. Occupant parse `.result.pane.pane_id`. Land `pane_id` is closed `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id` / `.result.retired`. There is no new RPC. CLI USAGE stays `dory pane close [--current | --pane <id>]`. Occupant verb stays `dory pane close`. Last pane in a tab still goes `tab.close`. Do not recook tab.create root_pane pane_id. Do not recook split pane_id. Do not recook neighbor pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant close pane_id | `git show HEAD:rust/src/server.rs` |
| Skill close pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant workspace create root_pane pane_id

Cite land with `git show HEAD:rust/src/envelope.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.create`. Occupant parse `.result.root_pane.pane_id`. Land `pane_id` is the new root `pane.id` (same as nested `.result.root_pane.id`). Keep `.result.workspace.id` / `.result.tab.id` / `.result.root_pane.id` / `.result.occupant`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace create [--cwd <path>]`. Occupant verb stays `dory workspace create`. Do not add `tab.close` `tab_id` this slice. Do not recook tab.create root_pane pane_id. Do not recook pane.close pane_id. Do not recook split pane_id. Do not recook neighbor pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace create root_pane pane_id | `git show HEAD:rust/src/envelope.rs` |
| Skill workspace create root_pane pane_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant close tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.close`. Occupant parse `.result.tab.tab_id`. Land `tab_id` is closed `tab.id` (same as nested `.result.tab.id`). Keep `.result.tab.id` / `.result.retired`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab close [<id> | --current]`. Occupant verb stays `dory tab close`. Do not recook workspace.create root_pane pane_id. Do not recook pane.close pane_id. Do not recook tab.create root_pane pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant close tab_id | `git show HEAD:rust/src/server.rs` |
| Skill close tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant close workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.close`. Occupant parse `.result.workspace.workspace_id`. Land `workspace_id` is closed `ws.id` (same as nested `.result.workspace.id`). Keep `.result.workspace.id` / `.result.retired`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace close [<id> | --current]`. Occupant verb stays `dory workspace close`. Do not recook tab.close tab_id. Do not recook workspace.create root_pane pane_id. Do not recook pane.close pane_id. Do not recook tab.create root_pane pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant close workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill close workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant tab create tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `tab.create`. Occupant parse `.result.tab.tab_id`. Land `tab_id` is the new `tab.id` (same as nested `.result.tab.id`). Keep `.result.tab.id` / `.result.root_pane.id` / `.result.root_pane.pane_id` / `.result.occupant`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory tab create [--workspace <id> | --current] [--cwd <path>]`. Occupant verb stays `dory tab create`. Do not recook tab.create root_pane pane_id. Do not recook workspace.close workspace_id. Do not recook tab.close tab_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant tab create tab_id | `git show HEAD:rust/src/server.rs` |
| Skill tab create tab_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant workspace create workspace_id

Cite land with `git show HEAD:rust/src/envelope.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.create` via `envelope::result_workspace`. Occupant parse `.result.workspace.workspace_id`. Land `workspace_id` is `ws.id` (same as nested `.result.workspace.id`). Keep `.result.workspace.id` / `.result.tab.id` / `.result.root_pane.id` / `.result.root_pane.pane_id`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace create [--cwd <path>]`. Occupant verb stays `dory workspace create`. Do not add `tab.tab_id` on workspace.create this slice. Do not recook workspace.close workspace_id. Do not recook tab.create tab_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace create workspace_id | `git show HEAD:rust/src/envelope.rs` |
| Skill workspace create workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant workspace create tab.tab_id

Cite land with `git show HEAD:rust/src/envelope.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.create` via `envelope::result_workspace`. Occupant parse `.result.tab.tab_id`. Land `tab_id` is the new `tab.id` (same as nested `.result.tab.id`). Keep `.result.workspace.id` / `.result.workspace.workspace_id` / `.result.tab.id` / `.result.root_pane.id` / `.result.root_pane.pane_id`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace create [--cwd <path>]`. Occupant verb stays `dory workspace create`. Do not add `pane.resize` `pane_id` this slice. Do not recook workspace.create workspace_id. Do not recook tab.create tab_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace create tab.tab_id | `git show HEAD:rust/src/envelope.rs` |
| Skill workspace create tab.tab_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant resize pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.resize`. Occupant parse `.result.pane.pane_id`. Land `pane_id` is resized `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id` / `.result.cols` / `.result.rows`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory pane resize [--current | --pane <id>] --cols N --rows N`. Occupant verb stays `dory pane resize`. Do not add `pane.write` `pane_id` this slice. Do not recook workspace.create tab.tab_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant resize pane_id | `git show HEAD:rust/src/server.rs` |
| Skill resize pane_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant write pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.write`. Occupant parse `.result.pane.pane_id`. Land `pane_id` is written `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory pane run [--current | --pane <id>] <text>`. Occupant verbs stay `dory pane run` / `dory pane send-keys` / `dory pane send-text`. Occupant send-keys/send-text stay `pane.write` + `"raw":true`. Do not recook pane.resize pane_id. `pane.focus` already `get_pane`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant write pane_id | `git show HEAD:rust/src/server.rs` |
| Skill write pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant read pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.read`. Occupant parse `.result.pane.pane_id`. Land `pane_id` is read `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id` / `.result.source` / `.result.text`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory pane read [--current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`. Occupant verb stays `dory pane read`. Default source stays `recent`. `--lines` still `tail_lines`. Do not recook pane.write pane_id. `pane.focus` already `get_pane`. Do not add `pane.wait` `pane_id` this slice. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant read pane_id | `git show HEAD:rust/src/server.rs` |
| Skill read pane_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant wait pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `pane.wait` settle via `tick_pane_wait`. Occupant parse `.result.pane.pane_id`. Land `pane_id` is waited `pane.id` (same as nested `.result.pane.id`). Keep `.result.pane.id` / `.result.matched` / `.result.text` / optional `.result.source` / `.result.lines`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory pane wait-output [--current | --pane <id>] [--match LIT | --regex RE] [--source visible|recent|recent-unwrapped] [--lines N] [--timeout MS]`. Occupant verb stays `dory pane wait-output`. Do not recook pane.read pane_id. Do not recook pane.write pane_id. Timeout error unchanged. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant wait pane_id | `git show HEAD:rust/src/server.rs` |
| Skill wait pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant divider a/b pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.divider`. Occupant parse `.result.a_pane_id` / `.result.b_pane_id`. Land `a_pane_id` / `b_pane_id` are `a` / `b` (same as `.result.a` / `.result.b`). Keep `.result.a` / `.result.b` / `.result.ratio`. There is no new RPC. There is no `pane.divider` RPC. There is no `tab.get`. CLI USAGE stays `dory pane divider [--a <id> | --current] --b <id> --ratio F`. Occupant verb stays `dory pane divider`. Do not recook pane.wait pane_id. Do not recook pane.send-keys result (`pane.write` already `pane_id`). Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant divider a/b pane_id | `git show HEAD:rust/src/server.rs` |
| Skill divider a/b pane_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant neighbor nested pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.neighbor`. Occupant parse `.result.pane.pane_id`. Land nested `pane_id` is neighbor `pane.id` (same as `.result.pane.id` and root `.result.pane_id`). Keep root `.result.pane_id` / `.result.pane.id`. There is no new RPC. There is no `pane.neighbor` RPC. There is no `tab.get`. CLI USAGE stays `dory pane neighbor [--current | --pane <id>] --direction left|right|up|down|prev|next [--cols N --rows N]`. Occupant verb stays `dory pane neighbor`. Do not recook root neighbor pane_id. Do not recook divider a/b pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant neighbor nested pane_id | `git show HEAD:rust/src/server.rs` |
| Skill neighbor nested pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant agent pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `agent_snapshot` nested `.result.agent`. Occupant parse `.result.agent.pane_id`. Land `pane_id` is occupant `pane.id` (same as `.result.agent.pane`). Keep `.result.agent.pane` a string. Keep wrapper `cwd` / `focused` / `tab_id` / `workspace_id`. There is no new RPC. There is no `agent.list` RPC. There is no `tab.get`. CLI USAGE stays `dory agent get [<name> | --current | --pane <id>]`. Occupant verb stays `dory agent get`. Do not recook neighbor nested pane_id. Do not recook divider a/b pane_id. Do not change the agent_snapshot wrapper. Do not recook founder `--wait`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant agent pane_id | `git show HEAD:rust/src/server.rs` |
| Skill agent pane_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant workspace nested root_pane pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `workspace.list` / `workspace.get` via `workspace_object`. Occupant parse list `.result.workspaces[].tabs[].root_pane.pane_id` and get `.result.tabs[].root_pane.pane_id`. Land nested `pane_id` is `tab.root_pane` (same as `root_pane.id`). Keep nested `id` / `root_pane.id` / `occupant` / `pane_count` / `focused` / `workspace_id` / `tab_id`. There is no new RPC. There is no `tab.get`. CLI USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. Occupant verbs stay `dory workspace list` / `dory workspace get`. Do not recook agent.pane pane_id. Do not recook workspace.create root_pane pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant workspace nested root_pane pane_id | `git show HEAD:rust/src/server.rs` |
| Skill workspace nested root_pane pane_id | `git show HEAD:skills/dory/SKILL.md` |


## Paid occupant snapshot pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `snapshot` via `live_snapshot`. Occupant parse `.pane_id`. Land `pane_id` is first pane id (same as `.pane`). Keep `.live` / `.workspace` / `.tab` / `.pane` / `.pid` / `.focused`. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. JSON stays `{"op":"snapshot"}`. Do not recook desk.tree pane pane_id. Do not recook workspace nested root_pane pane_id. Do not recook agent.pane pane_id. Do not wrap snapshot `pane` as an object. Do not change `desk.snapshot` this slice. Do not recook founder `--wait`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant snapshot pane_id | `git show HEAD:rust/src/server.rs` |
| Skill snapshot pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant snapshot tab_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `snapshot` via `live_snapshot`. Occupant parse `.tab_id`. Land `tab_id` is first tab id (same as `.tab`). Keep `.live` / `.workspace` / `.tab` / `.pane` / `.pane_id` / `.pid` / `.focused`. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. JSON stays `{"op":"snapshot"}`. Do not add `workspace_id` this slice. Do not wrap snapshot `pane` as an object. Do not change `desk.snapshot`. Do not recook snapshot pane_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant snapshot tab_id | `git show HEAD:rust/src/server.rs` |
| Skill snapshot tab_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant snapshot workspace_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `snapshot` via `live_snapshot`. Occupant parse `.workspace_id`. Land `workspace_id` is first workspace id (same as `.workspace`). Keep `.live` / `.workspace` / `.tab` / `.pane` / `.pane_id` / `.pid` / `.focused` / `.tab_id`. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. JSON stays `{"op":"snapshot"}`. Do not wrap snapshot `pane` as an object. Do not change `desk.snapshot`. Do not recook snapshot pane_id. Do not recook snapshot tab_id. Do not change `dead_snapshot`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant snapshot workspace_id | `git show HEAD:rust/src/server.rs` |
| Skill snapshot workspace_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant snapshot focused_pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `snapshot` via `live_snapshot`. Occupant parse `.focused_pane_id`. Land `focused_pane_id` is `world.focused` (same as `.focused`). Keep `.live` / `.workspace` / `.tab` / `.pane` / `.pane_id` / `.pid` / `.focused` / `.tab_id` / `.workspace_id`. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. JSON stays `{"op":"snapshot"}`. Do not wrap snapshot `pane` as an object. Do not change `desk.snapshot`. Do not change `dead_snapshot`. Do not recook snapshot pane_id/tab_id/workspace_id. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant snapshot focused_pane_id | `git show HEAD:rust/src/server.rs` |
| Skill snapshot focused_pane_id | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant snapshot cwd

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `snapshot` via `live_snapshot`. Occupant parse `.cwd`. Land `cwd` is `proc_cwd` on the first pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `.live` / `.workspace` / `.tab` / `.pane` / `.pane_id` / `.pid` / `.focused` / `.tab_id` / `.workspace_id` / `.focused_pane_id`. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. JSON stays `{"op":"snapshot"}`. Do not wrap snapshot `pane` as an object. Do not change `desk.snapshot`. Do not change `dead_snapshot`. Do not recook snapshot id family. Do not claim `foreground_cwd`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant snapshot cwd | `git show HEAD:rust/src/server.rs` |
| Skill snapshot cwd | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant snapshot occupant

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `snapshot` via `live_snapshot`. Occupant parse `.occupant`. Land `occupant` is first-pane `pane_occupant_json` (`null` or `{name,state,seen}`). Keep `.live` / `.workspace` / `.tab` / `.pane` / `.pane_id` / `.pid` / `.focused` / `.tab_id` / `.workspace_id` / `.focused_pane_id` / `.cwd`. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. There is no `agent.list`. JSON stays `{"op":"snapshot"}`. Do not wrap snapshot `pane` as an object. Do not change `desk.snapshot`. Do not change `dead_snapshot`. Do not recook snapshot cwd/id family. Do not add `agent` / `agent_status`. Do not claim Darwin occupant `done`/`idle`. Do not claim `foreground_cwd`. Do not claim `pane.zoom`.

| Landing | Owner |
|---|---|
| Occupant snapshot occupant | `git show HEAD:rust/src/server.rs` |
| Skill snapshot occupant | `git show HEAD:skills/dory/SKILL.md` |

## Paid occupant desk.snapshot focused_pane_id

Cite land with `git show HEAD:rust/src/server.rs`. Working-tree leftover `server.rs` is not the owner. Land wrap stays `desk.snapshot`. Occupant parse `.result.focused_pane_id`. Land `focused_pane_id` is `world.focused` (same as `.result.focused`). Keep `.result.focused` / `.result.text` / workspace list. There is no new RPC. There is no `dory snapshot` CLI. There is no `tab.get`. JSON stays `{"op":"desk.snapshot"}`. Do not recook live `snapshot` occupant/cwd/id family. Do not change `dead_snapshot`. Do not claim `pane.zoom`. Do not claim Darwin occupant `done`/`idle`.

| Landing | Owner |
|---|---|
| Occupant desk.snapshot focused_pane_id | `git show HEAD:rust/src/server.rs` |
| Skill desk.snapshot focused_pane_id | `git show HEAD:skills/dory/SKILL.md` |

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
