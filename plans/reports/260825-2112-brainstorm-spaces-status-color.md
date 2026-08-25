---
type: brainstorm
date: 2026-08-25
time: 21:12
status: accepted
founder:
  where-color: both
  focus-gold: research → keep-gold (state mark; not dye-row)
research: plans/reports/260825-2118-research-spaces-status-color.md
mode: founder-ui
reuses: plans/reports/260825-2105-brainstorm-herdr-depth-founder.md
authority:
  - rust/src/desk.rs
  - plans/reports/260824-1148-research-herdr-src-chrome.md
feeds: /ak:plan (chrome phase; after or beside flock sit — not --kind)
---

# Brainstorm — cột Spaces đổi màu theo trạng thái agent?

## Contract (2105 giữ; thêm lớp sơn)

| Field | Closed pending founder pick |
|---|---|
| **Outcome** | Ngồi Dory, **liếc cột Spaces** biết folder nào đang bận — bằng **màu**, không chỉ chữ `working`. Không cần trông giống Herdr. |
| **Constraints** | Crossterm. Token Dory (không copy RGB Herdr). Gold `FOCUSED_FG` hôm nay = “đang ngồi folder này”. `rollup_of` đã có. Không `--kind`. Không clone Herdr. |
| **Non-goals** | Pixel Herdr. Đổi nhận diện agent. Kéo sidebar. Nút Flow. Sơn desk trống rồi gọi là xong. |
| **Acceptance** | Wide: card Spaces (hoặc chấm/chữ trạng thái) đổi màu theo rollup `blocked`/`working`/`done`. Compact: chữ `B`/`W`/`D` cùng bảng màu. Gold “đang ngồi đây” không mất nghĩa. Test rollup + màu. Gate `cargo test --offline --locked`. |

## Live — không phải ý định

**Herdr (giấy 1148):** cột Spaces lấy `aggregate_state` của agent trong folder; compact = số + icon trạng thái. Năm từ: blocked / working / done / idle / unknown. Màu **là** glance.

**Dory hôm nay (`desk.rs`):**

- Đã biết rollup (blocked thắng working). Wide: `● flow working` — chữ cùng màu TEXT. Compact: `W`/`B`/`D` — cùng màu TEXT.
- Màu **vàng** chỉ khi **đang ngồi** folder đó (`FOCUSED_FG`). Không phải “đang làm việc”.
- Hàng Agents: `occ + st + p1` cùng TEXT. Không màu theo `st`.
- Idle wide: **giấu** chữ `idle` (card sạch).

Lỗ trải nghiệm: **dữ liệu có, sơn chưa nói**. Không thiếu skill Herdr.

## Có làm được như Herdr không?

**Được phần bạn thấy** (màu = bận/rỗi). **Không** sao RGB/icon Herdr.

Hướng nhỏ: tô **chữ trạng thái** (và chữ compact `W`) theo bảng Dory. Giữ chấm `●` vàng = “đang ngồi đây”. Tránh vàng = vừa focus vừa working.

| Trạng thái | Sơn đề xuất (Dory, không copy Herdr) |
|---|---|
| blocked | ấm (đỏ/cam mới, 1 token) |
| working | ACCENT teal đã có |
| done | MUTED hoặc 1 token xanh dịu |
| idle / trống | không tô (như đang giấu idle) |
| unknown | MUTED |
| đang ngồi folder | `●` vẫn vàng |

## Ba hướng

| # | Cách | Giả định nặng | Gãy trước |
|---|---|---|---|
| A | Tô chữ trạng thái + compact `B/W/D`. `●` giữ vàng | Liếc chữ màu là đủ | Founder muốn cả hàng Spaces nhuộm |
| B | Cả hàng Spaces đổi màu theo rollup | Glance mạnh như Herdr card | Mất vàng “đang ngồi”; working = vàng đụng focus |
| C | Không sơn; chờ đàn 2050 rồi tính | Màu không cần khi chưa có agent | Founder đã thấy lỗ khi ngồi Herdr |

**Khuyến nghị A** nếu chọn “có màu”. C nếu “đàn trước”. Không B trừ khi founder chấp nhận mất vàng = đang ngồi.

## Hỏi founder (form)

1. Tô màu ở đâu? Spaces / Agents / cả hai / đàn trước đã.
2. Folder đang ngồi: giữ chấm vàng hay để màu việc nuốt cả hàng?
