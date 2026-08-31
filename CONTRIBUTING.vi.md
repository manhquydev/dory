# Đóng góp cho Dory

Cảm ơn bạn muốn giúp. **Tiếng Anh** là ngôn ngữ gốc của tài liệu mới và comment trong code. Bản này là bản dịch của [CONTRIBUTING.md](CONTRIBUTING.md).

Đọc [Code of Conduct](CODE_OF_CONDUCT.md). Lỗ hổng bảo mật gửi theo [SECURITY.md](SECURITY.md), không mở issue công khai.

## Hai động cơ

Dory là một sản phẩm, hai động cơ. Đừng gộp.

| Động cơ | Người dùng gõ | Mã nguồn |
|---|---|---|
| **Desk** (Workplace OS) | `dory` | `rust/` — cửa sổ → thẻ → ô PTY sống |
| **Lamp** (Session OS) | `dory-serve` | `npm-wrapper/` — nhật ký tại `http://127.0.0.1:7380/` |

**Không** gắn bin `dory` vào gói npm. `@manhquy/dory` chỉ là đèn. Tên npm không scope `dory` là sản phẩm khác.

## Bạn có thể giúp

- Báo lỗi và bước tái hiện (nói rõ desk hay lamp)
- Tài liệu, bản dịch (`README.vi.md`, file này)
- Test cho verb / hành vi ô đã có
- UX và accessibility trang nhật ký
- Phím desk, layout, CLI occupant (không có cờ `--kind`)

Thay đổi nhỏ có test tốt hơn thiết kế lại lớn. Mở issue trước khi refactor lớn.

## Phát triển

### Desk

Cần Rust stable gần đây (CI dùng `dtolnay/rust-toolchain@stable`; crate edition 2024).

```bash
git clone https://github.com/manhquydev/dory.git
cd dory
cargo test --manifest-path rust/Cargo.toml --offline --locked
cargo build --manifest-path rust/Cargo.toml --release
```

Trên macOS, đường socket ngắn quan trọng (`TMPDIR=/tmp` trên CI). Một số test occupant `done`/`idle` chỉ chạy Linux.

### Lamp

Node `>=22.14.0`.

```bash
cd npm-wrapper
npm ci
npm test
```

Script doctor chỉ đo máy; **không** cài Node:

```bash
bash scripts/dory-lamp-doctor.sh
```

## Pull request

1. Fork và nhánh từ `main`.
2. Tập trung một việc. Không commit secret, token, hay file `.env`.
3. Thêm hoặc sửa test khi đổi hành vi.
4. Chạy test desk và/hoặc lamp tương ứng file bạn sửa.
5. Điền PR template.

Maintainer sẽ không merge thay đổi gắn lamp `bin` thành `dory`, publish tên không scope `dory`, hoặc thêm installer cài Node/Rust hộ user.

## Giấy phép

Khi đóng góp, bạn đồng ý phần việc của mình thuộc [MIT License](LICENSE) của kho này.
