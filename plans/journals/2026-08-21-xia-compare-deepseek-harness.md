---
title: Xia compare DeepSeek Harness
date: 2026-08-21
session: xia-compare-deepseek-harness
summary: "Học 1/2: máy phiên / nhật ký / duyệt / web local. Báo cáo chờ ký. Không plan. Không máy."
---

# Journal: 2026-08-21 — Xia compare DeepSeek Harness

## Context

Xia `--compare` đúng lệnh hiến pháp bước 2, cửa sổ `dory`. Nguồn `deepseek-ai/deepseek-harness` `master@528c682`. Local chỉ giấy.

`ak journal create` từ cwd này ghi nhầm lên `/home/manhquy/Downloads/plans/journals/` (parent chưa đăng ký project `dory`). Bản canonical nằm trong repo này.

## What Happened

- Pack 19 file docs (58k tokens). Không chạy `dsh`, không clone vào repo này.
- Researcher: `plans/reports/260821-1416-research-deepseek-harness.md` (DONE_WITH_CONCERNS).
- Scout: `plans/reports/260821-1416-scout-dory.md`.
- Compare: `plans/reports/260821-1416-xia-compare-deepseek-harness.md` — chờ operator ký.
- Duyệt local: `plans/visuals/xia-compare-deepseek-harness.html`.
- Không `/ak:plan`. Không `package.json`. Không `--port`.

## Reflection

Hiến pháp đã viết sẵn lệnh. Việc khó là không để xia trượt sang `/ak:plan` hay ôm monorepo. Luật nhật ký dsh và câu Dory là cùng một ý — dễ muốn copy format. Đó là bẫy.

## Decision

Học luật nhật ký + fail-closed + workspace-trước-compose + duyệt ba khe + UI chiếu. Từ chối Cordis, CLI dsh, format `~/.dsh`, PATH Claude/Codex, web-as-window.

## Next

Operator đã ký 2026-08-21. Dừng. Lượt sau: xia Herdr `--compare`. Cấm plan trước chữ ký Herdr.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
