---
title: "Session OS nontech door"
description: "Thaw Node lamp UI so a non-tech user can see the session folder, type a goal, read a human journal, and run Flow only after confirm."
status: completed
priority: P1
effort: 6h
branch: main
tags: [feature, frontend, dory, session-os, lamp]
blockedBy: []
blocks: []
created: 2026-08-24
---

# Session OS nontech door

## Overview

Người non-tech, sau khi **ai đó đã bật đèn** (`node bin/dory.js serve --workspace /abs`), mở `http://127.0.0.1:7380/`: thấy thư mục phiên, gõ mục tiêu, đọc nhật ký tiếng người, Flow chỉ sau xác nhận. File journal là sự thật. Không cần tty để *làm việc trên trang*. Rust `dory` trần giữ desk (0817).

Scope Challenge: HOLD. Reuse lamp. Không mux. Không .app.

## Contract (closed)

| Field | Closed |
|---|---|
| Outcome | Cửa Session OS trên loopback. Journal file = sự thật. Cards/copy chỉ HTML. |
| Constraints | CHARTER hình B. Bind `127.0.0.1:7380` only. Cấm `/workplace` mới, `node-pty`, detect, `--kind`, exec herdr, Xia `--copy`. Không lật 0847/0011/0859. Không đổi `DORY_ENV`. Zero npm deps. |
| Non-goals | Cowork runtime. Desk onboarding. Clone Herdr/Ratatui. `0.0.0.0`. Một chrome hai cửa. Đổi `dory` trần thành web. |
| Start | Helper CLI: `dory serve --workspace /abs` (Node bin). Không folder picker FS. Trang **chiếu** path. |

## Scout (verified)

| Surface | Today | Cite |
|---|---|---|
| Lamp CLI | `serve --workspace <abs>` only; relative/missing → 2 | `src/cli.js:5`, `src/cli.js:27-35` |
| GET `/` | `<pre id="journal">` = escaped **raw JSONL** | `src/serve.js:55-73`, `src/serve.js:120-125` |
| POST `/note` | `{text}` → `journal/note` | `src/serve.js:127-136` |
| POST `/flow` | spawn ngay, default `status`, 15s | `src/serve.js:137-174` |
| Journal | `<ws>/.dory/journal.jsonl` append-only | `src/journal.js:4-6`, `src/journal.js:26-37` |
| Workplace HTTP | leftover; **do not grow** | `src/serve.js:191`, `src/workplace/http.js:16-84` |
| Rust split | `'serve' is the Node journal lamp, not this binary` | `rust/src/main.rs:56`, `rust/src/main.rs:69-71` |
| Desk SKU | bare `dory` = desk | `rust/src/main.rs:50`, `rust/src/main.rs:64-65` |
| Law line tests | `Not a workplace. Not a pane. Not a terminal.` | `test/phase1.test.js:139-141`, `test/phase2.test.js:43-44` |
| Flow test | POST `/flow` **no body** | `test/phase1.test.js:179` |

Paper: `plans/reports/260822-session-os-phase1.md`, `260822-stack-decision.md`, `260824-0936-review-dory-open-tech-nontech-herdr.md` (chọn A).

## Design

```text
helper (một lần, tty)     node bin/dory.js serve --workspace /abs
        │                 bind 127.0.0.1:7380
        ▼
browser GET /  ──read──►  /abs/.dory/journal.jsonl
        │                 cards tiếng Việt; type English trong file
POST /goal {text}  ──append──►  session/goal
POST /note {text}  ──append──►  journal/note   (giữ)
POST /flow {confirm:true} ──append invoke/result──►  flow.sh status
                 confirm !== true → 403, không spawn, không invoke
```

Facts = journal lines. Copy/cards = `src/page.js` only. Fail-closed: server đòi `confirm === true`. UI hiện `bin` + `argv` + `cwd` trước nút.

## Cross-Plan Dependencies

| Relationship | Plan | Status |
|---|---|---|
| File-disjoint | `260822-0847-workplace-skill-mux` | pending **paper** — do not flip |
| Do not flip | `260823-0011-close-coding-occupancy` | completed |
| Do not flip | `260823-0859-section-11-real-repo` | completed |
| Desk stays | `260824-0817-desk-sit-down-like-herdr` | completed |

Không `blockedBy`. Không đụng `rust/**`, `src/workplace/**`, `skills/dory/**`.

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Human journal page](./phase-01-start.md) | Done |
| 2 | [Goal box](./phase-02-goal-box.md) | Done |
| 3 | [Flow confirm](./phase-03-flow-confirm.md) | Done |
| 4 | [Two-door docs](./phase-04-two-door-docs.md) | Done |

Tuần tự. `src/serve.js` + `src/page.js` không song song.

## File ownership

| Phase | Create | Modify | Forbidden |
|---|---|---|---|
| 1 | `src/page.js` | `src/serve.js`, `src/journal.js`, `test/phase1.test.js`, `test/phase5.test.js` | `src/workplace/**`, `rust/**` |
| 2 | `test/session-os-door.test.js` | `src/page.js`, `src/serve.js` | same |
| 3 | — | `src/page.js`, `src/serve.js`, `test/phase1.test.js`, `test/session-os-door.test.js` | same |
| 4 | — | `README.md`, `src/cli.js`, `src/page.js` (footer helper) | same |

## Test matrix

| Layer | What |
|---|---|
| Unit | `parseJournalLines` skips bad line; labels for known types |
| HTTP | GET `/` cards + path; POST `/goal`; POST `/flow` confirm/deny |
| Regression | phase1 loopback/workspace; phase2 hide `w1:t1`; phase5 no herdr spawn |
| Not in CI | real browser, tty sit, `.app` |

Gate: `node --test` from repo root. Không `cargo test` (không đụng rust).

## Backwards compatibility

- Journal: thêm type `session/goal`. Line cũ vẫn parse.
- `POST /note` giữ.
- `POST /flow` **gãy** bare POST — cùng phase 3 sửa `test/phase1.test.js:179`.
- GET `/` **gãy** raw-dump assert `test/phase1.test.js:142` — cùng phase 1.
- Giữ câu luật English để phase2 xanh.

## Rollback

Mỗi phase: revert đúng file ownership. Journal leftover vô hại. Không migration. Không revert rust.

## Kill if

CHARTER kill. Thêm route `/workplace/*`. `node-pty` / npm dep. `0.0.0.0`. Lật checkbox 0847/0011/0859. Đổi nghĩa `dory` trần / `DORY_ENV`. Gộp hai cửa một chrome. Cowork turns/tools.

## Success Criteria

- [x] GET `/` đọc được; không dump JSONL thô cả file
- [x] Form goal → `session/goal` (file); `/note` vẫn sống
- [x] `POST /flow` chỉ khi `confirm === true`; UI hiện lệnh trước
- [x] Trang chiếu `--workspace`; không invent store
- [x] `node --test` xanh; không browser CI
- [x] README hai cửa (desk vs serve); rust USAGE không đổi nghĩa `dory` trần

## Open questions

None. Contract đóng. Helper ≠ .app đã gọi ra.
