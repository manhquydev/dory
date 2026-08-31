# Dory

[English](README.md) · [Tiếng Việt](README.vi.md)

Môi trường làm việc trên máy bạn, cho phần mềm nhiều agent. Gõ `dory` để mở **desk** (giao diện terminal). Muốn xem nhật ký phiên trên trình duyệt thì dùng **lamp** — phần khác, lệnh khác.

[![CI](https://github.com/manhquydev/dory/actions/workflows/ci.yml/badge.svg)](https://github.com/manhquydev/dory/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Dory không phải bản mới của [flow-skill](https://github.com/manhquydev/flow-skill) (người chấm) hay flow-deck (bảng treo tường). Mỗi thứ một việc. Dory là chỗ **làm**.

**Repo công khai, mời đóng góp.** Issue, bản dịch, test, patch nhỏ đều được. Đọc [CONTRIBUTING.vi.md](CONTRIBUTING.vi.md) và [Code of Conduct](CODE_OF_CONDUCT.md).

Bản tiếng Anh là bản gốc: [README.md](README.md).

## Hai phần

| Phần | Lệnh | Dùng để | Cần |
|---|---|---|---|
| **desk** (Workplace OS) | `dory` | Cửa sổ → thẻ → ô terminal đang chạy | Terminal thật, binary Rust |
| **lamp** (Session OS) | `dory-serve` | Nhật ký phiên trên trình duyệt, `http://127.0.0.1:7380/` | Node `>=22.14.0` |

Gõ `dory` thì ra desk. Lamp không thay desk. Gói npm **không** thêm lệnh `dory` vào `PATH`.

```
desk giữ process sống.   lamp chỉ chiếu nhật ký phiên.
Đóng giao diện ≠ tắt việc.
```

## Cài đặt

### Nhật ký trên trình duyệt (npm)

Đã có trên [npm](https://www.npmjs.com/package/@manhquy/dory). Chạy trong thư mục phiên:

```bash
npx @manhquy/dory
```

Mở `http://127.0.0.1:7380/`. Thư mục khác: `--workspace /abs`. Cố định bản: `npx @manhquy/dory@0.1.0`. Bản thử: `@next`.

```bash
# kiểm tra Node, registry, và lệnh trùng trên PATH — không cài Node
bash scripts/dory-lamp-doctor.sh

npm uninstall -g @manhquy/dory    # chỉ gỡ lamp
```

**Đừng** chạy `npm i -g dory` (không có `@manhquy`). Tên đó là sản phẩm khác.

Chưa có Node: tự cài từ [nodejs.org](https://nodejs.org/en/download) hoặc [fnm](https://github.com/Schniz/fnm). Dory không cài hộ Node hay Rust.

Tài liệu gói: [`npm-wrapper/README.md`](npm-wrapper/README.md) · [Tiếng Việt](npm-wrapper/README.vi.md)

### Desk (tự build)

Chưa có file cài sẵn. Build từ Rust:

```bash
git clone https://github.com/manhquydev/dory.git
cd dory
cargo build --manifest-path rust/Cargo.toml --release
# thêm rust/target/release/dory vào PATH
dory
```

Chạy `dory server` rồi `dory` để mở desk: Spaces và Agents bên trái, hàng thẻ, nhiều ô bên phải. Ô trống dùng `$SHELL` (có rc). Thẻ mới theo thư mục của ô đang gõ; cửa sổ mới theo thư mục lúc bạn gõ `dory`. Server cũ còn `--norc`: `dory server stop` rồi `dory`.

## Phím desk

Prefix là `Ctrl-b`. Không có phím trần `x` / `1` / `w`.

| Phím / chuột | Việc |
|---|---|
| Click card / agent / chip / ô | Đổi ô đang gõ |
| Kéo vách | Đổi tỉ lệ |
| Kéo chọn ≥ 2 ô trên tile | Copy OSC 52 (footer `copied` = đã gửi) |
| `Ctrl-b h/j/k/l` | Ô theo hướng |
| `Ctrl-b n` / `p` / `1..9` | Thẻ cùng cửa sổ |
| `Ctrl-b c` | Thẻ mới |
| `Ctrl-b v` / `-` | Tách ô phải / dưới |
| `Ctrl-b z` | Phóng ô đang chọn; ô khác vẫn chạy |
| `Ctrl-b w` | Chọn cửa sổ (không tạo mới) |
| `Ctrl-b Shift-n` | Cửa sổ mới |
| `Ctrl-b b` | Thu sidebar 26↔4↔0 |
| `Ctrl-b Ctrl-b` | Gửi `C-b` vào ô |
| `Ctrl-b x` | Đóng ô (hỏi lại nếu là ô cuối trên thẻ) |
| `Ctrl-b Shift-x` | Đóng thẻ |
| `Ctrl-b Shift-d` | Đóng cửa sổ |
| `Ctrl-b q` hoặc `Ctrl-b d` | Rời giao diện; PTY vẫn chạy |
| `Ctrl-b ?` | Bảng phím |

`dory attach --plain` là client PTY thô (không sidebar). Trong ô (`DORY_ENV=1`) occupant gọi CLI, không mở desk. Muốn tắt việc thì `dory server stop`.

Gõ `dory serve` trên binary Rust chỉ nhắc: nhật ký trình duyệt là Node, không phải desk.

## Tình trạng

Repo mới mở công khai. Desk viết bằng Rust. Lamp là gói npm `@manhquy/dory` (chỉ có lệnh `dory-serve`). CI desk trên Windows chỉ là job thông báo; phân loại occupant `done`/`idle` trên Darwin chưa làm.

| Việc | Tình trạng | Owner |
|---|---|---|
| Vì sao có sản phẩm | Đã chốt | [CHARTER.md](CHARTER.md) |
| Skill / CLI / socket | Accepted | `skills/dory/` · `rust/` |
| Gói npm (lamp) | `@manhquy/dory@0.1.0` | `npm-wrapper/` |
| File cài desk | Chưa | build từ `rust/` |

## Đóng góp

Đọc [CONTRIBUTING.vi.md](CONTRIBUTING.vi.md) ([English](CONTRIBUTING.md)).

- [Mở issue](https://github.com/manhquydev/dory/issues/new/choose)
- [Mở pull request](https://github.com/manhquydev/dory/pulls)
- Bảo mật: [SECURITY.md](SECURITY.md) — báo cáo riêng, đừng mở issue công khai

Khi đóng góp, bạn đồng ý phần việc thuộc giấy phép MIT.

## Người đóng góp

Danh sách trên GitHub: [contributors](https://github.com/manhquydev/dory/graphs/contributors). PR lần đầu được chào đón — tài liệu và test cũng tính.

## Bảo mật

Xem [SECURITY.md](SECURITY.md). Đừng dán token vào issue hay pull request.

## Giấy phép

[MIT](LICENSE) © 2026 manhquy và [những người đóng góp Dory](https://github.com/manhquydev/dory/graphs/contributors).

`rust/vendor/portable-pty` giữ file giấy phép riêng.
