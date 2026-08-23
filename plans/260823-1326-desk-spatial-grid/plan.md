---
title: "Desk spatial grid"
description: "Daemon keeps a BSP split tree; desk paints N live PTYs on one sit-down face."
status: completed
priority: P1
effort: 14h
branch: main
tags: [feature, dory, desk, tui]
blockedBy: []
blocks: []
created: 2026-08-23
---

# Desk spatial grid

## Overview

Close the sit-down gap vs Herdr: after `pane.split`, **one desk face** shows every live pane in the **active tab**, tiled. Daemon owns geometry (axis + ratio). Desk stays a **socket client** (crossterm + vt100). Not a Herdr clone.

Successor slice of `plans/260822-0847-workplace-skill-mux/` — crate already has mux verbs; 0847 paper phases stay untouched. Also do **not** flip `260823-0011-close-coding-occupancy` or `260823-0859-section-11-real-repo` checkboxes.

## Outcome

Gõ `dory` → nhiều ô sống cùng tab. Tách / kéo divider / click ô như chỗ ngồi. Không còn một ống + cây.

## Evidence (do not invent)

- Compare: `plans/reports/260823-1328-compare-dory-herdr-now.md`
- Herdr docs (no source): `plans/reports/260823-1320-research-herdr-sit-down.md`
- Desk shipped (one PTY on purpose): `plans/reports/260823-1318-desk-tui.md`
- Xia learn-only: `plans/reports/260821-1436-xia-compare-herdr.md`
- CHARTER.md: no `herdr`/`dsh` exec; TUI not identity

## Why now (crate truth)

- `Tab { id, root_pane, panes: Vec<Pane> }` — `rust/src/server.rs:66`
- `split_pane` **pushes** a pane; `direction` returned not stored — `server.rs:629-669`
- `desk.tree` flat walk — `server.rs:756`
- `desk.neighbor` list wrap — `server.rs:825`
- Desk: `SIDEBAR = 22`; one `UnixStream`; one `vt100::Parser` — `desk.rs:22`, `desk.rs:117-119`
- `pane.attach` = dedicated socket, replay + live — `server.rs:314`, `server.rs:857`
- `take_attach` **steals focus** — `server.rs:736`
- Handshake leftover: server writes leftover into PTY (`server.rs:222`); client `into_inner` drops BufReader tail (`desk.rs:743`)
- Envelope hand-rolled, no serde — `envelope.rs:1`

## Design (own BSP, textbook)

```text
enum SplitDir { Right, Down }
enum Layout {
  Leaf { pane: String },                    // wN:pN
  Split { dir: SplitDir, ratio: f32, a, b } // a = left/top
}
Tab { id, root_pane, panes, layout: Layout }
```

- New tabs: `Leaf { root_pane }`. Sites: `create_workspace` `server.rs:413`, `create_tab` `server.rs:439`.
- Compat: `ensure_layout` — if leaf-set ≠ `panes` ids, synthesize chain of `Split{Right, 0.5}` in vec order.
- `pane.split` replaces the focused **leaf** with `Split{dir, 0.5, Leaf{old}, Leaf{new}}`, then `panes.push` (keep ID law).
- Integer tiles: exclusive, no gap. `a = round(ratio*span).clamp(1, span-1)`. Content box = client cols/rows.
- Min ratio `0.05..=0.95`. Degenerate span `< 2` → one child gets all.

### Wire (hand-rolled JSON)

**`desk.layout`** — active tab (or `"tab"`) given content cols/rows:

```json
{"op":"desk.layout","cols":80,"rows":22}
{"op":"desk.layout","tab":"w1:t1","cols":80,"rows":22}
```

```json
{"ok":true,"result":{"tab":"w1:t1","focused":"w1:p2","cols":80,"rows":22,"cells":[
  {"id":"w1:p1","x":0,"y":0,"w":40,"h":22,"occ":"","st":""},
  {"id":"w1:p2","x":40,"y":0,"w":40,"h":22,"occ":"coder","st":"working"}
]}}
```

Missing `tab` → tab of `world.focused`. Missing cols/rows → error. `occ`/`st` same as `desk.tree`.

**`desk.divider`** — drag ratio (locked name):

```json
{"op":"desk.divider","a":"w1:p1","b":"w1:p2","ratio":0.4}
```

Lowest `Split` with `a` in subtree A and `b` in B. Swapped → set `1-ratio` on canonical A. Clamp. New `json_f32_field` next to `json_u16_field` (`server.rs:1650`).

**`pane.attach`** — add `"no_focus":true` for background tiles. Default `false` (today: attach focuses **and** sets `occ.seen = true`, `server.rs:736-742`). Validation S1: `no_focus` skips **both** `world.focused` and `seen`. Same flag name as existing `pane.split` `no_focus` (default true, `server.rs:288`) — two ops, one meaning. Do not add `steal_focus` / `mark_seen`.

**`desk.neighbor`** — keep `next`/`prev` global list wrap. Add `left|right|up|down` (same tab, needs `cols`/`rows`). No wrap if none.

**`prefix+z` zoom** — client overlay only. Resize focused PTY to full content while zoomed; unzoom restores cell sizes. Do not persist in `Layout`.

### Desk client

- **N attach sockets** (one per visible cell). Do not multiplex — would need a new frame on one stream; `pane.attach` already exists.
- Keyboard → focused stream only. Click tile → `pane.focus` + keyboard target; **do not** drop sibling streams.
- Active tab only. Hidden tabs keep PTYs (detach ≠ kill).
- Each leaf: `pane.resize` to the **inset** painted rect (not raw `desk.layout` cell). Coalesce one layout then N resizes; skip unchanged.
- Divider chrome: `desk.layout` tiles stay exclusive (no `gap` field). Desk **insets before paint/resize**: steal the last col of A on a vertical shared edge (`A.x+A.w == B.x`), last row of A on a horizontal edge. PTY `(w,h)` = painted rect, not the raw cell. Hit-test that stolen line (P3 drag). Validation S1.
- After attach ack: if `BufReader.buffer()` nonempty, feed **parser** (Unix can coalesce ack+replay). Today that tail is dropped (`desk.rs:743`).

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Layout model and RPCs](./phase-01-start.md) | Completed |
| 2 | [Desk paints N tiles](./phase-02-desk-paints-n-tiles.md) | Completed |
| 3 | [Drag ratio and hjkl](./phase-03-drag-ratio-and-hjkl.md) | Completed |
| 4 | [Docs and isolated proof](./phase-04-docs-and-isolated-proof.md) | Completed |

Sequential. No parallel file overlap: P1 `layout.rs`+`server.rs`; P2–P3 `desk.rs`; P4 docs + USAGE.

## Data flow

```text
human key/mouse → desk.rs
  Ctrl-b v/-  → pane.split → Layout rewrite + new PTY
  click tile  → pane.focus + keyboard = that UnixStream
  drag edge   → desk.divider → ratio → desk.layout → pane.resize each leaf
  type        → write on focused attach only
daemon World  → desk.layout cells → N × pane.attach (no_focus) + 1 focused
PTY drain     → proxy_attach thread per socket → vt100::Parser per tile → paint
Ctrl-b q      → drop streams; HeldPty stays
```

## Refuse

- Xia `--copy` / Herdr source as code. Ratatui-as-identity.
- `--kind`, marketplace, remote, named sessions, plugin host.
- Flip paper phases on `0847` / `0011` / `0859`.
- Flow gates inside Dory. serde. Windows host this slice.
- Change ID grammar (`wN` / `wN:tN` / `wN:pN`). Detach = kill.
- Multiplex attach (unless N-attach fails — then stop and replan).
- `pane.close` / radar SKU / persist zoom / split IDs.

## Compatibility

- `pane.split` result still `{pane,direction,occupant}`. CLI split/list/agent unchanged.
- `desk.tree` + `neighbor` next/prev unchanged.
- In-memory `World` — restart daemon after deploy. `ensure_layout` is defensive.
- `cargo test --offline --locked` stays the gate.

## Test matrix

| Layer | What |
|---|---|
| Unit `layout.rs` | synthesize, split leaf, tiles, neighbor, divider, clamp |
| Integration `p5_layout.rs` | RPC desk.layout / divider / spatial neighbor / no_focus attach |
| Regression | `p5_attach` tree+neighbor; `server.rs` split heuristic; USAGE tests |
| Desk unit | click → id; leftover bytes → parser |
| E2E (P4) | isolated XDG: two PTYs tiled on socket; detach leaves them |

## Rollback

| Phase | Revert |
|---|---|
| 1 | Drop `layout.rs` + new ops; `Tab` loses `layout`; old push-split |
| 2 | Desk one-stream again; daemon tree still valid |
| 3 | Drop drag/hjkl/zoom; tiles stay |
| 4 | Docs only |

## Success Criteria

- [x] After split, desk shows both (or all) panes tiled on one face; `Ctrl-b v` focuses the **new** pane (keep today’s desk `no_focus:false`)
- [x] Click a tile focuses it (keyboard follows)
- [x] `desk.layout` returns tiling rects for the active tab
- [x] Drag divider changes ratio; tiles relayout
- [x] Prefix `h/j/k/l` spatial focus; `n/p` still list-walk
- [x] Detach leaves PTYs; `server stop` still kills
- [x] Existing CLI split/list/agent still work
- [x] IDs unchanged; `DORY_ENV=1` still the skill gate
- [x] `cargo test --offline --locked` green
- [x] No ratatui; no herdr/dsh in the shipped path

## Open questions

None. Validation S1 closed: `no_focus` skips focus+seen; PTY inset on shared edge; split focuses new pane; attach flag stays `no_focus`.

<!-- slug: desk-spatial-grid -->

## Validation Log

### Verification Results
- **Tier:** Standard (4 phases → Fact Checker + Contract Verifier)
- **Claims checked:** 28
- **Verified:** 25 | **Failed:** 1 | **Unverified:** 2
- **Date:** 2026-08-23

#### Failures
1. [Contract Verifier] `take_attach` always sets `occ.seen = true` (`rust/src/server.rs:742`) in addition to `world.focused` (`:736-738`). Plan’s `no_focus` only names the focus steal. **N background attaches would mark every occupant seen** and collapse `done` → `idle`. Same assignment exists on `pane.focus` (`:711`) and another site (`:1348`). Interview Q1.

#### Unverified
1. [Fact Checker] Handshake leftover-as-PTY-replay is timing-dependent (Unix coalescing). Line `desk.rs:743` comment claims leftover is client→server; plan correctly treats that as optimistic. Not disproven.
2. [Fact Checker] Integer “no gap” tiles vs “1-cell chrome overdraw” (`plan.md` Design vs P2 draw). Geometry exclusive + paint-over is a product choice, not a false cite. Interview Q2.

#### Verified (sample)
- `Tab` struct `server.rs:66`; only two `Tab {` sites: `create_workspace` `:413`, `create_tab` `:439`
- `split_pane` push + direction not stored `server.rs:629-668`
- `desk.tree` `:756`; `desk.neighbor` list wrap `:825` (`_` → next, so `left` today means **next**)
- `pane.split` already has `no_focus` default **true** (`server.rs:288`); CLI always sends it (`main.rs:263`); desk split sends `false` (`desk.rs:453`)
- `take_attach` `:717`; dispatch `pane.attach` `:314`; `proxy_attach` `:857`
- `HeldPty::attach_io` `pty.rs:259` (Arc clone — N clients legal)
- Desk one stream/parser `desk.rs:117`; `attach_pane` `:266`; `open_attach` `:728`; `draw_pane` `:608`; `SIDEBAR` `:22`; `pane_size` `:898`
- `desk.neighbor` consumers: `desk.rs:473`, `attach.rs:312`, `p5_attach.rs:206` (only `next`/`prev`)
- `p5_attach` neighbor assert `:206`; USAGE `main.rs:15-50`

#### Contract callers (must not “update all callers”)
- `take_attach`: 1 dispatch site — signature can gain `no_focus`
- `desk_neighbor`: 3 clients above + dispatch — new steps must **not** fall through to `next`
- `Tab {`: 2 create sites only
- `pane.split` JSON result stays `{pane,direction,occupant}` — do not rename its existing `no_focus`

### Session 1 — 2026-08-23
**Trigger:** Cook review gate → operator chose validate
**Questions asked:** 4

#### Questions & Answers

1. **[Contract]** `pane.attach` always sets `occ.seen=true`. What must `no_focus` skip?
   - Options: skip focus+seen | skip focus keep seen | extra `mark_seen` flag
   - **Answer:** skip focus+seen
   - **Rationale:** N background attaches must not collapse `done` → `idle`

2. **[Architecture]** Divider 1-cell: overdraw PTY, shrink PTY, or `gap` on RPC?
   - Options: shrink PTY | overdraw | add `gap` to `desk.layout`
   - **Answer:** shrink PTY (inset last col/row of A)
   - **Rationale:** PTY size equals painted rect; RPC tiles stay exclusive

3. **[Assumption]** After `Ctrl-b v`, focus new pane or source (Herdr 0.8.2)?
   - Options: focus new | focus source | defer, default new
   - **Answer:** focus new
   - **Rationale:** Keep today’s desk `pane.split` `no_focus:false`

4. **[Scope]** Reuse `no_focus` on attach vs rename?
   - Options: keep `no_focus` | `steal_focus` | docs only
   - **Answer:** keep `no_focus` on attach
   - **Rationale:** Same meaning as `pane.split`; two ops, one name

#### Confirmed Decisions
- `pane.attach` `no_focus`: skip `world.focused` and `occ.seen`
- Desk insets A’s trailing col/row before `pane.resize` / paint; no `gap` field
- `Ctrl-b v` / `-` focuses the new pane
- Flag name stays `no_focus` (do not add `steal_focus` / `mark_seen`)

#### Action Items
- [x] Propagate S1 into phase 1–3 (and P4 proof if it attaches with `no_focus`)

#### Impact on Phases
- Phase 1: `take_attach` + test that `seen` is unchanged
- Phase 2: inset helper; resize to painted rect
- Phase 3: drag hit-test uses inset/stolen line
- Phase 4: proof still uses exclusive `desk.layout` cells (sum to cols)

### Whole-Plan Consistency Sweep
- Re-read `plan.md` + four phases after S1 edits.
- Searched: `overdraw`, `mark_seen`, `steal_focus`, `gap`, `seen`, `no_focus`, focus-new vs source.
- Unresolved contradictions: none after phase propagation below.
- Stale Q2 “overdraw vs exclusive” closed: exclusive RPC + client inset.

