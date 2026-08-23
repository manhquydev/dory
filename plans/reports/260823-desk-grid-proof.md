---
type: proof
date: 2026-08-23
time: 14:10
plan: plans/260823-1326-desk-spatial-grid
---

# Desk grid — isolated socket proof

Gate is `cargo test --offline --locked`, especially `p5_layout::two_attach_streams_live_after_split_detach_leaves`.

Isolated `XDG_RUNTIME_DIR`:

1. `dory server`
2. `pane.split --direction right`
3. `desk.layout cols=80 rows=22` → two exclusive cells, widths sum to 80
4. Two `pane.attach` (`no_focus` on both)
5. `pane.write` `echo GRID_A` / `echo GRID_B`
6. Each attach stream observes its own mark
7. Drop clients; `pane list` still has both ids
8. `dory server stop`

Human tty (optional, not CI): `dory` then `Ctrl-b v` shows two live tiles. Detach `Ctrl-b q`.

Gate rerun: `cargo test --offline --locked` **116/116**. `p5_layout` 3/3.

No `herdr` exec. No `:7380`. No Ratatui.
