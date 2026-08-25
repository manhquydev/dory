---
type: test
date: 2026-08-25
time: 21:24
scope: desk sidebar status-color (2109) + full crate
command: cargo test --offline --locked
cwd: rust/
exit: 0
status: pass
---

# Test Report — 2026-08-25 — desk sidebar status-color

Fresh official gate this turn. No factory TTY sit. Default sock not stopped. No `desk.rs` edit. No leftover Herdr pane recycle. No commit.

## Test Results Overview

- **Total**: 168 tests / 19 suites
- **Passed**: 168 | **Failed**: 0 | **Skipped**: 0 | **Ignored**: 0
- **Duration**: ~24s (compile 1.05s + run; wall 24.2s)
- **Preflight**: `cargo check --offline --locked` exit 0 (5 existing dead_code warnings)
- **Flake retry**: none. First official run exit 0. No ECONNRESET on `p5_discover` / `p5_occupant`.

| Suite | Passed |
|---|---|
| unit `src/main.rs` | 103 |
| `ensure_bin` | 1 |
| `flow_taxi` | 10 |
| `p3_nested_from_env` | 1 |
| `p3_second_master` | 1 |
| `p5_attach` | 6 |
| `p5_close` | 7 |
| `p5_discover` | 6 |
| `p5_inside` | 1 |
| `p5_layout` | 3 |
| `p5_live_loop` | 1 |
| `p5_occupant` | 13 |
| `p5_prompt_paste` | 2 |
| `p5_prompt_unknown` | 1 |
| `p5_real_repo` | 1 |
| `p5_report` | 5 |
| `p5_s11` | 1 |
| `p5_skill_occ` | 1 |
| `pane_io` | 4 |

## Coverage Metrics

| Metric | Value | Threshold | Status |
|---|---|---|---|
| Lines | not measured | 80% | N/A — no tarpaulin/llvm-cov in crate |
| Status-color locks | present | — | PASS |

Named locks this run (all ok): `status_fg_table`, `sidebar_status_color_focused_wide_working_keeps_gold_dot`, `sidebar_status_color_compact_working_is_accent_w`, `sidebar_status_color_agent_blocked_colors_word_only`, `sidebar_status_color_clips_lead_keeps_status_word`, `sidebar_status_color_compact_agent_keeps_occ_initial`, `sidebar_status_color_empty_shell_is_dot_not_unknown`, `empty_shell_space_card_is_folder_not_unknown`, `sidebar_wide_rule_is_full_width_when_agents_exist`, `sidebar_hides_agents_when_empty`, `agents_stay_at_sidebar_bottom_when_spaces_overflow`, `sidebar_hit_is_not_tree_row_index`, `working_pty_frame_does_not_wipe_pane`.

## Failed Tests

None.

## UI Test Results

Skipped. Desk is TTY Crossterm, not a web app. Factory sit forbidden.

## Build Status

- **Build**: PASS (`Finished test` + `cargo check` exit 0)
- **Warnings**: pre-existing `ids.rs` / `pty.rs` dead_code (check: 5; test bin: 1 + 5)
- **Dependencies**: `--locked` resolved

## Critical Issues

None from this run.

## Recommendations

1. Known ECONNRESET flakes (`p5_discover`, `p5_occupant`) — not this cook; first run this turn was clean.
2. Coverage tool not in tree — do not invent a % for ship.

## Unresolved Questions

- Sit TTY squint (gold ● vs accent status word) — factory cấm.
