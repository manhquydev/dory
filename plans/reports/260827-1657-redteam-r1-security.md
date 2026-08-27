# RT Security + Fact Check — 260827-1657 isolate flow next unlock

Lens: Security Adversary + factory-chair bleed. Role: Fact Checker. Plan review only.
Did not exec paid judge. Did not invoke factory dory. Did not start default sock. Did not sit t13.

Scope: attach XDG, stop identity, setsid server, leftover ELF as SIT_DORY, factory FLOW_* inherit, ISO/bin, PI_CODING leak, sit t13, factory sock.

Scout: leftover `attach.rs` still `spawn_server`; HEAD `ensure_server` does not; `prepare_bind` unlinks a dead factory sock then binds; 0242 injects factory `PI_CODING_AGENT_DIR`; `pty.rs` never strips it; sit pane is a factory Herdr shell; land ELF sha from research-02 is not in the cook contract.

---

## Finding 1: Leftover ELF as SIT_DORY is path-prefix theater; dead factory sock is a spawn invitation
- **Severity:** Critical
- **Location:** plan.md Bẫy 8, 18, 23; phase-01-start.md Implementation Steps §1; research-02 §5 (dropped)
- **Flaw:** Leftover worktree `ensure_server` still `spawn_server()` when ping misses. HEAD `ensure_server` only pings and returns 1. `prepare_bind` treats a non-connectable factory sock as stale, unlinks it, and binds. Trap 18 then calls existence-without-connectable a pass. Phase 1 refuse of “leftover ELF” copies AOE5 path prefixes only (`rust/target`, `flock.6yaatuxg`, factory XDG, `~/.local/bin/dory`). Research-02 pins the intended binary at `land-4b70f79` sha `2ef20730…` ino `1065212`. This plan never hashes `SIT_DORY`. A leftover spawn ELF copied to `/tmp/dory`, a rewritten land tree, or any `aoe5.*` cache path is accepted. Leftover `main.rs` still advertises “Starts the server if needed.” Bare `dory` and `dory attach` both call `ensure_server`.
- **Failure scenario:** Cook exports leftover `rust/target/debug/dory` (spawn hunk `602479094e84d31ad6f017775a3d55aeb485c644`) to any path the case statement misses and passes it as `SIT_DORY`. Sit pane has factory `XDG_RUNTIME_DIR`. Factory sock is dead (the plan’s success signal). One `dory` / `dory attach` without isolate XDG unlinks `/run/user/…/dory/default/dory.sock` and mints the founder chair. Scripted `factory_must_dead` after the XDG-prefixed ATTACH_CMD does not see a later bare attach. End-of-script `connectable=0` already passed.
- **Evidence:** leftover `dory/rust/src/attach.rs:379-452` (`spawn_server`); leftover `dory/rust/src/main.rs:50,70-71`; `git show HEAD:rust/src/attach.rs:326-332` (no spawn); `git show HEAD:rust/src/desk.rs:165-167`; `git show HEAD:rust/src/socket.rs:111-116`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:433-445`; `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:65,75,85`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md:77`; `dory/plans/reports/260827-1656-research-02-isolate-next.md:134-148`
- **Suggested fix:** Hash-pin `SIT_DORY` to the land ELF (sha + ino). Refuse any ELF whose `ensure_server` is not HEAD no-spawn. After every dory argv and at teardown, scan `/proc/*/cmdline` + environ for leftover/isolate ELF whose `XDG_RUNTIME_DIR` equals `FACTORY_XDG`. Treat a stale factory sock plus leftover ELF argv as FAIL, not “existence OK.”

## Finding 2: “PI_CODING server-only” is false — occupants inherit factory `~/.omp/agent`
- **Severity:** Critical
- **Location:** plan.md Bẫy 15, 20; phase-01-start.md Architecture + steps 4, 5, 8
- **Flaw:** Step 4 orders 0242 `:346-353` verbatim. That line **sets** `PI_CODING_AGENT_DIR="$FACTORY_AGENT_DIR"` (`$FACTORY_HOME/.omp/agent`). 1910 `:277-278` **unsets** it. Land `pty.rs` copies `DORY_*` onto occupant PTYs and never removes `PI_CODING_AGENT_DIR`. No rust file mentions the var. Trap 15’s “server only” is a comment, not a boundary. Phase 1 step 5 says “Start coord + omptest on isolate (omp)” and drops AOE5 `start_omp` flags `--no-session --no-skills --no-rules --no-extensions`. Step 8 repeats a text ban. No snap of factory `~/.omp/agent` mtime/ino (1122 Finding 8, still open).
- **Failure scenario:** Isolate server starts with factory agent dir. Coord/omptest inherit it. Bare `omp` (skills on) Writes factory `agent.db` / sessions / skills while “writing” `$ISO_REAL/flow/00-idea.md`. Leftover-5 hashes and repo `.dory/` snap do not cover `~/.omp`. Cook receipt still COOK_PASS. Founder omp store is mutated.
- **Evidence:** `dory/scripts/dory-isolate-flock-prompt.sh:346-353`; `dory/scripts/dory-isolate-flow-sit.sh:277-278`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:588-595,635-636`; `dory/rust/src/pty.rs:187-197`; `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:72,77`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md:23,83-84`; `dory/plans/reports/260827-1122-rt-security.md:67-73`
- **Suggested fix:** Keep PI on the setsid line only if omp cannot start without it. Start occupants with the AOE5 `--no-session --no-skills --no-rules --no-extensions` argv. Snap factory `~/.omp/agent` ino/mtime at entry; FAIL on change. `rv_sit` must read occupant `/proc/<pid>/environ` and FAIL if `PI_CODING_AGENT_DIR` is factory.

## Finding 3: Sit pane is the factory chair — PATH/`XDG` never measured there
- **Severity:** High
- **Location:** plan.md Herdr sit tab + Bẫy 8, 19; phase-01-start.md steps 1, 7; phase-02-independent-next-unlock-test.md step 4
- **Flaw:** Sit is a factory Herdr shell (“shell sạch”). It keeps factory `XDG_RUNTIME_DIR` and factory `PATH`. Scripted ATTACH_CMD (1910 `:331`) prefixes isolate XDG and an abs `"$SIT_DORY"` **once**. `path_dory_empty` in AOE5 runs in the cook script’s shell (`type -a dory`). Phase 2 `type -a dory` empty is the test tab, not the sit pane. Phase 1 never `herdr pane send-text` a PATH/`XDG` probe. After detach, the sit pane is still a factory tty until phase 3 closes wave tabs. Leftover ELF on that PATH plus Finding 1’s stale-sock bind is the chair mint.
- **Failure scenario:** Cook pane PATH is sanitized. Sit tab is a login shell with `~/.local/bin` or leftover `rust/target/debug` on PATH. Cook send-texts the good ATTACH_CMD. Needles match. `factory_must_dead` passes. Operator or a retry types `dory` / `dory attach` in the same pane. Leftover `ensure_server` binds founder default. Phase 2 still prints TEST_PASS because it measured the test tab.
- **Evidence:** `dory/scripts/dory-isolate-flow-sit.sh:331`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:214-220,545,692`; `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:65,76,95-99`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md:77,86`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-02-independent-next-unlock-test.md:45`; leftover `dory/rust/src/attach.rs:429-451`; `git show HEAD:rust/src/socket.rs:90-95,111-116`
- **Suggested fix:** Before attach, from the sit pane: assert `echo "$XDG_RUNTIME_DIR"` is factory, `type -a dory` is empty, and no leftover/isolate ELF is on PATH. After attach, `factory_must_dead`. Close the sit pane before COOK_PASS, or keep probing factory sock until the tab is destroyed.

## Finding 4: ISO/bin PATH pin amplifies leftover ELF; 1910 had neither pin nor PI
- **Severity:** High
- **Location:** plan.md Bẫy 20; phase-01-start.md Architecture + step 4; research-02 §6
- **Flaw:** Plan copies 0242 ISO/bin + `PATH="$ISO_REAL/bin:$PATH"` onto the isolate server. `bin/dory` is a symlink to `SIT_DORY`. Occupants inherit that PATH. 1910 setsid has **no** ISO/bin and **unsets** PI. Research-02 admits 1910 children become command-not-found after factory PATH `dory` is gone, and copies ISO/bin so coord can type `dory …`. Combined with Finding 1 (SIT_DORY hash not pinned) and Finding 2 (factory PI), every occupant `dory` is leftover-spawn-capable and pointed at factory omp.
- **Failure scenario:** Accepted leftover ELF (Finding 1) is linked at `$ISO_REAL/bin/dory`. Coord prompt says “Run exactly: dory agent prompt omptest …”. Occupant `dory` is leftover. Occupant `dory attach` / bare `dory` calls leftover `ensure_server`. Isolate XDG keeps that spawn off the founder sock; occupant `PI_CODING_AGENT_DIR` still writes factory `~/.omp`. One env slip (`unset XDG_RUNTIME_DIR` / copy factory XDG) and leftover spawn hits founder default.
- **Evidence:** `dory/scripts/dory-isolate-flock-prompt.sh:340-352`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:582-595`; `dory/scripts/dory-isolate-flow-sit.sh:277-281`; `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:77`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md:23,83`; `dory/plans/reports/260827-1656-research-02-isolate-next.md:151-159`; leftover `dory/rust/src/attach.rs:379-413`
- **Suggested fix:** Keep ISO/bin → land-hash ELF only. After pin, `realpath` + sha must match the land pin. Occupant PATH must not suffix factory leftover `rust/target`. Do not treat 0242 PATH as “safer than 1910” without the hash pin.

## Finding 5: Trap “refuse factory FLOW_*” is two names; taxi inherit still judges factory KB
- **Severity:** High
- **Location:** plan.md Bẫy 4; phase-01-start.md taxi helper + step 1; phase-01 Risk “Factory FLOW_* inherit”
- **Flaw:** Trap 4 says refuse factory `FLOW_*` at entry. Step 1 and AOE5 law only test `FLOW_BIN` and `FLOW_PROJECT_ROOT`. `flow.sh` also reads `FLOW_GLOBAL_KB` (default `$HOME/.claude/flow/playbooks`, `mkdir` on promote), `FLOW_FORCE`, `FLOW_EVAL_MANIFEST`, `FLOW_HARNESS_*`. Taxi helper pins two vars and does not `env -u` the rest. `dory flow --` `invoke_flow` inherits the process env (`flow.rs` `Command::new(bin)`). 0242 setsid does not strip `FLOW_*`, so an exported factory `FLOW_GLOBAL_KB` lands on the isolate server and every occupant.
- **Failure scenario:** Factory pane exported `FLOW_GLOBAL_KB=/home/manhquy/.claude/flow/playbooks` (or `FLOW_FORCE=1`). Entry refuse passes. Cook taxi `next` with `HOME=$ISO_REAL/home` still uses factory playbooks. Occupant `dory flow -- promote|recall` (ISO/bin `dory` on PATH) writes factory KB. Journal `cwd` can still look isolate. Phase 1 risk signal “journal cwd not under ISO / args ≠ next” does not fire.
- **Evidence:** `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:61`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md:27-34,77,101`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:52-54,261-266`; `/home/manhquy/.claude/skills/flow/runner/flow.sh:23-31,503-506,635-638,1920-1926`; `git show HEAD:rust/src/flow.rs:160-167`; `dory/plans/reports/260827-1122-rt-security.md:35-41`
- **Suggested fix:** Refuse any factory `FLOW_*` at entry. Taxi and setsid: `env -u` the class, then pin only `FLOW_BIN` + `FLOW_PROJECT_ROOT` + `FLOW_LOG_DISABLE` + `DO_NOT_TRACK`. Assert factory `~/.claude/flow` mtime unchanged.

## Finding 6: Phase 2/3 sit-t13 and “ELF argv on factory XDG” have no measurement
- **Severity:** High
- **Location:** phase-02-independent-next-unlock-test.md steps 1–4; phase-03-review-and-ship.md `rv_sit`; plan.md Herdr + Bẫy 6
- **Flaw:** Paid law is exact `w13:t13` / `w13:p2R` / `*wP:*` on both `SIT_*` and `herdr pane get` `tab_id`, plus sit ≠ cook pane and no agent. Phase 1 lists those strings. Phase 2 step 1 does not. Phase 2 only “Mint test sit tab” and pass the three env vars. `rv_sit` accept-iff is “Sit ≠ t13” — the 1122 false-equality (`t13` vs `w13:t13`) restated as a lens name. “no factory `dory` argv” / “leftover-or-isolate ELF argv on factory XDG” still has no `/proc` recipe (1122 Finding 3). Land ELF is an isolate ELF; a slogan “refuse isolate ELF” either blocks the cook or matches nothing.
- **Failure scenario:** Test tab operator passes `SIT_TAB=w13:t13`. If the new script’s refuse is copied, it dies. If phase 2 is followed as written, send-text hits founder sit. Leftover attach (Finding 1) recycles or mints factory. Phase 3 “leave `t13`” cannot untype into it. Review marks `rv_sit` accept because the receipt says “sit ≠ t13.”
- **Evidence:** `dory/scripts/dory-isolate-flow-sit.sh:189-217`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:474-505`; `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:63,95`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-02-independent-next-unlock-test.md:33-46`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-03-review-and-ship.md:31,69-75`; `dory/plans/reports/260827-1122-rt-security.md:51-57`
- **Suggested fix:** Phase 2 independent asserts: `herdr pane get` `tab_id` ≠ `w13:t13`, pane ≠ `w13:p2R`/`*wP:*`, no agent. `rv_sit` copies those exact strings. `/proc` scan: leftover `…/dory/rust/target/debug/dory` or isolate `…/land-4b70f79/…/dory` with `XDG_RUNTIME_DIR=$FACTORY_XDG` is FAIL.

## Finding 7: `rv_sit` / `rv_next` cannot fail Findings 1–5
- **Severity:** High
- **Location:** phase-03-review-and-ship.md Review lenses
- **Flaw:** `rv_sit` accepts on sit ≠ t13, compound stop, factory sock dead, “no factory dory argv”, attach 1910 `:331`. `rv_next` accepts on journal bin/args/codes/stdout and “occupant wrote PASS.” Neither lens reads `SIT_DORY` sha, sit-pane PATH, occupant `PI_CODING` environ, factory `~/.omp` snap, or `/proc` ELF×XDG. `rv_left` passing leftover attach.rs mint (`60247909…` = spawn hunk) is treated as success. Review critical-0 is then a paper stamp on a live chair.
- **Failure scenario:** Cook copies 1910 attach and 0242 setsid. Journal is honest `[1,0]` `args=["next"]`. Sit needles match. Factory sock is dead. Leftover ELF is `SIT_DORY` via a path gap. Occupants inherit factory PI. Review four lenses ACCEPT. Founder omp and/or a later bare sit-pane `dory` mint the chair after REVIEW_ACCEPT.
- **Evidence:** `dory/plans/260827-1657-isolate-flow-next-unlock/phase-03-review-and-ship.md:25-32,65`; leftover `dory/rust/src/attach.rs:379-413` hash `602479094e84d31ad6f017775a3d55aeb485c644` (plan.md:82); `git show HEAD:rust/src/attach.rs:326-332`; `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:123`
- **Suggested fix:** Make `rv_sit` fail unless land ELF sha matches, sit-pane `type -a dory` is empty, occupant environ PI is not factory, factory `~/.omp/agent` snap is unchanged, and `/proc` has zero leftover/isolate ELF on `FACTORY_XDG`. Do not count leftover attach mint as safety.

## Finding 8: Stop identity is specified; leftover stop argv is not — `SIT_DORY` leftover still holds the gun
- **Severity:** Medium
- **Location:** plan.md Bẫy 7, Constraints; phase-01-start.md step 12
- **Flaw:** Compound stop body (1910 `:69-100` / AOE5 `:102-134`) is the right identity gate: ISO real, not factory XDG, not leftover isolate, sock not symlink, realpath == `ISO_SOCK`, under `ISO_REAL`, then `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`. The argv is still `"$SIT_DORY"`. If Finding 1 accepts leftover ELF, stop is leftover `server stop` (leftover `server.rs` is leftover-5 mint `4de1554a…`). Identity abort on symlink/mismatch is good. Identity pass + leftover binary is an unreviewed stop implementation. Land `connect_control` still lets nonempty `DORY_SOCKET` win; empty `DORY_SOCKET=` falls through to XDG default. Compound stop unsets it. A cook who “copies the one-liner” from Constraints without the gates (the 1122 hole) and uses leftover ELF still shoots factory if `ISO_REAL`/`sock` identity is wrong.
- **Failure scenario:** Cook implements Constraints’ one-liner and skips `iso_identity_ok` because phase-01 step 12 says “Stop = 1910 `compound_stop`” without pasting `:69-98`. `ISO_REAL/dory/default/dory.sock` is a symlink to founder sock, or `ISO_SOCK` was never set and a leftover ELF stop with leaked factory XDG hits default. Desk dies. Final `connectable=0` “passes.”
- **Evidence:** `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:21,64`; `dory/plans/260827-1657-isolate-flow-next-unlock/phase-01-start.md:19,88`; `dory/scripts/dory-isolate-flow-sit.sh:69-100`; `dory/scripts/dory-isolate-aoe5-flow-judge.sh:102-133`; `git show HEAD:rust/src/server.rs:1870-1881`; leftover mint table `dory/plans/260827-1657-isolate-flow-next-unlock/plan.md:83`
- **Suggested fix:** Paste `compound_stop` gates into phase-01 step 12, not the one-liner alone. `SIT_DORY` for stop must be the same land-hash ELF as start. After stop: factory sock still non-connectable AND isolate sock gone. Abort (do not stop) on identity fail.

---

## Plan status

Phases 1–3 Pending. Do not cook.

Findings 1–2 are chair-mint / factory-store. Findings 3–7 are the measurement gaps that let a copy of AOE5 law stamp PASS. Finding 8 is the 1122 stop hole if the cook copies the slogan instead of `:69-100`.

1122 Findings 1 and 3 were “fixed” on paper by citing 1910 `:331` and 0242 `:346-353`. They are not closed: leftover spawn ELF is still the worktree mint, sit-pane PATH/`XDG` is still unmeasured, PI is still factory-injected, and `SIT_DORY` is still unhashed.
