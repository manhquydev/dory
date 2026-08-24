---
title: "Phase 4: Zoom keeps streams"
status: done
phase: 4
priority: P1
effort: 2h
dependencies: [1, 2]
---

# Phase 4: Zoom keeps streams

## Overview

`prefix+z` phóng ô focus trên mặt. Anh em vẫn attach + parse. Bỏ `retain` đang cắt stream.

## Requirements

- Functional: zoom → focused vẽ full content box + `pane.resize` full; sibling giữ size cũ, `stream` còn, `pump_pty` chạy. Unzoom → `desk.layout` restore mọi ô.
- Non-functional: zoom không ghi `Layout`.

## Architecture

Hôm nay `reconcile_tiles` `wanted` chỉ chứa focus khi `zoomed` → `retain` drop tile → drop UnixStream.

Sửa: `wanted` luôn mọi cell **của tab đang vẽ**. Không `Tile.hidden` — `draw_tiles` derive `zoomed && id != focused`. Full-box = content rect sau title+tab+sidebar+gutter (metrics Phase 2), không `rows-2` cũ. `pane.resize` focus lúc zoom **và** unzoom. Hidden-tile I/O không set `chrome_dirty`. Tab switch: `zoomed = false`.

## Related Code Files

- Modify: `rust/src/desk.rs` (`Tile`, `reconcile_tiles`, `draw_tiles`, `pump_pty`)
- Test: unit “zoomed wanted ids == all cells” nếu tách helper; hoặc integration attach count (khó không TTY) — ưu tiên helper + không drop stream trong logic thuần

## Implementation Steps

1. `Tile { hidden: bool }`. Zoom set hidden trên sibling; không remove.
2. `draw_tiles` skip hidden. `write_pty` chỉ focus.
3. Unzoom `hidden=false` + resize theo inset.
4. Suite.

## Todo

- [x] Keep N streams on zoom
- [x] Tests helper

## Success Criteria

- [x] Sau `z`, `tiles.len()` không giảm
- [x] `z` lần hai khôi phục lưới
- [x] Suite xanh

## Risk Assessment

PTY zoomed size ≠ sibling — Herdr cũng vậy. Signal: TUI occupant vỡ khi unzoom. Response: resize sibling về cell, không rebuild parser (vt100 `set_size` đã dùng).
