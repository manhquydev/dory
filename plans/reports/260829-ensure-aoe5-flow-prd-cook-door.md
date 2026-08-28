---
type: cook-door
date: 2026-08-29
plan: 260829-0054-isolate-flow-prd-unlock
phase: 01
writer: scripts/dory-isolate-aoe5-flow-prd.sh
traps: [3, 4, 5, 6, 8]
verdict: DOOR_PASS
aoe5_prd: PASS
dory_invoked: 0
sat_t13: 0
closed_wP_w15_w16_t13: 0
herdr_server_stop: 0
sit: w13:t20 / w13:p97
door: w13:t32 / w13:p9E
factory_sock: /run/user/1000/dory/default/dory.sock
factory_sock_connectable: 0
path_dory: empty
---

# Cook door — AOE5p isolate flow prd (phase 01)

**Verdict: DOOR_PASS.** Cook pane `w13:p99` printed `aoe5-prd=PASS`.

Static read of `scripts/dory-isolate-aoe5-flow-prd.sh` (1121 lines, 34873 bytes) against plan traps 3/4/5/6/8 plus attach/setsid law. Did not invoke `dory`. Did not sit `w13:t13`. Did not `herdr server stop`. Did not close `wP` / `w15` / `w16` / `t13`.

## Live doors (this pane, after `aoe5-prd=PASS`)

| Door | Value |
|---|---|
| sit | `herdr pane get w13:p97` → `tab_id=w13:t20` `pane_id=w13:p97` `agent=None` `focused=false` |
| this door | `HERDR_TAB_ID=w13:t32` `HERDR_PANE_ID=w13:p9E` ≠ `t13` |
| PATH `dory` | empty (`type: dory not found`; `PATH` has no `dory` component) |
| factory default sock | `/run/user/1000/dory/default/dory.sock` lexists=false; `dory/` and `dory/default/` absent; python3 `AF_UNIX` timeout 1s → `connectable=0` `FileNotFoundError` |
| `wP` / `w15` / `w16` | still listed; not closed |
| `w13:t13` | still listed; not sat |

Probe = python3 `socket.AF_UNIX` connect timeout 1s. No `mkdir`. No factory `dory` argv. No leftover ELF. No isolate ELF on factory XDG.

Cook stdout on `w13:p99`: `SIT_PANE=w13:p97` `SIT_TAB=w13:t20` `FACTORY_CONNECTABLE=0` `FACTORY_SOCK=none`.

## Throughout cook (script seams)

`factory_must_dead` FAIL-only-if-connectable at isolate start `:880`, split `:898`, agent start `:906`, taxi1 `:967`, attach `:1009`, occupant poll loop `:601`, taxi2 `:1060`, before stop `:1075`. Preflight `:719`. Post-teardown `:1086`. `fail()` re-checks `:636`.

`path_dory_empty` at start `:778`, before stop `:1076`, after teardown `:1094`. Sit PATH probe `:973-994` (`type -a dory` must not succeed).

Cook reached `aoe5-prd=PASS` `exit 0`, so those seams did not fire.

## Trap 3 — no source/exec paid names

`$0` case includes judge + next + scope on first paste:

```
47:case "$0" in
48:  *dory-flock-hop.sh|*dory-isolate-flow-sit.sh|*dory-isolate-flock-roster.sh|*dory-isolate-flock-report.sh|*dory-isolate-flock-prompt.sh|*dory-isolate-aoe5-flow-judge.sh|*dory-isolate-aoe5-flow-next.sh|*dory-isolate-aoe5-flow-scope.sh)
```

`self_refuse_paid` `:263-284` regex adds `dory-isolate-aoe5-flow-judge|dory-isolate-aoe5-flow-next|dory-isolate-aoe5-flow-scope`. Called `:644`.

`^\s*(source|\.|exec)\s+` → **0**. Paid names appear only in `$0` case, self-refuse regex, and header comment `:6`. No `source` / `.` / `exec` of 1910/0043/0227/0242/hop/judge/next/scope. Isolate server is `/bin/bash -c 'cd "$0" && exec "$1" server'` `:859` (SIT_DORY), not a paid script.

## Trap 4 — refuse factory `FLOW_*`; taxi pin harness

Refuse any factory `FLOW_*` `:61-63`. Taxi `:329-336` pins `FLOW_PROJECT_ROOT` + `FLOW_BIN` (realpath `flow.sh`, not `/bin/true`) + `FLOW_LOG_DISABLE=1` + `FLOW_HARNESS_DISABLE=1` + `DO_NOT_TRACK=1`. `self_refuse_paid` requires `FLOW_HARNESS_DISABLE=1` in file text.

## Trap 5 — sit refuse `w13:t13` / `p2R` / `wP`; no `herdr server stop`

```
709:if [ "$SIT_PANE" = "w13:p2R" ]; then
713:  *wP:*) fail "SIT_PANE is wP" ;;
715:if [ "$SIT_TAB" = "w13:t13" ]; then
733:if [ "$GOT_TAB" = "w13:t13" ]; then
766:  if [ "$GOT_TAB" != "$SIT_TAB" ] || [ "$GOT_TAB" = "w13:t13" ] || [ -n "$GOT_AGENT" ]; then
```

`pane_fields` = `herdr pane get`. Split path re-checks `t13`. `herdr server stop` → **0**. `herdr (tab|workspace|pane) close` → **0**. Isolate stop is `compound_stop` only.

## Trap 6 — `compound_stop` identity then isolate XDG stop

`:111-143`: `iso_identity_ok` → sock exists / not symlink / `realpath` == `ISO_SOCK` / under `ISO_REAL` **then**

```
141:XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET "$SIT_DORY" server stop >/dev/null
```

`iso()` exists `:84-92` for `iso_mut` occupants. Stop does not call `iso()` / `DORY_SOCKET=`. Land sha pin `2ef20730…` `:27` / `:652`. Leftover ELF `3ba0e3bc…` stat-only `:231-244`.

## Trap 8 — no `--wait`; taxi2 IFF `cmp` PASS 02

`prompt --wait` → **0**. Coord prompt `:571` is `--timeout 180000`. Self-refuse concatenates `"prompt --" + "wait"` `:277`. Occupant text bans `--wait` `:580` `:584`. Poll `:597-607` ~180s + `factory_must_dead` each 2s. Taxi2 only after `cmp -s` PASS 02 `:1029`.

`flow -- gate` → **0**. Taxi is `flow -- next` `:336`. `pane run` → **0**. `occ.report` → **0**.

## Attach 1910 / setsid 0242

`:996` isolate XDG attach:

```
ATTACH_CMD="cd \"$ISO_REAL\" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \"$SIT_DORY\" attach"
```

Attach = `send-text` `:1001` + `send-keys enter` `:1002`. `wait-output` pane id first (`:1004`, `:1063`).

`:843-860` = 0242 skeleton (`mkdir`/`ln -sfn` ISO/bin **then** `setsid`). `realpath` bin/dory == `SIT_DORY` `:845`. Land sha `:848`. `"${_flow_u[@]}"` unset FLOW_* `:854`. `PI_CODING_AGENT_DIR` only on setsid `:857` (plus factory refuse `:57` and occupant ban). Occupants `:900-905` AOE5 `start_omp --no-session --no-skills --no-rules --no-extensions`.

## Cook land (p99)

| Signal | Value |
|---|---|
| `aoe5-prd` | PASS |
| Sit | `w13:p97` / `w13:t20` |
| Land ELF | `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3` |
| Taxi | codes `1,0` |
| Journal | abs `flow.sh`, `args=["next"]`, taxi1 `FAIL: gate for stage 02-scope is not clean`, taxi2 `unlocked stage 3 (flow/03-prd.md)` |
| 03 sha | `219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4` == `_templates/03-prd.md` |
| Factory sock | `FACTORY_CONNECTABLE=0` |

## Doors held

| Door | Held |
|---|---|
| source/exec paid (judge/next/scope included) | yes |
| sit `w13:t13` / `p2R` / `wP` | refused in script; live sit `t20`/`p97` |
| `compound_stop` identity then isolate XDG stop | yes |
| attach 1910 isolate XDG | yes |
| 0242 mkdir/ln then setsid | yes |
| `prompt --wait` / `flow -- gate` / `pane run` | absent |
| factory default sock connectable | 0 at T_end; script seams throughout |
| PATH `dory` | empty |
| invoke factory/leftover/isolate `dory` from this door | not done |
| sit `t13` from this door | not done |
| `herdr server stop` | not done |
| close `wP`/`w15`/`w16`/`t13` | not done |
