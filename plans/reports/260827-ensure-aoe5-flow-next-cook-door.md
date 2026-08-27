---
type: cook-door
date: 2026-08-27
plan: 260827-1657-isolate-flow-next-unlock
phase: 01
writer: scripts/dory-isolate-aoe5-flow-next.sh
traps: [3, 6, 7, 19, 20, 31]
verdict: DOOR_PASS
dory_invoked: 0
sat_t13: 0
---

# Cook door — AOE5n isolate flow next (phase 01)

**Verdict: DOOR_PASS**

Static rg of `scripts/dory-isolate-aoe5-flow-next.sh` (959 lines, 29307 bytes) against plan traps 3/6/7/19/20/31. Did not invoke `dory`. Did not sit `w13:t13`.

## Trap 3 — no source/exec paid names

`$0` case includes judge on first paste:

```
43:case "$0" in
44:  *dory-flock-hop.sh|*dory-isolate-flow-sit.sh|*dory-isolate-flock-roster.sh|*dory-isolate-flock-report.sh|*dory-isolate-flock-prompt.sh|*dory-isolate-aoe5-flow-judge.sh)
```

`self_refuse_paid` `:259-277` regex adds `dory-isolate-aoe5-flow-judge`. Called `:528`.

rg `^\s*(source|\.|exec)\s+` → **0**. Paid names appear only in `$0` case, self-refuse regex, and header comment `:5`. No `source` / `.` / `exec` of 1910/0043/0227/0242/hop/judge.

## Trap 31 — judge on first paste

Held by trap 3 `$0` + `self_refuse_paid`. Phase-01 copy-table only.

## Trap 6 — sit refuse `w13:t13`

```
593:if [ "$SIT_PANE" = "w13:p2R" ]; then
597:  *wP:*) fail "SIT_PANE is wP" ;;
599:if [ "$SIT_TAB" = "w13:t13" ]; then
614:if [ "$GOT_TAB" != "$SIT_TAB" ]; then
617:if [ "$GOT_TAB" = "w13:t13" ]; then
650:  if [ "$GOT_TAB" != "$SIT_TAB" ] || [ "$GOT_TAB" = "w13:t13" ] || [ -n "$GOT_AGENT" ]; then
```

`pane_fields` = `herdr pane get`. Split path re-checks `t13`. rg `herdr server stop` → **0**. `server stop` only in `compound_stop` `:137` (isolate) and occupant ban text.

## Trap 7 — `compound_stop` identity

`:107-138` matches 1910 `:69-100`: `iso_identity_ok` → sock exists / not symlink / `realpath` == `ISO_SOCK` / under `ISO_REAL` **then**

```
137:XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET "$SIT_DORY" server stop
```

`iso()` exists `:80-88` for `iso_mut` occupants. Stop does not call `iso()` / `DORY_SOCKET=`.

## Trap 19 — attach 1910 XDG

`:843` verbatim 1910 `:331`:

```
ATTACH_CMD="cd \"$ISO_REAL\" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \"$SIT_DORY\" attach"
```

Attach = `send-text` `:848` + `send-keys enter` `:849`. rg `pane run` → **0**. `wait-output` pane id first (`:851`, `:901`).

## Trap 20 — 0242 mkdir/ln `:340-353`

`:709-726` = 0242 `:340-353` skeleton (`mkdir`/`ln -sfn` ISO/bin **rồi** `setsid`). `realpath` bin/dory == `SIT_DORY` `:711-713`. Land sha extra `:714-716` (S1). `"${_flow_u[@]}"` extra `:720` (trap 4 unset FLOW_*). `PI_CODING_AGENT_DIR` only on setsid `:723` (plus factory refuse `:53` and occupant ban). Occupants `:766-771` AOE5 `start_omp --no-session --no-skills --no-rules --no-extensions`.

## `--wait` / `flow -- gate`

| Needle | rg |
|---|---|
| `flow -- gate` | **0**. Taxi `:329` is `flow -- next`. Self-refuse concatenates `"flow -- " + "gate"` `:275`. |
| `prompt --wait` | **0**. Coord prompt `:455` is `--timeout 180000`. Self-refuse concatenates `"prompt --" + "wait"` `:273`. |
| `--wait` | occupant ban only `:464` `:468` (`Do not pass --wait`). Not a flag. |
| `occ.report` | **0**. |

## Doors held

| Door | Held |
|---|---|
| source/exec paid (judge included) | yes |
| sit `w13:t13` / `p2R` / `wP` | refused |
| `compound_stop` identity then isolate XDG stop | yes |
| attach 1910 `:331` isolate XDG | yes |
| 0242 `:340-353` mkdir/ln then setsid | yes |
| `prompt --wait` / `flow -- gate` | absent |
| invoke factory/leftover/isolate `dory` from this door | not done |
| sit `t13` from this door | not done |
