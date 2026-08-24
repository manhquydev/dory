---
title: "Phase 1: Paint hold"
status: done
phase: 1
priority: P1
effort: 3h
dependencies: []
---

# Phase 1: Paint hold

## Overview

Chỗ ngồi đứng yên khi gõ. Gutter wrap đã nằm trong `desk.rs` chưa commit. Thêm dirty-paint: chrome không vẽ lại mỗi byte PTY.

## Requirements

- Functional: title/footer/tile không chiếm cột host cuối. Motion chuột không drag → không redraw. Phím vào ô focus không `draw()` cả sidebar.
- Non-functional: vẫn `crossterm` + `vt100`. Không TTY trong pane factory.

## Architecture

`loop_ui` hôm nay: mọi `Event::Key` set `progressed = true` → `draw()` cả chrome (`desk.rs:210-237`). Phải **xóa** assignment đó. Thay `dirty` + `progressed` bằng `chrome_dirty` / `tiles_dirty`. `write_pty` không set chrome. `pump_pty` chỉ `tiles_dirty`. Prefix / layout / resize / tree-sig → chrome.

Working tree đã có: `DisableLineWrap`, `bar_line` `cols-1`, `pane_size` `cols - SIDEBAR - 2`, unit tests.

## Related Code Files

- Modify: `rust/src/desk.rs` (`loop_ui`, `draw`, `draw_tiles`, `handle_key`, `handle_mouse`)
- Test: `desk::tests` (đã có gutter); thêm “key to pty does not require chrome path” nếu tách được helper

## Implementation Steps

1. Giữ vá wrap. Xác nhận tests `bar_line_leaves_last_column`, `pane_size_leaves_host_gutter`.
2. Sửa `loop_ui`: Key→PTY không set `progressed`. Chỉ `draw_title`/`draw_sidebar`/`draw_footer` khi `chrome_dirty`. Helper test: key PTY không đi chrome path.
3. `MouseEventKind::Moved` không drag: return sớm (đã có một phần).
4. `cargo test --offline --locked`.
5. Checklist TTY cho operator (không chạy `dory` trong Herdr pane).

## Todo

- [x] Dirty split chrome / tiles
- [x] Suite xanh
- [x] Ghi checklist ngồi TTY trong phase success (không giả vờ đã ngồi)

## Success Criteria

- [x] Gõ một chữ không queue title/sidebar/footer trừ khi prefix/tree đổi
- [x] Cột cuối host trống trên title/footer/content
- [x] Suite xanh

## Operator TTY checklist (not run from factory/agent pane)

Sit on a real tty outside Herdr/agent, then:

1. `dory` — title + tab chips + Spaces/Agents + one live pane; last host column empty.
2. Type in the pane — chrome (title/sidebar/footer) stays put; host does not wrap-scroll.
3. `Ctrl-b w` opens picker (does not mint). `Ctrl-b Shift-n` mints a workspace.
4. `Ctrl-b n/p` changes tabs; `dory attach --plain` `n/p` still walks panes.
5. `Ctrl-b z` zooms; sibling output still moves after unzoom.
6. Drag ≥2 cells — footer `copied`. Click once — no copy.
7. `Ctrl-b q` detaches; `dory pane list` still shows the pane.

Factory/agent pane must not run `dory` or `herdr` (steals TTY). This cook did not sit that checklist.

## Risk Assessment

TTY trong agent pane làm mất TTY — **cấm**. Signal: host cuộn alternate screen. Response: nới gutter, không thêm Ratatui.
