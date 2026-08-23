---
type: research
date: 2026-08-23
time: 13:20
mode: product-docs-only
status: done
constraint: no clone herdrdev/herdr; no Xia --copy; no Herdr source as copyable code
---

# Research: official Herdr sit-down (2026-08-23)

**Question:** what a human sees/does after `herdr`, so Dory desk (sidebar + **one** live PTY, no spatial grid) can be compared honestly.  
**Not this report:** Ratatui, their crate, `--copy`, implementation recipes.

**Version:** `https://herdr.dev/latest.json` = **0.8.2** (2026-08-19). Docs still mention 0.7.0 plugin floor / 0.7.3 session-local plugins. GitHub still lists **v0.7.5** (2026-07-21). Some installs may be 0.7.5; face below is 0.8.x docs + 0.8.2 notes. Sit-down shape (sidebar + tiled panes + detach) already existed at 0.7.5.

**Weight:** herdr.dev docs + homepage + GitHub README (web) > `latest.json` notes (UI nouns) > third-party blogs (Better Stack: outdated CLI names; ignore “use Ratatui”). Did **not** run TUI, screenshot, or read their tree.

## 1. Face after `herdr`

`herdr` from a project dir **launches or attaches** the default background session. No socket babysitting. Empty session **auto-opens one workspace**. Close the window or detach → server + agents keep running; `herdr` again = same room.

Client face (docs + homepage + 0.8.2 notes, not pixels):

- **Left sidebar:** Spaces (workspaces) + Agents. Status rolls up. Collapsed rows still show marks. `prefix+b` toggles. Click a row = jump to that live layout.
- **Tab bar** (desktop; optional bottom). Status chips: zoom, host, clock.
- **Center: tiled live terminals** — one tab’s BSP grid. Marketing: “three workspaces, four agents… click the sidebar, it’s a live layout, not an image.”
- **Mouse first.** Keyboard optional.
- **Modes:** terminal (keys → focused pane) / prefix / navigate (persistent workspace nav).
- **Session ≠ workspace.** Default session is the room. Named sessions (`herdr session attach work`) = separate servers. Prefer workspaces first.

## 2. Workspace / tab / pane — how splits LOOK

| Layer | Official meaning | Visible at once? |
|---|---|---|
| Session | Persistent server namespace | One attached client UI |
| Workspace | Project container; sidebar rollup | One focused; others in sidebar |
| Tab | One layout (`agents`, `logs`, …) | One tab’s tiles |
| Pane | Real PTY; survives detach | **Yes: all tiles in that tab** unless zoomed |

Splits: **right or down**, BSP tree (`split` + `ratio` + two children). **Multiple panes on screen together.** Drag **internal dividers** (and optional outer borders). Neighbor/edges/focus-direction assume a geometric grid. `prefix+z` / `pane.zoom` hides siblings (`pane_visible` false). New TUI split **returns focus to the source pane** (0.8.2). Hidden tabs still run; only the active tab’s tiles paint.

IDs (docs): workspace `w1`, tab `w1:t1`, pane `w1:p1`. Closed IDs not reused.

**Dory gap:** one live PTY = attach/switcher. Herdr sit-down = **see the herd spatially**.

## 3. Default keys (prefix `ctrl+b`)

Mouse covers the same actions. Prefix = one reserved chord so pane apps keep `ctrl+c` etc. `prefix+?` = live help.

| Action | Default |
|---|---|
| New tab | `prefix+c` |
| Split right / down | `prefix+v` / `prefix+minus` |
| Focus panes | `prefix+h/j/k/l` |
| Next / prev tab | `prefix+n` / `prefix+p` |
| Tab 1–9 | `prefix+1..9` |
| Workspace nav | `prefix+w` |
| New workspace | `prefix+shift+n` |
| Toggle sidebar | `prefix+b` |
| Zoom | `prefix+z` |
| Detach (leave running) | `prefix+q` |
| Close pane / tab / workspace | `prefix+x` / `shift+x` / `shift+d` |
| Copy mode | `prefix+[` |
| Resize mode | `prefix+r` |
| Goto picker | `prefix+g` |

Prefix-free option: extra `ctrl+alt+…` chords (docs warn OS/terminal steal many). Prefix remappable (`keys.prefix = "ctrl+a"`).

## 4. Mouse-native

Stated as **the** UI; keys are optional.

- Click pane / tab / workspace / agent → focus.
- Drag split borders → resize.
- Right-click → menus (split, new tab). Frame right-click always Herdr menu. Per-pane: send right-click to mouse-reporting apps.
- Drag-select copies (no `ctrl+c`). Double-click token copies. `copy_on_select = false` keeps selection until copy.
- Ctrl-click opens OSC 8 + visible `http(s)` (macOS: Cmd is bypass-only while capture on).
- Wheel / page keys scroll; typing jumps to bottom (direct-attach docs).
- `ui.mouse_capture = false` hands mouse back to the host terminal.

## 5. Occupant five-state (documented)

Agent = recognized process **in** a pane. Pane exists without one. States ([concepts](https://herdr.dev/docs/concepts/)):

| State | Meaning |
|---|---|
| `blocked` | Needs input / approval / decision (strict screen match) |
| `working` | Actively running |
| `done` | Finished **and unseen** |
| `idle` | Finished/waiting **and seen** in focused UI |
| `unknown` | Agent present, classification not confident — **not** “done” |

`done`/`idle` share idle-underneath; **focus tab or `pane`/`agent` focus marks seen**. CLI read does **not**. Sidebar rollup: blocked paints pane/tab/workspace; working paints workspace active; done stays until viewed. “Never hunt for the stuck one.”

## 6. Their product extras — **not a copy target**

- **Marketplace** — GitHub topic `herdr-plugin`; [herdr.dev/plugins](https://herdr.dev/plugins/).
- **`--kind`** — `agent start name --kind claude|codex|…` (~20 CLIs). Human path is still type `claude` in a pane.
- **Remote** — `herdr --remote host` thin client; or SSH then `herdr`; phone SSH; TUI narrows.
- **Plugins / integrations** — host surface + `integration install`.
- **Also theirs:** named sessions, worktrees, direct `agent attach` (one PTY, no desk), experimental pane graphics, Windows, 20-agent detection farm.

Do not clone these to “feel like Herdr.”

## 7. “Sitting down” in their words (one paragraph)

They never write that phrase. The ritual is: from the repo, type `herdr` → you are in a room that already exists (or an auto-made first workspace) → **click** the herd, **see several live terminals at once**, type the agent you already use, walk away (`prefix+q` / close lid) → type `herdr` and the furniture is still there. Homepage: “Give them somewhere to live.” README: “run your agents, split panes, walk away.” Sitting down = **re-enter a persistent spatial desk**, not open a fresh prompt.

## Why Dory desk can feel “too different / not as good”

| | Herdr (official) | Dory desk (stated) |
|---|---|---|
| Spatial | N live tiles + drag borders | **One** PTY |
| Mouse | First-class desk | Sidebar click + prefix (planned) |
| Status | 5-state rollup across projects | Not this product’s face |
| Detach | Server owns herd | Daemon can own PTYs |
| Extras | marketplace, `--kind`, remote | Explicitly out of scope |

A user who sat in Herdr is judging **glanceability**: two agents side by side, click-to-that-layout, drag the wall, copy with the mouse, sidebar that shouts `blocked`. Sidebar + one tube is a **chooser**. Zoom/split keys without a grid feel like tmux muscle memory bolted on a list. Persistence without tiles still feels empty. That gap is **product geometry**, not “wrong widget crate.”

**Ranked (Dory, no clone):** (1) simultaneous live tiles if “don’t feel different” is the bar; (2) sidebar as status radar, not only a tree; (3) mouse desk (click/drag/copy) before more prefix chords; (4) detach already the right metaphor. **Do not** take marketplace, `--kind`, remote, plugins, or their TUI stack.

## Sources

- https://herdr.dev/docs/quick-start/ · concepts · keyboard · socket-api (capability list only) · how-to-work · agents · agent-automation · plugins · persistence-remote · cli-reference  
- https://herdr.dev/ · https://herdr.dev/plugins/ · https://herdr.dev/latest.json  
- https://github.com/herdrdev/herdr README + releases (web)

## Unresolved

- Pixel chrome (exact sidebar split Spaces vs Agents) — no live capture this pass.  
- 0.7.5 vs 0.8.2 face deltas beyond changelog nouns.  
- Whether Dory daemon will ever store BSP geometry (desk-tui says no today).
