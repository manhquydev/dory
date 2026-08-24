---
title: Session OS nontech door
date: 2026-08-24
summary: Node lamp is a Vietnamese journal+goal door; Flow is fail-closed; bare dory stays desk.
---

# Session OS nontech door

## What happened

Independent first-open review: non-tech cannot sit at bare `dory` (Workplace OS / tty / Ctrl-b). Session OS is the other door. Cook implemented plan `260824-0944` on the Node lamp only.

## Decision

- Bare Rust `dory` stays the desk (0817).
- Helper starts `node bin/dory.js serve --workspace /abs` → `http://127.0.0.1:7380/`.
- Journal file is truth. GET `/` is Vietnamese cards, not a JSONL dump.
- `POST /goal` → `session/goal`. `POST /note` unchanged.
- `POST /flow` requires `confirm === true` or 403 (no spawn, no invoke). Called-out break; tests updated.
- No `/workplace` growth, no rust edit, no Herdr clone, no .app.

## Evidence

- `node --test` 16/16.
- Reviewer 8/10, 0 critical, HARD-GATE not triggered.
- PM: `plans/reports/pm-2026-08-24-1013-session-os-nontech-door.md`. Store `dory/260824-0244` completed/closed.

## Next steps

- Operator commit decision (src/ + test/ still untracked).
- Non-goals remain: Pew self-start, Cowork runtime, desk onboarding.

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
