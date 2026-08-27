---
type: research
date: 2026-08-27
time: 11:16
kind: isolate-aoe5-project
pane: researcher-02
head: ee0350c
rust_land: b544f5f
isolate: land-4b70f79
default_sock: absent
path_dory: gone
scope: how to run a real project inside isolate Dory (occupants + coord prompt + taxi) without default occupancy, leftover-5 fold, or rust
---

# Research — AOE 5 isolate project (no rust, no leftover fold)

**Answer:** write a **new** isolate driver. Copy identity / stop / sit-pane law from 1910+0242. Do not exec those scripts. Mint a throwaway project **under the isolate**, start occupants, coord `prompt` (no `--wait`), taxi **real** `FLOW_BIN` (not `/bin/true`). Flow already glances. C no-spawn already landed. **AOE 5 does not need rust.**

Did not implement. Did not invoke `dory`. Did not start default sock. Did not sit t13. Did not exec leftover or isolate ELF. Did not recook 1910/0043/0227/0242.

## 1. Live table (this pane, 11:16)

| Item | Live | Note |
|---|---|---|
| **HEAD** | `ee0350cf9fece1ddc8494547e6176ceda57c50a1` `docs(plan): record no-spawn ship ops` | Paper over rust. |
| **rust land** | `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2` `fix(attach): do not auto-start server on sit` | `git log -1 -- rust/`. C no-spawn. `ensure_server` ping-miss → `Err(1)`, no spawn (`git show HEAD:rust/src/attach.rs:326-332`). |
| **isolate HEAD** | `b544f5f` (detached) | `/home/manhquy/.cache/dory-isolates/land-4b70f79`. Same rust land. Not paper `ee0350c`. |
| **leftover 5 mint** | **MATCH** ×5 | porcelain ` M`, unstaged. Do not fold. |
| README | `68190a5ffa073c082aa318aad5ed032e13cc90e3` MATCH | |
| attach | `602479094e84d31ad6f017775a3d55aeb485c644` MATCH | |
| main | `373d688636ff7315ccd665f450069d8284eb47ff` MATCH | |
| server | `4de1554ad56e248cdcf42f02111b7389b08dae82` MATCH | working ≠ `HEAD:rust/src/server.rs` `dfca2ac5…`. Leftover on purpose. **Not land.** |
| p5_attach | `9c28fc3e0f3666498a8952411242d5301f7911de` MATCH | |
| **sock connectable** | **False** FileNotFound | `/run/user/1000/dory/default/dory.sock` lexists=0. Python AF_UNIX 1s. Did not start it. |
| **PATH dory** | **gone** COUNT=0 | `type -a` not found. `~/.local/bin/dory` lexists=0 this turn. |
| **isolate ELF / SIT_DORY** | `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` | ELF 64, ino=1065212 size=18474184 mtime=10:56. **Stat only. Do not exec.** |
| **factory leftover ELF** | `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory` | ino=2490742 size=18568240. 1910 refuses this path (`scripts/dory-isolate-flow-sit.sh:182-186`). Do not exec. Do not cargo. |
| **w13 tabs** | **`w13:t13` only** | `herdr tab list`: this cook `w13:p2R` agent=cursor focused. Other ws (`wP:t16` …) exist. **Sit pane ≠ t13.** Close only a **new** wave tab. |

## 2. What 1910/0043/0227/0242 already paid vs AOE 5 hole

North star AOE 5 = *real project completed inside Dory; Flow judges; zero `herdr`/`dsh` on Dory runtime* (`plans/reports/260822-north-star-aoe.md:33`). Founder: đàn + giao việc, then chiếu biên lai; sit default trống ≠ sâu (`plans/reports/260825-2105-brainstorm-herdr-depth-founder.md:28-31,43-45`). Two chairs: factory Herdr *builds*; shipped Dory never calls `herdr`/`dsh` (`CHARTER.md:43-44,61`; `CAPACITY-FREEZE.md:18-19,29`).

| Receipt | Paid (isolate only) | Not paid |
|---|---|---|
| **1910** sit | Isolate mint + taxi + sit attach + footer `Flow 0. $TOKEN`. `JOURNAL_OK=1`. Factory sock dead. | Occupants. Coord. **AOE 5.** `FLOW_BIN=/bin/true` (`scripts/dory-isolate-flow-sit.sh:308-309`). |
| **0043** roster | Four names on Agents. Sibling `workspace list` while **unknown**. | Ready words. Report-then-prompt. AOE 5. |
| **0227** report | `omptest/omprev/groktry done` + `coord` not `unknown`. | Sibling (stall). AOE 5. |
| **0242** prompt | Ready words **+** sibling after `report idle`. `prompt --timeout` not `--wait`. rust land then `5a60953`; now under `b544f5f`. | `--wait`. Default. **AOE 5.** |

Eval hole still open (`plans/reports/260827-1012-eval-aoe.md:29,52-56`): `/bin/true` taxi + sibling `workspace list` ≠ a project Flow judged. 0242 closed stall. Not AOE 5. C no-spawn (`b544f5f`) closed attach spawn. Not AOE 5.

**AOE 5 hole (only unpaid product increment here):** occupants write a **real artifact** in a **non-leftover, non-default** project cwd; coord prompts that work; taxi **real** `flow.sh`; journal `flow/result` names that binary; glance already paints it. Factory sock stays dead. Leftover 5 stay ` M`.

## 3. New-script-only method

Do **not** exec or source 1910/0043/0227/0242/hop. New file. Copy law, not bodies.

### COPY (identity, stop, sit pane)

From 1910 + 0242 (same compound):

| Law | Cite | Copy as |
|---|---|---|
| `HERDR_ENV=1`; refuse dirty `DORY_*` / `PI_CODING_AGENT_DIR` on **factory** | 1910 `:25-35`; 0242 `:31-47` | Factory chair stays clean. |
| ISO identity: real dir, not symlink, not `FACTORY_XDG`, not leftover `flock.6yaatuxg`, not `/tmp` | 1910 `:56-67`; 0242 `:82-93` | Mint `$CACHE/proj.XXXXXX` (new prefix). |
| **Stop:** `XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET "$SIT_DORY" server stop` | 1910 `:98-99`; 0242 `:124-125` | Ban `iso()` / `DORY_SOCKET=` on stop. |
| SIT_DORY ≠ `~/.local/bin/dory`, ≠ factory `rust/target/*` | 1910 `:177-186`; 0242 `:211-219` | Pin isolate ELF path above. Do not cargo. |
| Sit pane ≠ cook pane, ≠ `w13:p2R`, ≠ `wP`, ≠ `w13:t13`; no agent; split if dirty | 1910 `:189-224`; 0242 `:230-270` | Mint a **new** w13 (or other) tab. This cook is t13. |
| Factory sock not connectable throughout | 1910 `:120-125`; 0242 `:146-151` | Python AF_UNIX. Never start default. |
| Leftover hash snap + fail if changed | 0242 `:162-166,546-548` | Mint, do not fold. |
| Isolate `HOME=$ISO_REAL/home`; `PATH=$ISO_REAL/bin:$PATH` → `ln -sfn "$SIT_DORY"` | 0242 `:340-344` | Occupants see isolate `dory`, not factory argv. |
| Isolate **server only:** `PI_CODING_AGENT_DIR=$FACTORY_HOME/.omp/agent` | 0242 `:346-352` | Factory flock MAY set this **only** on isolate server. Unset on factory. 1910 unsets it (`:277-278`) because no occupants. AOE 5 needs 0242's set. |
| Attach via factory herdr `send-text` + `enter` + `wait-output` | 1910 `:336-346`; 0242 `:462-463` | See §5. |
| Coord prompt: no `--wait`; inner text forbids `--wait` / bare `dory` / `server stop` / herdr | 0242 `:500-504` | Keep. Change the **work** text. |
| Wipe isolate on teardown | 1910 `:103-118` | Project lives **under** ISO so wipe is one tree. |

### NEW (the hole)

| New | Why |
|---|---|
| **Project dir** `$ISO_REAL/hello` (or sibling under ISO). Not factory `…/flow/dory`. Not leftover tree. Not default cwd. | Occupant writes + taxi cwd = world-state. |
| **`DORY_WORKSPACE_DIR=$PROJECT`** on taxi (1910 used `$ISO_REAL` `:307-309`). | Journal at `$PROJECT/.dory/sessions/s1.jsonl` (`git show HEAD:rust/src/flow.rs:105-118`). |
| **`FLOW_BIN` = real flow-skill** `/home/manhquy/.claude/skills/flow/runner/flow.sh` (exists, executable). | 1910 `/bin/true` is chrome, not a judge (`flow.rs:39,87-100`). |
| Taxi argv: `doctor` or `status` — **never** `next`/`card`/`check` (Dory has no those buttons; `flow.rs:3`). | Flow stays foreign (`CHARTER.md:30-32`). |
| Occupant work = **write a file** in `$PROJECT`, then report idle. Coord `prompt` drives that. | Sibling `workspace list` already paid. That is not a project. |
| Close **only** the wave sit tab. Never t13 / p2R / wP. | Two chairs. |
| Do not recook paid scripts. Do not cargo. Do not exec leftover ELF. | Traps §7. |

Sketch (new script only; not to run here):

```
mint ISO + PROJECT under ISO
SIT_DORY=<isolate ELF>   # not factory rust/target, not PATH dory
setsid … HOME=ISO/home PATH=ISO/bin:… PI_CODING_AGENT_DIR=$FACTORY_HOME/.omp/agent \
  XDG_RUNTIME_DIR=$ISO_REAL  $SIT_DORY server     # isolate XDG only
split + agent start coord/omptest (0242 roster subset is enough for first slice)
report idle → sit attach (send-text, not pane run)
coord prompt: omptest writes $PROJECT/hello.sh with token, then report idle
cd $PROJECT && DORY_SOCKET=$ISO_SOCK DORY_ENV=1 DORY_WORKSPACE_DIR=$PROJECT \
  FLOW_BIN=/home/manhquy/.claude/skills/flow/runner/flow.sh \
  $SIT_DORY flow -- doctor
assert: hello.sh on disk; journal flow/result.bin is real flow.sh not /bin/true
XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET $SIT_DORY server stop
herdr tab close $WAVE_TAB only
```

## 4. Occupant allowlist — how they write a file

HEAD land (`git show HEAD:rust/src/server.rs:1208-1209`):

```
fn comm_allowlisted(comm: &str) -> bool {
    matches!(comm, "sleep" | "cat" | "sh" | "bash" | "true" | "false")
}
```

`omp` **absent**. This list is **status classify**, not a write ACL.

`classify_word` (`:1245-1288` HEAD): if `occ.report` set → that word (`idle`+!seen → `done`). Else `comm` must be allowlisted **and** appear in descendant comms; else **`unknown`**. `sleep|true|false` → `working`. `sh|bash|cat` + ready needle → idle/done.

So:

- **0242 occupants are `omp` / `grok`.** argv0 comm is not allowlisted → without `agent report` they stay `unknown`. That is paid and fine.
- **They write files with omp/grok tools** (Write, or omp-spawned `sh`/`bash`). Occupant comm stays `omp`. Child `sh` writing `hello.sh` does not need `omp` on the allowlist.
- A **shell occupant** (`agent start -- bash`) *could* `cat > file` because `cat`/`bash` are allowlisted — that is not the đàn. Do not start shell occupants and call it AOE 5.
- **Do not add `omp` to the allowlist.** That would classify the agent as a shell and break report words.

World-state proof = file bytes on disk under `$PROJECT`, not a green Agents word.

## 5. herdr `pane run` vs `send-text`; sit ≠ t13; close wave only

| Move | Use? | Why |
|---|---|---|
| `herdr pane send-text` + `send-keys enter` + `wait-output` | **Yes** (paid) | Types attach into an empty factory shell. Attach is a long-lived TUI. 1910 `:336-346`. |
| `herdr pane run <PANE> <CMD>…` | **No** for attach | Factory herdr can `run` a command in a pane (`herdr pane run --help`). Attach does not exit. `run` replaces/waits the pane process. Risk: exec isolate ELF on the **factory** pane with factory XDG. Ban. |
| Sit on `w13:t13` / `w13:p2R` | **No** | This cook. 1910 `:192-199`. |
| `herdr tab close` factory t13 / wP | **No** | Close **only** the wave tab the new script minted. |

Factory herdr **drives** sit (hai ghế). Dory runtime path must not call `herdr`/`dsh` (`flow.rs:133-146` refuses those names as `FLOW_BIN` / argv). Occupant prompts must not run `herdr`.

## 6. First-slice project (recommended)

**`hello-receipt` under the isolate.** Tiny. Completable in one isolate sit. Not leftover dory. Not default.

| Field | Pick |
|---|---|
| Path | `$ISO_REAL/hello/` (wiped with ISO) |
| Artifact | `hello.sh` — `#!/bin/sh` + `echo "$TOKEN"` (coord gives token) |
| Who writes | `omptest` via coord `prompt` (omp tools). Coord reports idle after. |
| Judge | `dory flow -- doctor` with **real** `FLOW_BIN=…/flow/runner/flow.sh` from `$PROJECT`. Journal must show that path + a code. Glance already reads last `flow/result` (`HEAD:rust/src/desk.rs` `refresh_flow_glance` / `poll_flow_glance` / `footer_line`). |
| Not this slice | Full flow card cycle. `next`/`card`/`check`. Default sock. Leftover tree as cwd. Four-name roster (coord+omptest enough). `--wait`. Rust. Cargo. |

Ranked alternatives (do not pick these first):

| Rank | Idea | Why lose |
|---|---|---|
| 1 | **hello-receipt** (above) | Fits isolate + paid glance + real FLOW_BIN. |
| 2 | Pre-seed a full `flow/` + one card | Realer judge, not tiny; freeze says flow-skill maintenance; easy to launder as AOE 5. |
| 3 | Reuse leftover dory as project | Folds leftover 5 / cargo trap. Ban. |
| 4 | Sit default + PATH dory | Sock absent; C no-spawn fail-closed; factory chair. Ban. |

## 7. Traps

1. **Leftover cargo** — factory `rust/target/debug/dory` exists. 1910 refuses it as `SIT_DORY`. Isolate ELF already built. Do not cargo. Do not fold leftover 5.
2. **Factory dory argv** — PATH name gone. `~/.local/bin/dory` absent. Any bare `dory` is a miss or a future retarget. Always `"$SIT_DORY"`. Isolate `bin/dory` → isolate ELF.
3. **Start default** — sock FileNotFound. Starting it is not AOE 5. Ban.
4. **`FLOW_BIN=/bin/true` called AOE 5** — 1910 paid chrome. Eval: not a judge (`1012-eval-aoe.md:29,73`).
5. **Recook paid scripts** — 0242 already has occupants+prompt. Hole is project cwd + real FLOW_BIN. New script.
6. **`prompt --wait` / `occ.report=Working`** — founder named `--wait`; 0927 rejected Working as cook. 0242 uses `--timeout`, then report. Do not reopen.
7. **Fold leftover 5** — hashes MATCH. Stay ` M`.
8. **`iso()` / `DORY_SOCKET=` on `server stop`** — stop abort law. XDG + `env -u DORY_SOCKET` only.
9. **`herdr pane run` attach / exec isolate ELF on factory XDG.**
10. **Sit t13 / close wP.** This cook is t13. Wave tab only.
11. **Cite leftover `server.rs` as land.** Working `4de1554a` ≠ HEAD `dfca2ac5`. Land = `b544f5f` blobs via `git show HEAD:`.

## 8. Does AOE 5 need rust?

**No.**

- **C already landed.** `ensure_server` no-spawn on HEAD rust `b544f5f` (`attach.rs:326-332`). Paper `ee0350c` only records it. Isolate worktree already at `b544f5f` with ELF on disk.
- **Glance already exists.** Desk polls journal `flow/result` and paints footer. 1910 already proved chrome with `/bin/true`. AOE 5 is a **real** `FLOW_BIN` + a **real** file, not a new paint.
- Occupants + coord prompt already isolate-paid (0242). Allowlist does not block writes (§4).
- Rust would be needed only if taxi/glance/occupancy were missing. They are not.

Cooking rust now is the wrong chair: leftover dirty, cargo trap, recook risk.

## Trade-off (ranked)

| Option | Completes AOE 5 hole | Complexity | Leftover risk | Verdict |
|---|---|---|---|---|
| **New isolate script** (copy law, new project+FLOW_BIN) | Yes | Low | Low if hash-snap | **Do this** |
| Recook 1910/0242 | No (same `/bin/true` or same list) | Wasted | High | Ban |
| Rust glance / allowlist / `--wait` | No | High | Folds leftover | Ban |
| Default sit | No | Looks busy | Starts sock | Ban |

## Sources

- `scripts/dory-isolate-flow-sit.sh` (1910)
- `scripts/dory-isolate-flock-roster.sh` (0043)
- `scripts/dory-isolate-flock-report.sh` (0227)
- `scripts/dory-isolate-flock-prompt.sh` (0242)
- `plans/reports/260827-1012-eval-aoe.md`
- `plans/reports/260825-2105-brainstorm-herdr-depth-founder.md`
- `plans/reports/260822-north-star-aoe.md`
- `CHARTER.md` `CAPACITY-FREEZE.md`
- `git show HEAD:rust/src/{server,attach,flow,desk}.rs` — not leftover working tree
- Live: `git log -1`, `git log -1 -- rust/`, `git hash-object` leftover 5, python sock, `herdr tab list`, isolate/factory ELF `stat`

## Unresolved

- `flow.sh doctor` exit code on a dir with only `hello.sh` (no `flow/`) — not invoked this pane. First cook must record live `flow/result.code` + `bin` path.
- Whether glance footer shows `doctor` the same as 1910 `Flow 0. $TOKEN` — 1910 used `flow -- $TOKEN` with `/bin/true`. Real `doctor` stdout shape unproven.
- Isolate ELF spawn-strings not re-`strings`'d this pane (C already receipted). Do not exec to check.
- `herdr pane run` exact wait/replace semantics not exercised (help only).

## Next (not this pane)

One new isolate script. Pin `SIT_DORY` to land-4b70f79 ELF. New wave tab. `$ISO_REAL/hello`. Real `FLOW_BIN`. Coord prompt writes `hello.sh`. Taxi `flow -- doctor`. Stop law. Close wave tab. Leftover 5 MATCH after. Factory sock still dead.
