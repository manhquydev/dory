---
title: "Phase 4: Two-door docs"
status: done
phase: 4
priority: P1
effort: 1h
dependencies: [3]
---

# Phase 4: Two-door docs

## Overview

README nói **hai cửa**. Trang + CLI nhắc helper bật đèn. Rust desk/USAGE **không** đổi nghĩa `dory` trần. Không .app.

## Context

- README hôm nay: `dory` = desk (`README.md:7-15`); SKU phủ nhận serve (`README.md:53`); "Hai động cơ" (`README.md:57-60`); Node spike row (`README.md:74`)
- Node ready line: `src/cli.js:43-45`
- Rust đã tách: `rust/src/main.rs:50`, `:56`, `:69-71` — **không sửa** trừ khi cook vô tình đụng; acceptance = không đổi nghĩa
- Helper: scout — folder picker không list FS; `--workspace` CLI giữ

## Requirements

- Functional: README section "Hai cửa" — (1) `dory` desk + tty; (2) `node bin/dory.js serve --workspace /abs` rồi browser `http://127.0.0.1:7380/`. Ghi rõ **không phải .app**; cần người bật đèn. Footer trang + stderr CLI echo workspace + URL.
- Non-functional: không đổi `DORY_ENV` copy. Không lật 0847. Không `0.0.0.0`. `usage()` (`src/cli.js:4-6`) thêm một dòng serve.

## Architecture

Docs only + một dòng CLI. Không data-store mới.

```text
Cửa A  rust binary   dory            desk (0817)   cần tty
Cửa B  Node bin      dory serve      lamp :7380    sau khi A-helper bật
```

Không gộp chrome. Page footer (copy, không fact): `Bật đèn: node bin/dory.js serve --workspace <path-đã-chiếu>`

`cli.js` sau listen:

```text
dory: journal projection on http://127.0.0.1:{port}/ workspace={abs} (not a workplace)
```

## Related Code Files

- Modify: `README.md` — hai cửa; giữ desk table; không xóa Workplace OS
- Modify: `src/cli.js` — usage + ready line có workspace
- Modify: `src/page.js` — footer helper (copy only)
- Delete: none
- Forbidden: `rust/**` (USAGE đã đúng), `src/workplace/**`, `skills/dory/**`, `CHARTER.md`, `CAPACITY-FREEZE.md`, plans 0847/0011/0859 checkboxes

## Implementation Steps

1. README: sau "Hai động cơ" hoặc thay SKU sentence `:53` — bảng hai cửa. Helper block copy-paste. "Không phải icon/.app." Link plan này optional.
2. `usage()`: `usage: dory serve --workspace <abs-dir>` giữ; thêm `opens http://127.0.0.1:7380/ (journal lamp, not desk)`.
3. Ready stderr: thêm `workspace=`.
4. Page footer tiếng Việt: cần người chạy lệnh trên; path = path đang chiếu.
5. Assert trong `test/session-os-door.test.js`: GET `/` chứa `serve --workspace` hoặc "bật đèn"; không chứa `0.0.0.0`.
6. Grep rust USAGE: `Bare \`dory\` opens the desk` vẫn (`rust/src/main.rs:50`). **Không** edit rust.
7. `node --test`. Không `cargo test` trừ khi nghi đụng — không nên nghi.

## Todo

- [x] README hai cửa + helper ≠ .app
- [x] CLI usage/ready
- [x] Page footer
- [x] Không sửa rust/desk/DORY_ENV
- [x] `node --test` xanh

## Success Criteria

- [x] Người đọc README biết cửa nào là desk, cửa nào là serve
- [x] Helper là lệnh Node + `--workspace` abs
- [x] `rust/src/main.rs` USAGE nghĩa `dory` trần không đổi
- [x] `node --test` xanh

## Risk Assessment

| Risk | L×I | Mitigation |
|---|---|---|
| README biến serve thành SKU chính | M×H | Desk table giữ đầu file. Signal: `dory` không còn là desk ở đoạn mở. Response: revert copy, desk first. |
| Cook sửa rust "cho đồng bộ" | M×H | Forbidden list. Signal: rust diff. Response: drop rust hunk. |
| .desktop / installer lọt | L×H | Non-goal. Signal: file mới `.desktop`. Response: delete. |

## Security Considerations

Docs không dạy bind public. Giữ `127.0.0.1`.

## Next Steps

Handoff: `/ak:plan validate` rồi `/ak:cook` plan dir. Không implement trong turn plan.
