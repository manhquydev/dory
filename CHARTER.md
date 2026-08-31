# Dory Charter

Formerly `HIEN-PHAP.md`. Citations of that path still bind.

# Hiến pháp Dory

- **Status:** Accepted (operator 2026-08-21 — tên riêng `dory`; hình **B** từ 2026-08-22)
- **Tên sản phẩm / lệnh / kho:** Dory / `dory`
- **Họ (poster):** Flow — cạnh `flow-skill` và `flow-deck`. Lệnh không phải `flow`.
- **Tên tạm đã nghỉ:** Xưởng / `xuong` (quá tiếng Việt; không dùng lại).
- **Đích:** local Agent Operating Environment — [north star](plans/reports/260822-north-star-aoe.md)
- **Lịch:** [CAPACITY-FREEZE.md](CAPACITY-FREEZE.md) (điều kiện 2, 2026-08-22)

Đọc tờ này lạnh trước khi thêm file nào khác vào thư mục này.

WHERE hiện tại (paid / unpaid, leftover door): [docs/README.md](docs/README.md).

## Bốn hộp (đừng gộp)

| Hộp | Việc | Nhà | Trạng thái |
|---|---|---|---|
| Thẩm phán | Quyết đúng/sai, giữ biên lai | `flow-skill` | Sống. **Bảo trì** 6–12 tháng (freeze). |
| Bảng | Chiếu trạng thái thẻ | `flow-deck` | Đóng băng. Không tính năng mới. |
| Máy phiên | Session OS: chọn thư mục, chạy phiên, sửa file, ra lệnh, ủy thác, hỏi fail-closed; nhật ký là sự thật; web chiếu log | Dory — động cơ 1 | Cổng mở. Phase 1 là mile đầu. |
| Cửa sổ chỗ làm | Workplace OS: cửa sổ → thẻ → ô; máy chủ giữ PTY; một agent điều khiển agent khác | Dory — động cơ 2 | Cổng mở. Hàng: `rust/` + `skills/dory`. Không phải web. |

Dory là **một gia đình, hai động cơ**. Không phải một đống. Không phải nâng cấp deck. Không phải nâng cấp flow.

## Mũi tên

```
flow-skill  (flow.sh; cổng và biên lai)
     ▲
     │  Dory gọi flow.sh như mọi máy chủ lạ — KHÔNG BAO GIỜ ngược
     │
Dory  (Session OS + Workplace OS)
```

- flow-skill **không** được chứa chữ `dory`, chữ xưởng, hay chữ deck.
- flow-deck **không** được chứa máy phiên hay máy cửa sổ.
- Vắng Dory **không** được làm hỏng cổng flow.

## Hàng và nhà máy

- **Nhà máy** dùng môi trường xây riêng để *xây* Dory.
- **Hàng ra khỏi Dory** cấm gọi CLI chỗ làm ngoại lai lúc chạy. Học, không thuê vòng chạy.

## Cổng chết — commit động cơ

Cấm thêm `package.json`, `go.mod`, vòng agent, máy cửa sổ / PTY, nhân plugin, cho đến khi **cả hai** đúng:

1. Hai báo cáo so sánh đã ký, viết **trong repo này**, không viết trong `flow-deck`. — **đã đủ** 2026-08-21.
2. Đủ người (hơn hai người full-time hoặc tiền tương đương) **hoặc** capacity-freeze memo: ngừng tính năng flow-skill 6–12 tháng. — **đã đủ** 2026-08-22: [CAPACITY-FREEZE.md](CAPACITY-FREEZE.md) (operator grant → CTO authored).

Cả hai đúng. `/ak:plan` được phép. Engine commits được phép **trong repo này**, dưới điều kiện giết.

## Điều kiện giết

Tờ này chết — và phải dừng mã — nếu bất kỳ điều nào:

- Nhét Dory vào repo hoặc lệnh `flow-deck`.
- Nhét Dory vào `@manhquy/flow-skill` hoặc cây skill của flow.
- Hàng xuất Dory gọi CLI chỗ làm ngoại lai như vòng chạy thật.
- Có commit động cơ trước khi hai báo cáo xia được ký. (lịch sử; không mở lại)
- Xia chạy từ cửa sổ `flow-deck` rồi ghi báo cáo vào `flow-deck/plans/`.

## Hình đã chọn

**B — Mở động cơ.** Đích AOE trên tường. flow-skill bảo trì. Deck đóng băng. Dory nhận lịch.

Cấm hình C (một tên ôm hết). Quay hình A chỉ nếu freeze bị rút trước review.

## Học (đã xong 2026-08-21, cửa sổ `dory`)

1. Đóng cửa sổ `flow-deck`. Mở thư mục này. — xong
2. Báo cáo so sánh thứ nhất — xong, ký: `plans/reports/260821-1416-xia-compare-deepseek-harness.md`
3. Báo cáo so sánh thứ hai — xong, ký: `plans/reports/260821-1436-xia-compare-herdr.md`
4. `--port` / `--copy` / `--fast` vẫn cấm (học ≠ thuê mã nguồn).

Không giao học thứ ba. Máy mở từ Phase 1.

## Một câu

Xây Dory ở kho `dory` thành AOE; flow giữ cổng, deck giữ bảng — không gộp, không thuê vòng chạy của người ta.
