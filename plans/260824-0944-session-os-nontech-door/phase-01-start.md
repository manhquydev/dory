---
title: "Phase 1: Human journal page"
status: done
phase: 1
priority: P1
effort: 2h
dependencies: []
---

# Phase 1: Human journal page

## Overview

GET `/` chiếu nhật ký tiếng người (thẻ) + path thư mục phiên. File `.dory/journal.jsonl` vẫn là store. Bỏ dump JSONL thô trong `<pre>`.

## Context

- Landed lamp: `plans/reports/260822-session-os-phase1.md`
- Review cửa non-tech: `plans/reports/260824-0936-review-dory-open-tech-nontech-herdr.md` §5 A
- Render hôm nay: `src/serve.js:55-73`

## Requirements

- Functional: parse JSONL → thẻ; `session/open` = "Mở phiên"; `journal/note` = "Ghi chú"; type lạ = "Sự kiện" + type English; dòng hỏng không 500. Hiện `workspace` escaped. Giữ câu `Log projection` + `Not a workplace. Not a pane. Not a terminal.`
- Non-functional: `lang="vi"`. Escape mọi field (`src/serve.js:47-53`). Không đọc `.dory/sessions/*`. Không folder picker. Zero deps.

## Architecture

Data in: `journal.readBytes()` (`src/serve.js:121`) + `workspace` từ `startServer` (`src/serve.js:183-188`).

Transform: `parseJournalLines(bytes)` trong `src/journal.js` — split `\n`, skip empty, `JSON.parse`, bad → `{type:"_broken"}`. Không ghi file.

Out: `renderJournalPage(bytes, {workspace})` chuyển sang `src/page.js`. `serve.js` import + `escapeHtml` đi theo (hoặc export từ page). Container `#journal` là `<ol>` thẻ, không `<pre>` cả file.

```text
GET / → readBytes → parseJournalLines → cards HTML
         workspace string → <code id="workspace">
```

`createWorkplace` / `handleWorkplace` **không đụng** (`src/serve.js:113-118`, `src/serve.js:191`).

## Related Code Files

- Create: `src/page.js` — labels, `escapeHtml`, `renderJournalPage`
- Modify: `src/journal.js` — thêm `parseJournalLines`
- Modify: `src/serve.js` — `renderJournalPage` gọi page; xóa template cũ `55-73`
- Modify: `test/phase1.test.js` — bỏ `html.includes(escapeHtml(fileText))` (`:142`)
- Modify: `test/phase5.test.js` — thêm `"src/page.js"` vào list `:95-105`
- Delete: none
- Forbidden: `src/workplace/**`, `rust/**`, `README.md` (phase 4)

## Implementation Steps

1. `parseJournalLines(buf)` + unit trong `test/phase1.test.js` (bad line, empty, known types).
2. `src/page.js`: map type → nhãn Việt; mỗi event một `<li data-type="...">`; hiện `text` / `workspace` / `code` nếu có, escaped. `_broken` → "Dòng hỏng".
3. Header: `Thư mục phiên` + escaped workspace. Giữ đoạn luật English (`test/phase2.test.js:43-44` sống).
4. `createHandler` GET `/` truyền `workspace` vào render (`src/serve.js:113-125`).
5. Sửa test reconstruct: 200 HTML; law lines; note `'hello <b>journal</b> & "bytes"'` xuất hiện escaped, không raw `<b>`; `assert.ok(!html.includes(fileText.trim().split("\n")[0]))` hoặc tương đương — dòng JSON `session/open` không là body chính. Path workspace có trên trang.
6. `node --test test/phase1.test.js test/phase2.test.js test/phase5.test.js`.

## Todo

- [x] `parseJournalLines` + thẻ Việt
- [x] Chiếu workspace path
- [x] Đổi assert dump thô; giữ law line
- [x] `page.js` trong phase5 herdr-grep
- [x] Không đụng workplace/rust

## Success Criteria

- [x] GET `/` không chứa raw file bytes trong một `<pre>`
- [x] Người đọc được "Mở phiên" / "Ghi chú" + path
- [x] XSS: `<b>` trong note không thành HTML
- [x] `test/phase2.test.js` "hides workplace IDs" xanh
- [x] `node --test` xanh

## Risk Assessment

| Risk | L×I | Mitigation |
|---|---|---|
| Gãy `test/phase1.test.js:142` dump | H×H | Đổi assert **cùng** phase. Signal: test fail. Response: fix assert, không giữ pre. |
| Mất law line → phase2 đỏ | M×H | Copy nguyên `Not a workplace. Not a pane. Not a terminal.` |
| Parse nổ GET | M×H | Bad line = thẻ `_broken`, không throw |

Assumption: journal = workspace file only. Nếu cook đọc `sessions/` — **replan**, đó là workplace leak.

## Security Considerations

Escape all. Loopback giữ (`src/serve.js:39-45`). Body limit không đổi. Không list FS.

## Next Steps

Phase 2 (goal box) phụ thuộc page.js + GET `/` cards.
