---
phase: 4
title: "Phase 4: Docs and isolated proof"
status: completed
priority: P2
effort: 2h
dependencies: [3]
---

# Phase 4: Docs and isolated proof

## Overview

Docs match the grid face. Isolated XDG proof: two live PTYs tiled on the socket; detach leaves them. No paper-phase flips on older plans.

## Context

- README sit-down table: `README.md:15-26` still “one PTY / click cây”
- USAGE const: `rust/src/main.rs:15-50` — “sidebar + live pane”
- USAGE tests: `main.rs:635-638`
- Skill: `skills/dory/SKILL.md:34` “sidebar + live pane”; Layout section `:86` is CLI split (keep)
- Help test: `rust/tests/p5_attach.rs:254`
- Proof style: isolated harness like `p5_attach.rs:49` — no herdr, no `:7380`

## Requirements

- Functional: README / USAGE / skill say **N live tiles**, click tile, drag, `hjkl`; occupant verbs unchanged
- Non-functional: `rg -i herdr` on shipped rust/skill stays learn-only (no exec); `cargo test --offline --locked`; do not edit `0847` / `0011` / `0859` phase checkboxes

## Architecture

Docs are projections of the crate. Proof is a **socket** test plus a short report — not a Herdr screenshot, not Xia `--copy`.

Proof loop (isolated `XDG_RUNTIME_DIR`):

```text
dory server
  pane.split --direction right
  desk.layout cols=80 rows=22  → 2 cells, exclusive abut (w sums to 80; no gap field)
  <!-- Updated: Validation Session 1 - exclusive tiles; inset is client-only -->
  two pane.attach (no_focus + focus)
  pane.write / pane.run distinct bytes on each
  both attach streams observe their own output
  client disconnect
  pane list still has both ids
  dory server stop
```

Optional human line in the report: sat at a real tty, saw two tiles. Not a gate if CI has no tty — the socket proof is the gate.

## Related Code Files

- Modify: `README.md` (Mở table + “Xong tới đâu” desk row `:60`)
- Modify: `rust/src/main.rs` `USAGE` (`:47`) + the assert block (`:635`)
- Modify: `skills/dory/SKILL.md` sit-down sentence (`:34`); add one line under Layout that **humans** see a grid; occupants still do not re-attach
- Create: `plans/reports/260823-desk-grid-proof.md` (cook fills date/time)
- Create if missing: extra asserts in `rust/tests/p5_layout.rs` (preferred) — do not invent `USAGE.md`
- Delete: none
- Do **not** touch `plans/260822-0847-workplace-skill-mux/**` phase files

## Implementation Steps

1. Rewrite README key table: click tile, drag divider, `Ctrl-b h/j/k/l`, `z`, `n/p`, detach.
2. USAGE: “sidebar + tiled live panes”; keep `--plain` / `DORY_ENV` / no `--kind`.
3. Skill: human desk = grid; skill gate still `test "${DORY_ENV:-}" = 1`.
4. Extend `p5_layout` (or a `p5_grid` test) with the two-attach proof above.
5. Write the proof report with commands + JSON snippets (redact local paths if any).
6. `cargo test --offline --locked`.
7. Confirm `rg -n "ratatui|herdr|dsh|--kind" rust/src skills/dory` — no new exec / identity.

## Todo

- [x] README + USAGE + skill note
- [x] Isolated two-PTY socket proof
- [x] Suite green; older plan papers untouched

## Success Criteria

- [x] A stranger reading README knows they will see **multiple** live panes
- [x] `dory --help` / USAGE tests still pass
- [x] Skill still first-action `DORY_ENV=1`; no occupant re-attach
- [x] Proof: two cells from `desk.layout`; two attach streams live; detach ≠ kill
- [x] `cargo test --offline --locked` pass
- [x] `0847` / `0011` / `0859` checkboxes unchanged

## Risk Assessment

| Risk | L×I | Mitigation | If broken |
|---|---|---|---|
| Docs drift from keys | M×L | Copy prefix list from `desk.rs` footer after P3 | Fix docs, not the crate, if keys differ |
| Proof needs a tty | L×L | Socket proof is CI; tty note optional | Do not skip socket test |
| Mentions of Herdr in README | L×L | Compare-as-learn only; no “run herdr” | CHARTER kill if shipped path execs herdr |

## Security Considerations

Proof uses temp XDG like `p5_attach`. No customer data. Do not publish sockets or env dumps with secrets.

## Rollback

Revert docs + proof test. Grid code from P1–P3 stays.

## Next Steps

Cook this plan. Do not start occupancy/radar SKU from here.
