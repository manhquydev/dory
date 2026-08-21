# Dory

Chỗ làm việc cho người vibe-coding với nhiều agent. Họ trên poster: Flow. Lệnh sau này: `dory`.

**Đây chưa phải phần mềm.** Chưa có nút bấm, chưa có lệnh chạy. Trong thư mục này chỉ có giấy và bài học.

Không phải bản mới của bảng thẻ (`flow-deck`). Không phải bản mới của người chấm (`flow-skill`).

## Ba nhà, đừng gộp

| Nhà | Việc | Dùng được hôm nay? |
|---|---|---|
| **flow-skill** | Chấm đúng/sai, giữ biên lai | Có. Việc chính tháng này. |
| **flow-deck** | Bảng treo tường: thẻ nào đang làm / cổng FAIL / xong | Có. Bảng v1 xong, không thêm nghề. |
| **Dory** | Chỗ *làm*: chạy phiên agent, chia cửa sổ | Chưa. Chỉ bản vẽ. |

flow-skill quyết. flow-deck nhìn. Dory (sau này) làm. Mũi tên chỉ một chiều: chỗ làm **gọi** `flow.sh`. Flow không biết tên Dory.

Hôm nay bạn vẫn làm trên Herdr / Cursor. Đó là dụng cụ đang thuê. Dory là chỗ làm nhà mình — chưa xây.

## Đích

Hai phần, một chỗ:

1. **Phiên trên máy** — chọn thư mục dự án, chạy agent, sửa file, ra lệnh, ủy thác, hỏi trước việc nguy hiểm. Mọi thứ máy thấy phải ghi lại (nhật ký là sự thật).
2. **Cửa sổ chỗ làm** — cửa sổ → thẻ → ô. Máy chủ giữ tiến trình thật. Một agent điều khiển agent khác. Rời cửa sổ không phải tắt việc.

Hàng khi có: **không** gọi `dsh` hay `herdr` lúc chạy. Học từ họ, không thuê vòng chạy của họ.

## Xong tới đâu

| Việc | Tình trạng |
|---|---|
| Đặt tên, tách khỏi bảng và người chấm | Xong |
| Học DeepSeek Harness (phiên, nhật ký, hỏi, web local) | Xong, đã ký — [báo cáo](plans/reports/260821-1416-xia-compare-deepseek-harness.md) |
| Học Herdr (cửa sổ → thẻ → ô, trạng thái, điều phối) | Xong, đã ký — [báo cáo](plans/reports/260821-1436-xia-compare-herdr.md) |
| Chương trình chạy được (`dory serve`, cửa sổ, mở agent) | **Chưa. Không có mã máy.** |
| Chỗ vibe-coding cho người dùng | Chưa — sau khi có chương trình |

Học xong không phải sản phẩm. Sản phẩm = chương trình chạy được. Cái đó đang là **không**.

Chưa được viết chương trình cho đến khi **một** trong hai: có thêm người (hoặc tiền tương đương), hoặc bạn viết tay ngừng làm tính năng flow-skill trong 6–12 tháng.

Luật cấm (đọc khi cần chặn): [HIEN-PHAP.md](HIEN-PHAP.md).

## License

Chưa chọn bản công khai. Kho local, chỉ giấy.
