---
phase: 3
title: "Review and ship receipts"
status: pending
priority: P1
effort: "20m"
dependencies: [2]
---

# Phase 3: Review and ship receipts

## Overview

Hostile review of cook+test against plan traps. Ship paper only. Never leftover 5. Never `scripts/`.

## Requirements

- Functional: review ACCEPT with critical 0, or FAIL with named trap.
- Functional: leftover 5 unstaged **and** uncached; hashes mint.
- Non-functional: no rust; no default sit; no `herdr server stop`.

## Architecture

Reviewers read receipts + live hashes + `git show HEAD:rust/src/attach.rs:332-370`. Ship `ak:git` pathspec only:

- `plans/260827-0940-unlink-leftover-path-dory/`
- `plans/reports/260827-unlink-path-leftover-*`
- plus this wave’s brainstorm if already untracked and needed: `plans/reports/260827-0927-brainstorm-eval-team.md`

Refuse: leftover 5, `README.md`, `rust/`, `scripts/`, `git add -u`, `git add rust/`.

## Related Code Files

- Read: cook + test receipts, this plan
- Create: `plans/reports/260827-unlink-path-leftover-review.md`
- Optional add: pathspec above
- Do not add: leftover 5

## Implementation Steps

1. New review tab 4–6 OMP, specialized. `REVIEW_TAB` ≠ cook ≠ test ≠ `t13`.
   - fold: `git status --porcelain` + `git diff --cached --name-only` + leftover-5 `hash-object` == mint. No `scripts/` mutation. No leftover-5 staged.
   - sock: `$XDG_RUNTIME_DIR/dory/default/dory.sock` not connectable
   - retarget: `type -a dory` empty; no `ln` to isolate
   - spawn: receipts prove no `dory`/ELF invoke (no new connectable default)
   - ELF: leftover binary still present; sha matches cook snapshot
2. Review FAIL if any trap 1–16 fired.
3. Write review receipt ACCEPT / REJECT.
4. Ship only if ACCEPT. `ak:git` pathspec above. Never leftover 5. Never push unless user asked (this wave: no push).
5. Close only wave tabs this factory minted. Leave `t13` / `wP` / `w15`.

## Success Criteria

- [x] Review ACCEPT, critical 0
- [x] Leftover 5 mint and not in the commit (staged or unstaged add)
- [x] Sock not connectable
- [x] `type -a dory` empty
- [x] Wave tabs closed; factory `t13` remains

## Risk Assessment

| Risk | Signal | Response |
|---|---|---|
| Ship folds leftover | leftover 5 in index | Reject commit. Unstage. |
| Review cites leftover attach as land | leftover `:379` | Reject; cite HEAD `:332-370`. |
| `git add plans/` vacuum | extra untracked plans | Reject. Pathspec only. |

## Next Steps

None in rust. C (`ensure_server` no-spawn) stays later. Hop PATH `dory` stays known-broken.
