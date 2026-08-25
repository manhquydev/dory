---
title: Desk sidebar status color
date: 2026-08-25
summary: "Spaces + Agents status color from fields; gold ● wide focused only; clip lead; BLOCKED_FG 232,96,88."
---

# Desk sidebar status color

## What happened

Cook 2109 sơn màu trạng thái trên cột Spaces và Agents từ field `st`, không parse `text`. `SideHit` giữ `lead`/`st`/`tail`. Sit: Workspace|Agent → `sidebar_row_spans` (2–3 Print); Chrome|Rule → một Print + `sidebar_paint_text`.

Compact không vàng: glyph/chữ đầu `occ` = `status_fg`. Wide focused Space: `●`+label `FOCUSED_FG`; chữ st ACCENT/BLOCKED_FG/MUTED. Clip lead; reserve mid+tail bằng `display_width`.

Red-team + validate đã khóa trước cook. Compact từng reserve `" working"` — sửa: compact không mid.

## Decision

Giữ gold `●` wide only. Không nhuộm hàng. Không hex Herdr. `BLOCKED_FG` = 232,96,88. `working` = ACCENT. Một writer `desk.rs`.

## Evidence

`cargo test --offline --locked` 168/168. Review accept 8/10 · 0 critical. Plan `dory/260825-1409` completed 7/7.

## Next steps

Land `desk.rs` only — leftover attach/server/main/README/`p5_attach.rs` không gộp. 1145 close-ghost vẫn sequential trên cùng file. Không commit trừ khi hỏi. Sit TTY ngoài factory.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
