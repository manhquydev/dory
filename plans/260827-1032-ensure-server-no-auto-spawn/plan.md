---
title: "Ensure server no auto-spawn"
description: "HEAD ensure_server ping-miss trả Err, không spawn. USAGE bỏ auto-start. Một p5 đảo. Leftover 5 mint. Cargo chỉ isolate."
status: pending
priority: P1
effort: 2h
branch: main
tags: [dory, attach, spawn, leftover, factory]
blockedBy: []
blocks: []
created: 2026-08-27
---

# Ensure server no auto-spawn

## Contract

| Field | Closed |
|---|---|
| Outcome | Sit không tự đẻ daemon. `ensure_server` ping-miss → `Err` + stderr `dory: server not running; start with \`dory server\``; không `Command::new(exe).arg("server").spawn()`. Ngồi lần đầu = `dory server` rồi `dory`. USAGE / attach help / HEAD README `:15` cùng increment. Invert test (temp XDG): bare `dory` fail-closed; `workspace list` fail. Isolate sit `:281` server rồi `:331` attach — không phải C proof; không xóa `:281`. |
| Constraints | Land method 0130/0242: copy-aside leftover 5 → `git checkout HEAD --` attach/main/`p5_attach`/README → hunk trên HEAD blob → commit pathspec đó → restore leftover bytes. Cargo **chỉ** `/home/manhquy/.cache/dory-isolates/land-4b70f79`. Không invoke `dory` / leftover ELF / isolate ELF trên factory XDG. Không start default. Không sit `t13`. Không fold leftover 5. Cite `git show HEAD`, không leftover `attach.rs:379`. |
| Non-goals | Retarget PATH. Hop → `SIT_DORY`. Sit PATH-pin. Leftover README `## Now` rewrite. `occ.report = Working` / `--wait` (B). Recook 1910/0043/0227/0242/0940. `desk.rs`. `server.rs`. `rm` leftover ELF. `herdr server stop`. Close `wP`/`w15`/`t13`. |
| Acceptance | `git show HEAD:rust/src/attach.rs` không còn `cmd.arg("server")` + `cmd.spawn()` trong `ensure_server`. USAGE không còn “Starts the server if needed”. HEAD `p5_attach` test mới fail-closed PASS trên isolate cargo. Leftover 5 mint. Sock `$XDG_RUNTIME_DIR/dory/default/dory.sock` **connectable=0** (AF_UNIX; không probe thư mục session; không “absent”). PATH `type -a dory` empty. Isolate ELF sau rebuild **không** còn chuỗi `dory: start server:` / `server did not come up`. Leftover ELF sha + spawn-strings **giữ**. Review critical 0. |

Nguồn: [1020 brainstorm](../reports/260827-1020-brainstorm-eval-team.md) · [1012 eval-spawn](../reports/260827-1012-eval-spawn.md) · [1012 eval-next](../reports/260827-1012-eval-next.md) · [1027 research-01](../reports/260827-1027-research-01-spawn.md) · [1027 research-02](../reports/260827-1027-research-02-leftover.md) · kongming GO C

## Scope Challenge

```
- Existing: HEAD ensure_server spawn :332-367; 46 p5 `start()` / 1 help / 1 invert; leftover 5 mint; PATH gone; `dory.sock` connectable=0; isolate land-4b70f79 at 5a60953
- Requested: implement filtered C via plan gates then Herdr cook/test/review/ship
- Complexity: 1 rust fn + USAGE/README + 1 test rewrite. 3 phases. 0 new types
- Mode: HOLD
```

## Bẫy

1. Sửa leftover working `attach.rs` (`60247909`, recycle/`spawn_server` leftover `:379`) rồi commit — cấm. Land = `git show HEAD:rust/src/attach.rs:332-367`.
2. `cargo` leftover tree `/home/manhquy/Downloads/flow/dory` — cấm. Rebuild leftover ELF `3ba0e3bc…`.
3. Quên restore leftover bytes → mint gãy. Abort = **bốn** `cp` KEEP có dest: `$KEEP/attach.rs`→`rust/src/attach.rs`, `$KEEP/main.rs`→`rust/src/main.rs`, `$KEEP/p5_attach.rs`→`rust/tests/p5_attach.rs`, `$KEEP/README.md`→`README.md`. **Không** `cp`/`git restore` `server.rs`. Không xóa KEEP. Không recopy WT→KEEP sau checkout.
4. Cite leftover `attach.rs:379` / leftover `server.rs:929` as land — cấm.
5. Gõ `dory` / leftover ELF `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory` / isolate ELF `…/land-4b70f79/rust/target/debug/dory` trên factory XDG — **STOP** riêng, không phụ thuộc `DORY_*` set. Verify = `git show HEAD` + isolate cargo + connectable trên **`dory.sock`**. Không spawn default.
6. Retarget `ln -sfn` isolate → `~/.local/bin/dory` — cấm. 0940 trap 1.
7. `rm` leftover ELF — cấm.
8. Rewrite leftover README `## Now` — remint `68190a5f`. HEAD README `:15` thì checkout HEAD blob, commit, restore leftover README.
9. New test ghi đè leftover `p5_attach.rs` rồi `git add` leftover — remint `9c28fc3e`. Checkout HEAD `p5_attach.rs`, đảo test, commit HEAD blob, restore leftover.
10. Isolate worktree quên `reset --hard NEWHEAD` — SIT_DORY ELF `dc0e867a…` vẫn spawn.
11. Sit `w13:t13` / `p2R`. Close `wP` / `w15` / `t13`. `herdr server stop`.
12. `occ.report = Working` / isolate `--wait` (B). Hop → `SIT_DORY`. Recook 1910/0043/0227/0242/0940.
13. Silent `Err(1)` không stderr — test và người ngồi không đọc được. Khóa câu: `dory: server not running; start with \`dory server\``.
14. Giữ tên test `bare_dory_without_tty_starts_server` + assert `needs a tty` — FAIL sau hunk. Đổi tên + đảo assert.
15. Claim “factory doors held.” PATH name gone; leftover ELF vẫn spawn. C khóa cửa **sản phẩm** trên HEAD inode.
16. `git add` leftover 5 / `desk.rs` / leftover `server.rs`. Ban `git add -u` / `git commit -a`. Cached rust = đúng bốn path. Ship paper pathspec only.
17. Cook/test/review: refuse nếu `DORY_*`/`PI_CODING_AGENT_DIR` set **và** refuse leftover/isolate ELF argv. Unset `DORY_SOCKET` = đường mint leftover — không phải “cửa đã khóa.”
18. `dory server` / `run_foreground` — không đụng. C không phá verb tường minh.
19. `ping()` vẫn `DORY_SOCKET` rồi mới XDG (`server.rs:1838-1846`). Bind vẫn XDG. Không nấu `server.rs`. Test + cargo: `env -u DORY_SOCKET`.
20. `NEWHEAD` phải gán sau commit: `NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)`. Isolate `reset --hard "$NEWHEAD"` only. Cấm factory `reset --hard`.
21. Cargo **absolute** `--manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml`. Relative `rust/Cargo.toml` từ leftover cwd = leftover cargo.
22. Sock probe = `UnixStream::connect("$XDG_RUNTIME_DIR/dory/default/dory.sock")`. Không connect thư mục `…/default`. Dead inode = warn, không STOP, không `rm`, không `dory server stop`.
23. Scout test count = 46 `start()` / 1 help / 1 invert — không “47/48 already server.”

## Approaches

**C** — `ensure_server` ping-miss `Err`, no spawn + USAGE + đảo 1 p5. **Chọn.**
**B** — `occ.report = Working`. **Cấm** (0242 trap 10).
**Retarget** — PATH isolate. **Cấm** (0940).

## Scout 10:32

- Callers HEAD: `ensure_server` def `attach.rs:332`; calls `sit` `:136`, `run_with_pane` `desk.rs:166`. Bare `dory` = `main.rs:69` → `desk::run`. `dory server` = `run_foreground`, không gọi `ensure_server`.
- 46 HEAD `p5_*` `start()` + 1 help-only + **1** invert: `p5_attach.rs:213` `bare_dory_without_tty_starts_server`.
- Leftover 5 mint. Isolate `land-4b70f79` clean, attach == HEAD `cf00a2fa`.
- PATH gone. `dory.sock` connectable=0.

## Herdr

Sau gate sạch: tab mới trên `w13`, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Mỗi tab 4–6 OMP. Không split `t13`. Không ngồi factory. Close only wave tabs.

| Tab | Job |
|---|---|
| cook | Land hunk + isolate cargo + cook receipt |
| test | Independent: HEAD blob / leftover mint / sock / PATH / no factory `dory` |
| review | Spawn gone / leftover mint / USAGE / fold / no factory invoke |
| ship | Rust commit pathspec + paper plan/receipts; never leftover 5 |

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Land no-spawn](./phase-01-start.md) | Pending |
| 2 | [Independent no-spawn test](./phase-02-independent-no-spawn-test.md) | Pending |
| 3 | [Review and ship](./phase-03-review-and-ship.md) | Pending |

## Success Criteria

- [ ] `git show HEAD:rust/src/attach.rs` `ensure_server`: ping miss → eprintln + `Err`; **no** `arg("server")` spawn
- [ ] USAGE `main.rs` + attach module/help không còn auto-start sentence; help test vẫn thấy `Bare \`dory\` opens the desk`
- [ ] HEAD README `:15` nói `dory server` rồi `dory` (HEAD blob; leftover README restored)
- [ ] HEAD `p5_attach` test fail-closed PASS trên isolate cargo
- [ ] Leftover 5 `git hash-object` = mint; leftover ELF sha `3ba0e3bc…` unchanged
- [ ] `$XDG_RUNTIME_DIR/dory/default/dory.sock` connectable=0; `type -a dory` empty
- [ ] Isolate ELF strings: no `dory: start server:` / `server did not come up`
- [ ] Leftover ELF sha `3ba0e3bc…` + spawn-strings still present
- [ ] No factory `dory` / leftover ELF / isolate ELF argv; no leftover-tree cargo

## Red Team Review

### Round 1 — 2026-08-27 specialized (security spawn / leftover restore)

| # | Finding | Sev | Disposition |
|---|---|---|---|
| S1 | Leftover/isolate ELF still mint default | Critical | **Accept paper** — C = HEAD inode. Isolate ELF strings after rebuild must drop spawn text. Leftover ELF sha+strings **kept**. Hard argv ban. |
| S2 | `ping()` DORY_SOCKET-first vs XDG bind | Critical | **Accept paper** — no `server.rs` hunk. Test/cargo `env -u DORY_SOCKET`. |
| S3 | Sock probe on session **directory** / “absent” | Critical | **Accept** — connectable on `…/default/dory.sock` only |
| S4 | Refuse-if-`DORY_*`-set inverted | High | **Accept** — ELF argv STOP independent of env |
| S5 | Inverted p5 missing `env_remove(DORY_SOCKET)` | High | **Accept** |
| S6 | Phase 3 no refuse / no ELF ban | High | **Accept** |
| S7 | Architecture relative cargo = leftover cargo | High | **Accept** — absolute isolate manifest |
| S8 | Scout “47/48 already server” | Medium | **Accept** — 46/1/1 |
| F1 | Restore leftover 5 via git includes `server.rs` | Critical | **Accept** — restore exactly four named KEEP cps; never `server.rs` |
| F2 | KEEP basename `cp` has no dest map | Critical | **Accept** — four dests spelled |
| F3 | Re-copy KEEP after checkout poisons mint | Critical | **Accept** — WT→KEEP only while mint; then refuse |
| F4 | `git add rust/` / `commit -a` folds leftover server | Critical | **Accept** — cached exactly four |
| F5 | Restore-before-commit / `git add -u` folds leftover attach | Critical | **Accept** — commit then restore; phase-3 allowlist |
| F6 | Relative cargo leftover ELF | Critical | **Accept** — same S7 |
| F7 | `NEWHEAD` unbound / factory `reset --hard` | Critical | **Accept** — bind after commit; isolate only |
| F8 | New test path instead of checkout `p5_attach` | High | **Reject cook** — HEAD still has `bare_dory_without_tty_starts_server`; must invert that blob. Leftover cargo false-PASS already trap 21 |
| F9 | Mid-checkout `git restore` leftover 5 | High | **Accept** — four named abort cps |

Rejected as cook: hop/`SIT_DORY`, sit PATH-pin, leftover README Now, B, retarget, `rm` leftover ELF, `server.rs` ping rewrite, new-path-only test.

### Round 2 — 2026-08-27 (updated plan, env/NEWHEAD/scout/sit)

| # | Finding | Sev | Disposition |
|---|---|---|---|
| R2.1 | `env -u DORY_SOCKET DORY_ENV` treats DORY_ENV as COMMAND | High | **Accept** — three `-u` flags |
| R2.2 | Phase 3 `$NEWHEAD` unbound | High | **Accept** — re-bind on review tab |
| R2.3 | Scope/Scout still “47/48” + “sock absent” | High | **Accept** |
| R2.4 | Isolate sit `:331` after `:281`; Outcome `workspace list fail` unqualified | High | **Accept paper** — invert temp XDG only; do not run/delete isolate sit |
| R2.5 | HEAD sidebar pin not `:31` | Medium | **Accept** — leave `HEAD:README.md:31` `22↔4↔0` |
| R2.6 | Help test does not forbid auto-start sentence | Medium | **Accept** — add `assert!(!contains("Starts the server if needed"))` |

### Whole-Plan Consistency Sweep

- Files reread after R1+R2: plan.md, phase-01, phase-02, phase-03
- Decision deltas: sock=`dory.sock` connectable; named four KEEP cps; no `server.rs` restore; `NEWHEAD` bound per tab; absolute isolate cargo; three `env -u`; ELF argv ban; 46/1/1; invert-only list-fail; HEAD README `:31` leave 22; help forbids auto-start
- Reconciled stale: “47/48”, “sock absent”, directory connect, relative `rust/Cargo.toml`, unbound NEWHEAD, restore leftover 5, `env -u A B` as command
- Unresolved contradictions: 0

## Validation Log

### Session 1 — 2026-08-27
**Trigger:** User ordered plan → red-team → validate loops then Herdr cook/test/review/ship. Settled by 1020 brainstorm (letter C) + this turn.
**Questions asked:** 0 live (settled; no material fork left)

#### Questions & Answers

1. **[Architecture]** C hunk vs retarget vs B?
   - Options: C no-spawn (Recommended) | Retarget PATH | B report=Working
   - **Answer:** C no-spawn
   - **Rationale:** 1020 kongming GO C.

2. **[Scope]** Rewrite leftover `p5_attach` via new-path-only?
   - Options: Invert HEAD `p5_attach` + restore leftover (Recommended) | New path only
   - **Answer:** Invert HEAD blob + restore
   - **Rationale:** HEAD still has `bare_dory_without_tty_starts_server`.

3. **[Risks]** Factory leftover/isolate ELF exec?
   - Options: STOP argv (Recommended) | Allow “to verify”
   - **Answer:** STOP
   - **Rationale:** Those ELFs still spawn until isolate rebuild; leftover ELF kept.

#### Confirmed Decisions
- C = HEAD `ensure_server` fail-closed + USAGE + invert one p5 + HEAD README `:15`
- Leftover 5 mint via four named KEEP cps; never `server.rs`
- Cargo isolate absolute only; `NEWHEAD` per tab
- Sock = `dory.sock` connectable=0
- Isolate sit not C proof

#### Action Items
- None remaining on the plan

#### Verification Results
- **Tier:** Standard (3 phases)
- **Claims checked:** 10
- **Verified:** 10 | **Failed:** 0 | **Unverified:** 0
- HEAD `ensure_server` `git show HEAD:rust/src/attach.rs:332-367` — VERIFIED
- Callers `sit` `:136` + `desk.rs:166` — VERIFIED
- `dory server` `main.rs:77-78` ≠ `ensure_server` — VERIFIED
- USAGE auto-start `main.rs:50` — VERIFIED
- Invert test `p5_attach.rs:213` — VERIFIED
- `connect_control_quiet` DORY_SOCKET-first `server.rs:1884-1885` (leftover line nos; HEAD blob same split) — VERIFIED via research-01 HEAD `:1838-1846`
- Isolate sit server `:281` then attach `:331` — VERIFIED
- HEAD README sit `:15` + sidebar `:31` `22↔4↔0` — VERIFIED
- `ak plan validate` format valid — VERIFIED
- Leftover 5 mint (researcher-02) — VERIFIED this session prior measure

#### Whole-Plan Consistency Sweep
- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas checked: 12
- Reconciled stale references: 8
- Unresolved contradictions: 0

<!-- slug: ensure-server-no-auto-spawn -->

