# COOK — AOE 5 isolate fail-then-pass flow judge

You are `aoe5_cook`. Skills ON (ak:cook). Implement plan `/home/manhquy/Downloads/flow/dory/plans/260827-1122-aoe-5-isolate-flow-judge/` phase 1.

## Create

`scripts/dory-isolate-aoe5-flow-judge.sh`

Copy **law** from 1910 + 0242. **Do not** `source`/`exec` those scripts or hop.

## Required env (factory sets)

`SIT_PANE` `SIT_TAB` `SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory`

## Must copy verbatim

- 1910 `compound_stop` `:69-100` + `iso_identity_ok` + `factory_must_dead` + sock_connectable
- 1910 ATTACH `:331` — `cd "$ISO_REAL" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE "$SIT_DORY" attach`
- 0242 server `:346-353` `setsid` + `XDG_RUNTIME_DIR="$ISO_REAL"` + `HOME="$ISO_REAL/home"` + `PI_CODING_AGENT_DIR` **only on that line**
- 1910/0242 sit refuse: exact `w13:t13` `w13:p2R` `*wP:*` + `herdr pane get` tab_id
- 0242 isolate `bin/dory` → SIT_DORY; occupants coord+omptest; report idle; prompt **no `--wait`**

## Must NOT copy

- 1910 taxi `:308-309` `FLOW_BIN=/bin/true`

## New

- Prefix ISO `aoe5.XXXXXX`. `PROJECT=$ISO_REAL`. Mint FAIL `$ISO_REAL/flow/00-idea.md` from `/home/manhquy/.claude/skills/flow/_templates/00-idea.md`
- `FLOW_BIN=/home/manhquy/.claude/skills/flow/runner/flow.sh` (realpath, basename `flow.sh`, `-x`)
- Refuse factory `DORY_*` `PI_CODING_AGENT_DIR` `FLOW_BIN` `FLOW_PROJECT_ROOT`; `HOME==FACTORY_HOME`
- Taxi helper in phase-01 (HOME isolate prefix + FLOW_PROJECT_ROOT=$ISO_REAL)
- Taxi 1 → journal new result bin+code=1+`GATE stage 00-idea`; file still FAIL
- Attach; wait-output `Flow 1. gate` (pane id **first**)
- Coord prompt: omptest Writes **exact PASS bytes** from phase-01; 0242 bans including `~/.omp` `agent.db`
- **Poll** file MATCH PASS (180s). Factory does not Write.
- Taxi 2 → exactly two `flow/result` codes `[1,0]`
- wait-output `Flow 0. gate`
- Leftover mint table (plan trap 23). `desk.rs` == HEAD. Repo `.dory/` unchanged. factory sock dead
- Copy journal to receipt **before** wipe. Stop = compound_stop

## STOP

- rust / leftover 5 / cargo leftover / factory `dory` argv / leftover ELF / isolate ELF on factory XDG
- start default / sit t13 / `herdr server stop` / `--wait` / `occ.report=Working`
- `git add -A` / fold leftover / claim company Phase 5

Write `/home/manhquy/Downloads/flow/dory/plans/reports/260827-ensure-aoe5-flow-judge-cook.md`
Reply `COOK_PASS` or `COOK_FAIL`.
