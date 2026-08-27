---
type: research
date: 2026-08-27
time: 10:27
topic: land rust C without reminting leftover 5
did_not: invoke dory; cargo leftover tree; exec leftover/isolate ELF; start default sock; retarget PATH; implement
---

# Research Report: leftover 5 vs increment C land

## Executive Summary

Leftover 5 WT `git hash-object` **MATCH** researcher-02 mint. Leftover ELF sha256 unchanged (`3ba0e3bc…`). PATH name `dory` still gone. Default sock still absent (`FileNotFoundError`). Isolate worktree `land-4b70f79` is clean, detached at rust land `5a60953`, attach blob == `git show HEAD:rust/src/attach.rs` (`cf00a2fa…`).

Land C the same way 0130 / 0242 landed rust: **copy-aside leftover bytes → `git checkout HEAD --` the land file → hunk on HEAD blob → commit those paths only → restore leftover bytes → `git reset --hard` + `cargo` only in isolate worktree.** File for C is `rust/src/attach.rs` (USAGE optionally `rust/src/main.rs`). Never leftover working `server.rs` as land.

**Ranked choice:** isolate cargo at `/home/manhquy/.cache/dory-isolates/land-4b70f79` (`--manifest-path rust/Cargo.toml`). Reject leftover-tree cargo. Reject in-place leftover `attach.rs` edit. Reject `ln -sfn` isolate onto `~/.local/bin/dory`.

## Research Methodology

- Sources: live `git hash-object` / `sha256sum` / `stat` / `type` / AF_UNIX FileNotFound (no exec); researcher-02 mint; 0130/0242 plan+phase+cook; 0940 unlink receipts; 1012/1020 C notes
- Date range: 2026-08-26..27
- Terms: leftover 5, copy-aside, land-4b70f79, ensure_server, 60247909

## Live re-measure (2026-08-27 ~10:28, leftover tree, no cargo)

### Leftover 5 vs researcher-02 mint

Mint: `plans/260827-0940-unlink-leftover-path-dory/research/researcher-02-leftover.md:9-13`. Same SHAs in `plans/reports/260827-isolate-flock-prompt-leftover.snap:2-6` and keep `~/.cache/dory-isolates/leftover-keep-0130/`.

| File | live `git hash-object` | mint | |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5f…` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `60247909…` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d6886…` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554a…` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e…` | MATCH |

Porcelain: ` M` those five only. Cached empty. `desk.rs` WT == HEAD `4c788562e4fdda10c8edd2878ed1fdd46050c218`. Keep dir `cmp` == leftover WT for all five.

Paper HEAD `53f3cff` `docs(plan): record leftover PATH dory unlink`. Rust land still `5a60953` (`git log -1 -- rust/` ; `git diff 5a60953 HEAD -- rust/` empty).

### HEAD blobs (land cite — `git show HEAD`, never leftover WT)

| Path | `git rev-parse HEAD:…` |
|---|---|
| `rust/src/attach.rs` | `cf00a2faa1c622734bad53beb41a7c871ef77ec0` |
| `rust/src/main.rs` | `2fd5b78bdd46878879bb9519d8cb683ed34af6f2` |
| `rust/src/server.rs` | `dfca2ac5010e8b659e6e7e64889f06ff60ae2391` |
| `rust/tests/p5_attach.rs` | `768757060c8725ef8c71c6fd4228201bce257257` |
| `README.md` | `d13d8acca494e6ec4bd825e0631ef12d4aee48db` |

C hunk cite:

- `git show HEAD:rust/src/attach.rs:135-138` — `sit` → `ensure_server`
- `git show HEAD:rust/src/attach.rs:332-367` — ping miss → `Command::new(current_exe()).arg("server")` + `setsid` + `spawn()`
- `git show HEAD:rust/src/attach.rs:369-372` — `ping` = `rpc_line_quiet`
- `git show HEAD:rust/src/main.rs:50` — USAGE “Starts the server if needed”
- `git show HEAD:rust/src/server.rs:1501` — `occ.report = None` (0242; **not** leftover WT `server.rs`)

Leftover WT `server.rs:929` `fn workspace_live_cwd` is **not land**. Leftover WT `attach.rs:379` `ensure_server` (recycle / `ping_ok_abi` / `spawn_server`) is **not land**.

WT vs HEAD leftover 5: 409/49 insert/delete. Leftover `attach.rs` 645 lines vs HEAD 474. Leftover `attach.rs` `workspace_live_cwd` count 0. Leftover `server.rs` has `workspace_live_cwd` ×2 at `:929` `:976`. HEAD `server.rs` count 0.

### Leftover ELF (observe; not exec'd)

| field | value |
|---|---|
| path | `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory` |
| sha256 | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| size | `18568240` |
| inode | `2490742` |
| mtime_epoch | `1787716801` |
| mtime | `2026-08-26 11:00:01.084955950 +0700` |
| `strings` `workspace_live_cwd` | 59 |

Unchanged vs 0940 cook (`plans/reports/260827-unlink-path-leftover-cook.md:57-61`). C on HEAD rust does **not** rewrite this inode.

### Isolate `land-4b70f79` (observe; not exec'd)

| field | value |
|---|---|
| worktree | `/home/manhquy/.cache/dory-isolates/land-4b70f79` |
| `.git` | `gitdir: …/dory/.git/worktrees/land-4b70f79` |
| `git worktree list` | factory `53f3cff [main]`; isolate `5a60953 (detached HEAD)` |
| porcelain | empty |
| isolate `attach.rs` hash | `cf00a2fa…` == `HEAD:rust/src/attach.rs` |
| `Cargo.toml` | `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml` |
| ELF | `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` |
| ELF sha256 | `dc0e867acc5ad43db5c6e4cbb32ed670819b60fe6741c9b1201ae4e1473e6608` |
| ELF size / inode / mtime | `18493816` / `1063590` / `2026-08-27 02:47:14` (0242 rebuild) |
| `strings` `workspace_live_cwd` | 0 |
| SIT_DORY pin | `plans/reports/260827-isolate-flock-prompt-dory.txt` = that ELF |

Pre-C isolate ELF still has HEAD `ensure_server` spawn. After C commit: `reset --hard NEWHEAD` then cargo **here**. Do not exec this ELF on factory XDG.

### PATH / sock / env (still)

| check | live |
|---|---|
| `hash -r`; `type -a dory` | `type: dory: not found` TYPE_EXIT=1 |
| `command -v dory` | empty |
| `~/.local/bin/dory` | absent (`-e`/`-L` fail) |
| PATH walk name `dory` | COUNT=0 |
| sock | `$XDG_RUNTIME_DIR/dory/default/dory.sock` = `/run/user/1000/dory/default/dory.sock` |
| lexists / exists | False / False |
| connectable | False (`FileNotFoundError`; 1s AF_UNIX; no start) |
| `DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` | UNSET |

PATH still gone. Sock still absent. Do not retarget. Do not start default.

## How 0242 / 0130 cooked rust

Both: isolate worktree cargo + leftover copy-aside. Not leftover-tree cargo.

### 0130 (`desk_tree` cwd)

Cite: `plans/260827-0130-land-desk-tree-cwd/plan.md:38-49` `:81-96`; `phase-01-start.md:26-53`; cook `plans/reports/260827-land-desk-tree-cwd-cook.md:14-24`.

```
KEEP=~/.cache/dory-isolates/leftover-keep-0130
cp leftover 5 + hash-object == mint
cp rust/src/server.rs $KEEP/server.rs
git checkout HEAD -- rust/src/server.rs
# hunk on HEAD blob only ("cwd": world.cwd). no workspace_live_cwd
git add rust/src/server.rs   # only
git commit
cp $KEEP/server.rs rust/src/server.rs
hash leftover 5 == mint
# worktree:
git -C ~/.cache/dory-isolates/land-4b70f79 reset --hard NEWHEAD
cargo build --manifest-path rust/Cargo.toml   # cwd = land-4b70f79
```

Cook: commit `f6614d9`; leftover `server.rs` restored; cargo only worktree.

### 0242 (`occ.report = None`)

Cite: `plans/260827-0242-clear-report-on-prompt/plan.md:21` `:40-48`; `phase-01-start.md:24-38` `:72`; cook `plans/reports/260827-isolate-flock-prompt-cook.md:20-22`.

Same keep (`leftover-keep-0130`; re-hash). Checkout HEAD `server.rs`. Hunk `occ.report = None` before stall (`git show HEAD:rust/src/server.rs:1498-1504`). New test `rust/tests/p5_prompt_after_report.rs` (not leftover `p5_attach`). Commit those paths. Restore leftover `server.rs` → hash `4de1554a…`. Worktree `reset --hard 5a60953`. `cargo test` + `cargo build` **only in worktree**.

0240 research named the method: `plans/reports/260827-0240-research-next-scope.md:35` `:62`.

## Exact leftover `attach.rs` restore (WT stays `60247909`)

C land file = HEAD `attach.rs`. Same increment may also drop USAGE on HEAD `main.rs:50` (`plans/reports/260827-1020-brainstorm-eval-team.md:119-124`; `plans/reports/260827-1012-eval-next.md:59-61`). Leftover `main.rs` then needs the same copy-aside/restore.

```
KEEP=~/.cache/dory-isolates/leftover-keep-0130
# 0. refuse if leftover 5 hash-object ≠ mint. do not delete KEEP on abort.
# 1. re-copy leftover 5 into KEEP (already mint; re-hash KEEP == WT == researcher-02)
cp README.md rust/src/attach.rs rust/src/main.rs rust/src/server.rs rust/tests/p5_attach.rs \
   $KEEP/   # names in KEEP: README.md attach.rs main.rs server.rs p5_attach.rs
# 2. checkout HEAD blobs only (WT attach becomes cf00a2fa…)
git checkout HEAD -- rust/src/attach.rs
# if USAGE in same commit:
git checkout HEAD -- rust/src/main.rs
# 3. hunk only on those HEAD blobs:
#    git show HEAD:rust/src/attach.rs:332-367  ping miss → Err; no spawn
#    git show HEAD:rust/src/attach.rs:135-138  sit still calls ensure_server
#    git show HEAD:rust/src/main.rs:50         drop auto-start sentence
#    optional new test (not leftover p5_attach.rs)
# 4. git add only those HEAD paths (+ new test). never leftover 5. never desk.rs.
git commit
# 5. restore leftover bytes IMMEDIATELY
cp $KEEP/attach.rs rust/src/attach.rs
# if main was checked out:
cp $KEEP/main.rs rust/src/main.rs
# 6. gate
git hash-object rust/src/attach.rs
# must be 602479094e84d31ad6f017775a3d55aeb485c644
# leftover 5 all MATCH researcher-02. porcelain M those five. cached empty.
# 7. isolate only
git -C /home/manhquy/.cache/dory-isolates/land-4b70f79 reset --hard NEWHEAD
# cwd that worktree:
cargo test --manifest-path rust/Cargo.toml
cargo build --manifest-path rust/Cargo.toml
# SIT_DORY=.../land-4b70f79/rust/target/debug/dory
```

Abort between checkout and restore: `cp $KEEP/attach.rs rust/src/attach.rs` (and `main.rs` if checked out). Keep dir stays. Signal of miss: attach hash ≠ `60247909…`.

Factory leftover tree after land: working `attach.rs` is leftover again (intentional). `git show HEAD:rust/src/attach.rs` is the C blob.

## Files C must NOT edit (leftover working copies)

Do not open-and-save these leftover WT files. Do not remint. Checkout HEAD + restore is the only legal touch.

| Leftover WT | mint | why |
|---|---|---|
| `README.md` | `68190a5f…` | Now rewrite remints. 1020/0940: leftover-README receipt only. |
| `rust/src/attach.rs` | `60247909…` | leftover recycle/`spawn_server` at `:379`. Not land. |
| `rust/src/main.rs` | `373d6886…` | leftover USAGE + `DORY_SKIP_ONBOARD`. Checkout HEAD if USAGE lands, then restore. |
| `rust/src/server.rs` | `4de1554a…` | leftover `workspace_live_cwd`. **Never land. Never cite.** |
| `rust/tests/p5_attach.rs` | `9c28fc3e…` | leftover test. New test = new path (0242 pattern). |

Also do not edit clean `desk.rs` (`4c788562…`). Do not `git add` leftover 5.

## PATH still gone? Sock still absent?

**Yes / yes.** See live table. 0940 unlink held. Do not `ln -sfn` isolate `land-4b70f79` onto `~/.local/bin/dory` (`plans/260827-0940-unlink-leftover-path-dory/plan.md:38`; 0927/1020 reject retarget).

## Trap list — cook edits leftover working `attach.rs` in place

1. **Remint `60247909`.** Leftover 5 mint breaks. 0130 trap 4; 0242 trap 4.
2. **Wrong ABI.** Leftover `ensure_server` is `:379` (`ping_ok_abi` / recycle / `spawn_server` `:429`). HEAD land is `:332-367` (`ping()` then inline `Command::new`). In-place leftover hunk is not C.
3. **`git add` leftover `attach.rs` folds leftover** (+223 lines vs HEAD: recycle, ABI, `DORY_RECYCLE`). Land must be `git show HEAD` blob after checkout.
4. **Sit line numbers.** Leftover `sit` `:145`. HEAD `sit` `:135-138`. Cite leftover `:379` as land = 0940 review reject.
5. **Keep vs WT diverge.** Keep stays `60247909`. Restore keep → lose in-place edit. Skip restore → mint gone.
6. **Checkout HEAD after in-place leftover edit, no keep** → leftover attach **gone**. 0130 R2 trap 1.
7. **`cargo` leftover tree** `/home/manhquy/Downloads/flow/dory` rebuilds leftover ELF `3ba0e3bc…` (59× `workspace_live_cwd`, own `ensure_server`). 0130 trap 3; 0242 trap 3. C does not rewrite that ELF.
8. **Exec leftover ELF or isolate ELF on factory XDG** → mint `/run/user/1000/dory/default` + leftover live-cwd paint. 1020 C fail-first. Observe only.
9. **Retarget PATH** after A → “correct” name still hits `ensure_server` until C is **in the exec’d inode**. Rejected.
10. **Cite leftover working `server.rs` as land** (`:929` live-cwd). Forbidden. 0242 land is `git show HEAD:rust/src/server.rs:1501`.
11. **Stage leftover 5 / README / `p5_attach` / leftover `main.rs`.** Fold. USAGE = checkout HEAD `main.rs`, commit, restore `373d6886`.
12. **Worktree stays `5a60953`.** SIT_DORY ELF `dc0e867a…` still spawns. Must `reset --hard NEWHEAD` then cargo isolate.
13. **Crash mid-checkout.** Restore from KEEP immediately. Do not delete KEEP (`plans/260827-0130-land-desk-tree-cwd/plan.md:108`).
14. **`dory` / `dory attach` / `dory server` in factory.** Starts default. Cook never invokes.
15. **New test overwrites leftover `p5_attach.rs`.** Remints `9c28fc3e`. Use a new path.

## Comparative Analysis

| Option | Leftover 5 | Default sock | PATH | Cargo | Rank |
|---|---|---|---|---|---|
| **A. 0130/0242: copy-aside + HEAD checkout + isolate cargo** | mint holds | stays absent if no factory exec | stays gone | isolate only | **1 — do this** |
| B. Edit leftover WT `attach.rs` in place, commit | remints `60247909`; may fold leftover ABI | risk if leftover ELF rebuilt + exec’d | n/a | leftover cargo likely | reject |
| C. `cargo` leftover tree | ELF `3ba0e3bc` mutates | leftover ELF still spawns | n/a | leftover | reject |
| D. `ln -sfn` isolate → `~/.local/bin/dory` | mint | pre-C isolate ELF still `ensure_server` → default | retarget | n/a | reject (0940 trap 1) |
| E. Cook only in isolate, never restore leftover | isolate ok; factory leftover untouched **only if factory files never checked out** | ok if no factory exec | ok | isolate | 2 — cargo ok; **commit still needs factory HEAD checkout + restore** or worktree commit + merge without touching leftover 5 |

E without factory checkout can `git -C land-4b70f79` edit (already HEAD attach) and commit on a branch, then merge pathspec **only** `rust/src/attach.rs` (+ USAGE/test) onto factory `main` **without** checking out leftover files. Still: do not `git merge` a tree that replaces leftover WT. Safest paid path is still A (0130), because factory leftover stays dirty and merge tools will try to touch it.

## Implementation Recommendations

### Ranked choice

1. **0130/0242 copy-aside on leftover `attach.rs` (`60247909`) + isolate cargo `land-4b70f79`.** Same keep dir. Cite `git show HEAD` only.
2. Isolate-only commit + pathspec merge — only if cook never checks out leftover 5 on factory. Higher merge-conflict risk. Not how 0130/0242 paid.
3. Everything else — reject.

### Isolate cargo path (recommended)

```
/home/manhquy/.cache/dory-isolates/land-4b70f79
```

Commands (after factory land + leftover restore):

```
git -C /home/manhquy/.cache/dory-isolates/land-4b70f79 reset --hard NEWHEAD
cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml
cargo build --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml
```

SIT_DORY after rebuild: `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory`.

Do not: `cargo` in `/home/manhquy/Downloads/flow/dory`. Do not retarget PATH. Do not start `/run/user/1000/dory/default`. Do not exec leftover ELF `3ba0e3bc…` or current isolate ELF `dc0e867a…` on factory XDG.

### Adoption / fit

Method is paid twice (0130 `f6614d9`, 0242 `5a60953`). Keep dir already holds mint copies. Isolate worktree already mapped. C file swap: `server.rs` → `attach.rs`. Low process risk. Product risk: leftover ELF / isolate ELF still spawn until those inodes are rebuilt **and** not exec’d on factory XDG. C does not close leftover-ELF door.

## Resources

- Mint: `plans/260827-0940-unlink-leftover-path-dory/research/researcher-02-leftover.md`
- 0130: `plans/260827-0130-land-desk-tree-cwd/plan.md` + `phase-01-start.md` + `plans/reports/260827-land-desk-tree-cwd-cook.md`
- 0242: `plans/260827-0242-clear-report-on-prompt/plan.md` + `phase-01-start.md` + `plans/reports/260827-isolate-flock-prompt-cook.md`
- C scope: `plans/reports/260827-1020-brainstorm-eval-team.md:119-127`; `plans/reports/260827-1012-eval-next.md:57-67`
- Unlink A: `plans/reports/260827-unlink-path-leftover-cook.md`

## Unresolved

- Founder sit-without-spawn contract still medium (1020). This report does not cook.
- Whether USAGE `main.rs` + attach help land in the same C commit (1020 says same increment). If yes, copy-aside leftover `main.rs` too.
- New rust test path name (must not be leftover `p5_attach.rs`).
- Isolate worktree is at `5a60953` not paper `53f3cff` (docs only; rust identical). Reset to **C** NEWHEAD, not `53f3cff`.

## Limitations

Did not exec leftover or isolate ELF. Did not `cargo`. Did not invoke `dory`. Sock probe = FileNotFound on absent path only. `strings` on ELF is observe-only.
