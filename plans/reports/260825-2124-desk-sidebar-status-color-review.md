---
type: review
date: 2026-08-25
time: 21:24
role: code-reviewer
stages: 1+2
plan: plans/260825-2109-desk-sidebar-status-color/plan.md
phase: plans/260825-2109-desk-sidebar-status-color/phase-01-start.md
cwd: /home/manhquy/Downloads/flow/dory
base_sha: b54c6bb27c74e033aad9053cd998c97731111737
head_sha: uncommitted desk.rs
verdict: accept
score: 8
critical_count: 0
hard_gate: none
---

# Stages 1+2 — `260825-2109-desk-sidebar-status-color`

Quality + spec review of the status-color cook only. Product writer is `rust/src/desk.rs`. Leftover `attach.rs` / `server.rs` / `main.rs` / `README.md` / `p5_attach.rs` are stacked cooks — not scored, not folded. No product edit. No commit. No factory TTY sit. Default sock not touched.

Score **8/10**. **critical_count 0**. **hard_gate none**. Stage 1 **PASS**. Stage 2 **(a)–(e)** hold.

## Score card

| Field | Value |
|---|---|
| **score** | **8/10** |
| **critical_count** | **0** |
| **hard_gate** | **none** — `--kind`, Herdr RGB, dye-row, parse-`text` for `st`, clip-eats-st, compact gold, compact Agents→B/W/D, factory TTY, second writer, glance_rows working-wins: all closed |
| **verdict** | **accept** — Stage 1 all required rows PASS; (a)–(e) hold; locked doors unviolated |

**warnings**

1. Working tree still carries leftover `attach.rs` / `server.rs` / `main.rs` / `README.md` / `p5_attach.rs`. Not 2109 writers. A status-color land that folds them is the real process risk.
2. `sidebar_row_spans` (`2657-2688`) does not clip. Clip lives in `clip_lead` → `occup_side_hit` (`2605-2616`, `2642`). Sit rebuilds the model at the same `side` every frame, so paint matches. A later caller with raw `lead` or a different width overflows the gutter.
3. Phase wording said `lead`/`tail` **raw**. Stored `lead` is already clipped (`2645-2653`). `tail` is raw. `text` is the padded concat for old glance/hit locks. Sit does not paint `text` on Workspace|Agent.
4. No 2109 journal. Plan success list does not require one. Hairline did.
5. Official crate gate this turn was 168/168 (`260825-2124-desk-sidebar-status-color-test.md`). Known ECONNRESET flake class on occupant/server tests is not this cook.

**suggestions**

1. Land `desk.rs` only. Do not fold leftover files into 2109.
2. Optional: clip again inside `sidebar_row_spans` (phase step 3) so the helper is total. Sit does not need it today.
3. Optional: one spans lock that wide empty-shell mid is `""` (compact `·` already locked).
4. Sit squint (gold ●+label vs ACCENT word) is operator-only. Factory sit forbidden.

**side-effect flags**

| Flag | Value | Evidence |
|---|---|---|
| `hit_index_shift` | **false** | New fields on `SideHit` (`2560-2569`). No extra Spaces rows. Rule still first `agents_hits` row (`2774`). Glance row 4 still `w2:p1` (`3876-3878`). |
| `overflow_pin_regressed` | **false** | `agents_stay_at_sidebar_bottom_when_spaces_overflow` ok. `model[6]` Agents, `model[7]` Agent, no `w8`. Compact last row Agent. |
| `empty_occ_fake_band` | **false** | Empty `occ`: no Rule, no `"Agents"` (`3666-3673`, `3914-3916`). `ah==0` → no TITLE_BG band (`3676`). |
| `menu_workspace_cards_changed` | **false** | `menu_hit` still Workspace-only (`2993-2994`). Glance col 1 row 4 → `w2` (`3927-3941`). Rule/Agent `pane` path rejected. |
| `public_contract` | **false** | Still only `pub fn run` / `run_with_pane` (`162`, `166`). `SideHit` / `status_fg` / spans private. No `DESK_ABI`, CLI, JSON from this cook. |
| `rgb_herdr` | **false** | `BLOCKED_FG` = `Rgb{232,96,88}` (`156-160`). Not a documented Herdr/Catppuccin hex in-tree. 1120 tokens unchanged. |
| `dye_row` | **false** | `sidebar_row_bg` still SIDE_BG / TITLE_BG only (`2551-2556`, `1581`). Status is fg on glyph/word, not row bg. |
| `compact_gold` | **false** | Compact `lead_fg = status_fg` (`2669-2670`). Focused `w1` working → ACCENT `W`, not `FOCUSED_FG` (`3793-3796`). |
| `clip_eats_st` | **false** | `clip_lead` reserves `display_width(mid)+display_width(tail)` (`2612-2615`). Long CJK fixture keeps `" working"` (`3820-3838`). |
| `server_rs_this_cook` | **false** | 2109 writer is `desk.rs`. `server.rs` dirt is leftover. |
| `journal_written` | **false** | No `plans/journals/2026-08-25-desk-sidebar-status-color.md`. |
| `working_tree_leftover` | **true** | vs HEAD `b54c6bb`: desk.rs + stacked cooks; attach/server/main/README/`p5_attach.rs` also dirty. |

## Code Review Summary

### Scope
- Files: `rust/src/desk.rs` (status-color hunks only). Plans not scored as product.
- LOC (2109): `BLOCKED_FG`, `SideHit.{st,lead,tail}`, `status_fg` / `show_status_word` / `compact_space_ch` / `clip_lead` / `chrome_side_hit` / `occup_side_hit` / `sidebar_row_spans`, `draw_sidebar` split, `working_only_rows` + six color locks ≈ 220 lines on stacked leftover desk.rs.
- Focus: Stage 1 spec vs plan Success Criteria + phase Print table + red-team Accept; Stage 2 (a)–(e); locked doors; hit/overflow/empty-occ/menu.
- Scout findings: three callers still share `sidebar_model` (paint `hits[y]`, left-click `sidebar_focus_at`, right-click `menu_hit`). Color is fg-only. Clip is construction-time. `glance_rows` still blocked-wins. Dependents of `SideHit` are `desk.rs` only.

### Overall Assessment

Sit path is two arms: Workspace|Agent → `sidebar_row_spans` (2–3 `Print`); Chrome|Rule → one `Print` + `sidebar_paint_text`. Status comes from `hit.st` (`rollup_of` / `normalize_st`), not `text`. Gold is wide focused `●+label` only. Compact glyph/occ-initial is `status_fg`. Rule still bypasses `pad_cols`. Overflow pin, empty hide, and glance row 4 stay put. Public surface, crates, and 1120 tokens are untouched.

Residual risk is land hygiene (leftover tree) and clip living one layer below the named helper — not sit paint.

### Critical Issues

None.

### High Priority

None.

### Medium Priority

1. Leftover dirty files — commit hygiene, not a status-color defect. Folding `server.rs` into a 2109 land would create a public-contract side effect this cook does not own.
2. Clip is in `occup_side_hit`, not `sidebar_row_spans`. Phase step 3 named the latter. Sit is safe because `draw_sidebar` rebuilds hits at the same `side` (`1573-1586`). Do not “fix” unless a second caller appears.
3. No 2109 journal. Not a success-box item. Process gap vs hairline.

### Low Priority

1. Print table compact row says 1 Print. Architecture + cook brief say lead / mid / pad+tail = 2–3. Sit prints lead + rest (mid empty) = 2. Visual: glyph `status_fg`, pad `TEXT`. Not a dye-row. Do not collapse to one Print just to match the table cell.
2. `draw_sidebar` still inlines `rows.saturating_sub(3)` (`1573`) instead of `sidebar_paint_height`. Hit/menu already use the helper. Pre-existing 1333 nit.
3. No named spans lock that wide empty/idle mid is empty. `empty_shell_space_card` already forbids `"unknown"` in `text`. Compact `·` locked.
4. Long-label fixture is CJK cwd, not a folder named `working-copy`. Red-team #2 is closed by not parsing `text`. Extra name lock would be assertion-tightness (suppressed).
5. `pad_display` (`2598-2603`) pads then clips the `text` field only. Sit Workspace|Agent does not print `text`. Leave it.
6. Gutter `│` is always TITLE_BG (`1612-1617`). Pre-existing chrome seam. Do not retint.

### Edge Cases Found by Scout

- Three callers share `sidebar_model(..., rows-3)`: paint `hits[y]`, left-click `sidebar_focus_at` (`mouse_row-2`), right-click `menu_hit`. Adding `st`/`lead`/`tail` does not shift indices. Glance: Spaces + w1 + w2 → idx 2 = mouse row 4 = `w2:p1`. Tests ok.
- `glance_rows` w1 still rollup **blocked** (`3572-3574`). w1:p1 `st=working` has `occ=""` and is ignored by `rollup_of` (`2346`). Working-wins uses `working_only_rows` (`3697-3724`), not glance. Locked door held.
- Focused compact working is ACCENT `W` (`3786-3797`). `sidebar_workspace_focused` is ignored on the compact arm (`2669-2670`). Agents never gold: kind ≠ Workspace (`2594-2596`, `3809-3810`).
- Wide focused working: lead `FOCUSED_FG`, mid `" working"` `ACCENT`, rest `TEXT` (`3766-3783`). Label rides with `●` (phase lead + cook brief). Research “dot-only gold” lost to the Print table. Not a defect.
- Clip: budget = `side - dw(mid) - dw(tail)`. Prefix kept (`clip_to` `3442-3456`). Long 名称 label drops `"must-clip"` / `称`; mid stays (`3827-3833`). `pad_display` clip-from-start would eat tail/mid if `clip_lead` failed; concat is reserved so it does not.
- Compact Agents: `" c"` from `coder`, not `B` (`3842-3853`). `compact_space_ch` is Spaces-only (`2731`).
- Empty shell: `rollup_of` `""`, compact `" ·"`, `status_fg` MUTED, no `"unknown"` (`3856-3868`, `3612-3625`). Wide empty: no mid (`show_status_word` `2571-2573`).
- Rule/Chrome never enter spans (`1584` vs `1593-1601`). `sidebar_paint_text(Rule)` still rebuilds `─` × width. `pad_cols(sidebar_rule(26))==13` lock kept (`3678-3682`).
- Overflow height 8, 1 occupant: `ah==3`. `[6]` Agents, `[7]` Agent, no `w8`. Compact height 6: last row Agent. Unchanged.
- Empty `occ`: `agent_region_rows(..., 0)==0`. No TITLE_BG band. No fake Agents chrome. 1333 hide kept.
- `display_width` still maps non-ASCII → 2 (`3426-3428`). `●` / CJK / `─` depend on that lie. Do not change it. Status words are ASCII; reserve is honest.
- `workspace==""`: focused false (`2595`). Agent `workspace` stored `""` (`2746`, `2783`).
- Tiny TTY (`height` 1–2) can show rule / header and clip the occupant. Pre-existing `agent_region_rows` reserve. Do not “fix” in this cook.
- Tile `desk.divider` drag is not a Spaces/Agents splitter. Width cycle still 26↔4↔0.
- MUTED `done`/`unknown` word on SIDE_BG is an unread sit question. Factory sit forbidden. Not a code defect.

### Positive Observations

One helper for occup rows, Rule/Chrome stay on the 1959 helper, `st` is a field, working-wins has its own fixture. The `pad_cols(sidebar_rule(26))==13` lock still fails a silent revert to the old Rule Print path. Tokens and crates were not touched.

### Recommended Actions

1. Land `desk.rs` status-color only. Do not include leftover `server.rs` / `attach.rs` / `main.rs` / README / `p5_attach.rs`.
2. Optional: clip inside `sidebar_row_spans`. Not blocking.
3. Leave plan checkboxes to the lead. Store is still `pending` / phase todos unchecked. This review does not flip plan state.

### Metrics
- Type coverage: n/a (Rust; `cargo test` typechecks). No new `desk.rs` warning.
- Test coverage: no line %. Official gate this turn **168/168** exit 0 (`260825-2124-desk-sidebar-status-color-test.md`). This review re-ran 13 named locks: **13/13** ok. Did not re-sit the full crate (flake class is occupant/server, not desk).
- Linting issues (this cook): 0 new. Pre-existing `dead_code` in `pty.rs` / `ids.rs` only.

### Unresolved Questions

- After paint: is gold on `●+label` (phase) enough, or does the operator want dot-only gold (research)? Sit squint only.
- Land 2109 as a desk.rs-only commit on top of stacked uncommitted cooks, or wait for a clean tree?

---

## Mandatory checks (acceptance)

| Check | Status | Evidence |
|---|---|---|
| (a) Wide Spaces: st word colored; focused `●` gold | **PASS** | `show_status_word` (`2571-2573`); mid `status_fg` (`2683-2684`); focused lead `FOCUSED_FG` (`2671-2672`, `3776-3778`). |
| (a) Wide Agents: only st token colored; occ + short TEXT | **PASS** | lead TEXT (`3810`); mid `BLOCKED_FG` (`3811-3812`); rest_fg TEXT (`2685-2686`); tail `" {short}"` (`2786`). |
| (a) Compact Spaces: `B/W/D/I/U/·` by rollup; no gold | **PASS** | `compact_space_ch` (`2583-2591`, `2731`); `lead_fg = status_fg` (`2669-2670`, `3795`). |
| (a) Compact Agents: occ initial, not B/W/D | **PASS** | `occ.chars().next()` (`2741-2748`); `" c"` not `B` (`3849-3850`). |
| (a) Rule/Chrome: one Print + helper; no `pad_cols` on `─` | **PASS** | `1593-1601`; `sidebar_paint_text` (`2543-2548`). |
| (a) Idle/empty: no fake working color; empty `·` not unknown | **PASS** | `status_fg` else MUTED (`2579`); empty shell (`3857-3868`). |
| (b) Agents pin bottom on overflow | **PASS** | Test ok. `model[6]` Agents, `model[7]` Agent, no `w8`. |
| (b) Glance row 4 still `w2:p1` | **PASS** | `3876-3878`, `3922-3925`. |
| (b) Hide Agents when empty | **PASS** | `sidebar_hides_agents_when_empty` ok. No Rule. |
| (b) Hit/menu on workspace cards | **PASS** | `menu_hit` glance → `w2` (`3927-3941`). Workspace-only. |
| (c) No public contract | **PASS** | `pub` still `run` / `run_with_pane` only. Helpers private. |
| (d) Existing paint helpers | **PASS** | `queue!` + spans / `sidebar_paint_text`. TITLE_BG/SIDE_BG reuse. `rollup_of` / `normalize_st` kept. |
| (e) Tests pass | **PASS** | Official this turn 168/168 exit 0. This review: 13 named locks ok. |
| Non-goal dye whole row | **PASS** | No status bg. Approach A. |
| Non-goal Ratatui / GPUI | **PASS** | Header still `Not Ratatui`. `Cargo.toml` unchanged. |
| Non-goal Herdr RGB / `--kind` / factory TTY | **PASS** | No `--kind` in `desk.rs`. `BLOCKED_FG` is new Dory warm. No TTY sit this review. |
| Non-goal `server.rs` / commit / 1145 | **PASS** | Leftover `server.rs` is not this cook. HEAD still `b54c6bb`. |
| One writer `desk.rs` | **PASS as 2109** | Leftover other files are not this cook. |
| Working-wins not hung on `glance_rows` | **PASS** | New `working_only_rows` (`3697`). Glance still blocked (`3574`). |

---

## Stage 1 — Spec compliance

| # | Requirement | Status | Notes |
|---|-------------|--------|-------|
| 1 | Wide Spaces: st word (if any, ≠ idle) colored by rollup; focused `●` = `FOCUSED_FG` | PASS | `2770` mid via `st`; `3775-3778`. Lead is `●+label` gold (phase Print table + cook brief). |
| 2 | Wide Agents: **only** st token colored; `occ` + short id TEXT | PASS | `2785-2786`, `3807-3812`. |
| 3 | Compact Spaces: `B/W/D/I/U/·` by rollup; no gold | PASS | `2731`, `3793-3796`. |
| 4 | Compact Agents: first `occ` char; not rewritten to B/W/D | PASS | `2741-2748`, `3849-3850`. |
| 5 | Rule/Chrome: one Print + `sidebar_paint_text`; never `pad_cols` on `─` | PASS | `1593-1601`, `2543-2548`, `3678-3683`. |
| 6 | Idle/empty: no fake working color; empty shell `·` not `unknown` | PASS | `2579`, `3614-3625`, `3857-3868`. |
| 7 | Gate `cargo test --offline --locked` | PASS | Official 168/168 exit 0 this turn. Review: 13/13 named locks. |
| 8 | `SideHit` adds `st`, `lead`, `tail` (not `pad_cols`'d) | PASS | `2566-2568`. Lead is pre-clipped, not padded. Tail raw. See warning 3. |
| 9 | Chrome/Rule: `st=""`, empty lead/tail; paint via helper | PASS | `2619-2628`, `1593-1601`. |
| 10 | Workspace `st = rollup_of`; Agent `st = normalize_st(&pane.st)` | PASS | `2724` / `2758`; `2740` / `2777`. |
| 11 | Wide Spaces `lead = " {●\|○} {label}"`, `tail=""`, mid ` {st}` iff show | PASS | `2759-2769`; `show_status_word` `2571-2573`. |
| 12 | Wide Agents `lead = " {occ}"`, mid if show, `tail = " {short}"` | PASS | `2785-2786`. |
| 13 | Compact Spaces mid/tail empty; whole glyph `status_fg` | PASS | `2731-2733`, `2663-2670`. |
| 14 | `status_fg`: blocked=`BLOCKED_FG` `{232,96,88}`; working=`ACCENT`; done/idle/unknown/`""`=`MUTED` | PASS | `156-160`, `2575-2580`, `3756-3762`. |
| 15 | `focused` = Workspace && desk_ws nonempty && hit.ws == desk_ws | PASS | `2594-2596`. |
| 16 | Clip **lead** only; reserve mid+tail via `display_width`; concat `== side` | PASS | `2605-2616`; locks `3779-3782`, `3813-3816`, `3835-3838`. Clip site is `occup_side_hit`, not spans. |
| 17 | One writer `desk.rs`; no `--kind`; no factory TTY | PASS | `--kind` absent in `desk.rs`. This review did not sit TTY. |
| 18 | Print: Rule/Chrome — no mid; MUTED via helper; 1 Print | PASS | `1593-1601`. |
| 19 | Print: Wide Space empty/idle — no mid; gold if focused else TEXT; 1–2 Prints | PASS | `1587-1591` (lead + rest). |
| 20 | Print: Wide Space working/blocked/done/unknown — mid `status_fg`; 3 Prints | PASS | lead + mid + rest when `show_status_word`. |
| 21 | Print: Wide Agent — mid if `st≠"" && ≠idle`; lead TEXT; 3 Prints | PASS | `2663-2667`, `2673-2674`. Idle Agent = 2 Prints (table “3” is the show-mid case). |
| 22 | Print: Compact Space/Agent — no mid; `status_fg`; table says 1 Print | PASS | Architecture + cook brief = 2–3. Sit = lead + rest (2). Mid empty. See Low 1. |
| 23 | RT#1: paint from field, do not parse `text` / suffix | PASS | `hit.st` only. No `rsplit` on `text` for `st`. `rsplit` on pane id is short-id (`2778`). |
| 24 | RT#2: `clip_to` must not eat status word; `working-copy` not a parse target | PASS | Reserve mid+tail. CJK lock keeps `" working"`. |
| 25 | RT#3: Rule/Chrome keep 1959 helper | PASS | Same `sidebar_paint_text` / `sidebar_rule`. |
| 26 | RT#4: compact color = st; gold only wide `●` | PASS | `2669-2672`, `3795`. |
| 27 | RT#5: do not search `st` inside `occ`/cwd | PASS | `st` from rollup / pane field. |
| 28 | RT#6: new fixture; do not hang working-wins on `glance_rows` | PASS | `working_only_rows` (`3697`). Glance still blocked (`3574`). |
| 29 | RT#7: compact Agents ≠ B/W/D | PASS | `3849-3850`. |
| 30 | RT#8: pad / reserve via `display_width` | PASS | `2612`, `2677-2679`, `2598-2602`. |
| 31 | RT#9: no `focused` on `workspace==""` | PASS | `2595`. |
| 32 | RT#10: one helper `sidebar_row_spans` | PASS | `2657`; `draw_sidebar` `1585-1586`. |
| 33 | RT#11: idle/`""`/unknown table | PASS | `status_fg_table` `3756-3762`. |
| 34 | Non-goal dye-row / Ratatui / Herdr clone / hook / 2050 sit / `server.rs` / commit / 1145 | PASS | Approach A. Leftover `server.rs` not this cook. HEAD `b54c6bb`. |
| 35 | Keep `rollup_of` / `normalize_st` | PASS | `2337-2356`, `2312-2319`. Occupied-only rollup kept (`2346`). |
| 36 | `BLOCKED_FG` is new Dory warm, not Herdr hex | PASS | `{232,96,88}`. No in-tree Herdr `#E86058`. |
| 37 | Founder both columns | PASS | Spaces + Agents both painted. |

**Stage 1 spec compliance: PASS** on product (a)–(e). No missing requirement. No unjustified extra (constructors / `clip_lead` / `show_status_word` are the phase architecture). Clip-site and compact Print-count are notes, not MISSING.

---

## Pre-Landing Review (base checklist)

Pre-Landing Review: 3 issues (0 critical, 3 informational)

**CRITICAL** (blocking):
- none

**Issues** (non-blocking):
- [working tree] Leftover `server.rs` / `attach.rs` / `main.rs` / README / `p5_attach.rs` sit next to `desk.rs`.
  Fix: commit 2109 as `desk.rs` only.
- [rust/src/desk.rs:2657] `sidebar_row_spans` pads rest but does not clip lead (phase step 3 named this helper).
  Fix: optional `clip_lead(&hit.lead, hit.st, &hit.tail, side)` at the top of spans. Sit path already clipped.
- [plans/journals] No 2109 journal.
  Fix: optional; plan success list does not require it.

Suppressions applied: compact 1-vs-2 Print table cell (architecture wins); fitted/assertion tightness (`working-copy` name); 1333 `rows-3` spelling; gutter TITLE_BG (pre-existing); “fix `display_width`” (would break labels); tiny-height chrome preference (pre-existing `agent_region_rows`); research dot-only gold (phase lead + cook brief chose `●+label`); `pad_display` on `text` only; `unwrap_or('·')` on agents already filtered.

---

## Verification

Official this turn (`260825-2124-desk-sidebar-status-color-test.md`):

```text
cd /home/manhquy/Downloads/flow/dory/rust && cargo test --offline --locked
```

Exit **0**. 103 unit + 65 integration = **168/168**. No TTY. Default sock not stopped.

This review re-ran named locks only (no full crate, no sock):

```text
cargo test --offline --locked -- --test-threads=1 \
  status_fg_table \
  sidebar_status_color_focused_wide_working_keeps_gold_dot \
  sidebar_status_color_compact_working_is_accent_w \
  sidebar_status_color_agent_blocked_colors_word_only \
  sidebar_status_color_clips_lead_keeps_status_word \
  sidebar_status_color_compact_agent_keeps_occ_initial \
  sidebar_status_color_empty_shell_is_dot_not_unknown \
  empty_shell_space_card_is_folder_not_unknown \
  sidebar_wide_rule_is_full_width_when_agents_exist \
  sidebar_hides_agents_when_empty \
  agents_stay_at_sidebar_bottom_when_spaces_overflow \
  sidebar_hit_is_not_tree_row_index \
  rollup_blocked_beats_working_and_keeps_unknown
```

**13 passed / 0 failed**. Warnings: pre-existing `dead_code` only.

## CHARTER

Hình B. Desk stays a crossterm + vt100 socket client. No Ratatui identity, no GPUI, no Herdr RGB, no `--kind`. Status color is fg marks on the existing writer.

## Plan follow-up

Phase 1 todos and plan success boxes are still unchecked; plan `status: pending`. Product paint matches the contract. This review does not flip plan state. Next process step if the operator asks to land: `desk.rs` only. Do not reopen the cook for leftover files or optional spans re-clip.
