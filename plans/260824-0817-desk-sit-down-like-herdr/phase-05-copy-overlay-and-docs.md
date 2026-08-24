---
title: "Phase 5: Copy overlay and docs"
status: done
phase: 5
priority: P2
effort: 3h
dependencies: [3, 4]
---

# Phase 5: Copy overlay and docs

## Overview

Kéo-chọn copy (OSC 52). Overlay prefix/`?`. Docs khớp phím mới.

## Requirements

- Functional: drag trên tile **≥ 2 cell** (không divider, không tile hidden/zoom-sibling); mouse-up OSC 52. Click không copy. Không nhận biết host nhận OSC — footer “copied” = đã gửi, không “copy failed”. Không `Ctrl-c`. `prefix+?` → `Mode::Help` (enum Phase 3).
- Non-functional: không crate clipboard mới nếu OSC 52 đủ. Docs: README, USAGE, `skills/dory/SKILL.md`.

## Architecture

Selection: cell range trên tile đang kéo. Help/Select chỉ **thêm** variant vào `Mode` Phase 3; `handle_key` đã chặn `write_pty`.

## Related Code Files

- Modify: `rust/src/desk.rs`, `rust/src/main.rs` USAGE, `README.md`, `skills/dory/SKILL.md`
- Test: selection text helper (fixture cells), OSC payload encode

## Implementation Steps

1. Drag-select + OSC 52 + toast footer “copied”.
2. `prefix+?` help (bảng phím phase 3).
3. Sync docs. `--plain` `n/p` vẫn ô — ghi rõ.
4. Suite. Isolated proof optional (RPC close + layout; không TUI).

## Todo

- [x] Copy-on-select
- [x] Help overlay
- [x] Docs

## Success Criteria

- [x] Unit: selection → đúng chuỗi
- [x] README bảng phím = desk
- [x] SKILL không bảo occupant dùng desk
- [x] Suite xanh

## Risk Assessment

Host không OSC 52 → copy im. Signal: không paste được. Response: footer “copy failed”; không thêm wl-copy PATH. Right-click menu = ngoài wave.
