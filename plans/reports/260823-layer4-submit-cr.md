---
type: cook
date: 2026-08-23
time: 00:28
status: landed
feeds: plans/260823-0011-close-coding-occupancy
---

# Phase 2 — submit after live bracketed-paste is CR

`agent_prompt` when `CSI ? 2004 h` is live: wrap text, then write `\\r` (Enter). LF after paste is a compose line, not submit (4f: text sat in the omp editor).

`send-keys enter` stays `\\n` (line-oriented fixtures). Classifier unchanged. No `--kind`. No `omp` in cargo tests.

Proof: `p5_prompt_paste` + factory `260823-layer4f2-omp-factory.md` (`get` = `done` after report).
