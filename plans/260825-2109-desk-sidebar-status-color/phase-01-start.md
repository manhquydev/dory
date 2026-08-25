---
title: "Phase 1: Status marks"
status: completed
---

# Phase 1: Status marks

## Overview

Sơn màu trạng thái trên Spaces và Agents từ **field**, không parse `text`. Wide: `●` vàng = đang ngồi. Compact: màu = st (không vàng). Rule/Chrome không vào span path.

## Requirements

- Functional: `SideHit` thêm `st`, `lead`, `tail` (raw, **chưa** `pad_cols`). Chrome/Rule: `st=""`, `lead`/`tail` empty; paint qua `sidebar_paint_text`.
- Functional: Workspace `st = rollup_of`. Agent `st = normalize_st(&pane.st)`.
- Functional: Wide Spaces `lead = " {●|○} {label}"`, `tail=""`, mid = ` {st}` chỉ khi `st` nonempty và ≠ `idle`.
- Functional: Wide Agents `lead = " {occ}"`, mid = ` {st}` nếu show, `tail = " {short}"`.
- Functional: Compact Spaces `lead = " {B|W|D|I|U|·}"`, mid/tail empty; **toàn glyph** `status_fg` (ngồi không vàng).
- Functional: Compact Agents `lead = " {occ.chars().next()}"` — **không** đổi thành B/W/D; màu `status_fg(st)`.
- Functional: `status_fg`: blocked = `BLOCKED_FG` (Rgb ấm mới); working = `ACCENT`; done/idle/unknown/"" = MUTED.
- Functional: `focused` = `kind==Workspace && !desk_ws.is_empty() && hit.workspace == desk_ws`. Agents không vàng.
- Functional: Clip **lead** tới `side - display_width(mid) - display_width(tail)`. Concat `display_width` == `side` trước `│`.
- Non-functional: một writer `desk.rs`. Không sit factory TTY. Không `--kind`.

<!-- Updated: Red Team Session 1 - structured spans; compact no gold; Rule helper -->

## Architecture

```
sidebar_sections → SideHit { lead, st, tail, kind, workspace, text? }
draw_sidebar:
  Rule | Chrome | blank → sidebar_paint_text + 1 Print
  Workspace | Agent     → sidebar_row_spans → Print lead / mid / pad+tail
```

`text` (nếu giữ) chỉ cho test glance/hit — **không** dùng để tìm `st`.

### Print table

| Row | mid shown | lead fg | mid fg | Prints |
|---|---|---|---|---|
| Rule / Chrome | no | MUTED via helper | — | 1 |
| Wide Space empty/idle | no | gold if focused else TEXT | — | 1–2 |
| Wide Space working/blocked/done/unknown | yes | gold if focused else TEXT | `status_fg` | 3 |
| Wide Agent | yes if st≠"" && ≠idle | TEXT | `status_fg` | 3 |
| Compact Space/Agent | no | `status_fg` (· = MUTED) | — | 1 |

## Related Code Files

- Modify: `rust/src/desk.rs`
- Test: **new** fixtures (không treo `glance_rows` cho working-wins). Giữ rule/empty-shell tests.

## Implementation Steps

1. `SideHit { st, lead, tail, ... }`. `push` lưu raw lead/tail; optional `text` = pad(lead+mid+tail) cho test cũ.
2. `status_fg` + `BLOCKED_FG`.
3. `sidebar_row_spans(hit, side, focused)` — clip lead, never parse occ/cwd.
4. `draw_sidebar`: Rule/Chrome cũ; Workspace/Agent spans; `focused` predicate trên.
5. Tests: mapping; focused wide `●` vàng + word ACCENT khi chỉ `working`; compact `W` ACCENT; Agent chỉ `blocked` màu; label dài CJK clip lead; empty `·`; Rule `sidebar_paint_text`.

## Todo

- [x] Fields + constructors
- [x] spans + draw
- [x] Tests + gate

## Success Criteria

- [x] Bảng Prints
- [x] Founder both-columns
- [x] keep-gold **wide only**
- [x] Gate xanh

## Risk Assessment

- Clip lead / CJK: test label ≥15 ASCII + 1 CJK. Signal: `│` lệch. Response: `display_width`.
- 1145 leftover: sequential. Signal: other desk.rs cook. Response: stop.
- `unknown` MUTED: không hook.
