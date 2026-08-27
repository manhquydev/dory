---
type: test-path
date: 2026-08-27
time: 17:28
plan: 260827-1657-isolate-flow-next-unlock
phase: 02
role: ts_path
pane: w13:p7S
tab: w13:t2K
verdict: PATH_PASS
started_default: no
sat_t13: no
invoked_dory: no
path_dory: empty
sock_connectable: 0
rust_log: b544f5f
factory_flow: absent
---

# TEST path — after independent next-unlock run

**Verdict: PATH_PASS.** After run. Did not start default. Did not sit `w13:t13`. Did not invoke factory `dory`.

This pane = roster `ts_path` (`w13:p7S` / `w13:t2K`). Not `ts_run` / `ts_jrnl` / `ts_left` / `ts_sit`. Cook receipt and cook journal `aoe5n.nOWHtI` not used as proof.

## Identity

| Role | Pane | Tab |
|---|---|---|
| ts_path (this) | `w13:p7S` | `w13:t2K` |
| ts_run | `w13:p7F` | `w13:t2K` |
| sit (cook roster; test mints its own) | `w13:p7D` | `w13:t2H` |
| factory | — | `w13:t13` |

`HERDR_ENV=1` `HERDR_PANE_ID=w13:p7S` `HERDR_TAB_ID=w13:t2K` ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. `HOME=/home/manhquy`. `XDG_RUNTIME_DIR=/run/user/1000`. cwd=`/home/manhquy/Downloads/flow/dory`.

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT` UNSET.

## After run

| Door | After run |
|---|---|
| PATH `dory` | empty |
| default sock `/run/user/1000/dory/default/dory.sock` | connectable=0 |
| `git log -1 -- rust/` | `b544f5f` |
| factory `flow/` | absent |

Did not re-probe those four (after-run is ground truth). Did not `UnixStream.connect`. Did not `type -a dory`. Did not `git log`. Did not list factory `flow/`. Did not `mkdir` `$XDG_RUNTIME_DIR/dory`.

## This pane did not

- start `/run/user/1000/dory/default`
- `mkdir` factory `dory/` or `dory/default`
- sit `w13:t13` / `w13:p2R` / `wP`
- invoke factory `dory` / leftover ELF / isolate ELF on factory XDG
- `dory server stop` default
- `herdr server stop`
- cargo leftover tree
- fold leftover 5
- write factory `flow/`
- recook / fill `01-research.md`

## Result

`PATH_PASS`. After independent run: PATH `dory` empty, default sock connectable=0, factory `flow/` absent, rust log `b544f5f`. Default not started.
