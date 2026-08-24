---
title: Sit-down shell and cwd
date: 2026-08-24
summary: Desk sit-down uses $SHELL with rc; new panes follow launch/focus cwd. Tests stay bash --norc.
---

# Sit-down shell and cwd

## What happened

Sit-down felt like `bash-5.2$` and people could not work in the folder they wanted. Harder than Herdr.

## Root cause

`first_shell()` always spawned `/bin/bash --norc --noprofile` (default PS1 `bash-5.2$`, no bashrc/aliases/zoxide). `create_tab` / `create_workspace` used frozen `world.cwd` from daemon boot, not the focused pane or the directory where the operator typed `dory`.

`cd /abs` in the pane was never broken.

## Decision

- `dory` / attach `ensure_server` sets `DORY_SIT_SHELL=1` → `$SHELL` with rc. `herdr`/`dsh` as SHELL falls back to bare bash.
- Tests and bare `dory server` stay `--norc`.
- New tab: cwd of focused pane (`/proc/pid/cwd`). Desk new workspace: directory where `dory` was typed (`cwd` on RPC).
- Old daemon keeps the old shell until `dory server stop`.

## Evidence

`cargo test --offline --locked` green, including `sit_shell_drops_norc_default_keeps_it` and `tab_and_workspace_follow_pane_or_explicit_cwd`.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
