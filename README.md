# Dory

Local **Agent Operating Environment** cho việc phần mềm nhiều agent. Họ trên poster: Flow. Lệnh: `dory`.

Không phải bản mới của bảng thẻ (`flow-deck`). Không phải bản mới của người chấm (`flow-skill`).

## Mở (như gõ `herdr`)

Trong một terminal thật:

```bash
dory
```

Lần đầu sẽ bật `dory server` rồi mở **desk**: cây cửa sổ → thẻ → ô bên trái, **nhiều PTY sống** của thẻ đang chọn bên phải.

| Phím / chuột | Việc |
|---|---|
| Click ô / cây | Đổi ô đang gõ |
| Kéo vách giữa ô | Đổi tỉ lệ |
| `Ctrl-b h/j/k/l` | Ô theo hướng |
| `Ctrl-b n` / `Ctrl-b p` | Ô kế / trước (danh sách) |
| `Ctrl-b c` | Thẻ mới |
| `Ctrl-b v` / `Ctrl-b -` | Tách ô phải / dưới (focus ô mới) |
| `Ctrl-b z` | Phóng ô đang chọn / trả lưới |
| `Ctrl-b w` | Cửa sổ mới |
| `Ctrl-b q` hoặc `Ctrl-b d` | Rời UI; PTY vẫn sống |

`dory attach --plain` là client PTY thô (không sidebar). Trong ô (`DORY_ENV=1`): skill vẫn gọi CLI. `dory server stop` mới tắt việc. Desk là client của socket, không phải clone TUI Herdr.

**Đích:** Session OS (nhật ký là sự thật) + Workplace OS (process sống là sự thật), gọi Flow như governance plane bên ngoài. Chi tiết: [north star](plans/reports/260822-north-star-aoe.md).

## Ba nhà, đừng gộp

| Nhà | Việc | Hôm nay |
|---|---|---|
| **flow-skill** | Chấm đúng/sai, giữ biên lai | Sống. **Bảo trì** 6–12 tháng. |
| **flow-deck** | Bảng treo tường: thẻ / cổng / xong | V1 đóng băng. |
| **Dory** | Chỗ *làm*: skill + CLI + daemon PTY | Stack **Rust** đã đóng. Node HTTP là spike. |

flow-skill quyết. flow-deck nhìn. Dory làm. Mũi tên một chiều: chỗ làm **gọi** `flow.sh`. Flow không biết tên Dory.

**SKU:** gõ `dory` để mở desk — cửa sổ → thẻ → ô (lưới sống). Agent **trong** ô gọi `skills/dory` → CLI → socket → PTY thật. Cổng skill `DORY_ENV=1`. Không phải `dory serve` HTTP `:7380`. Không phải clone TUI Herdr. Quyết định: [stack](plans/reports/260822-stack-decision.md) · [hợp đồng](plans/reports/260822-skill-cli-socket-contract.md) · [lưới](plans/260823-1326-desk-spatial-grid/plan.md).

Nhà máy (Herdr / Cursor) được thuê để xây. Hàng Dory **không** gọi `dsh` hay `herdr` lúc chạy.

## Hai động cơ

1. **Session OS** — chọn thư mục, chạy phiên, sửa file, ra lệnh, ủy thác, hỏi trước việc nguy hiểm. Nhật ký phiên là sự thật. Web local chiếu nhật ký, không phải cửa sổ.
2. **Workplace OS** — cửa sổ → thẻ → ô. Máy chủ giữ tiến trình thật. Một agent điều khiển agent khác. Rời UI không phải tắt việc.

## Xong tới đâu

Trạng thái dưới đây là **điểm vào**, không phải inventory. Đếm test / verb / phase: đọc owner, đừng chép vào đây.

| Việc | Tình trạng | Owner |
|---|---|---|
| Đặt tên, tách nhà, freeze, north star, stack Rust | Đóng | [CHARTER.md](CHARTER.md) · [CAPACITY-FREEZE.md](CAPACITY-FREEZE.md) · [north star](plans/reports/260822-north-star-aoe.md) · [stack](plans/reports/260822-stack-decision.md) |
| Hợp đồng skill / CLI / socket | Accepted | [hợp đồng](plans/reports/260822-skill-cli-socket-contract.md) |
| Workplace OS (PTY + `DORY_ENV=1` + skill) | Có — crate `rust/`, skill `skills/dory/SKILL.md` | Binary + skill. Plan giấy [0847](plans/260822-0847-workplace-skill-mux/plan.md) **không** lật từ giấy; không dùng phase markdown làm sự thật. |
| Occupant (argv PATH, `report`, không `--kind`) | Có | [occupant-lock](plans/reports/260822-1942-brainstorm-occupant-lock.md) · `rust/src/server.rs` · factory `plans/reports/260823-layer4f2-omp-factory.md` |
| Hợp đồng §11 (agent trong pane, Flow trên repo ngoài) | **PASS** | [bảng](plans/reports/260823-s11-table.md) · `rust/tests/p5_s11.rs`. Scout [260822-p5-accept-s11.md](plans/reports/260822-p5-accept-s11.md) là **PARTIAL cũ**, đừng chấm lại. `eval/phase5-project` vẫn không đủ. |
| Mở chỗ ngồi (`dory`) | Có — desk lưới (sidebar + N PTY), detach ≠ kill | `rust/src/desk.rs` · `layout.rs`. `--plain` vẫn ở `attach.rs`. |
| Node `:7380` + `script` + `/workplace` | Spike / học. Giữ cây. Không phải skill. | `src/` journal lamp |

Luật: [CHARTER.md](CHARTER.md) (formerly `HIEN-PHAP.md`).

## License

Chưa chọn bản công khai. Kho local.
