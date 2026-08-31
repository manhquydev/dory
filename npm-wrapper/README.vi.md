# `@manhquy/dory`

[English](README.md) · [Tiếng Việt](README.vi.md)

**Đèn nhật ký** của [Dory](https://github.com/manhquydev/dory) (Session OS). Gói này **không** phải desk.

[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

| | |
|---|---|
| Bin | chỉ `dory-serve` |
| Node | `>=22.14.0` (bạn tự cài Node; gói này không cài hộ) |
| Lệnh desk | `dory` — binary Rust, build từ repo, không nằm trong tarball này |

Sau `npm install`, gõ `dory` sẽ không mở desk. Đừng cài tên npm không scope `dory`; đó là sản phẩm khác.

Bản gốc tiếng Anh: [README.md](README.md).

## Cài đặt

```bash
npx @manhquy/dory
```

Chạy từ thư mục phiên. Mở `http://127.0.0.1:7380/`. Thư mục khác: `--workspace /abs`. Ghim: `npx @manhquy/dory@0.1.0`. Preview: `@next`.

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

Doctor chỉ đo Node và va chạm PATH. Không cài Node, không gắn `dory`.

## Kho mã

Nguồn, cách build desk, đóng góp, và chính sách bảo mật:

**https://github.com/manhquydev/dory**

- [Đóng góp](https://github.com/manhquydev/dory/blob/main/CONTRIBUTING.vi.md)
- [Bảo mật](https://github.com/manhquydev/dory/blob/main/SECURITY.md)
- [Code of Conduct](https://github.com/manhquydev/dory/blob/main/CODE_OF_CONDUCT.md)
- [Người đóng góp](https://github.com/manhquydev/dory/graphs/contributors)

## Giấy phép

[MIT](https://github.com/manhquydev/dory/blob/main/LICENSE) © 2026 manhquy và những người đóng góp Dory.
