---
phase: 2
title: "Phase 2: Desk paints N tiles"
status: completed
priority: P1
effort: 4h
dependencies: [1]
---

# Phase 2: Desk paints N tiles

## Overview

Desk becomes a grid client: one `pane.attach` + `vt100::Parser` **per visible cell**. Click a tile focuses it. Still no drag / hjkl (Phase 3).

## Context

- Today one stream / one parser: `rust/src/desk.rs:117-119`
- `attach_pane` **drops** the only stream then reconnects (`desk.rs:266-270`)
- Paint one rect: `draw_pane` `desk.rs:608` origin `SIDEBAR+1`
- `pane_size` is the full content box (`desk.rs:898`) — becomes the `desk.layout` request size
- `open_attach` handshake `desk.rs:728`; leftover dropped `desk.rs:743`
- `take_attach` focus steal fixed in P1 via `no_focus`
- `HeldPty::attach_io` is Arc clone — N clients per pane already legal (`pty.rs:259`)

## Requirements

- Functional: after split, both (all) active-tab panes paint live; click tile → `pane.focus` + keyboard to that stream; sidebar click still works; `--plain` untouched
- Non-functional: no ratatui; detach ≠ kill; hidden tabs not attached; `cargo test --offline --locked`

## Architecture

```text
struct Tile {
  id: String,
  x: u16, y: u16, w: u16, h: u16,   // content-relative
  stream: Option<UnixStream>,
  parser: vt100::Parser,
}
struct Desk { tiles: Vec<Tile>, focused: String, /* sidebar state unchanged */ }
```

**Attach policy:** N sockets. Not a multiplex (new framing; refuse unless N-attach fails).

**Reconcile** (on split / tab change / 400ms tree tick if pane set changes):

1. `desk.layout` with `pane_size(cols, rows)`
2. For each cell: compute inset rect; keep stream if id+inset size match; else `pane.resize` to inset + `parser.set_size`
3. Open missing: `open_attach(id, w, h)` with **`no_focus: true`** except the focused id (or: all `no_focus` + explicit `pane.focus` once)
4. Drop streams whose id left the cell set (do not kill PTY)
5. Keyboard `write_pty` → `tiles[focused].stream` only
6. `pump_pty` reads **every** tile (nonblocking)

**Click (content area):** `cell_at(mx - SIDEBAR - 1, my - 1)`. On divider line: no focus change (P3 owns drag). On cell: `pane.focus` + set `focused`; do **not** drop siblings.

**Focus switch vs today:** `attach_pane` must stop being “close the only pipe.” Rename path: `focus_tile` (RPC focus) vs `reconcile_tiles` (sockets).

**Leftover (High):** after ack `read_line`, if `reader.buffer()` nonempty, `parser.process(&leftover)` **then** `into_inner`. Unix can deliver `ack\n` + replay in one read. Comment at `desk.rs:743` is optimistic. Server leftover→PTY (`server.rs:222`) unchanged.

**Resize:** one `desk.layout`, then per-leaf `pane.resize` only if `(w,h)` changed. Terminal `Resize` event: coalesce to the latest size in the same loop turn.

**Tab switch:** drop all tiles, layout new tab, attach its cells.

**Zoom:** not this phase.

## Data flow

```text
split / tree change
  → pane.split (P1 tree)
  → desk.layout
  → N attach (no_focus) + pane.resize
  → N proxy_attach threads (server.rs:172)
  → pump all parsers → draw each cell at (SIDEBAR+1+x, 1+y)
click cell → pane.focus → focused id → keys to that stream
Ctrl-b q → drop all streams; World/PTYs stay
```

## Related Code Files

- Modify: `rust/src/desk.rs` only (struct, attach, pump, draw, mouse, split/neighbor follow-up)
- Create: none (hit-test lives in `layout.rs` from P1; desk may parse cells or call a tiny local `cell_at` on the RPC list)
- Delete: none
- Do **not** edit `server.rs` / `attach.rs` / `main.rs` this phase

## Implementation Steps

1. Replace `stream`+`parser` with `tiles: Vec<Tile>`.
2. `open_attach`: add `no_focus`; **forward BufReader leftover into parser**.
3. `reconcile_tiles` from `desk.layout` JSON (parse `cells` like `parse_items` `desk.rs:747` — hand-rolled, no serde).
4. `draw_pane` → `draw_tiles`: **inset** each cell before paint/resize — steal last col of A when `A.x+A.w == B.x`, last row of A when `A.y+A.h == B.y`. PTY `(w,h)` = painted rect. Draw the stolen line as divider chrome (no overdraw of PTY cells). `desk.layout` cells stay exclusive (no `gap`).
<!-- Updated: Validation Session 1 - shrink PTY, not overdraw -->
5. `handle_mouse`: content click → focus tile; sidebar unchanged (`desk.rs:385`).
6. `place_cursor` uses focused tile origin + that parser cursor.
7. Footer/title: mention click tiles. Prefix hint can stay until P3.
8. Tests: leftover handshake feeds parser; `cell_at` mapping; existing `parse_tree_walks_split_siblings` (`desk.rs:1008`).
9. `cargo test --offline --locked`.

## Todo

- [x] Multi-tile attach + paint
- [x] Click-to-focus without dropping siblings
- [x] Leftover replay into parser
- [x] Resize coalesce; hidden tabs not attached

## Success Criteria

- [x] After `Ctrl-b v`, both panes visible and live (not only the new one); focus lands on the **new** pane (desk already sends `no_focus:false`)
- [x] Click the other tile: next keys go there; sibling keeps updating
- [x] Sidebar click still focuses / can change tab
- [x] `Ctrl-b q` detach; `pane list` still shows both
- [x] `--plain` path untouched
- [x] Suite green

## Risk Assessment

| Risk | L×I | Mitigation | If broken |
|---|---|---|---|
| N sockets + N `proxy_attach` threads | M×M | Active tab only; drop on tab switch | Soft-cap later; do not multiplex this slice |
| Leftover drop loses replay | H×H | Process `buffer()` into parser | First paint blank until next output — fix handshake |
| `take_attach` without `no_focus` steals focus | H×H | P1 flag; attach focused last as belt | Focus lands on last cell — bug |
| Resize storm on every split | M×M | Skip unchanged `(w,h)` | Debounce 50ms if still chatty |
| BufReader leftover on **server** writes keys into PTY | L×H | Keep one `writeln` JSON line | Do not batch extra bytes on the attach socket |

## Security Considerations

Same peer-uid. More fds = more attach threads writing the same `AttachIO` mutex (`pty.rs:316`). Keyboard isolation is client-side: only focused stream `write_all`. Do not send keys to all tiles.

## Rollback

Revert `desk.rs`. Daemon layout RPCs remain; humans use `--plain` / one-PTY desk.

## Next Steps

Phase 3: drag `desk.divider`, prefix `hjkl`, optional `z`.
