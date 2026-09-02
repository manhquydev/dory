---
name: dory
description: "Control Dory, a workplace skill mux for coding agents. Use only when the user explicitly mentions Dory or asks to use Dory to inspect or control workspaces, tabs, or panes. Do not use merely because a task could benefit from a background terminal, delegation, or parallel work. Requires DORY_ENV=1."
---

# Dory

Dory organizes terminals into workspaces, tabs, and panes. Occupants talk to this session through the `dory` CLI.

## Gate

First action, exactly:

```bash
test "${DORY_ENV:-}" = 1
```

If the check fails, say “I am not running inside a Dory-managed pane” and **stop**. No curl. No `X-Dory-Inside`. No guessed socket. No `:7380`.

When the check passes, `dory` on `PATH` (or `"$DORY_BIN"` if set) is the client of this session. Do not open sockets yourself. Do not call `herdr` or `dsh`.

## Learn the current CLI

The installed binary is the authority. After the gate:

```bash
dory --help
dory workspace
dory tab
dory pane
dory agent
```

A group with no subcommand prints usage and exits 2. That is discovery. Do not invent flags if `--help` differs. Bare `dory` / `dory attach` is the **human** desk (sidebar + tiled live panes). Occupants already inside a pane do not re-attach. Humans see a grid after split; occupants still use CLI verbs, not the desk. `dory attach --plain` is the raw PTY client. `dory serve` is the Node journal lamp, not this binary.

Live `--help` ships:

- `workspace create [--cwd <path>]` / `list` / `get [<id> | --current]` / `close [<id> | --current]`
- `tab create [--workspace <id> | --current] [--cwd <path>]` / `list [--workspace <id> | --current]` / `close [<id> | --current]`
- `pane list [--workspace <id> | --current]`
- `pane get [--current | --pane <id>]`
- `pane current [--current | --pane <id>]`
- `pane close [--current | --pane <id>]`
- `pane split [--current | --pane <id>] [--direction right|down] [--ratio F] [--no-focus]`
- `pane run [--current | --pane <id>] <text>`
- `pane wait-output [--current | --pane <id>] [--match LIT | --regex RE] [--source visible|recent|recent-unwrapped] [--lines N] [--timeout MS]`
- `pane send-keys [--current | --pane <id>] <key>`
- `pane send-text [--current | --pane <id>] <text>`
- `pane read [--current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`
- `pane resize [--current | --pane <id>] --cols N --rows N`
- `pane focus [--current | --pane <id>]`
- `pane neighbor [--current | --pane <id>] --direction left|right|up|down|prev|next [--cols N --rows N]`
- `pane layout [--tab <id> | --current] --cols N --rows N`
- `pane divider [--a <id> | --current] --b <id> --ratio F`
- `agent start <name> [--pane <id> | --current] [--timeout MS] -- <argv>` / `prompt|wait|get|read|focus|send-keys|report`
- `agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>`
- `agent wait [<name> | --current | --pane <id>] [--until idle|done|blocked|working|unknown] [--timeout MS]`
- `agent get [<name> | --current | --pane <id>]`
- `agent read [<name> | --current | --pane <id>] [--source visible|recent|recent-unwrapped] [--lines N]`
- `agent focus [<name> | --current | --pane <id>]`
- `agent send-keys [<name> | --current | --pane <id>] <key>`
- `agent report [--current | --pane <id>] --state working|blocked|idle`
- `flow -- <args>`
- `tree`

`dory flow -- <args>` is a taxi: exec `FLOW_BIN` or `flow.sh`, cwd = the workspace directory (`DORY_WORKSPACE_DIR` or the pane cwd). Timeout **15000 ms**, then SIGTERM / 1s / SIGKILL. Preserve the judge exit; timeout or missing code exits 1. Occupant must not wait forever. No `next` / `card` / `check` inside Dory. Refuse `herdr`, `dsh`, `@deepseek-ai/dsh`.

## Envelope and IDs

Success: JSON on stdout, `{"ok":true,"result":…}`. Runtime errors: JSON on stderr, exit 1. Usage: exit 2.

Parse IDs from `.result…`. Never invent `w1` / `w1:t1` / `w1:p1`. Closed tab and pane IDs are not reused.

| Command | Next IDs |
|---|---|
| `workspace create` | `.result.workspace.id`, `.result.tab.id`, `.result.root_pane.id` |
| `tab create` | `.result.tab.id`, `.result.root_pane.id` |
| `pane split` | `.result.pane.id` |

`--current` reads injected `DORY_PANE_ID`. Omitting `--current` and `--pane` is usage (exit 2), not “the focused pane.”

Injected context (after the gate):

```bash
printf '%s\n' "$DORY_WORKSPACE_ID" "$DORY_TAB_ID" "$DORY_PANE_ID"
```

## Discover

Inspect. `list --workspace` / `get <id>` do not require `DORY_ENV`. `list --current` / `get --current` require `DORY_ENV=1`. Mutating verbs still do.

```bash
dory workspace list
dory workspace get <id>
dory workspace get --current
dory tab list --workspace <id>
dory pane list --workspace <id>
dory pane get --current
dory pane get --pane <id>
```

Keep `dory workspace get <id>` as inspect (no env). Exactly one of positional `<id>` or `--current`. Both / neither / extra (including Herdr `--label` / `--kind`) → usage 2. `--current` requires `DORY_ENV=1` and reads `DORY_WORKSPACE_ID` (exit 1 outside env / empty / invalid). JSON stays land `{"op":"workspace.get","workspace":"<id>"}`. No `tab.get`. No new RPC. This is Dory get-current, not Herdr implicit focused get.

Keep `dory workspace list` and `dory workspace get [<id> | --current]`. Parse `.result.workspaces[].tab_count` / `.result.workspaces[].pane_count` on list and `.result.tab_count` / `.result.pane_count` on get. Keep `.result.workspaces[].workspace.id` / `.result.workspace.id` and `tabs[]`. Land `tab_count` is `ws.tabs.len()`; `pane_count` is the sum of `tab.panes.len()`. JSON numbers. JSON stays land `workspace.list` / `{"op":"workspace.get","workspace":"<id>"}`. No `tab.get`. No `--label`. No `workspace.focus`. No new RPC. This is Dory workspace list/get counts wrapping land `workspace.list` / `workspace.get`, not Herdr `--label` / `workspace.focus`.

Keep `dory workspace list` and `dory workspace get [<id> | --current]`. Parse list `.result.workspaces[].focused` and get `.result.focused`. Land `focused` is JSON boolean: workspace contains the pane whose id is `world.focused`. Keep `workspace.id` / `tab_count` / `pane_count` / `tabs[]`. USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. JSON stays land `workspace.list` / `{"op":"workspace.get","workspace":"<id>"}`. No `workspace.focus`. No `--label`. No new RPC. This is Dory workspace list/get focused wrapping land `workspace_object`, not Herdr `workspace.focus`.

Keep `dory workspace list` and `dory workspace get [<id> | --current]`. Parse list `.result.workspaces[].tabs[].focused` and get `.result.tabs[].focused`. Land nested `focused` is JSON boolean: that tab contains the pane whose id is `world.focused`. Keep workspace `id` / `tab_count` / `pane_count` / workspace-level `focused` / `tabs[]` `id` / `root_pane` / `occupant`. No `pane_count` on nested tabs. No `active_tab_id`. USAGE stays `dory workspace list` and `dory workspace get [<id> | --current]`. JSON stays land `workspace.list` / `{"op":"workspace.get","workspace":"<id>"}`. No `workspace.focus`. No `tab.get`. No `tab.focus`. No `--label`. No new RPC. This is Dory workspace list/get nested tabs focused wrapping land `workspace_object`, not Herdr `workspace.focus` / `tab.focus`.

Keep `dory pane get --pane <id>`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind` / `--format`) → usage 2. `--pane <id>` is inspect (no env). `--current` requires `DORY_ENV=1` and reads `DORY_PANE_ID` (exit 1 outside env / empty / invalid). JSON stays land `{"op":"pane.get","pane":"<id>"}`. Parse `.result.pid` and `.result.cwd`. Land `cwd` is `proc_cwd` on the pane child (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `dory pane current` as the same `pane.get` RPC. There is **no** `pane.process-info`. Do not parse argv / cmdline / foreground. No `--format`. No `--kind`. No `pane.zoom`. This is Dory pane get inspect, not a new verb and not Herdr foreground argv.

Occupants inspect with:

```bash
dory pane current --current
```

or `--pane <id>`. Omit target → usage 2. Same land RPC as `pane get`: `pane.get`. Keep `dory pane get`. This is an id target, not extra Herdr inspect flags. No `pane.zoom`. No `--kind`. There is no `pane.neighbor` RPC; the occupant verb is `dory pane neighbor` wrapping `desk.neighbor`.

Occupants already inside a pane may

```bash
dory pane list --current
dory tab list --current
```

Keep `dory pane list [--workspace <id> | --current]`. Exactly one of `--workspace <id>` or `--current`. Both / neither / extra (including `--kind` / `--format`) → usage 2. `--workspace <id>` is inspect (no env). `--current` requires `DORY_ENV=1` and reads `DORY_WORKSPACE_ID` (exit 1 outside env). JSON stays land `{"op":"pane.list","workspace":"<id>"}`. Parse `.result.panes[].id`, `.result.panes[].pid`, `.result.panes[].cwd`, `.result.panes[].occupant`. Land `cwd` is `proc_cwd` on each pane child (same as `pane get`: `/proc/{pid}/cwd` with `world.cwd` fallback). Keep `dory tab list [--workspace <id> | --current]` as land `tab.list`. Keep `dory tree` for the live occupant roster (`occ` / `st`). Keep `dory pane get` for a single pane. There is **no** `pane.process-info`. Do not parse argv / cmdline / foreground. No `--format`. No `--kind`. No `pane.zoom`. This is Dory list-roster feel for Herdr `pane list` cwd, not a new RPC and not `dory tree`.

Keep `dory pane list [--workspace <id> | --current]`. Parse `.result.panes[].focused`. Land `focused` is JSON boolean: `pane.id == world.focused`. Keep `.result.panes[].id` / `.result.panes[].pid` / `.result.panes[].cwd` / `.result.panes[].occupant`. USAGE stays `dory pane list [--workspace <id> | --current]`. JSON stays land `{"op":"pane.list","workspace":"<id>"}`. No `pane.process-info`. No `pane.zoom`. No new RPC. This is a field on `pane.list`, not implicit focused list (`--current` still required to omit `--workspace`). Do not add `focused` on `pane.get` this slice.

Keep `dory tab list [--workspace <id> | --current]`. Exactly one of `--workspace <id>` or `--current`. Both / neither / extra (including Herdr `--label` / `--kind` / `--format`) → usage 2. `--workspace <id>` is inspect (no env). `--current` requires `DORY_ENV=1` and reads `DORY_WORKSPACE_ID` (exit 1 outside env). JSON stays land `{"op":"tab.list","workspace":"<id>"}`. Parse `.result.tabs[].id`, `.result.tabs[].occupant`, `.result.tabs[].pane_count`. Land `pane_count` is `tab.panes.len()` as a JSON number. Keep first-pane `occupant` (`null` or object). Keep `dory pane list` / `dory tree` as they are. No `tab.get`. No `--label`. No `--format`. No `--kind`. No `pane.zoom`. This is Dory tab-list feel for Herdr `tab list` pane_count, not `tab.get` and not `--label`.

Keep `dory tab list [--workspace <id> | --current]`. Parse `.result.tabs[].focused`. Land `focused` is JSON boolean: tab contains the pane whose id is `world.focused`. Keep `.result.tabs[].id` / `.result.tabs[].occupant` / `.result.tabs[].pane_count`. USAGE stays `dory tab list [--workspace <id> | --current]`. JSON stays land `{"op":"tab.list","workspace":"<id>"}`. No `tab.get`. No `--label`. No `tab.focus`. No new RPC. This is a field on `tab.list`, not implicit focused list (`--current` still required to omit `--workspace`).

Occupants read the live roster with

```bash
dory tree
```

Parse `.result.focused` and `.result.items[]`. Each item has `k` = `w` | `t` | `p`. Workspace rows may include `cwd` (`world.cwd`). Pane rows (`k=p`) may include `occ` (name), `st` (five words), and `cwd` via land `proc_cwd` (`/proc/{pid}/cwd` with `world.cwd` fallback). Keep `dory pane get` / `dory pane list` for `pid`. Extra args → usage 2. Land RPC is `desk.tree`. This is Dory tree feel for Herdr roster cwd, not `agent list` and not `pane.process-info`. No `--format`. No `--kind`. No `pane.zoom`.

Create a workspace or tab only when the user asked for that topology. Create does not start an occupant.

## Layout

Honor a requested `--direction`. Otherwise omit it: a wide pane splits right, a tall/narrow pane splits down. New panes inherit the caller cwd. Pane split still has no `--cwd`. Humans sitting at the desk see the new pane tiled on the same face; occupants already inside a pane still use these CLI verbs and do not re-attach.

Default: sibling in the current tab, keep user focus:

```bash
dory pane split --current --direction right --no-focus
```

Replace `right` with `down` when appropriate. Read the new pane from `.result.pane.id`.

Optional `--ratio` on the pair just created:

```bash
dory pane split --current --direction right --ratio 0.4 --no-focus
```

`--ratio` optional. Omit → land `split_leaf` default `0.5`. Present → land `set_ratio` (clamps `[0.05, 0.95]`). CLI does not re-clamp. Keep `dory pane divider` for an existing pair.

Keep `dory pane split --pane <id>`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind` / `--cwd`) → usage 2. `--direction` optional `right` | `down`. `--ratio F` optional. Missing value / non-float / twice → usage 2. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID` (exit 1 outside env / empty / invalid). `--pane <id>` keeps an explicit pane. JSON stays land `{"op":"pane.split","pane":"<id>","no_focus":true}` plus `"direction":"…"` only when set, plus `"ratio":F` only when set (JSON number, not string). There is **no** new split RPC. Pane split still has no `--cwd`. No `pane.zoom`. No `--kind`.

Occupants may pin cwd only on create:

```bash
dory workspace create --cwd /abs/path
dory tab create --workspace "$DORY_WORKSPACE_ID" --cwd /abs/path
dory tab create --current --cwd /abs/path
```

Omit `--cwd` → land uses focused-pane cwd / desk cwd (`spawn_cwd`). This is Dory create-cwd (land `cwd` field), not Herdr `--label` / `--no-focus` on create. No `--kind`.

Keep `dory tab create --workspace <id>` and optional `--cwd`. Exactly one of `--workspace <id>` or `--current`. Both / neither / extra (including Herdr `--label` / `--no-focus` / `--kind`) → usage 2. Mutating: `DORY_ENV=1`. `--current` reads injected `DORY_WORKSPACE_ID` (exit 1 outside env / empty / invalid). JSON stays land `{"op":"tab.create","workspace":"<id>"}` plus optional `cwd`. No `tab.current` RPC. No `pane.split --cwd`. This is Dory create-current, not Herdr `--label` / `--no-focus` on create.

Occupants resize with:

```bash
dory pane resize --current --cols N --rows N
```

or `--pane <id>`. Omit target → usage 2. Both `--cols` and `--rows` are required. Land RPC is `pane.resize`.

Occupants focus with:

```bash
dory pane focus --current
```

or `--pane <id>`. Omit target → usage 2. Land RPC is `pane.focus`. This is an id target, not Herdr `pane focus --direction left|right|up|down`. Keep `agent focus <name>` as a different verb. No `pane.zoom`. There is no `pane.neighbor` RPC; the occupant verb is `dory pane neighbor` wrapping `desk.neighbor`.

Occupants walk to a spatial neighbor with

```bash
dory pane neighbor --current --direction left --cols N --rows N
```

or `--pane <id>`. Spatial `left|right|up|down` still requires `--cols` and `--rows`. JSON stays land `desk.neighbor` with `from`, `step`, `cols`, `rows`.

Occupants walk the land global pane ring with

```bash
dory pane neighbor --current --direction next
```

or `--pane <id>` / `--direction prev`. Ring `prev|next` forbids `--cols` / `--rows` (present → usage 2). JSON is land `{"op":"desk.neighbor","from":"<id>","step":"prev"}` or `"next"`. Exactly one of `--current` or `--pane <id>`. `--pane` inspects. `--current` reads `DORY_PANE_ID`. Omit target / omit `--direction` / extra (including `--kind` / `--amount`) → usage 2. Parse `.result.pane.id`. There is no `pane.neighbor` RPC; occupant verb wraps `desk.neighbor`. This is the land global pane ring, not desk chrome tab n/p and not attach sit. Keep `dory pane focus` as id-only. No `pane.zoom`. No `--kind`.

To focus a neighbor, compose neighbor then focus:

1. Run `dory pane neighbor --current --direction next` (or spatial with `--cols --rows`).
2. Parse `.result.pane.id`.
3. Run `dory pane focus --pane <id>`.

Do not pass `--direction` to `dory pane focus`. Keep `dory pane focus --current` as the self-target.

Occupants inspect this tab's tile geometry with

```bash
dory pane layout --current --cols 120 --rows 40
```

or `--tab <id>`. Exactly one of `--tab <id>` or `--current`. Both / neither / extra (including `--kind` / `--direction` / `--amount`) → usage 2. `--cols` and `--rows` required. `--tab <id>` is inspect (no env). `--current` requires `DORY_ENV=1` and reads `DORY_TAB_ID` (exit 1 outside env / empty / invalid). Parse `.result.tab` and `.result.cells[]` (`id`, `x`, `y`, `w`, `h`, `occ`, `st`). JSON stays land `{"op":"desk.layout","tab":"<id>","cols":N,"rows":N}`. There is no `pane.layout` RPC; occupant verb wraps `desk.layout`. This is Dory layout inspect, not Herdr implicit focused layout and not `pane.zoom`. Keep `dory pane resize` / `dory pane neighbor` as they are.

Occupants move a shared split with

```bash
dory pane divider --current --b <sibling-id> --ratio 0.4
```

or `--a <id>`. Exactly one of `--a <id>` or `--current`. Both / neither / extra (including `--kind` / `--direction` / `--amount` / `--cols` / `--rows` / `--tab`) → usage 2. `--b <id>` and `--ratio F` required. Bad number / omit → usage 2. Mutating: `DORY_ENV=1` (exit 1 outside env) for both `--a` and `--current`. `--current` reads `DORY_PANE_ID` for the first pane (exit 1 outside env / empty / invalid). Keep `--a <id>`. JSON stays land `{"op":"desk.divider","a":"<id>","b":"<id>","ratio":F}`. There is no `pane.divider` RPC; occupant verb wraps `desk.divider`. Land returns `"no shared split"` when panes do not share a split — print the envelope (exit 1). Do not invent sibling discovery. Land clamps ratio to `[0.05, 0.95]`. CLI parses `f32` and does not re-clamp. This is Dory divider, not Herdr `--direction` / `--amount`. Keep `dory pane resize` / `dory pane layout` as they are. No `pane.zoom`.

```bash
dory tab create --workspace "$DORY_WORKSPACE_ID"
dory tab create --current
dory tab close <id-from-create>
dory pane close --pane <id>
dory workspace close <id>
```

Occupants may

```bash
dory tab close --current
dory workspace close --current
dory pane close --current
```

Keep `dory tab close <id>` and `dory workspace close <id>`. Exactly one of positional `<id>` or `--current`. Both / neither / extra → usage 2. Mutating: `DORY_ENV=1`. `--current` reads `DORY_TAB_ID` / `DORY_WORKSPACE_ID` (exit 1 outside env / empty id). Land RPCs stay `tab.close` / `workspace.close`. Do not close tabs, panes, or workspaces you did not create unless the user asked. Do not close the factory chair. Closing the last live pane is refused. Occupants use these CLI verbs; do not sit the human desk. This is Dory close-current, not Herdr implicit focused close. No `--kind`. No `pane.zoom`.

Keep `dory pane close --pane <id>`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Mutating: `DORY_ENV=1`. `--current` reads `DORY_PANE_ID` (exit 1 outside env / empty / invalid). `--pane <id>` keeps an explicit pane. JSON stays land `{"op":"pane.close","pane":"<id>"}`. Closing the last live pane is refused. Do not close the factory chair. Do not close panes you did not create unless the user asked. Occupants use this CLI; do not sit the human desk. This is Dory close-current, not implicit focused close. No `--kind`. No `pane.zoom`. Keep `dory tab close` / `dory workspace close` as they are.

## Run a command in another pane

```bash
dory pane run --pane <id-from-split> "just test"
dory pane wait-output --pane <id-from-split> --match "test result" --timeout 120000
dory pane read --pane <id-from-split> --source recent-unwrapped
```

`pane run` sends the remaining text plus Enter. `wait-output` needs exactly one of `--match <text>` or `--regex <pattern>`. Already-present output can match. `--timeout` is milliseconds; do not assume an indefinite wait.

Read `--source`: `visible` (viewport), `recent` (default), `recent-unwrapped` (soft wraps joined; prefer for logs).

To target the calling pane, pass `--current`. Do not omit the target.

```bash
dory pane run --current "just test"
dory pane wait-output --current --match "test result" --timeout 120000
dory pane read --current --source recent-unwrapped
```

```bash
dory pane wait-output --pane <id-from-split> --match "test result" --source recent-unwrapped --lines 120 --timeout 120000
dory pane wait-output --current --match "test result" --lines 80
```

Keep `dory pane wait-output --pane <id>`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Exactly one of `--match` or `--regex`. Both / neither / empty → usage 2. `--source` optional: `visible` | `recent` | `recent-unwrapped`. Omit `--source` → land still matches `recent_unwrapped` (not `pane read` default `recent`). `--lines N` optional, `N >= 1`. Tails that snapshot. Does not fetch more history than land holds. Missing / `0` → usage 2. `--timeout` milliseconds (default 5000). Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID` (exit 1 outside env / empty / invalid). `--pane <id>` keeps an explicit pane. JSON stays land `{"op":"pane.wait","pane":"<id>","timeout":MS}` plus `"match"` or `"regex"`, plus `"source":"…"` only when set, plus `"lines":N` only when set. There is **no** new wait RPC. Keep match XOR regex. No `pane.zoom`.

```bash
dory pane read --current --source recent-unwrapped --lines 120
dory pane read --pane <id-from-split> --source recent-unwrapped --lines 120
```

Keep `dory pane read --pane <id>`. `--lines N` optional, `N >= 1`. Omit → full land snapshot. Missing / `0` / extra (including `--kind` / `--format` / `detection`) → usage 2. Tails the land snapshot (`tail_lines`). Does not fetch more history than land holds. Does not mark seen. JSON adds `"lines":N` only when the flag is set. Land op stays `pane.read`. No `agent.list`. No `pane.zoom`. No `--kind`.

```bash
dory pane send-keys --pane <id-from-split> enter
dory pane send-keys --current esc
```

Keep `dory pane send-keys --pane <id>`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind` / a positional name) → usage 2. `<key>` required. Allowlist `enter` | `esc` | `ctrl+c` only. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID` (exit 1 outside env / empty / invalid). `--pane <id>` keeps an explicit pane. JSON stays land `{"op":"pane.write","pane":"<id>","text":"…","raw":true}` (no extra newline). There is **no** `pane.send-keys` RPC. Keep `dory pane run` as text + Enter. Keep `dory agent send-keys` as the occupant-named verb. Do not expand the allowlist. No `--kind`. No `pane.zoom`.

```bash
dory pane send-text --pane <id-from-split> hello
dory pane send-text --current hi
```

Keep `dory pane send-text --pane <id>`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. `<text>` required (join remaining argv with a space, same as `pane run`). Missing text → usage 2. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID` (exit 1 outside env / empty / invalid). `--pane <id>` keeps an explicit pane. JSON stays land `{"op":"pane.write","pane":"<id>","text":"…","raw":true}` (no extra newline). There is **no** `pane.send-text` RPC. Keep `dory pane run` as text + Enter. Keep `dory pane send-keys` as allowlist keys. Keep `dory agent send-keys` as the occupant-named verb. No `--kind`. No `pane.zoom`.

## Occupant

Layout stays a pane verb. Occupant start never creates, splits, or moves a pane. Two occupants = `pane split` (parse `.result.pane.id`) then `agent start --pane <id> -- <argv>`. Keep `dory agent start <name> --pane <id>` after split. Exactly one of `--pane <id>` or `--current`. Both / neither / extra (including `--kind`) → usage 2. Mutating: `DORY_ENV=1`. `--current` reads injected `DORY_PANE_ID` (exit 1 outside env / empty / invalid). `--` + argv still required. `--current` occupies the **calling** pane. After split, start the sibling with `--pane <id-from-split>`, not `--current`. JSON stays land `{"op":"agent.start",…,"pane":"<id>"}`. No `agent.list`. No `--kind`. This is Dory start-current, not implicit focused start and not a PATH farm.

Name: `[a-z][a-z0-9_-]{0,31}`, unique among live occupants. No `--kind`. Coding occupants start as argv after `--`, never `--kind`.

After the env gate, a coding occupant inside the pane that is ready for prompts must run:

```bash
dory agent report --current --state idle
```

That self-report is how `wait` leaves `unknown`.

```bash
dory agent start <name> --pane <id-from-split> -- <argv>
dory agent start cook --current -- omp
dory agent prompt <name> --wait -- <text>
dory agent prompt --current -- hi
dory agent prompt --pane <id-from-split> -- hi
dory agent wait <name>
dory agent wait --current
dory agent wait --pane <id-from-split>
dory agent get <name>
dory agent get --current
dory agent get --pane <id-from-split>
dory agent read <name> --source recent-unwrapped
dory agent read --current --source recent-unwrapped
dory agent read --pane <id-from-split> --source recent-unwrapped
dory agent read --current --source recent-unwrapped --lines 120
dory agent read --pane <id-from-split> --source recent-unwrapped --lines 120
dory agent focus <name>
dory agent focus --current
dory agent focus --pane <id-from-split>
dory agent send-keys <name> enter
dory agent send-keys --current enter
dory agent send-keys --pane <id-from-split> esc
dory agent report [--current | --pane <id>] --state working|blocked|idle
```

`start` / `prompt` / `wait` / `focus` / `send-keys` / `report` require `DORY_ENV=1`. `send-keys` allowlist: `enter`, `esc`, `ctrl+c`.

Exactly one of `<name>` or `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Text required. Mutating: `DORY_ENV=1`. `--current` reads `DORY_PANE_ID`. `--pane <id>` keeps an explicit pane. Keep `<name>`. Named JSON stays land `{"op":"agent.prompt","name":"<name>",…}`. Pane arms send `pane` and omit `name`. There is **no** `agent.list` RPC. `--wait` / `--timeout` stay as they are. No `--kind`. No `pane.zoom`.

Keep `dory agent get <name>` as inspect (no env). Exactly one of `<name>` or `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Named and `--pane` stay inspect (no env). `--current` reads `DORY_PANE_ID` and needs `DORY_ENV=1`. Named JSON stays land `{"op":"agent.get","name":"<name>"}`. Pane arms send `pane` and omit `name`. There is **no** `agent.list` RPC. No `--kind`. No `pane.zoom`.

Keep `dory agent wait <name>`. Exactly one of `<name>` or `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID`. `--pane <id>` keeps an explicit pane. Keep `<name>`. Named JSON stays land `{"op":"agent.wait","name":"<name>",…}`. Pane arms send `pane` and omit `name`. There is **no** `agent.list` RPC. `--until` / `--timeout` stay as they are. Do not teach a new wait farm. No `--kind`. No `pane.zoom`.

Keep `dory agent read <name>` as inspect (no env). Exactly one of `<name>` or `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Named and `--pane` stay inspect (no env). `--current` reads `DORY_PANE_ID` and needs `DORY_ENV=1`. Keep `--source visible|recent|recent-unwrapped` (default `recent`). `--lines N` optional, `N >= 1`. Omit → full land snapshot. Missing / `0` / extra (including `--kind` / `--format` / `detection`) → usage 2. Tails the land snapshot (`tail_lines`). Does not fetch more history than land holds. Does not mark seen. Named JSON stays land `{"op":"agent.read","name":"<name>","source":"…"}`. JSON adds `"lines":N` only when the flag is set. Pane arms send `pane` and omit `name`. There is **no** `agent.list` RPC. `agent read` does not mark seen. No `--kind`. No `pane.zoom`.

Keep `dory agent focus <name>`. Exactly one of `<name>` or `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID`. `--pane <id>` keeps an explicit pane. Keep `<name>`. Named JSON stays land `{"op":"agent.focus","name":"<name>"}`. Pane arms send `pane` and omit `name`. There is **no** `agent.list` RPC. Focus marks seen. `agent read` / `pane read` do not. Keep `dory pane focus` as a different verb. No `--kind`. No `pane.zoom`.

Keep `dory agent send-keys <name>`. Exactly one of `<name>` or `--current` or `--pane <id>`. Both / neither / extra (including `--kind`) → usage 2. `<key>` required. Allowlist `enter` | `esc` | `ctrl+c` only. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID`. `--pane <id>` keeps an explicit pane. Keep `<name>`. Named JSON stays land `{"op":"agent.send-keys","name":"<name>","key":"<key>"}`. Pane arms send `pane` + `key` and omit `name`. There is **no** `agent.list` RPC. Do not expand the allowlist. Keep `dory pane focus` / `dory agent focus` as different verbs. No `--kind`. No `pane.zoom`.

Keep `dory agent report --current`. Exactly one of `--current` or `--pane <id>`. Both / neither / extra (including `--kind` / a positional name) → usage 2. `--state working|blocked|idle` required. Mutating: `DORY_ENV=1` on every arm. `--current` reads `DORY_PANE_ID`. `--pane <id>` keeps an explicit pane. JSON stays land `{"op":"agent.report","pane":"<id>","state":"<state>"}`. There is **no** `name` field and **no** `agent.list` RPC. Occupant must `report`. Desk does not guess Claude/Codex. No `--kind`. No `pane.zoom`.

Five words: `working` | `blocked` | `idle` | `done` | `unknown`. `idle` = ready and seen. `done` = ready, unseen. `unknown` is not completion. Focus marks seen. `agent read` / `pane read` do not. Refuse `prompt` when `blocked`. `--wait` settles first of `idle|done|blocked`. `--until` only for a specific state.

## Flow taxi

Requires `DORY_ENV=1`. After `--`, remaining args go to the foreign judge. Empty `--` injects `status`: `dory flow --` is the same as `dory flow -- status`, not a distinct pass-through of zero args. No `--` is usage 2.

```bash
dory flow --
dory flow -- status
```

Timeout **15000 ms**, then SIGTERM, 1s grace, then SIGKILL. Timeout → `code=None`, process exit `unwrap_or(1)`. Occupant must not wait forever.

Taxi stdout is always `envelope::success(&event)` then `result.code.unwrap_or(1)`. JSON on stdout can look ok while the process exit is the judge code or 1. Do not treat this path as “runtime JSON stderr exit 1”. Envelope § above still holds for other RPCs.

`FLOW_BIN` overrides the judge path; otherwise `flow.sh` on `PATH`. Journal: `{cwd}/.dory/sessions/s1.jsonl` with `flow/invoke` then `flow/result`. Do not implement Flow gates inside Dory.

## Do not

- Start `dory server` from a pane (nested server is refused).
- Invent `--kind`, a PATH farm, or `agent start` that splits a pane.
- Grow Flow gates (`next` / `card` / `check`) inside Dory. Taxi only.
- Use HTTP, curl, `X-Dory-Inside`, or a guessed `DORY_SOCKET`.
- Drive another client’s focus by omitting a pane target.
- Treat `unknown` as done.
