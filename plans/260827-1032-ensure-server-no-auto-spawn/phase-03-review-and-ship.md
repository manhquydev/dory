---
phase: 3
title: "Review and ship"
status: pending
priority: P1
effort: "20m"
dependencies: [2]
---

# Phase 3: Review and ship

## Overview

Review critical 0: spawn hunk, leftover mint, USAGE, fold, no factory invoke. Ship rust already committed in phase 1. Paper plan + receipts second commit. Never leftover 5.

## Requirements

- Functional: review ACCEPT critical 0 against plan traps.
- Functional: paper commit = plan dir + `plans/reports/260827-ensure-server-no-spawn-*` + research 1027 if untracked. Not leftover 5. Not isolate ELF.
- Non-functional: factory `dory.sock` connectable=0; PATH empty; leftover mint after ship; leftover ELF sha unchanged.

## Architecture

Review tab ≠ cook ≠ test ≠ `t13`. Four OMP panes max if split: spawn / leftover / USAGE / fold.

Ship: `git add` pathspec paper only. Conventional commit. No `--no-verify`. No push unless user asks.

## Related Code Files

- Create: `plans/reports/260827-ensure-server-no-spawn-review.md`
- Create: `plans/reports/260827-ensure-server-no-spawn-ops.md` (optional)
- Do not `git add`: leftover 5, `desk.rs`, `scripts/`, leftover ELF

## Implementation Steps

1. New `w13` tab. Refuse dirty `DORY_*` / `PI_CODING_AGENT_DIR`. STOP leftover/isolate ELF argv. `NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)` — do not inherit cook-tab `$NEWHEAD`. Read cook + test receipts + `git show HEAD:rust/src/attach.rs` `ensure_server` + USAGE + leftover hashes.
2. Critical if any: spawn still present; leftover remint (incl. `server.rs` `4de1554a`); leftover-tree cargo; factory `dory`/leftover/isolate ELF; retarget; leftover README Now rewrite; `git add` leftover 5; isolate `rev-parse` ≠ this-tab `$NEWHEAD`; silent `Err(1)`; sock probe on session **directory**; isolate ELF still has `dory: start server:`.
3. ACCEPT only if critical 0.
4. Paper commit pathspec only. `git diff --cached --name-only` ⊆ `plans/260827-1032-ensure-server-no-auto-spawn/` + `plans/reports/260827-ensure-server-no-spawn-*` + `plans/reports/260827-1027-research-0{1,2}-*.md`. Leftover 5 stay ` M` unstaged. Ban `git add -u`.
5. Re-hash leftover 5 + leftover ELF. `dory.sock` connectable=0. PATH empty.
6. Close only wave tabs. Leave `t13` / `wP` / `w15`.

## Success Criteria

- [ ] Review critical 0
- [ ] Rust C already on `git show HEAD`
- [ ] Paper receipts committed without leftover 5
- [ ] Leftover mint + `dory.sock` connectable=0 + PATH empty after ship

## Risk Assessment

| Risk | Signal | Response |
|---|---|---|
| Stage leftover 5 | cached names | Unstage. Do not commit. |
| Push | remote update | Do not push. |
| Close `t13` | tab missing | FAIL. Close wave only. |

## Next Steps

Done. Do not recook hop / sit-pin / B.
