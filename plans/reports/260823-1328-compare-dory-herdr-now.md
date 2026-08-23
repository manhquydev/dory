---
type: compare
date: 2026-08-23
time: 13:28
status: done
mode: cook-research
authority:
  - CHARTER.md
  - plans/reports/260821-1436-xia-compare-herdr.md
  - plans/reports/260823-1318-desk-tui.md
  - plans/reports/260823-1320-research-herdr-sit-down.md
inspected:
  - live herdr 0.7.5 --help (TUI không mở)
  - live dory --help + desk PTY (13:06–13:18)
  - rust/src/desk.rs, server.rs Tab.panes
  - https://herdr.dev/docs/concepts/
supersedes: plans/reports/260823-1252-brainstorm-realtime-vs-herdr.md
---

# Đối sánh sống — Dory desk vs Herdr (sau TUI)

## Contract

| Field | Closed |
|---|---|
| Outcome | Chấm trung thực: Dory hôm nay trải nghiệm được tới đâu so Herdr; vì sao “khác quá / chưa bằng” |
| Constraints | Học docs + binary; không Xia `--copy`; không mở TUI `herdr` trong agent pane |
| Non-goals | Clone marketplace / `--kind` / remote; nấu mã từ tờ này |
| Acceptance | Bảng lớp cập nhật + một câu sit-down |

## Verdict

**Vòng chỗ làm Dory có.** Gõ `dory` mở desk: cây `w1/t1/p1`, một PTY sống, click sidebar, `Ctrl-b c/v/-/q`, detach ≠ kill.

**Mặt chỗ ngồi Herdr chưa có.** Herdr là **bàn không gian** (nhiều PTY cùng lúc, kéo vách, chuột là mặc định, sidebar trạng thái). Dory desk là **bộ chọn** (cây + một ống). Đó là lý do trải nghiệm lệch — không phải vì thiếu logo hay Ratatui.

Tờ 12:52 (attach thô, “đừng nấu TUI”) **cũ**. Desk 13:18 đã đóng “không còn bash trần”. Nó **không** đóng “bằng Herdr”.

## Bằng chứng

| | Herdr (docs 0.8.2 + binary 0.7.5 trên PATH) | Dory desk 13:18 |
|---|---|---|
| Gõ lệnh | `herdr` = launch/attach session nền | `dory` = desk (server tự lên) |
| Mặt | Sidebar Spaces/Agents + **lưới ô sống** | Sidebar cây + **một** PTY |
| Split | BSP; `right`/`down` **cất hình học**; kéo divider | `panes.push`; `direction` chỉ trả lời; **không lưu layout** |
| Chuột | Click ô/thẻ; kéo; right-click; copy-on-select | Click cây; wheel gửi mũi tên. Không kéo, không click ô khác trên lưới |
| Prefix | `c/v/-/hjkl/z/x/q` + zoom + close | `c/v/-/n/p/w/q` — n/p là list phẳng, không hướng |
| Năm từ | Sidebar rollup blocked/working/done/idle/unknown | Có trên socket/`report`; cây tô màu nếu occupant; không radar |
| Detach | `ctrl+b q` | `Ctrl-b q` — cùng metaphor |
| Cấm copy | marketplace, `--kind`, remote, plugin | Đúng — không có, không làm |

## Một câu

Dory hôm nay là chỗ **làm được** (daemon + skill + desk chọn ô). Herdr là chỗ **nhìn được đàn** (nhiều terminal sống trên một mặt). Operator ngồi Herdr rồi mở Dory sẽ thấy “khác quá” vì **hình học**, không vì thiếu lệnh.

## Wave kế (không nấu từ đây)

1. **Lưới** — daemon giữ cây split (ratio + hai con); desk vẽ N PTY. Đây là bar “đừng khác Herdr”.
2. **Radar** — giữ một PTY; sidebar/tab bar hiện năm từ + click agent. Rẻ hơn, vẫn là chooser.
3. **Giữ lệch** — factory ngồi Herdr để *xây*; Dory SKU = skill + CLI; desk đủ mở.

Không lấy marketplace / `--kind` / remote / source họ.
