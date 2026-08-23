---
title: Desk spatial grid shipped
date: 2026-08-23
summary: "Dory desk is a tiled live-PTY grid (BSP + N attach), not a one-PTY chooser; suite 116/116."
---

# Desk spatial grid shipped

## What happened

Operator said sit-down still felt unlike Herdr. Research: Dory was a chooser (tree + one tube); Herdr is a spatial desk. Cook `--auto` on `plans/260823-1326-desk-spatial-grid`.

Daemon now owns a textbook BSP (`rust/src/layout.rs`). `desk.layout` / `desk.divider` / spatial `desk.neighbor`. `pane.attach` `no_focus` skips both `world.focused` and `occ.seen`. Desk paints N attach streams (crossterm + vt100), insets the stolen last col/row of A, click-to-focus, drag divider, prefix hjkl + z. `Ctrl-b v` focuses the new pane.

Reviewer 6/10: `neighbor()` skipped `reconcile_tiles` unless zoomed, so cross-tab `n`/`p` dropped keys. Fixed: always retile; 400ms tick also retires if focused id is missing from tiles. Re-review 8/10, 0 critical.

## Decision

Geometry, not Ratatui or a Herdr clone. Socket proof is the gate (`p5_layout`), not a TTY screenshot. Do not flip 0847 / 0011 / 0859 papers.

## Next steps

Plan 1326 is completed (42/42). Do not recook it. Optional new slice only: `seen` integration test, coalesced-handshake fixture, keep streams on zoom. Ask before git commit (dory is its own repo; almost the whole crate is still uncommitted).

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
