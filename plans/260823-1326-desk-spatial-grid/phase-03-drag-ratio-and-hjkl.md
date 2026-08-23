---
phase: 3
title: "Phase 3: Drag ratio and hjkl"
status: completed
priority: P1
effort: 3h
dependencies: [2]
---

# Phase 3: Drag ratio and hjkl

## Overview

Mouse-drag a divider to change `ratio`. Prefix `h/j/k/l` is spatial. Optional `prefix+z` zoom is client-only. `n/p` stay list-walk.

## Context

- Divider RPC + `divider_at` already in P1 (`desk.divider`, `layout.rs`)
- Spatial `desk.neighbor` steps in P1; desk today only sends `next`/`prev` (`desk.rs:365`, `:470`)
- Prefix map: `desk.rs:358` — no hjkl/z
- Mouse: Down in sidebar or ignore content (`desk.rs:379`); wheel already goes to focused PTY (`:394`)
- Compare: Herdr mouse-first + `hjkl` — learn product, not source (`260823-1320-research-herdr-sit-down.md`)

## Requirements

- Functional: drag shared edge → `desk.divider` → relayout + resize leaves; `Ctrl-b h/j/k/l` focus spatial neighbor; `n/p` unchanged; optional `z` zoom overlay
- Non-functional: no persist zoom; no ratatui; no new CLI group

## Architecture

**Drag**

```text
MouseDown on divider_at(cx, cy)
  → drag = { a, b, dir, start_pos, start_ratio }
MouseDrag / move while button
  → ratio = clamp( pointer along axis / span , 0.05, 0.95 )
  → desk.divider {a,b,ratio}  (throttle: on move if |Δratio|≥0.02 OR mouse-up)
MouseUp
  → final desk.divider + reconcile_tiles
```

Hit-test: content-relative; grab the **stolen** trailing line of A (P2 inset), not a paint-over of B. Same geometry as `A.x+A.w == B.x` / `A.y+A.h == B.y`.
<!-- Updated: Validation Session 1 - drag the inset chrome -->

Do **not** send drag motion into the PTY.

**hjkl**

```text
Ctrl-b h/j/k/l
  → desk.neighbor from=focused step=left|down|up|right cols=W rows=H
  → if pane id ≠ focused: pane.focus (keep sibling streams)
```

`n/p` still `step=next|prev` (global wrap, `server.rs:825`). Cheap; keep.

**Zoom (`prefix+z`, optional but in scope)**

- Client bool `zoomed`. Not stored on `Tab`.
- Paint only focused tile at full content rect; other streams stay open (unzoom instant).
- While zoomed: `pane.resize` focused to full content; on unzoom restore all cell sizes via `desk.layout`.
- Click / hjkl / split unzoom first (or apply then unzoom). Pick: **unzoom on split and on tab change**; hjkl while zoomed moves focus and keeps zoom on the new pane (resize that one to full).

**Prefix hint** (`desk.rs:349`, `:672`): add `h/j/k/l` `z` drag.

## Data flow

```text
pointer on edge → desk.divider → tab.layout.ratio
  → desk.layout → N pane.resize → parsers set_size → paint
Ctrl-b l → desk.neighbor right → pane.focus → keyboard stream
Ctrl-b z → zoomed toggle → 1 or N resizes (not a Layout write)
```

## Related Code Files

- Modify: `rust/src/desk.rs` — `prefix_cmd`, `handle_mouse`, drag state, zoom flag, footer
- Create: none
- Delete: none
- Do **not** edit `server.rs` / `layout.rs` unless a P1 helper is missing — then a one-line call, not a new RPC

## Implementation Steps

1. Track `drag: Option<Drag>` and `zoomed: bool` on `Desk`.
2. Mouse: if drag active, update ratio; else if `divider_at`, start drag; else P2 click-focus.
3. Throttle divider RPCs; always commit on MouseUp.
4. Prefix `h/j/k/l` → spatial neighbor with current content cols/rows.
5. Prefix `z` → zoom toggle + resize policy above.
6. Split / new tab / new workspace: clear drag; unzoom.
7. Unit: divider hit on a 2-cell fixture; ratio from x; hjkl mapping. No TTY required.
8. `cargo test --offline --locked`.

## Todo

- [x] Drag → `desk.divider` → relayout
- [x] Prefix hjkl spatial; n/p list-walk kept
- [x] Optional prefix+z client zoom
- [x] PTY does not receive divider motion

## Success Criteria

- [x] Drag divider: both tiles change size; PTYs resize to new cells
- [x] Click tile still focuses (P2)
- [x] `Ctrl-b l` from left pane focuses right (same tab); no wrap if none
- [x] `Ctrl-b n` still walks the list (may leave the tab — today's neighbor)
- [x] `Ctrl-b z` fills the content box; `z` again restores the grid
- [x] Detach still ≠ kill
- [x] Suite green

## Risk Assessment

| Risk | L×I | Mitigation | If broken |
|---|---|---|---|
| Divider RPC every pixel | M×H | Throttle 0.02 / mouse-up | Visible jank — raise threshold |
| Drag vs click ambiguity | M×M | Start drag only on edge; click needs Down+Up in same cell | Tune 1-cell grab |
| hjkl vs occupant vim | L×L | Prefix only (`Ctrl-b`) | Do not steal raw h/j/k/l |
| Zoom resize fights grid | L×M | Unzoom before split; restore cells on unzoom | If persist wanted — replan (refuse persist) |

## Security Considerations

Ratio from client is clamped server-side (P1). Drag cannot target another tab's panes (`a`/`b` must live in one layout).

## Rollback

Revert desk drag/hjkl/zoom. Tiles from P2 remain.

## Next Steps

Phase 4 docs + isolated proof.
