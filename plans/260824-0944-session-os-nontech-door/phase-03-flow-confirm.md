---
title: "Phase 3: Flow confirm"
status: done
phase: 3
priority: P1
effort: 1.5h
dependencies: [2]
---

# Phase 3: Flow confirm

## Overview

Flow không chạy nếu chưa xác nhận. UI hiện đúng lệnh (`bin` + argv + cwd). Server fail-closed: `confirm !== true` → 403, không spawn, không `flow/invoke`.

## Context

- Handler hôm nay spawn ngay: `src/serve.js:137-174`
- Default argv `["status"]`: `src/serve.js:145`, `src/flow.js:22`
- `resolveFlowBin` từ chối herdr/dsh: `src/flow.js:7-16`
- Test bare POST: `test/phase1.test.js:179`

## Requirements

- Functional: GET `/` hiện preview `bin`, `args` (`status`), `cwd=workspace`. Checkbox/nút xác nhận. `POST /flow` body phải `confirm === true` (boolean). Thiếu/false/"yes" → 403. Có confirm → hành vi cũ (invoke + result, 15s).
- Non-functional: UI không free-argv (tránh farm). API vẫn nhận `args` **kèm** confirm (phase1 fake-flow). Không browser CI.

## Architecture

```text
GET /  render flowPreview { bin, args:["status"], cwd: workspace }
        resolveFlowBin throws → preview "từ chối: …", không nút chạy

POST /flow
  body = readJsonBody or {}
  if (body.confirm !== true) → 403 {ok:false, error:"confirm required"}
       NO append, NO spawn
  else existing path: resolveFlowBin → append flow/invoke → invokeFlow → flow/result
```

Đổi thứ tự vs hôm nay: hôm nay `content-length===0` skip body rồi chạy (`src/serve.js:146-151`). Sau phase 3, empty = `{}` = no confirm = 403.

UI:

```text
Sẽ chạy: <bin> status
Thư mục: <workspace>
[ ] Tôi xác nhận chạy lệnh trên
[Chạy Flow]  → POST {confirm:true}  // không gửi args từ form
```

Facts invoke/result vẫn English types trong file. Cards: "Chạy Flow" / "Kết quả Flow".

## Related Code Files

- Modify: `src/serve.js` — confirm gate trước spawn
- Modify: `src/page.js` — preview + confirm control
- Modify: `test/phase1.test.js` — POST `{confirm:true}` tại `:179`
- Modify: `test/session-os-door.test.js` — deny/allow
- Delete: none
- Forbidden: `src/workplace/**`, `rust/**`, `src/flow.js` (timeout/forbid giữ), `README.md`

## Implementation Steps

1. `requireFlowConfirm(body)` — `body.confirm === true` only.
2. `POST /flow`: đọc body luôn (kể cả length 0 → `{}`). Gate. Rồi `flowArgs` như cũ.
3. Preview: `try { resolveFlowBin(process.env) } catch` trên GET. Truyền vào `renderJournalPage`.
4. Sửa `test/phase1.test.js:179` thêm `headers` + `JSON.stringify({confirm:true})`. Assert invoke/result như cũ (`:193-202`).
5. Door tests:
   - POST `{}` / no body / `{confirm:"true"}` / `{confirm:1}` → 403; file **không** `flow/invoke`; fake bin không bị gọi (FLOW_BIN = script đếm; count=0).
   - POST `{confirm:true}` → 200 + invoke+result.
   - GET `/` chứa escaped bin name + `status` + workspace trước form.
6. `node --test`.

## Todo

- [x] Server `confirm === true`
- [x] Preview bin/argv/cwd trên GET `/`
- [x] Đổi phase1 flow POST
- [x] Deny tests không append invoke
- [x] Không argv box trên UI

## Success Criteria

- [x] Bare POST `/flow` không chạy Flow
- [x] Confirm + default status vẫn ghi invoke/result
- [x] Người thấy lệnh trước khi bấm
- [x] `node --test` xanh

## Risk Assessment

| Risk | L×I | Mitigation |
|---|---|---|
| Quên sửa `test/phase1.test.js:179` | H×H | Cùng commit với gate. Signal: phase1 flow đỏ. Response: add confirm, không nới gate. |
| `confirm: "true"` lọt | M×H | Strict `=== true`. Signal: string test đỏ nếu cook nới. Response: giữ boolean. |
| Preview lệch argv thật | M×H | UI không gửi args; server default `status` khi không có `body.args`. |

Assumption: loopback user = operator. Gãy nếu bind non-loopback — **kill** (`assertLoopbackHost`).

## Security Considerations

Fail-closed trước spawn. `FORBIDDEN_BIN` giữ (`src/flow.js:4-16`). Không hiện stdout/stderr raw không escape trên thẻ result. Timeout 15s giữ. Không `0.0.0.0`.

## Next Steps

Phase 4: README hai cửa + helper copy (không .app).
