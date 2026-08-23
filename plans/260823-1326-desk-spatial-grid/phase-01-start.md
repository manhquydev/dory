---
phase: 1
title: "Phase 1: Layout model and RPCs"
status: completed
priority: P1
effort: 5h
dependencies: []
---

# Phase 1: Layout model and RPCs

## Overview

Daemon owns a BSP tree. `pane.split` writes it. `desk.layout` / `desk.divider` / spatial `desk.neighbor` are testable with no TUI. Desk still one-PTY until Phase 2.

## Context

- Compare gap: `plans/reports/260823-1328-compare-dory-herdr-now.md`
- Current split: `rust/src/server.rs:629` pushes; direction not stored (`:668`)
- `Tab` today: `server.rs:66-70`
- JSON parsers: `json_u16_field` `server.rs:1650` — add `json_f32_field` (digits + one `.`)
- Envelope stays hand-rolled (`rust/src/envelope.rs:1`)

## Requirements

- Functional: `Layout` on every `Tab`; split rewrites the focused leaf; `desk.layout` tiles the active tab (exclusive, no `gap`); `desk.divider` sets ratio; `desk.neighbor` `left|right|up|down` uses those tiles; `pane.attach` accepts `no_focus` (skips focus **and** `seen`). Name is the same flag as existing `pane.split` `no_focus` — do not rename.
- Non-functional: no serde; IDs unchanged; `cargo test --offline --locked`; existing `p5_attach` tree/neighbor next/prev still pass

## Architecture

Pure geometry in `rust/src/layout.rs` (no `World`, no PTY):

```text
pub enum SplitDir { Right, Down }
pub enum Layout {
  Leaf { pane: String },
  Split { dir: SplitDir, ratio: f32, a: Box<Layout>, b: Box<Layout> },
}
pub struct Cell { pub id: String, pub x: u16, pub y: u16, pub w: u16, pub h: u16 }
```

Helpers (all unit-tested here):

| Fn | Job |
|---|---|
| `synthesize(&[id])` | 1 → Leaf; else fold `Split{Right, 0.5}` in vec order |
| `leaves(&Layout) -> Vec<String>` | inorder |
| `ensure_layout(layout, pane_ids)` | if leaf-set ≠ ids, synthesize |
| `split_leaf(layout, pane, dir, new_id)` | replace that Leaf |
| `set_ratio(layout, a, b, ratio)` | lowest split separating a/b; swap → `1-ratio` |
| `tiles(layout, x, y, w, h)` | exclusive integer cells |
| `cell_at(cells, x, y)` | click |
| `divider_at(cells, x, y)` | shared edge → `(a, b, dir)` |
| `neighbor(cells, from, step)` | overlap on cross-axis, nearest in dir; no wrap |

**Integer split:** `a = (ratio * span as f32).round() as u16; a.clamp(1, span-1)` when `span >= 2`.

**RPC data flow**

```text
desk.layout {tab?, cols, rows}
  → tab = given or tab-of(world.focused)
  → ensure_layout
  → tiles(0,0,cols,rows)
  → each cell + occ/st from classify_word (same as desk_tree :775)

desk.divider {a, b, ratio}
  → locate tab via a → set_ratio → clamp 0.05..=0.95

desk.neighbor {from?, step, cols?, rows?}
  → next|prev: today's list wrap (server.rs:825) — ignore cols
  → left|right|up|down: require cols+rows; tiles of from's tab; no wrap

pane.split
  → split_direction unchanged (server.rs:601)
  → spawn + panes.push unchanged (server.rs:659)
  → ALSO split_leaf on tab.layout
  → response JSON unchanged

pane.attach + no_focus:true → skip world.focused **and** occ.seen
  (today both happen :736-742). pane.focus still sets seen (:711).
<!-- Updated: Validation Session 1 - no_focus skips focus+seen -->
```

`create_workspace` / `create_tab`: `layout: Leaf { pane }`.

## Related Code Files

- Create: `rust/src/layout.rs`
- Create: `rust/tests/p5_layout.rs` (copy `start`/`rpc` shape from `rust/tests/p5_attach.rs:49`)
- Modify: `rust/src/main.rs:10` — add `mod layout;`
- Modify: `rust/src/server.rs` — `Tab` (`:66`), `create_workspace` (`:413`), `create_tab` (`:439`), `split_pane` (`:629`), `take_attach` (`:717`), `dispatch_line` (`:283-329`), `desk_neighbor` (`:825`), new `desk_layout` / `desk_divider`, `json_f32_field`
- Delete: none

## Implementation Steps

1. Add `layout.rs` + unit tests (synthesize / split / tiles / neighbor / divider / clamp / 1×N and 2×2 BSP).
2. `mod layout` in `main.rs`.
3. Put `Layout` on `Tab`; init Leaf at both create sites.
4. `split_pane`: after successful spawn, `split_leaf`. If leaf missing, `ensure_layout` then split.
5. Dispatch `desk.layout`, `desk.divider`; extend `desk.neighbor`; `no_focus` on attach.
6. Integration: isolated server, split right, `desk.layout` two cells, `w` sums to `cols`; split down on left; divider moves `x`; spatial neighbor; attach `no_focus` does not change `snapshot` focused **or** occupant `seen`; default attach still focuses + seen.
7. Run `cargo test --offline --locked`. Keep `p5_attach::desk_tree_lists_split_siblings` and `desk.neighbor` next.

## Todo

- [x] `layout.rs` BSP + tiling + neighbor + divider
- [x] `Tab.layout` + split writes tree
- [x] `desk.layout` / `desk.divider` / spatial neighbor / attach `no_focus`
- [x] `p5_layout.rs` + full suite green

## Success Criteria

- [x] Two-pane right split: cells `x+w` abut; no overlap/gap
- [x] `desk.divider` changes ratio; next `desk.layout` moves the edge
- [x] `desk.neighbor` `next` still returns the other pane (`p5_attach.rs:204`)
- [x] `desk.neighbor` `left` from the right pane returns the left (same tab)
- [x] Attach `no_focus` leaves `world.focused` and occupant `seen` alone
- [x] Default attach still focuses and may set `seen` (today)
- [x] `pane.split` CLI JSON unchanged; IDs still `wN:pN`
- [x] `cargo test --offline --locked` pass

## Risk Assessment

| Risk | L×I | Mitigation | If broken |
|---|---|---|---|
| `ensure_layout` Right-chain ≠ historical down-splits | L×M | New binary restarts empty World; synthesize only on mismatch | Operator restarts server |
| `json_f32_field` misses `0.5` / `1` | M×H | Accept `N` and `N.N`; unit test | Add parser cases, no serde |
| Spatial neighbor needs cols; desk forgets | L×M | Desk always sends content size (P3) | RPC errors without cols |
| File clash with P2 | — | P1 does not edit `desk.rs` | — |

Signal to replan: tiling cannot stay exclusive integers without a gap column. Then add optional `gap` in `desk.layout` — do not invent it now.

## Security Considerations

Same-uid socket only (`handle_client` `server.rs:186`). New ops are local geometry. Ratio clamp avoids 0-size PTY ioctl storms. No remote.

## Rollback

Revert `layout.rs`, `mod layout`, `Tab.layout`, new match arms. `panes.push` split remains.

## Next Steps

Phase 2 paints N tiles from `desk.layout`. Do not start TUI work here.
