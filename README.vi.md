# Dory

[English](README.md) · [Tiếng Việt](README.vi.md)

**Agent Operating Environment** chạy local cho việc phần mềm nhiều agent. Gõ `dory` để mở desk. Đèn nhật ký là động cơ khác.

[![CI](https://github.com/manhquydev/dory/actions/workflows/ci.yml/badge.svg)](https://github.com/manhquydev/dory/actions/workflows/ci.yml)
[![npm next](https://img.shields.io/npm/v/@manhquy/dory/next.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Dory không phải bản mới của [flow-skill](https://github.com/manhquydev/flow-skill) (người chấm) hay flow-deck (bảng treo tường). Mỗi nhà một việc. Dory là chỗ **làm**.

**Kho công khai, mời cùng đóng góp.** Issue, bản dịch, test, và patch nhỏ đều được. Đọc [CONTRIBUTING.vi.md](CONTRIBUTING.vi.md) và [Code of Conduct](CODE_OF_CONDUCT.md).

Bản gốc tiếng Anh: [README.md](README.md).

## Hai động cơ

| Động cơ | Lệnh | Cho ai | Cần |
|---|---|---|---|
| **Desk** (Workplace OS) | `dory` | Cửa sổ → thẻ → ô PTY sống | Terminal thật, binary Rust |
| **Lamp** (Session OS) | `dory-serve` | Nhật ký trên trình duyệt `http://127.0.0.1:7380/` | Node `>=22.14.0` |

Desk là cửa mặc định khi gõ `dory`. Đèn là cửa khác. Gói npm **không** đặt `dory` lên `PATH`.

```
Desk giữ process sống.   Đèn chiếu nhật ký phiên.
Rời UI ≠ tắt việc.
```

## Cài đặt

### Đèn (đã có trên npm)

```bash
npx @manhquy/dory
```

Chạy từ thư mục phiên. Mở `http://127.0.0.1:7380/`. Thư mục khác: `--workspace /abs`. Ghim: `npx @manhquy/dory@0.1.0`. Preview: `@next`.

```bash
# đo Node / registry / va chạm PATH — không cài Node
bash scripts/dory-lamp-doctor.sh

npm uninstall -g @manhquy/dory    # chỉ gỡ đèn
```

**Đừng** `npm i -g dory` (không scope). Tên đó là sản phẩm khác.

Thiếu Node: tự cài từ [nodejs.org](https://nodejs.org/en/download) hoặc [fnm](https://github.com/Schniz/fnm). Dory không cài hộ toolchain.

Tài liệu gói: [`npm-wrapper/README.md`](npm-wrapper/README.md) · [Tiếng Việt](npm-wrapper/README.vi.md)

### Desk (từ mã nguồn)

Chưa có bản binary phát hành. Build crate Rust:

```bash
git clone https://github.com/manhquydev/dory.git
cd dory
cargo build --manifest-path rust/Cargo.toml --release
# đưa rust/target/release/dory vào PATH
dory
```

`dory server` rồi `dory` mở desk: Spaces và Agents bên trái, hàng thẻ, nhiều ô sống bên phải. Ô trống dùng `$SHELL` (có rc). Thẻ mới theo cwd ô đang focus; cửa sổ mới theo thư mục lúc gõ `dory`. Server cũ còn `--norc`: `dory server stop` rồi `dory`.

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
| `Ctrl-b z` | Phóng ô đang chọn; ô anh em vẫn sống |
| `Ctrl-b w` | Picker cửa sổ (không tạo) |
| `Ctrl-b Shift-n` | Cửa sổ mới |
| `Ctrl-b b` | Thu sidebar 26↔4↔0 |
| `Ctrl-b Ctrl-b` | Gửi `C-b` vào ô |
| `Ctrl-b x` | Đóng ô (xác nhận nếu ô cuối trên thẻ) |
| `Ctrl-b Shift-x` | Đóng thẻ |
| `Ctrl-b Shift-d` | Đóng cửa sổ |
| `Ctrl-b q` hoặc `Ctrl-b d` | Rời UI; PTY vẫn sống |
| `Ctrl-b ?` | Bảng phím |

`dory attach --plain` là client PTY thô (không sidebar). Trong ô (`DORY_ENV=1`) occupant gọi CLI, không ngồi desk. `dory server stop` mới tắt việc.

Binary Rust gặp `dory serve` chỉ nhắc đây là đèn Node, không mở desk.

## Tình trạng

Cây công khai, giai đoạn sớm. Desk là Rust. Đèn là gói npm `@manhquy/dory` (chỉ `dory-serve`). CI desk trên Windows là job thông báo; phân loại occupant `done`/`idle` trên Darwin vẫn chưa làm.

| Việc | Tình trạng | Owner |
|---|---|---|
| Vì sao có sản phẩm | Đóng | [CHARTER.md](CHARTER.md) |
| Skill / CLI / socket | Accepted | `skills/dory/` · `rust/` |
| Đèn trên npm | `@manhquy/dory@0.1.0` | `npm-wrapper/` |
| Phát hành binary desk | Chưa | build từ `rust/` |

## Đóng góp

Đọc [CONTRIBUTING.vi.md](CONTRIBUTING.vi.md) ([English](CONTRIBUTING.md)).

- [Mở issue](https://github.com/manhquydev/dory/issues/new/choose)
- [Mở pull request](https://github.com/manhquydev/dory/pulls)
- Bảo mật: [SECURITY.md](SECURITY.md) — báo cáo riêng, không phải issue công khai

Khi đóng góp, bạn đồng ý phần việc thuộc giấy phép MIT.

## Người đóng góp

Danh sách sống trên GitHub: [contributors](https://github.com/manhquydev/dory/graphs/contributors). PR lần đầu được chào đón — tài liệu và test cũng tính.

## Bảo mật

Xem [SECURITY.md](SECURITY.md). Đừng dán token vào issue hay pull request.

## Giấy phép

[MIT](LICENSE) © 2026 manhquy và [những người đóng góp Dory](https://github.com/manhquydev/dory/graphs/contributors).

`rust/vendor/portable-pty` giữ file giấy phép riêng.
