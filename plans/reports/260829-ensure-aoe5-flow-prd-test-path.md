---
type: test-path
date: 2026-08-29
time: 01:06
plan: 260829-0054-isolate-flow-prd-unlock
phase: 02
role: pt_path
pane: w13:p9M
tab: w13:t33
verdict: PATH_PASS
started_default: no
sat_t13: no
invoked_dory: no
path_dory: empty
sock_connectable: 0
factory_flow: absent
---

# TEST path — after independent prd-unlock run

**Verdict: PATH_PASS.** After run. Did not start default. Did not sit `w13:t13`. Did not invoke factory `dory`.

This pane = roster `pt_path` (`w13:p9M` / `w13:t33`). Not `pt_run` / `pt_jrnl` / `pt_left` / `pt_sit`. Cook receipt and cook ISO `aoe5p.eGZMMi` not used as proof.

## Identity

| Role | Pane | Tab |
|---|---|---|
| pt_path (this) | `w13:p9M` | `w13:t33` |
| pt_run | `w13:p9A` | `w13:t33` |
| testsit (roster; test mints its own) | `w13:p98` | `w13:t31` |
| factory | — | `w13:t13` |

`HERDR_ENV=1` `HERDR_PANE_ID=w13:p9M` `HERDR_TAB_ID=w13:t33` ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. `HOME=/home/manhquy`. `XDG_RUNTIME_DIR=/run/user/1000`. cwd=`/home/manhquy/Downloads/flow/dory`.

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT` UNSET.

## After run

| Door | After run |
|---|---|
| PATH `dory` (`type -a dory`) | empty |
| default sock `/run/user/1000/dory/default/dory.sock` | connectable=0 |
| factory `flow/` | absent |

Did not re-probe those three (after-run is ground truth). Did not `UnixStream.connect`. Did not `type -a dory`. Did not list factory `flow/`. Did not `mkdir` `$XDG_RUNTIME_DIR/dory`.

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
- recook / fill `03-prd.md`

## Result

`PATH_PASS`. After independent run: PATH `dory` empty, default sock connectable=0, factory `flow/` absent. Default not started.
