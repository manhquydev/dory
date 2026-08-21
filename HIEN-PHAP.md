# Hiến pháp Xưởng

- **Status:** Accepted (operator 2026-08-21 — chấp nhận tên riêng; hình A)
- **Tên nội bộ:** Xưởng
- **Slug / kho / lệnh:** `xuong` (không có tiền tố `flow-`)
- **Poster:** được nói “gia đình Flow”. Lệnh, kho, cây skill: không.

Đọc tờ này lạnh trước khi thêm file nào khác vào thư mục này.

## Bốn hộp (đừng gộp)

| Hộp | Việc | Nhà | Trạng thái |
|---|---|---|---|
| Thẩm phán | Quyết đúng/sai, giữ biên lai | `flow-skill` | Sống. Việc chịu tải tháng này. |
| Bảng | Chiếu trạng thái thẻ | `flow-deck` | Đóng băng. Không tính năng mới. |
| Máy phiên | Chọn thư mục, chạy phiên, sửa file, ra lệnh, ủy thác, hỏi trước việc nguy hiểm; nhật ký phiên là sự thật | Xưởng — động cơ 1 | Chưa viết. Học bằng xia `--compare` (Harness). |
| Cửa sổ chỗ làm | Cửa sổ → thẻ → ô; máy chủ giữ tiến trình thật; một agent điều khiển agent khác | Xưởng — động cơ 2 | Chưa viết. Học bằng xia `--compare` (Herdr). |

Xưởng là **một gia đình, hai động cơ**. Không phải một đống. Không phải nâng cấp deck. Không phải nâng cấp flow.

## Mũi tên

```
flow-skill  (flow.sh; cổng và biên lai)
     ▲
     │  Xưởng gọi flow.sh như mọi máy chủ lạ — KHÔNG BAO GIỜ ngược
     │
Xưởng  (máy phiên + cửa sổ; sau này)
```

- flow-skill **không** được chứa chữ `xuong`, chữ xưởng, hay chữ deck.
- flow-deck **không** được chứa máy phiên hay máy cửa sổ.
- Vắng Xưởng **không** được làm hỏng cổng flow.

## Hàng và nhà máy

- **Nhà máy** (người, hôm nay) được dùng Herdr / Cursor / Harness để làm việc.
- **Hàng ra khỏi Xưởng** cấm gọi `dsh` hoặc `herdr` lúc chạy. Học, không thuê vòng chạy.

## Cổng chết — commit động cơ

Cấm thêm `package.json`, `go.mod`, vòng agent, máy cửa sổ / PTY, nhân plugin, cho đến khi **cả hai** đúng:

1. Hai báo cáo xia `--compare` (Harness, rồi Herdr) đã ký, viết **trong repo này**, không viết trong `flow-deck`.
2. Đủ người (hơn hai người full-time hoặc tiền tương đương) **hoặc** thư viết tay: ngừng tính năng flow-skill 6–12 tháng.

Chưa đủ thì Xưởng chỉ là giấy + học.

## Điều kiện giết

Tờ này chết — và phải dừng mã — nếu bất kỳ điều nào:

- Nhét Xưởng vào repo hoặc lệnh `flow-deck`.
- Nhét Xưởng vào `@manhquy/flow-skill` hoặc cây skill của flow.
- Hàng xuất xưởng gọi `dsh` / `herdr` như vòng chạy thật.
- Có commit động cơ trước khi hai báo cáo xia được ký.
- Xia chạy từ cửa sổ `flow-deck` rồi ghi báo cáo vào `flow-deck/plans/`.

## Hình đã chọn

**A — Ba nhà, lịch chồng.** Đích lớn trên tường. Tháng này không viết động cơ. flow-skill chịu tải. Deck đóng băng.

Cấm hình C (một tên ôm hết). Hình B (mở động cơ ngay) chỉ sau thư đốt flow-skill hoặc đủ người.

## Học (lượt sau, cửa sổ phải là `xuong`)

1. Đóng cửa sổ làm việc `flow-deck`. Mở thư mục này.
2. `/ak:xia https://github.com/deepseek-ai/deepseek-harness.git máy phiên, nhật ký, duyệt, web local --compare`
3. Ký báo cáo. Dừng.
4. `/ak:xia https://github.com/herdrdev/herdr.git cửa sổ thẻ ô, trạng thái agent, skill điều phối --compare`
5. Ký. Dừng.
6. Cấm `--port` / `--copy` / `--fast`. Cấm `/ak:plan` trước hai chữ ký.

## Một câu

Học Harness và Herdr, xây Xưởng ở kho `xuong`; flow giữ cổng, deck giữ bảng — không gộp, không thuê vòng chạy của người ta.
