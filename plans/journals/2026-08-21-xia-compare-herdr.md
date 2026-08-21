---
title: Xia compare Herdr
date: 2026-08-21
session: xia-compare-herdr
summary: "Học 2/2: cửa sổ/thẻ/ô, trạng thái agent, skill điều phối. Báo cáo chờ ký. Không plan. Không máy."
---

# Journal: 2026-08-21 — Xia compare Herdr

## Context

Xia `--compare` đúng lệnh hiến pháp bước 4, cửa sổ `dory`. Nguồn `herdrdev/herdr` `master@624dfd47`. Học 1/2 Harness đã ký.

## What Happened

- Pack 69 file (skill, CLI, detect, workspace/pane). Không chạy `herdr`, không install.
- Researcher: `plans/reports/260821-1435-research-herdr.md`.
- Scout: `plans/reports/260821-1436-scout-dory-engine-2.md`.
- Compare: `plans/reports/260821-1436-xia-compare-herdr.md` — chờ operator ký.
- Không `/ak:plan`. Không `package.json`. Không ghi `flow-deck`.

## Reflection

Dễ nhầm Herdr với máy phiên, hoặc nâng deck thành multiplexer. Hai sự thật phải tách: log (động cơ 1) và PTY (động cơ 2).

## Decision

Học topology + detach-strong + năm trạng thái + cổng `HERDR_ENV`. Từ chối binary, TUI-as-identity, PATH hosts, marketplace.

## Next

Operator đã ký 2026-08-21. Dừng. Cổng chết vẫn đóng (thiếu điều kiện người/thư). Cấm plan.

> Historical work record — not durable authority.
