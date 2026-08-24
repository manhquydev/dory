---
title: "Phase 2: Goal box"
status: done
phase: 2
priority: P1
effort: 1.5h
dependencies: [1]
---

# Phase 2: Goal box

## Overview

Ô mục tiêu trên GET `/`. Submit → `session/goal` append vào journal file. `POST /note` giữ nguyên.

## Context

- Phase 1 cards + `#workspace`
- Note API: `src/serve.js:127-136`
- `journal.append(type, fields)` generic: `src/journal.js:26-37`
- Contract: form goal → `journal/note` **hoặc** `session/goal` — chọn `session/goal`

## Requirements

- Functional: form goal (placeholder Việt, vd. "Bạn muốn làm gì?"). `POST /goal` `{text:string}` → append `{type:"session/goal", text}`. Trim; empty → 400. GET `/` sau đó có thẻ "Mục tiêu".
- Non-functional: JSON + optional `application/x-www-form-urlencoded` `text=` (form native, không bắt buộc JS). Không tty. Không second store.

## Architecture

```text
browser ──POST /goal {text}──► serve.js
                                  │
                                  ├─ typeof text !== "string" || trim=="" → 400
                                  └─ journal.append("session/goal", {text: trimmed})
                                       → 200 {ok, event}
                             file: <ws>/.dory/journal.jsonl
GET / cards: type session/goal → nhãn "Mục tiêu"
```

Giữ `POST /note` (`src/serve.js:127-136`) — không đổi shape. Goal **không** ghi `journal/note` (tránh lẫn "ghi chú" / "mục tiêu").

Inline script hoặc form `action="/goal" method="POST"` rồi redirect `/`. Tests dùng `fetch` JSON như phase1 note (`test/phase1.test.js:111-115`).

Route mới **sau** `handleWorkplace` (`src/serve.js:117`), cạnh `/note`. Không `/workplace/*`.

## Related Code Files

- Modify: `src/serve.js` — `POST /goal`
- Modify: `src/page.js` — form + label `session/goal`
- Create: `test/session-os-door.test.js` — goal HTTP + file line
- Delete: none
- Forbidden: `src/workplace/**`, `rust/**`, `src/journal.js` (append đã đủ), `README.md`

## Implementation Steps

1. Label `session/goal` → "Mục tiêu" trong `page.js`.
2. Form `#goal` trên trang: textarea `name="text"` + submit "Ghi mục tiêu".
3. Handler `POST /goal` mirror `/note` + trim + 400.
4. Test: startServer port 0; POST `{text:"viết README"}`; 200; file có `session/goal`; GET `/` chứa escaped text + "Mục tiêu"; empty/missing → 400; không dòng `session/goal` khi 400.
5. Test: `POST /note` vẫn `journal/note` (một assert trong file mới hoặc phase1).
6. `node --test`.

## Todo

- [x] `POST /goal` → `session/goal`
- [x] Form trên GET `/`
- [x] 400 text rỗng
- [x] `test/session-os-door.test.js`
- [x] Không grow workplace

## Success Criteria

- [x] File journal có event `session/goal` (English type)
- [x] Trang hiện thẻ "Mục tiêu", không raw JSON làm view
- [x] `/note` không gãy
- [x] `node --test` xanh

## Risk Assessment

| Risk | L×I | Mitigation |
|---|---|---|
| Cook gộp goal vào `/note` | M×M | Signal: không có `session/goal`. Response: sửa type. Nếu đã ship nhầm, UI nhận cả hai type — không cần replan. |
| Form POST không JSON | L×M | Nhận urlencoded **hoặc** test-only JSON; document cái tests dùng. |

## Security Considerations

Cùng `readJsonBody` + `BODY_LIMIT` (`src/serve.js:11`, `src/serve.js:91-110`). Escape text trên GET. Loopback = auth. Không CSRF token (local).

## Next Steps

Phase 3: Flow confirm trên cùng page.
