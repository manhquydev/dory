# Hiến pháp Dory

- **Status:** Accepted (operator 2026-08-21 — tên riêng `dory`; hình A)
- **Tên sản phẩm / lệnh / kho:** Dory / `dory`
- **Họ (poster):** Flow — cạnh `flow-skill` và `flow-deck`. Lệnh không phải `flow`.
- **Tên tạm đã nghỉ:** Xưởng / `xuong` (quá tiếng Việt; không dùng lại).

Đọc tờ này lạnh trước khi thêm file nào khác vào thư mục này.

## Bốn hộp (đừng gộp)

| Hộp | Việc | Nhà | Trạng thái |
|---|---|---|---|
| Thẩm phán | Quyết đúng/sai, giữ biên lai | `flow-skill` | Sống. Việc chịu tải tháng này. |
| Bảng | Chiếu trạng thái thẻ | `flow-deck` | Đóng băng. Không tính năng mới. |
| Máy phiên | Chọn thư mục, chạy phiên, sửa file, ra lệnh, ủy thác, hỏi trước việc nguy hiểm; nhật ký phiên là sự thật | Dory — động cơ 1 | Chưa viết. Học bằng xia `--compare` (Harness). |
| Cửa sổ chỗ làm | Cửa sổ → thẻ → ô; máy chủ giữ tiến trình thật; một agent điều khiển agent khác | Dory — động cơ 2 | Chưa viết. Học bằng xia `--compare` (Herdr). |

Dory là **một gia đình, hai động cơ**. Không phải một đống. Không phải nâng cấp deck. Không phải nâng cấp flow.

## Mũi tên

```
flow-skill  (flow.sh; cổng và biên lai)
     ▲
     │  Dory gọi flow.sh như mọi máy chủ lạ — KHÔNG BAO GIỜ ngược
     │
Dory  (máy phiên + cửa sổ; sau này)
```

- flow-skill **không** được chứa chữ `dory`, chữ xưởng, hay chữ deck.
- flow-deck **không** được chứa máy phiên hay máy cửa sổ.
- Vắng Dory **không** được làm hỏng cổng flow.

## Hàng và nhà máy

- **Nhà máy** (người, hôm nay) được dùng Herdr / Cursor / Harness để làm việc.
- **Hàng ra khỏi Dory** cấm gọi `dsh` hoặc `herdr` lúc chạy. Học, không thuê vòng chạy.

## Cổng chết — commit động cơ

Cấm thêm `package.json`, `go.mod`, vòng agent, máy cửa sổ / PTY, nhân plugin, cho đến khi **cả hai** đúng:

1. Hai báo cáo xia `--compare` (Harness, rồi Herdr) đã ký, viết **trong repo này**, không viết trong `flow-deck`. — **đã đủ** 2026-08-21.
2. Đủ người (hơn hai người full-time hoặc tiền tương đương) **hoặc** thư viết tay: ngừng tính năng flow-skill 6–12 tháng. — **chưa**.

Chưa đủ điều kiện 2 thì Dory chỉ là giấy + học. Cấm `/ak:plan`.

## Điều kiện giết

Tờ này chết — và phải dừng mã — nếu bất kỳ điều nào:

- Nhét Dory vào repo hoặc lệnh `flow-deck`.
- Nhét Dory vào `@manhquy/flow-skill` hoặc cây skill của flow.
- Hàng xuất Dory gọi `dsh` / `herdr` như vòng chạy thật.
- Có commit động cơ trước khi hai báo cáo xia được ký.
- Xia chạy từ cửa sổ `flow-deck` rồi ghi báo cáo vào `flow-deck/plans/`.

## Hình đã chọn

**A — Ba nhà, lịch chồng.** Đích lớn trên tường. Tháng này không viết động cơ. flow-skill chịu tải. Deck đóng băng.

Cấm hình C (một tên ôm hết). Hình B (mở động cơ ngay) chỉ sau thư đốt flow-skill hoặc đủ người.

## Học (đã xong 2026-08-21, cửa sổ `dory`)

1. Đóng cửa sổ `flow-deck`. Mở thư mục này. — xong
2. Xia Harness `--compare` — xong, ký: `plans/reports/260821-1416-xia-compare-deepseek-harness.md`
3. Xia Herdr `--compare` — xong, ký: `plans/reports/260821-1436-xia-compare-herdr.md`
4. Cấm `--port` / `--copy` / `--fast`. Cấm `/ak:plan` cho đến điều kiện 2 của cổng chết.

Không giao học thứ ba. Không mở máy. Tab Xia để idle.

## Một câu

Học Harness và Herdr, xây Dory ở kho `dory`; flow giữ cổng, deck giữ bảng — không gộp, không thuê vòng chạy của người ta.
