---
title: "Desk sidebar status color"
description: "Color Spaces + Agents status marks. Gold ● stays you-are-here. No Herdr RGB."
status: completed
priority: P1
effort: 2h
branch: main
tags: [feature, dory, desk, sidebar, chrome]
blockedBy: []
blocks: []
created: 2026-08-25
---

# Desk sidebar status color

## Contract

| Field | Closed |
|---|---|
| Outcome | Liếc cột Spaces **và** Agents biết trạng thái bằng **màu + chữ**. Folder đang ngồi: chấm `●` vẫn vàng. |
| Constraints | Một writer `desk.rs`. Crossterm. Token Dory (không hex Herdr). `rollup_of` / `normalize_st` giữ. Không `--kind`. Không factory TTY. Gate `cargo test --offline --locked`. |
| Non-goals | Nhuộm cả hàng. Ratatui. Clone Herdr. Hook/detect. Đàn 2050 sit. `server.rs`. Commit. 1145 close-ghost. |
| Acceptance | Phase 1. |

Nguồn: [brainstorm 2112](../reports/260825-2112-brainstorm-spaces-status-color.md) · [research 2118](../reports/260825-2118-research-spaces-status-color.md) · founder: both columns · research: keep-gold.

## Scope Challenge

```
- Existing: rollup + chữ st; vàng = cả hàng focused
- Requested: state color on Spaces + Agents; ● gold = sitting
- Complexity: 1 file, 1 phase
- Selected mode: HOLD SCOPE (research done)
```

1145 `in-progress` cùng `desk.rs` — sequential, không `blockedBy`. 1959 hairline completed.

## Approaches

A = chấm/chữ st màu; ● vàng giữ. B = nhuộm cả hàng. C = chờ đàn. **A.**

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Status marks](./phase-01-start.md) | Completed |

## Success Criteria

- [x] Wide Spaces: chữ st (nếu có, ≠ idle) màu theo rollup; `●` focused = `FOCUSED_FG`
- [x] Wide Agents: **chỉ** token st màu; `occ` và short id TEXT
- [x] Compact Spaces: `B/W/D/I/U/·` màu theo rollup (không vàng — không có `●`)
- [x] Compact Agents: chữ đầu `occ` (không đổi thành B/W/D), màu theo pane `st`
- [x] Rule/Chrome: một `Print` + `sidebar_paint_text` (không `pad_cols` `─`)
- [x] Idle/empty: không bịa màu việc; empty shell vẫn `·` không `unknown`
- [x] Gate `cargo test --offline --locked`

## Red Team Review

Vòng riêng 21:15. Lenses: Security + Assumption. User: triển khai ngay — adjudicate không hỏi từng finding.

| # | Finding | Sev | Disp | Note |
|---|---|---|---|---|
| 1 | Suffix ` {st}` sai Agents / compact / idle | Crit | **Accept** | Sơn từ field, không parse `text` |
| 2 | `clip_to` ăn đuôi st; `working-copy` không bắt | Crit | **Accept** | Clip **lead** only; reserve mid+tail |
| 3 | Rule qua `pad_cols` gãy hairline | Crit | **Accept** | Rule/Chrome giữ helper 1959 |
| 4 | Compact một glyph không vừa vàng vừa st | High | **Accept** | Compact: màu = st; vàng chỉ wide `●` |
| 5 | Tìm `st` trong `occ`/cwd | High | **Accept** | Không search tên |
| 6 | Fixture glance = blocked không phải working | High | **Accept** | Fixture mới |
| 7 | Compact Agents ≠ B/W/D | High | **Accept** | Giữ chữ `occ` |
| 8 | `display_width` 3 span + `●`/CJK | High | **Accept** | Pad theo `display_width` |
| 9 | `focused` trên `workspace==""` | Med | **Accept** | Chỉ `SideKind::Workspace` + ws nonempty |
| 10 | Hai tên helper / fallback nhuộm hàng | Med | **Accept** | Một helper `sidebar_row_spans` |
| 11 | Bảng idle/"" /unknown | Med | **Accept** | Bảng Prints trong phase |

### Whole-Plan Consistency Sweep

- Architecture phase = structured fields, không `rsplit` `text`.
- Compact sitting gold **xóa** khỏi acceptance (chỉ wide `●`).
- `pad_cols` / Rule không đổi.
- Không contradiction còn lại.

## Validation Log

Vòng riêng 21:20. Paper validate (user: triển khai ngay — không interview). Red Team đã có verification evidence → Light Fact Checker bổ sung.

### Verification Results
- **Tier:** Light
- **Claims checked:** 6
- **Verified:** 6 | **Failed:** 0 | **Unverified:** 0
- `SideHit` `desk.rs:2538` · `draw_sidebar` `1578` · `rollup_of` `2315` · `normalize_st` `2290` · `sidebar_paint_text` Rule `2521` · `clip_to` `3328`

Decisions (propagate): structured spans; compact no gold; clip lead; Rule helper. Zero contradiction.

### Whole-Plan Consistency Sweep
plan.md acceptance = phase Prints table. Ready to cook.

<!-- slug: desk-sidebar-status-color -->
