---
title: Desk spatial grid plan
date: 2026-08-23
summary: "Successor slice: daemon BSP + N live desk tiles. 0847 paper untouched."
---

# Desk spatial grid plan

## What happened

Operator compared live Dory desk to Herdr: mux works, sit-down face is a chooser (sidebar + one PTY) because `Tab.panes` is a flat vec and `split_pane` does not store axis/ratio. Chose plan the spatial grid — not radar, not keep-the-gap.

## Decision

Own textbook BSP (`Leaf` / `Split{Right|Down, ratio, a, b}`). New RPCs `desk.layout` + `desk.divider`. N `pane.attach` sockets (not multiplex). `pane.attach` `no_focus` so tiles do not steal focus. Client leftover after attach ack must feed the vt100 parser. Zoom is client-only. Do not flip `0847` / `0011` / `0859` paper phases. No GitHub publish. No Rust this session.

Plan: `plans/260823-1326-desk-spatial-grid/` (pinned via `ak plan use`).

## Next steps

Cook Phase 1 (`layout.rs` + RPCs, no TUI). Gate: `cargo test --offline --locked`.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
