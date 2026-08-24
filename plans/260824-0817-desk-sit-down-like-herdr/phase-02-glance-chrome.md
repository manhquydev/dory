---
title: "Phase 2: Glance chrome"
status: done
phase: 2
priority: P1
effort: 5h
dependencies: [1]
---

# Phase 2: Glance chrome

## Overview

Sidebar Spaces + Agents và một hàng tab chip — rollup từ `desk.tree` hiện có. Không RPC mới trừ khi thiếu field.

## Requirements

- Functional: card rollup từ **child `p.st`** (w/t không có `occ`/`st`). Hạng: `blocked` > `working` > `done` > `idle` > `unknown` (unknown ≠ idle, không ẩn). Click card/agent/chip = `pane.focus` qua **hit-list** (không `rows[row-1]`). `prefix+b` collapse. `prefix+Ctrl-b` (và `prefix` lần hai) vẫn `write_pty(&[0x02])`.
- Non-functional: fact trên socket; card chỉ `desk.rs`. Không detect farm.

## Architecture

`desk.tree` items: `w`, `t`, `p` + `occ`/`st`. Client:

```text
title
tab chips  (tabs of current workspace)
sidebar    Spaces (workspaces + rollup)
           ─
           Agents (p with occ)
tiles
footer
```

`Desk` chrome metrics (không `const SIDEBAR`): `sidebar_cols` 22|4|0, `top_rows` = title+tab bar. Mọi `pane_size`, `handle_mouse`, `draw_tiles`, `place_cursor` đọc metrics. Collapse → `reconcile_tiles` (PTY resize). Hit-list rect → `focus_pane` cùng lúc với paint.

## Related Code Files

- Modify: `rust/src/desk.rs` (`pane_size`, `handle_mouse`, `draw_*`, `place_cursor`, `prefix_cmd` `b`)
- Test: helpers group + rollup từ JSON tree fixture (không TUI)

## Implementation Steps

1. Helpers: `rollup(st)`, `tabs_of(ws)`, `agents_from(rows)`.
2. Vẽ tab bar; click chip → `pane.focus` pane đầu thẻ.
3. Sidebar hai panel; click agent → pane của occupant.
4. `prefix+b` toggle. `pane_size` theo sidebar + tab row.
5. Suite + visual unit (string width), không ngồi TTY trong CI.

## Todo

- [x] Rollup + tab chips + collapse
- [x] Click paths
- [x] Tests helpers

## Success Criteria

- [x] Hai workspace: card blocked thắng working
- [x] Chip thẻ đổi `desk.layout` tab (qua focus)
- [x] Collapse trả cột cho lưới
- [x] Suite xanh

## Risk Assessment

Cây topology quen vs Spaces — operator Herdr muốn radar. Signal: “không thấy blocked”. Response: Agents panel luôn hiện occupant `blocked` trước, không thêm scrape.
