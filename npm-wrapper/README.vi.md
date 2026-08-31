# `@manhquy/dory`

[English](README.md) · [Tiếng Việt](README.vi.md)

**Trang nhật ký** của [Dory](https://github.com/manhquydev/dory) (Session OS). Gói này **không** phải desk.

[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

| | |
|---|---|
| Lệnh | chỉ `dory-serve` |
| Node | `>=22.14.0` (bạn tự cài; gói này không cài hộ) |
| Desk | `dory` — binary Rust, build từ repo, không nằm trong tarball này |

Sau `npm install`, gõ `dory` sẽ không mở desk. Đừng cài tên npm `dory` không có `@manhquy`; đó là sản phẩm khác.

Bản tiếng Anh là bản gốc: [README.md](README.md).

## Cài đặt

Chạy trong thư mục phiên:

```bash
npx @manhquy/dory
```

Mở `http://127.0.0.1:7380/`. Thư mục khác: `--workspace /abs`. Cố định bản: `npx @manhquy/dory@0.1.0`. Bản thử: `@next`.

Chưa có Node? [nodejs.org](https://nodejs.org/en/download) hoặc [fnm](https://github.com/Schniz/fnm).

## Kiểm tra và gỡ

```bash
npm view @manhquy/dory name version bin
# bin phải là { "dory-serve": "bin/dory-serve.js" } thôi

npm uninstall -g @manhquy/dory
```

Từ bản clone repo:

```bash
bash scripts/dory-lamp-doctor.sh
```

Doctor chỉ kiểm tra Node và lệnh trùng trên PATH. Không cài Node, không gắn lệnh `dory`.

## Kho mã

Nguồn, cách build desk, đóng góp, và chính sách bảo mật:

**https://github.com/manhquydev/dory**

- [Đóng góp](https://github.com/manhquydev/dory/blob/main/CONTRIBUTING.vi.md)
- [Bảo mật](https://github.com/manhquydev/dory/blob/main/SECURITY.md)
- [Code of Conduct](https://github.com/manhquydev/dory/blob/main/CODE_OF_CONDUCT.md)
- [Người đóng góp](https://github.com/manhquydev/dory/graphs/contributors)

## Giấy phép

[MIT](https://github.com/manhquydev/dory/blob/main/LICENSE) © 2026 manhquy và những người đóng góp Dory.
