---
type: review-sit
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 03
lens: rv_sit
reviewer_pane: w13:p7V
reviewer_tab: w13:t2M
cook_sit_pane: w13:p7D
cook_sit_tab: w13:t2H
test_sit_pane: w13:p82
test_sit_tab: w13:t2P
sit_dory_sha: 2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3
factory_sock: /run/user/1000/dory/default/dory.sock
factory_connectable: 0
sat_t13: no
pane_run: no
verdict: SIT_ACCEPT
---

# REVIEW — rv_sit

**SIT_ACCEPT**

Accept iff `herdr pane get` `tab_id` ≠ `w13:t13`; pane ≠ `w13:p2R`/`*wP:*`; `SIT_DORY` sha `2ef20730…`; sit pane `type -a dory` empty; stop = `compound_stop` `:69-100`; factory sock dead; no leftover/isolate ELF argv on factory XDG; attach 1910 `:331`.

This pane = roster `rv_sit` (`w13:p7V` / `w13:t2M`). Did not sit `w13:t13`. Did not `send-text` / `send-keys` / `pane run` / `agent start` on sit or factory. Did not invoke factory `dory`. Cook/test receipts not used as sole proof.

## Sit identity (this turn)

`herdr pane get` (HERDR_ENV=1). No `.agent` key. `agent_status=unknown`.

| Role | Pane | Tab | cwd | ≠ t13/p2R/wP |
|---|---|---|---|---|
| cook sit | `w13:p7D` | `w13:t2H` | `…/aoe5n.nOWHtI (deleted)` | yes |
| test sit | `w13:p82` | `w13:t2P` | `…/aoe5n.X9Ll4T (deleted)` | yes |
| this lens | `w13:p7V` | `w13:t2M` | factory dory repo | yes |

`p7D` ≠ `p82`. Neither is cook pane `w13:p7E`. Neither is `w13:p2R`. Tabs `t2H`/`t2P` ≠ `w13:t13`. No `wP:` in pane/tab ids.

Script refuse (live): `SIT_PANE` ≠ `$HERDR_PANE_ID` / `w13:p2R` / `*wP:*`; `SIT_TAB` ≠ `w13:t13`; `pane_fields` `GOT_TAB` == `$SIT_TAB` and ≠ `w13:t13`; agent empty (`:590-624`).

Visible `--source visible` (read only): isolate desk chrome, not factory `t13`.

| Pane | Spaces | Agents | footer |
|---|---|---|---|
| `w13:p7D` | `aoe5n.nOWHtI` | `coord p1` / `omptest unknown p2` | `Flow 0. next` |
| `w13:p82` | `aoe5n.X9Ll4T` | `coord p1` / `omptest unknown p2` | `Flow 0. next` |

Post-wipe leftover TUI. Not a reject: factory sock still FileNotFound. Needles `Flow *. gate` not on these footers. Sequence `Flow 1. next` then `Flow 0. next` is cook/test `wait-output` (pane id first). Sit necessary, not sufficient — land is `rv_next`.

## SIT_DORY sha

Live `sha256sum` `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory`:

`2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3`

Script `LAND_SHA` same (`:24`). Pin check `:535-538`. ISO/bin `realpath` + sha `:711-716`. No hardcoded `land-4b70f79` in script. `case "$SIT_DORY"` refuses leftover `rust/target` (`:545-548`).

## Stop = 1910 `compound_stop`

Function body byte-equal to `scripts/dory-isolate-flow-sit.sh` `compound_stop` (`:69-101`; plan cites `:69-100`). Next copy `:107-139`. Identity then sock exists / not symlink / `realpath` == `ISO_SOCK` / under `ISO_REAL` **then**:

```
XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET "$SIT_DORY" server stop
```

`iso()` exists for occupants. Stop does not call `iso()` / `DORY_SOCKET=`.

## Attach = 1910 `:331`

`ATTACH_CMD` assignment byte-equal to 1910 `:331`:

```
cd "$ISO_REAL" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE "$SIT_DORY" attach
```

Next `:856`. `send-text` `:861` + `send-keys enter` `:862`. `pane run` **0** in script. `wait-output "$SIT_PANE"` pane id first (`:864`, `:914`).

Server start isolate XDG: 0242 `:340-353` skeleton `mkdir`/`ln -sfn` then `setsid` (`:709-726`). `XDG_RUNTIME_DIR="$ISO_REAL"`. `PI_CODING_AGENT_DIR` only that setsid line. Taxi uses `DORY_SOCKET=$ISO_SOCK` + `"$SIT_DORY"` (`:323-329`), not factory XDG default sock.

## Factory sock (this turn)

Probe = python3 `AF_UNIX` `UnixStream.connect` timeout 1s. Path `$XDG_RUNTIME_DIR/dory/default/dory.sock` = `/run/user/1000/dory/default/dory.sock`. Not the session dir. No `mkdir`.

```
lexists=False exists=False islink=False
dory_dir=False default_dir=False
connectable=0
err=FileNotFoundError: [Errno 2] No such file or directory
```

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT` unset. `HOME=/home/manhquy`. `XDG_RUNTIME_DIR=/run/user/1000`.

## Sit PATH

Factory this pane: `type -a dory` → `dory not found`. `~/.local/bin/dory` absent.

Sit-pane PATH: script probe **before** attach, redirect to isolate file, fail if rc=0 or `dory is ` (`:830-854`). Did not send `type -a dory` into post-attach TUI (S3 paper). `/proc` ELF×XDG scan not used (S7 reject gold-plate).

## No factory dory argv

| Check | Result |
|---|---|
| factory default sock | FileNotFound, connectable=0 |
| `$XDG_RUNTIME_DIR/dory` | absent |
| attach XDG | `$ISO_REAL` (1910 `:331`) |
| stop XDG | `$ISO_REAL` (`compound_stop`) |
| setsid XDG | `$ISO_REAL` |
| leftover/isolate ELF on factory XDG | not observed; sock dir absent |
| `herdr pane run` | absent in script |
| this lens invoke `dory` | no |

## This pane did not

- sit `w13:t13` / `w13:p2R` / `wP`
- `send-text` / `send-keys` / `pane run` / `agent start` on sit or factory
- start `/run/user/1000/dory/default`
- `mkdir` factory `dory/` or `dory/default`
- invoke factory `dory` / leftover ELF / isolate ELF on factory XDG
- `dory server stop` default
- `herdr server stop`
- cargo leftover / fold leftover 5
- fill `01-research.md`

## Result

`SIT_ACCEPT`. Cook sit `w13:t2H`/`w13:p7D` and test sit `w13:t2P`/`w13:p82` ≠ `w13:t13`. `SIT_DORY` sha `2ef20730…`. `compound_stop` == 1910. Attach == 1910 `:331`. Factory sock connectable=0. PATH `dory` empty. No `pane run`. `t13` not sat.
