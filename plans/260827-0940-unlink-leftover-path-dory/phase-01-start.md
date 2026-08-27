---
phase: 1
title: "Cook unlink"
status: pending
priority: P1
effort: "20m"
dependencies: []
---

# Phase 1: Cook unlink

## Overview

Snapshot leftover PATH symlink + leftover ELF + leftover-5 mint + XDG sock. If mint and target still match, `test -L` then `rm` **only** the symlink. Never invoke `dory` or the leftover ELF.

## Requirements

- Functional: after `hash -r`, `type -a dory` empty; `~/.local/bin/dory` gone.
- Functional: leftover ELF still on disk; sha/mtime unchanged.
- Functional: cook receipt with before/after evidence.
- Non-functional: leftover 5 = researcher-02 mint **before** rm; sock not connectable on `$XDG_RUNTIME_DIR`; no cargo; no rust/`scripts/` edit.

## Architecture

Distinct `w13` tab (not `t13`). Observe-only, then unlink if gates hold.

```
env refuse → mint leftover 5 → type -a → realpath leftover?
  → ELF sha → XDG connectable=0 → test -L → rm link
  → hash -r → type -a empty → ELF sha same → leftover 5 same
```

## Related Code Files

- Delete (symlink only): `~/.local/bin/dory`
- Do not modify: leftover 5, `desk.rs`, `scripts/`, HEAD rust
- Do not delete: leftover `rust/target/debug/dory`
- Create: `plans/reports/260827-unlink-path-leftover-cook.md`
- Create: `plans/reports/260827-unlink-path-leftover-before.txt`

## Implementation Steps

1. New `w13` tab, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Not `t13`. Record `COOK_TAB` / `COOK_PANE`.
2. Print and **refuse** if any set: `DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR`. Snapshot `XDG_RUNTIME_DIR`. Snapshot `git status --porcelain` (expect leftover 5 `M` and maybe `?? scripts/`).
3. Leftover 5 `git hash-object` must equal researcher-02 **full** SHAs. Mismatch → **STOP**. Do not unlink.
4. `type -a dory` + PATH walk for `*/dory`. More than one distinct realpath → **STOP**.
5. `test -L ~/.local/bin/dory`. `realpath` must be `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory`. Else **STOP**.
6. Snapshot leftover ELF sha256 + mtime + size. File must stay after cook.
7. Connectable check on `"$XDG_RUNTIME_DIR/dory/default/dory.sock"` (AF_UNIX, 1s). Connectable → **STOP** (do not start/stop default). Exists-but-not-connectable → warn, continue. Do not `dory server stop`. Do not `iso()` / `DORY_SOCKET=` on stop.
8. `rm` the **symlink path** `~/.local/bin/dory` only (`test -L` already held). Never `rm "$(readlink -f …)"`. Not `ln`. Not `rm` leftover ELF.
9. `hash -r`. `type -a dory` must be empty. Any remaining `dory` → FAIL (not “ok if ≠ leftover”).
10. Leftover ELF still exists; sha/mtime unchanged. Leftover 5 still mint. Sock still not connectable. Compare `git status --porcelain` to the **pre-rm snapshot**: FAIL only on new `scripts/` or rust drift. Pre-existing leftover 5 `M` and pre-existing `?? scripts/` are allowed.
11. Write cook receipt. `strings` leftover ELF allowed. Do not exec leftover or isolate ELF. Do not cargo. Do not git add leftover 5.

## Success Criteria

- [x] Researcher-02 leftover-5 mint checked **before** rm
- [x] `test -L` then rm symlink only; leftover ELF remains, sha unchanged
- [x] `hash -r` + `type -a dory` empty
- [x] `$XDG_RUNTIME_DIR/dory/default/dory.sock` not connectable
- [x] Receipt on disk; no `dory`/ELF argv on factory XDG
- [x] Distinct `COOK_TAB` ≠ `t13`

## Risk Assessment

| Risk | Signal | Response |
|---|---|---|
| Agent types `dory` | sock connectable | FAIL. Do not `server stop` unless user intends kill. |
| `rm` realpath ELF | leftover ELF missing | FAIL. |
| Retarget | `ln` or `type -a` non-empty | FAIL. |
| Cargo leftover | ELF sha changed, leftover 5 mint | FAIL. |

## Next Steps

Phase 2 on a **different** `w13` tab id, not a split of cook, not `t13`.
