---
type: research
date: 2026-08-25
time: 21:18
status: complete
topic: Spaces/Agents status color vs you-are-here
founder: both columns; do not self-decide gold vs dye-row — research
authority:
  - https://herdr.dev/docs/configuration/
  - https://herdr.dev/docs/config-reference/
  - https://herdr.dev/agent-guide.md
  - https://www.w3.org/WAI/WCAG21/Understanding/use-of-color.html
  - https://www.w3.org/WAI/WCAG22/Understanding/focus-visible
  - rust/src/desk.rs
  - plans/reports/260825-2112-brainstorm-spaces-status-color.md
  - plans/reports/260825-2105-brainstorm-herdr-depth-founder.md
feeds: /ak:plan chrome (state mark + word; not Herdr RGB)
---

# Research Report: màu trạng thái cột Spaces / Agents

## Executive Summary

Herdr **không nhuộm cả tên folder** bằng màu việc. Docs chính thức: mỗi hàng Spaces/Agents có **`state_icon`** (chấm màu, mặc định) + tên; “đang ngồi đây” là kênh khác (`active_row_bg` — nền hàng đang chọn). Có tùy chọn `status_indicators = "symbols"` để **hình** bổ sung **màu**.

Dory đã có chữ rollup (`working` / `B` `W`) nhưng sơn một màu; vàng = cả card khi đang ngồi — trùng kênh với “đang làm” nếu nhuộm hàng.

**Khuyến nghị (từ bằng chứng, không đoán):** tô **cả Spaces và Agents** (bạn đã chọn) bằng **chấm/chữ trạng thái màu** + **giữ chữ** `working`/`blocked`. Chấm `●` vàng (hoặc nền hàng nhẹ) = đang ngồi. **Không** nhuộm cả hàng theo việc. **Không** copy RGB/theme Herdr.

## Research Methodology

- Sources: 6 (Herdr config + config-reference + agent-guide; WCAG 1.4.1 + 2.4.7; Dory `desk.rs` + giấy 1148/2112)
- Date: docs Herdr sống 2026; WCAG 2.1/2.2
- Terms: Herdr `state_icon`, `status_indicators`, `active_row_bg`; WCAG use of color; focus visible

## Key Findings

### 1. Herdr làm gì (học docs, không clone mã)

| Việc | Herdr (docs) | Dory hôm nay |
|---|---|---|
| Glance trạng thái | Token `state_icon` — chấm màu | Không có chấm; chỉ chữ cùng TEXT |
| Chữ trạng thái | Token `state_text` (tuỳ layout) | Wide: chữ `working` nếu ≠ idle. Compact: `B/W/D/I/U` |
| Hàng Spaces mặc định | `["state_icon", "workspace"]` rồi git | `● label` + chữ st |
| Hàng Agents mặc định | `state_icon` + workspace/tab; hàng 2 = `agent` | `occ st p1` cùng TEXT |
| Đang ngồi / đang chọn | `active_row_bg` (nền), tách khỏi màu state | Cả hàng `FOCUSED_FG` vàng |
| Màu-không-đủ | `status_indicators = "dots"` \| `"symbols"` | Chỉ chữ (đủ 1.4.1) nhưng không màu |

Nguồn: [Configuration](https://herdr.dev/docs/configuration/), [Config reference](https://herdr.dev/docs/config-reference/). Theme ví dụ (Catppuccin) là **của họ** — không lấy hex.

### 2. Luật nhìn (W3C)

- **WCAG 1.4.1 Use of Color (A):** màu không được là *cách duy nhất*. Chữ `working` / `W` phải giữ. Chấm màu = thêm, không thay chữ.
- **WCAG 2.4.7 Focus Visible:** phải thấy **đang đứng chỗ nào**. Nhuộm hàng = màu việc sẽ **che** “đang ngồi” khi folder đang `working` (vàng Dory đụng vàng việc).

### 3. Best practice (gộp)

Hai kênh tách:

```
trạng thái agent  →  chấm + chữ (màu)
đang ngồi đây     →  ● vàng  hoặc  nền hàng (SIDE khác TITLE)
```

Herdr tách sẵn. Dory đang gộp “đang ngồi” vào màu chữ cả hàng.

### 4. Security / identity

Không `--kind`. Màu chỉ sơn `st` đã có (`report` / fixture). Không hook Herdr.

### 5. Performance

Sơn fg theo `rollup_of` / `normalize_st` — 0 I/O. Không đụng vòng PTY.

## Comparative Analysis

| Hướng | Trải nghiệm | Gãy |
|---|---|---|
| **A. Chấm + chữ màu; ● vàng giữ** (chọn) | Liếc như Herdr; vẫn biết đang ngồi | Yếu hơn nếu founder muốn “cả hàng cháy” |
| B. Nhuộm cả hàng | Glance mạnh, sai luật 2.4.7 khi đang ngồi + working | Mất “đây là chỗ tôi” |
| C. Chỉ đàn, chưa màu | An toàn | Bạn đã chọn cả hai cột — không C |

## Implementation Recommendations

### Cho founder

Làm **được** trải nghiệm Herdr ở chỗ bạn nêu (màu = trạng thái trên Spaces **và** Agents). Không cần giống ảnh Herdr.

Cách tốt nhất: **chấm màu cạnh tên** + chữ trạng thái giữ. Folder đang ngồi: **chấm ● vẫn vàng**.

### Cho dev (handoff plan)

1. `SideHit` mang `st` (rollup hoặc pane). `draw_sidebar` tô **chỉ** chấm/chữ st, không cả `label`.
2. Bảng Dory (không hex Herdr): `blocked` = 1 token ấm mới; `working` = `ACCENT`; `done` = MUTED hoặc 1 token; idle/empty = không chấm; `unknown` = MUTED.
3. Compact: màu chữ `B/W/D`.
4. Focus: `●` `FOCUSED_FG` khi `ws == focused`; không đổi cả hàng sang vàng.
5. Test: rollup blocked thắng working; focused card vẫn ● vàng khi st=working; không file herdr.

### Common pitfalls

- Copy `theme.custom` Herdr.
- Vàng = working **và** đang ngồi.
- Chấm không chữ (fail 1.4.1 trên compact nếu bỏ `W`).
- Sơn desk trống rồi gọi xong — cần đàn (2050) để thấy màu thật.

## Resources

- https://herdr.dev/docs/configuration/ — `state_icon`, `status_indicators`, `active_row_bg`
- https://herdr.dev/agent-guide.md — năm từ
- https://www.w3.org/WAI/WCAG21/Understanding/use-of-color.html
- https://www.w3.org/WAI/WCAG22/Understanding/focus-visible

## Next steps

`/ak:plan` phase chrome: state mark trên Spaces + Agents. Phase đàn 2050 riêng (màu không hiện nếu `occ=""`).

## Unresolved

Không. Câu “giữ vàng?” đã đóng bằng nghiên cứu: giữ. Không hỏi lại.
