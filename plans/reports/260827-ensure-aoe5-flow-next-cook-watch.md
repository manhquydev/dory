---
type: cook-watch
date: 2026-08-27
time: 17:26
plan: 260827-1657-isolate-flow-next-unlock
phase: 01
watcher: ck_watch
watcher_pane: w13:p7N
watcher_tab: w13:t2J
impl: ck_impl
impl_pane: w13:p7E
impl_tab: w13:t2J
sit_pane: w13:p7D
sit_tab: w13:t2H
cook_verdict: COOK_PASS
doors: HOLD
started_default: no
sat_t13: no
invoked_dory: no
---

# Watch — factory doors during ck_impl

**Doors: HOLD.** ck_impl receipt **COOK_PASS**. Watcher did not start default. Watcher did not sit `w13:t13`.

Probe = python3 `AF_UNIX` `UnixStream.connect` timeout 1s on `$XDG_RUNTIME_DIR/dory/default/dory.sock` = `/run/user/1000/dory/default/dory.sock`. Not the session dir. No `mkdir`. No factory `dory` argv. No leftover ELF exec. No isolate ELF on factory XDG.

## Identity

| Role | Pane | Tab | Agent |
|---|---|---|---|
| ck_watch (this) | `w13:p7N` | `w13:t2J` | omp |
| ck_impl | `w13:p7E` | `w13:t2J` | omp |
| sit (roster) | `w13:p7D` | `w13:t2H` | none |
| factory | — | `w13:t13` | pre-existing `working`; not sat |

`HERDR_PANE_ID=w13:p7N` `HERDR_TAB_ID=w13:t2J` ≠ `w13:t13` ≠ `w13:p2R`. Sit pane `tab_id=w13:t2H` every sample. Sit `.agent` empty. Watcher never `send-text` / `attach` / `agent start` on `t13` / `p2R` / `wP`.

## COOK_PASS

Source: `plans/reports/260827-ensure-aoe5-flow-next-cook.md` frontmatter `verdict: COOK_PASS` and body `**Verdict: COOK_PASS**`. Script `scripts/dory-isolate-aoe5-flow-next.sh` exit 0, elapsed 41.49s. Journal + `01.sha256` copied. ck_impl `agent_status=done` at T_end.

Visible `COOK_PASS` on `p7E` at 17:24:36 was **prompt text** (script still “Run isolate / write receipt”). Strict gate = receipt `verdict:` line. Receipt appeared 17:25:24.

## Door samples

41 polls 17:16:56–17:25:24 (12s then 15s) plus T0 (~17:15) and T_end 17:26:21+07:00. Every sample:

| Door | T0 | During (41) | T_end |
|---|---|---|---|
| PATH `dory` | empty (`type: dory not found`; `which=None`) | empty all 41 | empty |
| default sock connectable | 0 `FileNotFoundError` | 0 all 41 | 0 `FileNotFoundError` |
| factory `flow/` | ABSENT | ABSENT all 41 | ABSENT |
| leftover 5 porcelain | ` M` ×5 | ` M` ×5 all 41 | ` M` ×5 |
| leftover 5 mint | MATCH | MATCH all 41 | MATCH |
| `$XDG_RUNTIME_DIR/dory` | absent | absent all 41 | absent |
| `$XDG_RUNTIME_DIR/dory/default` | absent | absent all 41 | absent |
| repo `.dory/` | ABSENT | ABSENT | ABSENT |

Script file first existed poll n=21 17:21:11. PATH/sock/flow/leftover unchanged through write + run.

### sock (T_end)

```
path=/run/user/1000/dory/default/dory.sock
lexists=False exists=False islink=False
dory_dir_lexists=False default_dir_lexists=False
connectable=0
err=FileNotFoundError: [Errno 2] No such file or directory
elapsed_s=0.0000
dory_dir_after=False default_dir_after=False
```

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT` UNSET. `HOME=/home/manhquy`. `~/.local/bin/dory` absent.

### leftover 5 mint (T_end)

| path | porcelain | `git hash-object` | mint |
|---|---|---|---|
| `README.md` | ` M` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| `rust/src/attach.rs` | ` M` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| `rust/src/main.rs` | ` M` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| `rust/src/server.rs` | ` M` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| `rust/tests/p5_attach.rs` | ` M` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

`desk.rs` worktree `4c788562e4fdda10c8edd2878ed1fdd46050c218` == `HEAD:rust/src/desk.rs`. `git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit`. No leftover 5 staged.

## Isolate attach vs factory default

T_end `ps`: PID `2969442` `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory attach`.

Sit pane cwd = `/home/manhquy/.cache/dory-isolates/aoe5n.nOWHtI (deleted)`. Cache `aoe5n.*` dirs: none (wipe done).

This is isolate sit attach on `w13:t2H`, not factory default. Factory XDG `dory/` still absent. connectable=0. Watcher did not start it, did not `server stop default`, did not `herdr server stop`.

## Watcher did not

- start `/run/user/1000/dory/default`
- `mkdir` factory `dory/` or `dory/default`
- sit `w13:t13` / `w13:p2R` / `wP`
- invoke factory `dory` / leftover ELF / isolate ELF on factory XDG
- `dory server stop` default
- `herdr server stop`
- fold leftover 5 / cargo leftover tree
- write factory `flow/`

## Result

`COOK_PASS`. Factory doors held T0 → during ck_impl → T_end: PATH `dory` empty, default sock connectable=0, factory `flow/` absent, leftover 5 still ` M` mint.
