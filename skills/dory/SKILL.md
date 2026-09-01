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
- `pane split|run|read|wait-output`
- `pane resize [--current | --pane <id>] --cols N --rows N`
- `pane focus [--current | --pane <id>]`
- `pane neighbor [--current | --pane <id>] --direction left|right|up|down --cols N --rows N`
- `agent start|prompt|wait|get|read|focus|send-keys|report`
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

Parse IDs from `.result`. Omit `pane get` target → exit 2, not the focused pane.

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

`--current` reads injected `DORY_WORKSPACE_ID` and requires `DORY_ENV=1` (exit 1 outside env). `--workspace <id>` stays valid and does not require env. Exactly one of the two. Land RPCs stay `pane.list` / `tab.list` with `workspace`. This is Dory list-current, not Herdr implicit focused list and not `dory tree`. Keep `dory tree` for the live roster. No `--kind`. No `pane.zoom`.

Occupants read the live roster with

```bash
dory tree
```

Parse `.result.focused` and `.result.items[]`. Each item has `k` = `w` | `t` | `p`. Workspace rows may include `cwd`. Pane rows may include `occ` (name) and `st` (five words). Extra args → usage 2. Land RPC is `desk.tree`. This is Dory tree, not Herdr `agent list`. No `--kind`. No `pane.zoom`.

Create a workspace or tab only when the user asked for that topology. Create does not start an occupant.

## Layout

Honor a requested `--direction`. Otherwise omit it: a wide pane splits right, a tall/narrow pane splits down. New panes inherit the caller cwd. Pane split still has no `--cwd`. Humans sitting at the desk see the new pane tiled on the same face; occupants already inside a pane still use these CLI verbs and do not re-attach.

Default: sibling in the current tab, keep user focus:

```bash
dory pane split --current --direction right --no-focus
```

Replace `right` with `down` when appropriate. Read the new pane from `.result.pane.id`.

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

or `--pane <id>`. Omit target / omit `--direction` / `--cols` / `--rows` / extra flags including Herdr `--amount` → usage 2. Parse `.result.pane.id`. Land RPC is `desk.neighbor` with `from`, `step`, `cols`, `rows`. This is Dory neighbor, not Herdr `pane focus --direction`. Keep `dory pane focus` as id-only. No `pane.zoom`. No `--kind`.

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
```

Keep `dory tab close <id>` and `dory workspace close <id>`. Exactly one of positional `<id>` or `--current`. Both / neither / extra → usage 2. Mutating: `DORY_ENV=1`. `--current` reads `DORY_TAB_ID` / `DORY_WORKSPACE_ID` (exit 1 outside env / empty id). Land RPCs stay `tab.close` / `workspace.close`. Do not close tabs, panes, or workspaces you did not create unless the user asked. Do not close the factory chair. Closing the last live pane is refused. Occupants use these CLI verbs; do not sit the human desk. This is Dory close-current, not Herdr implicit focused close. No `--kind`. No `pane.zoom`.

## Run a command in another pane

```bash
dory pane run --pane <id-from-split> "just test"
dory pane wait-output --pane <id-from-split> --match "test result" --timeout 120000
dory pane read --pane <id-from-split> --source recent-unwrapped
```

`pane run` sends the remaining text plus Enter. `wait-output` needs exactly one of `--match <text>` or `--regex <pattern>`. Already-present output can match. `--timeout` is milliseconds; do not assume an indefinite wait.

Read `--source`: `visible` (viewport), `recent` (default), `recent-unwrapped` (soft wraps joined; prefer for logs).

To target the calling pane, pass `--current`. Do not omit the target.

## Occupant

Layout stays a pane verb. Occupant start never creates, splits, or moves a pane. Two occupants = `pane split` (parse `.result.pane.id`) then `agent start --pane <id> -- <argv>`.

Name: `[a-z][a-z0-9_-]{0,31}`, unique among live occupants. No `--kind`. Coding occupants start as argv after `--`, never `--kind`.

After the env gate, a coding occupant inside the pane that is ready for prompts must run:

```bash
dory agent report --current --state idle
```

That self-report is how `wait` leaves `unknown`.

```bash
dory agent start <name> --pane <id-from-split> -- <argv>
dory agent prompt <name> --wait -- <text>
dory agent wait <name>
dory agent get <name>
dory agent read <name> --source recent-unwrapped
dory agent focus <name>
dory agent send-keys <name> enter
dory agent report [--current | --pane <id>] --state working|blocked|idle
```

`start` / `prompt` / `wait` / `focus` / `send-keys` / `report` require `DORY_ENV=1`. `send-keys` allowlist: `enter`, `esc`, `ctrl+c`.

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
