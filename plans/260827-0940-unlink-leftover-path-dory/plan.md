---
title: "Unlink leftover PATH dory"
description: "Xóa symlink ~/.local/bin/dory khỏi leftover rust/target. Không retarget. Không invoke dory. Không fold leftover 5. Sock default vẫn vắng."
status: completed
priority: P1
effort: 1h
branch: main
tags: [dory, path, leftover, factory]
blockedBy: []
blocks: []
created: 2026-08-27
---

# Unlink leftover PATH dory

## Contract

| Field | Closed |
|---|---|
| Outcome | PATH **name** `dory` gone: `~/.local/bin/dory` deleted; `type -a dory` empty after `hash -r`. Leftover ELF at `rust/target/debug/dory` **kept**, not exec'd. Default sock **not connectable** on `$XDG_RUNTIME_DIR/dory/default`. Leftover 5 hash mint. |
| Constraints | Unlink only — không `ln` sang isolate. Không invoke `dory` / `dory attach` / `dory server` / leftover ELF / isolate ELF trên factory XDG. Không cargo leftover tree. Không sit `t13`. Không start default. Không fold leftover 5. Cite `git show HEAD`, không leftover `attach.rs`. Cook tab: refuse if `DORY_SOCKET`/`DORY_ENV`/`DORY_RECYCLE`/`PI_CODING_AGENT_DIR` set. |
| Non-goals | Retarget PATH. Sit PATH-pin. Land sit script. Hop → `SIT_DORY`. Rewrite leftover README Now. `ensure_server` no-spawn (C). `occ.report = Working` / `--wait` (B). Recook 1910/0043/0227/0242. Rust hunk. `herdr server stop`. Close `wP`/`w15`/`t13`. |
| Acceptance | Receipts: `test -L` then `rm` link only; `type -a dory` empty; leftover ELF sha unchanged; leftover 5 mint **before** rm; sock connectable=0 on `$XDG_RUNTIME_DIR`; review critical 0. |

Nguồn: [0927 brainstorm](../reports/260827-0927-brainstorm-eval-team.md) · [0918 eval-left](../reports/260827-0918-eval-left.md) · [0918 eval-sec](../reports/260827-0918-eval-sec.md) · kongming GO A=unlink

## Scope Challenge

```
- Existing: leftover symlink; isolate SIT_DORY pin; HEAD ensure_server still auto-spawns
- Requested: implement filtered A (unlink) via plan gates then Herdr cook/test/review/ship
- Complexity: 1 symlink + receipts. 0 rust files
- Mode: HOLD
```

## Bẫy

1. `ln -sfn` isolate `land-4b70f79` — cấm. Bare `dory` vẫn `ensure_server` (`git show HEAD:rust/src/attach.rs:332-370`) → đẻ `/run/user/1000/dory/default`.
2. Gõ `dory` / `dory attach` để “kiểm tra” — cấm. Verify = `ls` / `readlink` / `command -v` / python sock. Không spawn.
3. `rm` leftover `rust/target/debug/dory` — cấm. Chỉ `rm` symlink `~/.local/bin/dory`.
4. `cargo` leftover tree — cấm. Hash leftover 5 phải mint sau cook.
5. Cite leftover `attach.rs:379` / leftover `server.rs:929` as land — cấm. Land attach = HEAD `:332-370`.
6. Exec hop / 1910 / 0043 / 0227 / 0242 — cấm.
7. Sit `w13:t13` / `w13:p2R`. Close `wP` / `w15` / `t13`. `herdr server stop`. `dory server stop` default.
8. Git add leftover 5. Restore leftover rồi commit.
9. Sau unlink, bất kỳ `dory` còn trên PATH (`type -a` / PATH walk) → FAIL. Không “PASS vì ≠ leftover” (đó là retarget).
10. Unlink khi link không còn trỏ leftover (đã đổi) → dừng, ghi snapshot, không đoán.
11. `rm "$(readlink -f ~/.local/bin/dory)"` xóa ELF leftover — cấm. `test -L` rồi `rm` đúng path symlink.
12. Leftover-5 hash không phát hiện `cargo` leftover. Snapshot sha/mtime ELF `rust/target/debug/dory` trước/sau; đổi → FAIL.
13. `exists(/run/user/1000/...)` không phải cửa spawn. Snapshot `XDG_RUNTIME_DIR` + `DORY_*`. Connectable trên `$XDG_RUNTIME_DIR/dory/default/dory.sock`. Dead inode = warn, **không** STOP unlink. Không `dory server stop`. Không `iso()` / `DORY_SOCKET=` trên stop.
14. `git diff --name-only` bỏ staged leftover 5 và `?? scripts/`. Fold = `git status --porcelain` + `git diff --cached --name-only` + leftover-5 hash mint. Ship pathspec: plan dir + `plans/reports/260827-unlink-path-leftover-*` only.
15. Hop / USAGE / sit-child vẫn PATH `dory` sau unlink — **known broken**, không “unused”. Không nấu hop/sit-pin trong increment này. Không exec hop.
16. Cook mint tự chiếu. Gate researcher-02 full hashes **trước** `rm`. HEAD `5a60953` không phải cửa A (ship paper được phép đổi HEAD).

## Approaches

**A** — `rm` symlink `~/.local/bin/dory` after snapshot. **A.**
**B** — retarget isolate. **Cấm** (kongming).
**C** — rust `ensure_server` Err. **Không** — 0927 later, after PATH not leftover.

## Scout 09:40

- Symlink only PATH hit (`type -a` repeats `~/.local/bin/dory`). Leftover ELF strings `workspace_live_cwd` ×59. Sit binary ×0.
- Sock not present under live `XDG_RUNTIME_DIR=/run/user/1000`. Leftover 5 mint.
- Flock `:210-214` refuse PATH-as-SIT_DORY (inert after unlink). Leftover-tree refuse `:215-219` still live. Hop is the remaining PATH `dory` consumer — known broken after A, not patched here.
- HEAD `ensure_server` `git show HEAD:rust/src/attach.rs:332-370` still auto-spawns. C unpaid. Do not claim “factory doors held.”

## Herdr

Sau gate sạch: tab mới trên `w13`, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Mỗi tab 4–6 OMP. Không split `t13`. Không ngồi factory. Close only wave tabs.

| Tab | Job |
|---|---|
| cook | Unlink + cook receipt |
| test | Independent verify |
| review | Fold / sock / retarget / leftover hash |
| ship | Receipts + optional plan/receipt commit; never leftover 5 |

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Cook unlink](./phase-01-start.md) | Completed |
| 2 | [Independent PATH test](./phase-02-independent-path-test.md) | Completed |
| 3 | [Review and ship receipts](./phase-03-review-and-ship-receipts.md) | Completed |

## Success Criteria

- [x] `~/.local/bin/dory` gone (`test ! -e` / `test ! -L`)
- [x] After `hash -r`, `type -a dory` empty (no second inode)
- [x] Leftover ELF still exists; sha/mtime unchanged; not exec'd
- [x] `$XDG_RUNTIME_DIR/dory/default/dory.sock` not connectable before and after
- [x] Leftover 5 hashes = researcher-02 mint (checked **before** rm and after)
- [x] No `dory` / leftover ELF / isolate ELF invoked on factory XDG; no cargo leftover; no rust / `scripts/` edit

## Red Team Review

### Round 1 — 2026-08-27 specialized (security PATH / leftover fold / assumptions)

| # | Finding | Sev | Disposition |
|---|---|---|---|
| S1 | Sock gate hardcoded uid-1000 `exists` | Critical | **Accept** — `$XDG_RUNTIME_DIR` + connectable; refuse `DORY_*` |
| S2 | `iso()` / `DORY_SOCKET=` stop omitted | Critical | **Accept** — trap 13; do not exec hop |
| S3 | Cook no refuse dirty `DORY_*` | High | **Accept** — same as S1 |
| S4 | Unlink breaks hop; `blockedBy` 1910 | High | **Reject cook** — hop/`SIT_DORY` is non-goal. **Accept paper**: hop known-broken (trap 15) |
| S5 | “Doors held” while ELF still spawns | High | **Accept** — PATH name only; C unpaid |
| S6 | `git diff --name-only` misses staged | High | **Accept** — porcelain + cached |
| S7 | exists vs connectable deadlock | Medium | **Accept** — dead inode warn, do not STOP |
| S8 | Flock 210-214 inert hides hop | Medium | **Accept paper** (trap 15). Reject hop patch |
| F1 | Leftover-5 mint misses cargo | Critical | **Accept** — ELF sha/mtime |
| F2 | `rm` realpath deletes leftover ELF | Critical | **Accept** — `test -L` then rm link |
| F3 | Isolate leftover refuse untested | Critical | **Reject as A gate** — do not exec isolate scripts. Observe-only |
| F4 | Fold misses `git add` leftover 5 | Critical | **Accept** — same S6 |
| F5 | Six vs seven asserts | High | **Accept** — numbered A asserts |
| F6 | Cook/test same pane | High | **Accept** — distinct tab ids |
| F7 | Cook mint self-referential | High | **Accept** — researcher-02 hashes before rm |
| F8 | `strings` leftover marked FAIL | Medium | **Accept** — allow strings/cmp; fail exec only |
| A1 | “or ≠ leftover” = retarget | Critical | **Accept** — `type -a` empty only |
| A2 | One PATH dory is a snapshot | High | **Accept** — `type -a` before/after |
| A3 | Nobody needs PATH dory | High | **Accept paper** (trap 15). No hop/sit cook |
| A4 | Leftover ELF remains | High | **Accept** — success = name gone, ELF kept |
| A5 | `command -v` stale hash | High | **Accept** — `hash -r` + `type -a` |
| A6 | HEAD `5a60953` vs paper ship | High | **Accept** — drop HEAD pin as A gate |
| A7 | Isolate-exists smuggled into PASS | Medium | **Accept** — observe-only |
| A8 | Untracked `scripts/` invisible | High | **Accept** — status --short; no scripts/ edit |
| A9 | Sock UID hardcoded | Medium | **Accept** — same S1 |

Rejected as cook: hop patch, sit PATH-pin, land sit script, `ensure_server` C, `report=Working`, README Now, retarget, `rm` leftover ELF, exec isolate/hop.

### Round 2 — 2026-08-27 (updated plan, leftover-5 / rm / PATH / XDG / ship / tabs)

| # | Finding | Sev | Disposition |
|---|---|---|---|
| R2.1 | Phase 1 step 8 “operand realpath ≠ leftover ELF” contradicts step 5 (realpath **is** leftover) | Critical | **Accept** — deleted. `rm` symlink path only |
| R2.2 | Porcelain “no `?? scripts/`” fails pre-existing untracked scripts | Critical | **Accept** — snapshot porcelain; FAIL drift only |

### Whole-Plan Consistency Sweep

- Files reread after R1+R2: plan.md, phase-01, phase-02, phase-03
- Decision deltas: PATH empty-only; ELF kept+hashed; XDG connectable; DORY_* refuse; distinct tabs; leftover mint before rm; porcelain fold + snapshot; hop known-broken; rm symlink path only
- Reconciled stale: “or ≠ leftover”, “six asserts”, “factory doors held”, HEAD pin as A gate, hardcoded `/run/user/1000` as the only sock check, rm-operand contradiction, porcelain vs `?? scripts/`
- Unresolved contradictions: 0

## Validation Log

### Session 1 — 2026-08-27
**Trigger:** User ordered plan → red-team → validate loops then Herdr cook/test/review/ship. Settled by 0927 brainstorm + this turn (implement A).
**Questions asked:** 0 live (settled; no material fork left)

#### Questions & Answers

1. **[Architecture]** Unlink vs retarget vs rust C?
   - Options: Unlink only (Recommended) | Retarget | Rust C
   - **Answer:** Unlink only
   - **Rationale:** 0927 kongming A.

2. **[Scope]** Patch hop / sit PATH-pin now?
   - Options: No (Recommended) | Yes hop | Yes sit pin
   - **Answer:** No
   - **Rationale:** HOLD SCOPE. Hop known-broken after A.

3. **[Risks]** Remaining PATH `dory` after unlink?
   - Options: FAIL (Recommended) | PASS if ≠ leftover
   - **Answer:** FAIL
   - **Rationale:** “≠ leftover” is retarget.

#### Confirmed Decisions
- A = unlink PATH name; leftover ELF kept
- Empty `type -a dory` only
- No factory `dory` invoke
- Ship paper pathspec only
- Distinct Herdr tabs

#### Action Items
- None remaining on the plan

#### Verification Results
- **Tier:** Standard (3 phases)
- **Claims checked:** 10
- **Verified:** 10 | **Failed:** 0 | **Unverified:** 0
- HEAD `ensure_server` `git show HEAD:rust/src/attach.rs:332` — VERIFIED
- `session_paths` `rust/src/socket.rs:90-96` — VERIFIED
- Flock refuse `:210-219` — VERIFIED
- Hop PATH `dory` `scripts/dory-flock-hop.sh:62` — VERIFIED
- Leftover 5 hashes = researcher-02 — VERIFIED
- `ak plan validate` format valid — VERIFIED

#### Whole-Plan Consistency Sweep
- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas checked: 8
- Reconciled stale references: 6
- Unresolved contradictions: 0

<!-- slug: unlink-leftover-path-dory -->
