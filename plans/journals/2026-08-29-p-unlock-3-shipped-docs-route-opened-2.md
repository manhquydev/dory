---
title: P unlock-3 shipped; docs route opened
date: 2026-08-29
summary: Isolate P paid; docs/ + AGENTS.md + CHARTER WHERE; leftover 5 still mint
---

# P unlock-3 shipped; docs route opened

## What happened

P isolate unlock-3 shipped as `f1c966c` `feat(isolate): fail-then-pass flow.sh prd` plus paper `049e304`. Independent journal: taxi `flow -- next` codes `[1,0]`, FAIL `02-scope` then `unlocked stage 3 (flow/03-prd.md)`.

`ak plan status` on `260829-0054-isolate-flow-prd-unlock` is 3/3 phases, 5/5 tasks. No leftover product hunk in that plan. Company Phase 5 / fill 03 / default sit remain unpaid.

Docs gap after ship: no `docs/` route, no P journal, HEAD README "Xong tới đâu" does not list isolate taxi, leftover working README is a different blob.

## Decision

Approach A: add `docs/README.md` (WHERE), one CHARTER pointer, thin `AGENTS.md` deny-list, this journal. Do not edit leftover 5. Do not recook P. Do not claim Phase 5.

## Next steps

- Red-team the new docs for stale paid claims, leftover fold, Phase 5 lie, broken paths
- Keep leftover 5 `M` and rust log `b544f5f`
- Next product cook is not this wave

> Historical work record — not durable authority. Prefer docs/specs/ADRs for current decisions.
