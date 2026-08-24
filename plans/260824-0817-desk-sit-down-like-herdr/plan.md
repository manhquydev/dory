---
title: "Desk sit-down like Herdr"
description: "Rewrite Dory desk chrome and sit-down verbs so a human sitting `dory` gets Herdr’s glance + keys + close + zoom-alive + copy — own code, not their TUI."
status: completed
priority: P1
effort: 18h
branch: main
tags: [feature, dory, desk, sit-down]
blockedBy: []
blocks: []
created: 2026-08-24
---

# Desk sit-down like Herdr

Successor of `plans/260823-1326-desk-spatial-grid/` (grid shipped). Do **not** flip paper on `260822-0847-workplace-skill-mux`, `260823-0011-close-coding-occupancy`, `260823-0859-section-11-real-repo`.

## Contract

| Field | Closed |
|---|---|
| **Outcome** | Gõ `dory` trên tty: mặt ngồi (sidebar glance + tab bar + lưới) và phím/chuột chỗ ngồi khớp Herdr sit-down. Detach ≠ kill. |
| **Constraints** | CHARTER hình B. Crossterm + vt100. Không serde. Socket = luật; desk = client. `DORY_ENV` giữ. Xia `--copy` cấm. Không `exec herdr`/`dsh`. Gate: `cargo test --offline --locked`. |
| **Non-goals** | Clone `herdrdev/herdr`. Ratatui identity. Marketplace / `--kind` / remote / detect farm. Đục `DORY_ENV`. Cấy flow-skill. Session OS mới. |
| **Acceptance** | Prefix table Herdr trên desk (không phím trần `x`/`1`); sidebar rollup **năm** trạng thái kể cả `unknown`; tab bar; zoom không cắt stream; kéo-chọn (ngưỡng, không click) copy OSC 52; close không được để phòng trống; suite xanh. Không TUI trong pane factory. |

“Như Herdr” = **hành vi chỗ ngồi** (2015 absorb). Không = pixel Ratatui / plugin / PATH hosts.

## Scout (2026-08-24)

| Surface | Today |
|---|---|
| Grid | Daemon BSP + N attach + drag + click + `hjkl` — `layout.rs`, `desk.rs`, `desk.layout` |
| Paint | Full `draw()` mỗi phím. Wrap gutter **đã có trong working tree** (`DisableLineWrap`, `bar_line`, `pane_size` −1). Chưa commit, chưa ngồi TTY |
| Sidebar | Cây phẳng `desk.tree` `w→t→p` + `occ`/`st` trên pane. 22 cột cố định |
| Tabs | Title `w · t · p`. Không chip. `n/p` = ô phẳng |
| Keys | `w` = tạo cửa sổ. Không picker, không close prefix, không `1..9` |
| Close | Chỉ `tab.close`. Không `pane.close` / `workspace.close`. `layout.rs` không `remove_leaf` |
| Zoom | `reconcile_tiles` **drop** tile không focus → cắt attach |
| Copy | Không |
| Flow | Taxi giữ. Không đụng |

Research reused (không Xia thứ ba): `260821-1436-xia-compare-herdr.md`, `260823-1320-research-herdr-sit-down.md`, `260823-2015-herdr-codebase-sitdown.md`, `260823-2247-brainstorm-clone-herdr-flow.md` (không clone).

## Design

```text
daemon World     occ/st/layout/close     ── JSON RPC ──►  desk.rs chrome
     │                                              Spaces/Agents + tab chips
     │                                              prefix table + picker
HeldPty stays on zoom; desk keeps N attach streams
```

Fact trên socket. Card/màu/mode chỉ trong `desk.rs`.

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Paint hold](./phase-01-start.md) | Done |
| 2 | [Glance chrome](./phase-02-glance-chrome.md) | Done |
| 3 | [Keys and close](./phase-03-keys-and-close.md) | Done |
| 4 | [Zoom keeps streams](./phase-04-zoom-keeps-streams.md) | Done |
| 5 | [Copy overlay and docs](./phase-05-copy-overlay-and-docs.md) | Done |

## Success Criteria

- [x] Gõ trong ô không làm host wrap-scroll (gutter + không full-repaint chrome mỗi phím)
- [x] Sidebar: Spaces rollup + Agents; click nhảy; `prefix+b` collapse
- [x] Tab bar chip; `prefix+n/p` / `prefix+1..9` đổi **thẻ** cùng cửa sổ; `desk.neighbor` next/prev giữ ô
- [x] `prefix+w` = picker; `prefix+Shift+n` = tạo cửa sổ
- [x] `prefix+x` / `prefix+Shift+x` / `prefix+Shift+d` đóng; `prefix+q`/`prefix+d` detach; cấm phòng trống
- [x] Zoom: sibling attach vẫn `pump_pty`
- [x] Kéo-chọn ≥2 cell copy OSC 52 (không copy-on-click)
- [x] `cargo test --offline --locked` xanh; README / USAGE / SKILL khớp
- [x] Không Ratatui, không serde, không `herdr` trong runtime path

## Kill if

CHARTER kill, Xia `--copy`, `cargo add ratatui`, đục `DORY_ENV`, lật checkbox 0847/0011/0859.

## Red Team Review

### Session — 2026-08-24
**Findings:** 12 unique after dedupe (8 accepted, 4 rejected)
**Severity breakdown:** 4 Critical accepted, 4 High accepted, 0 Medium accepted as standalone (folded)

| # | Finding | Severity | Disposition | Applied To |
|---|---|---|---|---|
| 1 | Last pane/`tab.close` empties last workspace | Critical | Accept | Phase 3 |
| 2 | `Shift+d` collides with today’s detach `d`/`D` | Critical | Accept | Phase 3 |
| 3 | `pane.close` must update `Tab` triple (vec + layout + `root_pane`) | Critical | Accept | Phase 3 |
| 4 | Picker/confirm/help need `Mode` before `encode_key` | Critical | Accept | Phase 3, 5 |
| 5 | `loop_ui` `progressed=true` on every key kills dirty-paint | High | Accept | Phase 1 |
| 6 | Chrome metrics + hit-test (`SIDEBAR` const, tab row) | High | Accept | Phase 2, 4 |
| 7 | `prefix+b` steals send-`C-b`; keep `prefix+Ctrl-b` | High | Accept | Phase 2 |
| 8 | Phase 4 must depend on chrome geometry; no `Tile.hidden` | High | Accept | Phase 4 |
| 9 | Rollup walks `p.st` including `unknown`; hit-list ≠ tree index | High | Accept | Phase 2 |
| 10 | `desk.neighbor` next/prev stay pane-global; tab walk client-side | High | Accept | Phase 3 |
| 11 | New CLI close arms call `require_skill_env` | High | Accept | Phase 3 |
| 12 | Copy: min-drag, no click-copy, no fake OSC fail | High | Accept | Phase 5 |
| — | Occupant must not close a foreign workspace | High | **Reject** | Coordination product: occupant with `DORY_ENV` may close siblings. Env gate + refuse last-room is enough. |
| — | Copy is always clipboard exfil → drop feature | High | **Reject** | Threshold + no hidden tiles. Feature stays. |

### Whole-Plan Consistency Sweep

Decision delta applied across `plan.md` + all five phases:

- Close never empties the last live pane/workspace.
- All new sit-down chords are **prefix**; `SHIFT` is `KeyModifiers`, not `'N'`/`'D'` or-patterns.
- `Mode` lives from Phase 3; Phase 5 only adds Help/Select.
- Phase 4 `dependencies: [1, 2]`.
- `desk.neighbor` `next`/`prev` contract frozen for `--plain`.

Unresolved contradictions: **none**.

<!-- slug: desk-sit-down-like-herdr -->
