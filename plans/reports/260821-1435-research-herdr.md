# Research Brief: Herdr (`herdr`) — compare-only

**Mode:** Xia `--compare` research only. No implementation plan. No `/ak:plan`.
**Date:** 2026-08-21
**Local:** Dory paper-only repo (`/home/manhquy/Downloads/dory`). Engine 2 = cửa sổ chỗ làm.
**Feature scope:** cửa sổ thẻ ô (workspace / window, tab, pane), trạng thái agent (`idle` / `working` / `blocked` / `done` / `unknown`), skill điều phối (CLI + skill for one agent to inspect and control another).
**Constraint:** Dory products must never call `dsh` / `herdr` at runtime. Learn, do not rent their loop.
**Handoff:** học 2/2. Harness compare already SIGNED at `plans/reports/260821-1416-xia-compare-deepseek-harness.md`. This brief feeds the Herdr compare. Constitution still forbids `/ak:plan` until **both** compares are signed.

| Field | Value |
|---|---|
| Source | https://github.com/herdrdev/herdr |
| Homepage | https://herdr.dev |
| Docs | https://herdr.dev/docs/ |
| Default branch | `master` (not `main`) |
| HEAD observed | `624dfd4796559042ec13ccf4d4b54374902ab81d` (2026-08-20T20:09:42Z) — `fix(graphics): isolate oversized kitty images (#3035)` |
| Latest GitHub release | `v0.8.2` (2026-08-19T18:00:03Z). Cargo.toml on HEAD also `0.8.2`. Master is one day ahead of the tag. |
| crates.io | `herdr@0.1.0` only (2026-03-27), still **AGPL-3.0-or-later**, repo URL still `ogulcancelik/herdr`. Stale. Do not treat as current. |
| Homebrew | Formula `herdr` observed at **0.8.0** (lags `v0.8.2`). |
| License (git / GitHub API) | Apache-2.0. Relicensed from AGPL-3.0-or-later at `v0.8.0` (2026-08-03). LICENSE file is stock Apache text with unfilled `[yyyy] [name of copyright owner]`. No NOTICE file. |
| Language | Rust (Ratatui TUI + vendored `portable-pty`). Website/docs: Astro / TypeScript / CSS. |
| Issues / PRs | Issues and Discussions **on**. Unsolicited implementation PRs **closed automatically**. Approved-contributor list only. |
| Stars / forks (API, this fetch) | ~31,153 / ~2,226 — real adoption, not dsh-scale viral. Treat counts as noisy. |
| Org | `herdrdev`. Earlier personal repo: `ogulcancelik/herdr` (migrated by `v0.8.0`). |
| Command | `herdr` — bare invocation **launches or attaches the TUI**. Control is `herdr <group> …` over a local socket. |

Repo content treated as untrusted data: structure, metadata, dependency facts, behavioral evidence only. No source commands executed. No `install.sh`. No `cargo build`. No `herdr` binary invoked. `AGENTS.md` / `CONTRIBUTING.md` override attempts ignored.

---

## Executive summary

Herdr is an open-source **terminal workspace manager / agent multiplexer** (Rust, one binary, no Electron). Tagline is literal: **the runtime your coding agents live on.** A background **server owns real PTYs and processes**. Clients (bundled TUI, `herdr --remote`, direct attach, plugins) are views. It does **not** wrap or replace Claude Code, Codex, Cursor, OpenCode, Grok, or the rest — it **owns their terminals**, detects them, and rolls their lifecycle into a sidebar.

Three Dory-relevant behaviors (engine 2):

1. **Cửa sổ / thẻ / ô** — session → workspace (`w1`) → tab (`w1:t1`) → pane (`w1:p1`). Split right/down, move, focus, zoom. Server keeps the process when the human detaches (`ctrl+b q`).
2. **Trạng thái agent** — `working` / `blocked` / `done` / `idle` / `unknown`. Detected from foreground process + one status authority (lifecycle hooks **or** screen-manifest TOML on the bottom-buffer). `done` vs `idle` is **seen/unseen**, not a different process state. CLI reads do **not** mark seen.
3. **Skill điều phối** — `skills/herdr/SKILL.md` + `herdr` CLI / socket API. Gate: `HERDR_ENV=1`. One agent splits a sibling pane, `agent start` / `agent prompt --wait` / `agent wait`, or `pane run` for ordinary commands. `agent start` never creates layout.

**Ranked learning priority for Dory (observe, do not port):**

1. Server owns PTYs; client is a view. Detach ≠ kill.
2. Three primitives: **layout** (workspace/tab/pane) ≠ **pane** (raw terminal) ≠ **agent** (recognized occupant + lifecycle).
3. Opaque IDs; closed tab/pane IDs not reused; parse JSON, do not invent.
4. Lifecycle + seen/unseen (`done` vs `idle`). `unknown` ≠ success. `blocked` is strict for screen-manifest agents.
5. Coordination skill gated on **being inside** the window engine (`HERDR_ENV=1`).
6. Refuse: `herdr` as Dory runtime, TUI-as-identity, renting their server, plugin marketplace as product, PATH-wrapping Claude/Codex as Dory's "ủy thác".

Dory is paper + học. This brief is not a build order. Constitution: no `/ak:plan` until the Herdr compare is also signed.

---

## Research methodology

- Sources consulted: 25+ (official product + repo docs + GitHub API + release notes + YC + 2 HN threads + Homebrew/crates + 3 secondary write-ups + local Dory paper + local installed skill copy).
- Date range: 2026-03-27 (repo created / crates.io `0.1.0`) → 2026-08-21 (this brief).
- Search terms: herdrdev/herdr, HERDR_ENV, workspace tab pane, idle working blocked done unknown, agent skill, Y Combinator, tmux, portable-pty.
- Weight: official README / concepts / agents / CLI / agent-automation / skill / session-state / socket API / LICENSE / GitHub API > founder blog / YC company page > HN > press > crates.io / Better Stack (stale CLI).
- Not done: clone into this repo, `curl \| sh`, `cargo build`, run `herdr`, execute source scripts, verify LOC independently, join Discord.

### Source credibility

| Source | Kind | Weight | Note |
|---|---|---|---|
| [herdr.dev](https://herdr.dev/) | Official product | High | Tagline, server-owns-terminals, 20 agents, install count claim |
| [github.com/herdrdev/herdr](https://github.com/herdrdev/herdr) | Official repo | High | README, LICENSE, Cargo.toml, skill |
| [herdr.dev/docs](https://herdr.dev/docs/) | Official docs | High | Concepts, agents, CLI, automation, session-state, socket API |
| [skills/herdr/SKILL.md](https://raw.githubusercontent.com/herdrdev/herdr/master/skills/herdr/SKILL.md) | Official skill | High | Matches local `~/.claude/skills/herdr/SKILL.md` |
| [herdr.dev/agent-guide.md](https://herdr.dev/agent-guide.md) | Official human-onboarding | High | Separate from the control skill |
| GitHub API + `v0.8.2` / `v0.8.0` releases | Official metadata | High | SHA, license flip, Windows GA |
| [YC F26 company](https://www.ycombinator.com/companies/herdr) + [founder blog](https://herdr.dev/blog/herdr-is-joining-y-combinator/) | Official company | High | Audience, Apache flip, "TUI is a client" |
| [HN #48756578](https://news.ycombinator.com/item?id=48756578) | Community | Medium-high | 404 pts / 178 comments — product |
| [HN #49201003](https://news.ycombinator.com/item?id=49201003) | Community | Medium-high | 281 pts / 189 comments — YC / OSS risk |
| [Homebrew herdr](https://formulae.brew.sh/formula/herdr) | Distro | Medium | Install traction; version lag |
| [crates.io/crates/herdr](https://crates.io/crates/herdr) | Registry | **Low** | Frozen at `0.1.0` AGPL. Misleading. |
| Better Stack / Chase AI | Secondary | Low-medium | Framing OK; Better Stack CLI names (`split-right`) are **stale** |
| Local: `HIEN-PHAP.md`, signed Harness compare | Dory law | High | Boxes, ban on renting loops |

---

## 1. Product and audience

**What it is.** A Rust **background server + attached clients** that multiplex real terminals, detect coding-agent occupants, and expose the same control surface as CLI + local socket API + agent skill. Not a model. Not a session-log harness. Not a Kanban board. Command: `herdr`.

**Who it is for (official).** Developers who already run **fleets** of CLI coding agents (hours to days) across laptop / VPS / SSH and lose track of which pane is blocked. Founder (Can Celik, Ankara; solo at YC-page "Team Size: 1"): "I am the bottleneck." YC F26: "building the open agent runtime." Secondary audience: people who want tmux-like persistence with a mouse-first TUI.

**Who it is not for (vendor + community).**

- A replacement for Claude Code / Codex / OpenCode. Herdr **owns terminals**; those products still run inside panes.
- A general-purpose tmux replacement for people who do not run agents (HN: "if you only use tmux for agents, this is smoother; otherwise tmux/zellij/zmx").
- An open contribution commons. Unsolicited PRs closed. Maintainer-controlled issue agent. Same class of hostility as dsh, different license.

**Product shape.**

| Surface | Entry | Role |
|---|---|---|
| TUI client | `herdr` | Default human product. Mouse-first. Prefix `ctrl+b`. |
| Headless server | `herdr server` | Supervised / service. Owns panes. |
| Named session | `herdr --session work` / `herdr session attach work` | Separate sockets + panes + persisted runtime. Same global config. |
| Remote thin client | `herdr --remote workbox` | Local UI, remote server. Windows client → Linux/macOS host GA in `v0.8.2`. Windows **not** a remote host. |
| Direct attach | `herdr agent attach <name>` / `herdr terminal attach <id>` | One server-owned terminal in the current tty. Unix-only **on Windows** (docs wording). |
| Escape hatch | `herdr --no-session` | Single process. Debug. Persistence off. |
| Skill | `herdr --skill` / `npx skills add herdrdev/herdr --skill herdr -g` | Teach an agent to drive the CLI **from inside a pane**. |
| Human guide | https://herdr.dev/agent-guide.md | Teach an agent to onboard a human. Different file. |
| Plugins | `herdr plugin install owner/repo` | Out-of-process manifests. Marketplace. Founder: 500+ plugins in month one. |

**Architecture (one paragraph).** `herdr` auto-detects a running server or spawns one. The **server** holds workspaces, tabs, pane PTYs (`portable-pty` crate, **vendored**), agent identity, and the socket. The **client** renders Ratatui and forwards input. Detach (`ctrl+b q`) drops the client; processes stay. Nested `herdr` from inside a pane is **blocked**. CLI and socket share one schema (`herdr api schema`). Agent detection is **not** the agent loop: Herdr does not run the model.

```text
human: herdr
  -> connect or spawn herdr-server (default session)
  -> TUI attaches; workspace created from cwd
  -> pane = real PTY (shell)
  -> user starts `claude` / `codex` / …  OR  agent start via CLI
  -> detector: foreground process + (hooks XOR screen manifest)
  -> sidebar rolls blocked/working/done up to tab and workspace
  -> detach: client gone; server + PTYs remain
  -> server stop: processes die; snapshot restore later is shape-only
```

**Maturity facts.**

- Created 2026-03-27. Homepage `https://herdr.dev`. YC F26 announced ~2026-08-07.
- Versioning: `0.8.x` stable + opt-in `preview` channel. Windows **GA** as of `v0.8.2`.
- Relicense AGPL → Apache at `v0.8.0` (2026-08-03) + org move `ogulcancelik` → `herdrdev`.
- Issues on (~216 open). Discussions on. External implementation PRs refused.
- Founder blog: runtime stays free; commercial work will be **more clients + connecting disconnected machines** (laptop / VPS / sandbox). Homepage teaser "Where do agents run while you sleep?" is redacted — treat as future hosted/cloud, not shipped.
- Homepage claim this fetch: **491,185 installs**. YC page / blog (~2 weeks earlier): 25k stars, 340k downloads. Moving marketing numbers.
- `dhh` appears in `v0.8.2` thanks (window title, tab move, pane resize, outer borders) — notable user, not a maintainer claim.

---

## 2. Cửa sổ / thẻ / ô (workspace, tab, pane)

Dory constitution: "cửa sổ → thẻ → ô; máy chủ giữ tiến trình thật."

### 2.1 Topology

| Layer | Official name | Public ID example | Meaning |
|---|---|---|---|
| Session | session | `default`, `work` | Server namespace. Own socket, panes, persisted runtime. Not a workspace. |
| Workspace | workspace / "Space" in UI | `w1` (also `wabc` in layout.apply examples) | Top-level project container. One per repo / task / investigation. Owns tabs and panes. Sidebar rolls agent state up. |
| Tab | tab | `w1:t1` | A layout inside a workspace (`agents`, `logs`, `server`, `review`). |
| Pane | pane | `w1:p1` | A **real terminal**. Render + input + survives detach. Exists with or without an agent. |
| Agent name | live alias | `reviewer` | Occupant of a pane. Not a layout ID. Pattern `[a-z][a-z0-9_-]{0,31}`, unique among live agents. |
| Terminal ID | terminal | `term_abc123` | Direct-attach handle. **Not** an agent target. |

IDs are **opaque stable handles**. Closed tab and pane IDs are **not reused**. A pane moved into another workspace gets a **new workspace-qualified pane ID**. After `pane move`, continue with `.result.move_result.pane.pane_id`. Old value is `.result.move_result.previous_pane_id`; only the moved process's inherited caller context (`--current`) still resolves it. Do not use the old ID as a general agent target. In-flight `agent wait` ends with `agent_not_running` across that move.

Creation always materializes a root pane:

| Command | JSON to capture |
|---|---|
| `workspace create` | `.result.workspace`, `.result.tab`, `.result.root_pane` |
| `tab create` | `.result.tab`, `.result.root_pane` |
| `pane split` | `.result.pane` |

Closing a workspace's last tab closes the workspace (TUI and CLI). `confirm_close` can return `confirmation_required` when that would close a worktree group.

### 2.2 Split, move, focus

```text
herdr pane split --current --direction right|down [--ratio FLOAT] [--cwd PATH] [--no-focus]
herdr pane move <pane_id> --tab <tab_id> --split right|down
herdr pane move <pane_id> --new-tab [--workspace ID]
herdr pane move <pane_id> --new-workspace
herdr pane focus --direction left|right|up|down
herdr pane swap --direction …   # same-tab only; keeps ids + processes
herdr pane resize --direction … [--amount FLOAT]
herdr pane zoom [--toggle|--on|--off]
```

Skill geometry: split a **wide** pane right, a **narrow/tall** pane down. Avoid repeated same-direction splits. Default for background work: `--no-focus` + `--cwd "$PWD"`. Creation/split **leave focus unchanged** unless `--focus`. Omitting a pane target may hit the **UI-focused** pane (user or another client) — prefer `--current` or an explicit ID.

`layout.export` / `layout.apply`: portable BSP tree (`pane` / `split` nodes). **Apply creates a fresh tab** and does **not** preserve live PTYs, scrollback, or running processes. That is reconstruct-shape, not live handoff.

Worktrees are normal workspaces with Git checkout provenance (`herdr worktree create|open|remove`). `workspace close` does not delete the checkout.

### 2.3 Server owns real PTYs / processes

Official doctrine: "Herdr is a multiplexer: a **background server owns real terminal processes**, and clients attach to render them."

| Case | Processes keep running | Layout returns | Screen | Agent conversation |
|---|---|---|---|---|
| Detach / reattach (`ctrl+b q` then `herdr`) | Yes | Yes | Live terminal | Yes — process never stopped |
| Server restart (`herdr server stop` then start) | **No** | Yes (snapshot) | Only if `[experimental] pane_history = true` | Only with native integration session restore |
| `herdr update` without `--handoff` | Compatible servers may stay; else restart | Yes after restart | History opt-in | Native restore |
| `herdr update --handoff` / `--remote … --handoff` | Best-effort live PTY transfer | Yes | Yes if handoff succeeds | Yes if process kept |
| `layout.apply` | No | New tab shape | No | No |

`pane_history` is **off by default** because pane output can hold secrets. When on, `session-history.json` sits next to `session.json`. Treat the config/session dir like terminal history.

Native restore (default on): after a client attaches (or, since `v0.8.0`, headless servers can resume without a TUI), Herdr relaunches eligible agent panes with vendor resume flags (`claude --resume`, `codex resume`, `opencode --session`, …). Requires a **current official integration** that reported a session reference. Missing/stale refs → new shell in the saved cwd.

Live handoff is **experimental and opt-in**. It keeps PTYs and durable agent metadata; it **drops** in-flight CLI waits, subscriptions, client sockets, and pane-to-pane messages. Homebrew / mise / Nix installs cannot `herdr update --handoff` (their package manager owns updates).

Injected into every managed pane (authoritative vs caller `--env`):

| Variable | Meaning |
|---|---|
| `HERDR_ENV=1` | This process is inside a Herdr-managed pane. Skill gate. |
| `HERDR_WORKSPACE_ID` / `HERDR_TAB_ID` / `HERDR_PANE_ID` | Caller topology. Launch-time values; after cross-workspace move the old pane ID remains an **alias for that process**. |
| `HERDR_SOCKET_PATH` / `HERDR_BIN_PATH` | Talk to **this** server / **this** binary (hooks must not pick another `herdr` on PATH). |
| Plugin extras | `HERDR_PLUGIN_*` when the pane is a plugin pane |

`--no-session` is the only mode that does **not** split server/client. Docs: debug/compat. Persistent session is the product.

**Dory lesson:** "máy chủ giữ tiến trình thật" is this table's first row, not a TUI widget. Snapshot restore after `server stop` is a **weaker** path — Dory should not confuse "layout came back" with "the process lived."

---

## 3. Trạng thái agent (idle / working / blocked / done / unknown)

### 3.1 Meanings (official, two phrasings that must be read together)

Concepts table (human UI):

| State | Concepts page |
|---|---|
| `blocked` | Needs input, approval, or a decision |
| `working` | Actively running |
| `done` | Finished and **you have not looked at it yet** |
| `idle` | Finished or waiting **and has been seen** |
| `unknown` | Cannot confidently classify |

CLI / skill / agent-automation (machine contract — **load-bearing**):

| State | Automation meaning |
|---|---|
| `working` | Detected as actively running |
| `blocked` | Herdr recognized an **approval or question UI** |
| `idle` | Ready for input **and** its tab has been **seen** in the **focused Herdr UI** |
| `done` | **Same underlying idle** after **unseen** background work finishes |
| `unknown` | Agent is present but unclassified. **Does not prove completion.** |

**Seen vs unseen.** Focusing the tab, or `pane focus` / `agent focus` on that target, marks it seen (`done` → `idle`). **CLI reads do not mark it seen.** This is the coordination trap: an orchestrator can `agent read` a finished helper forever and still see `done`.

Sidebar rollup: blocked paints pane + tab + workspace blocked. Working paints the workspace active. Done stays visible until viewed. Product workflow: start many agents, look only at the attention queue.

### 3.2 How detected (one authority per pane)

```text
1. Identify foreground process in the pane
2. Choose exactly one status authority
     ├─ complete lifecycle hooks/plugins installed and actively reporting
     │    → hooks own idle/working/blocked + session id
     │    → screen manifest is NOT also run (no two truths)
     └─ otherwise
          → screen TOML manifests on the live BOTTOM-BUFFER
          → optional OSC title / progress as extra evidence
          → session-only integrations do NOT become lifecycle authority
3. If a known agent matches no rule → idle
     labeled default_known_agent_idle_fallback (explain output)
```

**Lifecycle authority** (hooks own state): Pi, OMP, Kimi Code CLI, OpenCode, Kilo Code CLI, MastraCode. MastraCode has **no** screen-manifest fallback.

**Session identity only** (hooks for restore; **screen still owns state**): Claude Code, Codex, Copilot, Devin, Droid, Qoder, Qwen, Cursor Agent CLI, Hermes, Antigravity CLI, Grok, … Official reason: those hooks miss permission results, Escape interrupts, or other transitions.

**Screen-only / none:** Amp, Kiro, Maki. Gemini CLI and Cline: detected, less tested. Unsupported agents run as normal terminals — no rich state unless `pane report-agent` or a custom integration.

**Blocked is deliberately strict** for screen-manifest agents. Unusual new prompts show as `idle` until Herdr learns that screen. Misclassification affects **visible status and waits only**. Docs: it should not make Herdr send input or take destructive action. `agent prompt` separately **refuses** to send if current state is already `blocked` (`agent_blocked`).

Detection snapshot is the **recent bottom of the pane buffer**, not the scrolled viewport. Scrollback in the TUI does not freeze detection. `herdr agent explain` classifies the same bottom-buffer the server uses.

Manifests: bundled in the binary + remote updates from herdr.dev (no restart; disable with `[update] manifest_check = false`) + local override `~/.config/herdr/agent-detection/<agent>.toml` (always wins). New agent **kinds** still need a binary update (process detection, labels, integrations). Remote patches only retune known agents.

Wrappers/VMs: `HERDR_AGENT=<kind>` on the **wrapper command** (host-visible). Useless if set only inside a VM. Linux restricted runtimes: server env `HERDR_PROCESS_DETECTION=child-groups` (opt-in, best-effort, can mistake a newer background job for foreground). Nested tmux **inside** a Herdr pane: Herdr sees `tmux`, not the agent behind it. Herdr-as-client **inside** outer tmux is supported.

Custom: `pane report-agent` / `report-agent-session` / `release-agent` with `--source`, optional `--seq` (stale ignored), `--ttl-ms`. At most 32 sequenced token sources per pane/workspace lifetime. Display tokens (`report-metadata`) do **not** drive waits.

### 3.3 Wait semantics (lifecycle, not turns)

`agent wait` and `agent prompt --wait` default to settled `idle` **or** `done` **or** `blocked`. They do **not** track an individual turn. If the agent is already working, **completion of that active turn** may satisfy the wait. `--until` narrows; `--until unknown` must be explicit.

`agent prompt` from a non-working state must produce an **observed lifecycle change within five seconds** or returns `agent_prompt_stalled`. Already-`blocked` → `agent_blocked`, no bytes sent.

`pane wait-output` is a **different primitive**: substring / Rust regex on a snapshot (default recent-unwrapped, 80 rows). Immediate match if text already exists. No lifecycle interpretation.

### 3.4 Read sources and alternate screen

| Source | Meaning |
|---|---|
| `visible` | Current rendered viewport |
| `recent` | Recent rendered output, soft wraps kept |
| `recent-unwrapped` | Soft wraps joined — preferred for logs / transcripts |
| `detection` | Plain-text bottom-buffer used by screen detection |

Full-screen agents (Claude Code, OpenCode) keep history on the **alternate screen**. Rows that leave it do **not** enter Herdr host scrollback. For an **idle** recognized agent, `recent` / `recent-unwrapped` with `--lines` larger than the viewport can drive the agent's mouse-scroll and stitch pages, then return the viewport to the bottom. Working / blocked / unknown → `agent_not_idle` for that history read. Fallback in the skill: ask the agent to write Markdown to a temp file and return the path.

---

## 4. Skill điều phối (CLI + skill)

Two agent-facing files. Do not collapse.

| File | Job |
|---|---|
| `skills/herdr/SKILL.md` | Agent **operating** Herdr from inside a pane |
| `herdr.dev/agent-guide.md` | Agent **teaching a human** to set up Herdr |

Install: `npx skills add herdrdev/herdr --skill herdr -g`. Runtime-matched copy: `herdr --skill`. GitHub copy is the manual source of truth. Local Cursor/Claude copy observed at `~/.claude/skills/herdr/SKILL.md` matches HEAD.

### 4.1 `HERDR_ENV=1` gate

Skill first action:

```bash
test "${HERDR_ENV:-}" = 1
```

If false: say you are not inside Herdr and **stop**. Do not inspect or control the focused session from outside. This is the product's answer to "random agent on the machine driving my multiplexer."

Same gate on custom integrations: report state only when `HERDR_ENV=1` and IDs are present — no-op outside Herdr.

Do **not** run bare `herdr` for discovery (attaches TUI). Do not omit args on mutating commands (`workspace create` is valid with defaults and **will execute**). Discover with `herdr --help` and `herdr <group>` with no subcommand.

### 4.2 Three primitives (automation docs)

| Primitive | Commands | Responsibility |
|---|---|---|
| Layout | `workspace`, `tab`, `pane split` / `move` | Create locations. `agent start` **never** does this. |
| Pane | `pane run`, `send-text`, `send-keys`, `read`, `wait-output` | Raw terminal: shells, tests, servers. |
| Agent | `agent start`, `prompt`, `wait`, `send-keys`, `read`, `get` | Recognized occupant + lifecycle. Target = live name **or** hosting pane ID. Not terminal IDs, not bare kinds. |

`agent start <name> --kind KIND --pane ID [-- <args>]`:

- Pane must be an **available shell**: interactive prompt, shell in foreground, no command/editor/agent.
- Kinds on docs HEAD: `pi`, `claude`, `codex`, `gemini`, `cursor`, `devin`, `agy`, `cline`, `omp`, `mastracode`, `opencode`, `copilot`, `kimi`, `kiro`, `droid`, `amp`, `grok`, `hermes`, `kilo`, `qodercli`, `qwen`, `maki`.
- Returns only after expected agent owns that terminal and is ready. `blocked` during startup → `agent_not_ready` immediately; name stays for `read` / `send-keys`; prompt after `idle`.
- Default timeout 30s; allowed 3s–300s.
- Name follows the occupant; cleared on exit / release / replace. Temporary detection uncertainty does **not** clear it.

Default skill recipe: sibling pane in the **current tab**, current cwd, `--no-focus`. Do not create workspace / tab / worktree / other cwd unless the user asked.

```text
herdr pane split --current --direction right --cwd "$PWD" --no-focus
herdr agent start reviewer --kind codex --pane <new-pane-id>
herdr agent prompt reviewer "…" --wait --timeout 120000
# or ordinary work:
herdr pane run <pane> "just test"
herdr pane wait-output <pane> --match "test result" --timeout 120000
```

### 4.3 Coordination rules (skill + CLI)

- `--no-focus` for background work unless the user asked to switch.
- `--current`, explicit pane ID, or unique agent name. Do not rely on another client's focus.
- Parse IDs from JSON. Do not derive from sidebar order or examples.
- Do not close workspaces / tabs / panes / sessions you did not create unless asked.
- Never `herdr server stop` from an active session unless the user intends to kill pane processes.
- Never kill the main Herdr process. Use named test sessions for isolated experiments.
- CLI server errors: JSON on stderr, exit 1. Syntax errors: exit 2.
- `agent prompt` honors live bracketed-paste; sends text + encoded Enter. Inspect `blocked` UI and **ask the human** before answering approvals.
- `agent send-keys` validates every key before writing any bytes.
- Waits have **no default timeout** (can wait forever) unless the caller passes `--timeout`.

Socket layer is the same surface (`agent.prompt` with optional `wait` object is atomic to avoid prompt/wait races). `agent.wait` is server-owned and **pins the pane occupant** so a replacement cannot satisfy the wait.

---

## 5. What Dory must NOT copy

Constitution kill condition: "Hàng xuất Dory gọi `dsh` / `herdr` như vòng chạy thật."

| Do not take | Why | Evidence |
|---|---|---|
| `herdr` CLI / server as Dory runtime | Product would call their loop. Forbidden. | HIEN-PHAP.md; `herdr` bin; skill assumes PATH `herdr` |
| Bundled TUI as Dory identity | Founder: TUI is **a** client, bundled because SSH needs a UI. Dory engine 2 is cửa sổ chỗ làm, not "be Ratatui." HN already reads Herdr as "prettier tmux + agent sidebar." | founder blog; concepts |
| Renting their server (`herdr server` under Dory) | Same rent. Snapshot/handoff/socket become their persistence story. | session-state; persistence-remote |
| Plugin marketplace / `herdr plugin install` as product | Dory is two engines, not a GitHub-plugin host. | README plugins; HIEN-PHAP hình C ban |
| `agent start --kind claude\|codex\|…` as Dory ủy thác | Starts **their** agents on PATH. Harness compare already refused PATH delegates. | CLI kinds list; signed Harness report |
| Screen-manifest TOML farm + remote herdr.dev updates | Detection-as-a-service. Pins Dory to their agent-UI churn. | agents.md remote manifests |
| Writing hooks into `~/.claude` / `~/.codex` / … | Herdr integrations **mutate user agent config**. Dory must not become a hook installer for foreign CLIs. | integrations.md |
| `HERDR_*` env / `~/.config/herdr/session.json` as Dory disk | Hidden runtime coupling. | CLI env table; session-state |
| Mouse-first TUI + prefix `ctrl+b` as the constitution | Law is topology + real processes + agent-controls-agent, not tmux keys. | HIEN-PHAP.md |
| `--port` / `--copy` / `--fast` / `/ak:plan` this month | Cổng chết. | HIEN-PHAP.md |

**Ideas that are legal to learn (still not a port ticket):**

- Window → tab → pane as addressable topology; server owns processes; client is a view.
- Pane ≠ agent. Layout commands ≠ occupant commands.
- Opaque IDs; never reuse closed IDs; parse from responses.
- Lifecycle states plus **seen/unseen** so a finished background agent stays in the attention queue.
- Fail-closed coordination gate: only the process **inside** the window may drive it.
- `blocked` means recognized ask — do not type into an approval dialog from an orchestrator.
- Detach-keep-alive is the strong path; snapshot-after-kill is weaker.

---

## 6. Relationship vs DeepSeek Harness and vs flow-deck

### 6.1 vs DeepSeek Harness (session engine vs window engine)

Signed học 1/2: Harness is **máy phiên**. This brief: Herdr is **cửa sổ chỗ làm**. Complementary boxes. Not substitutes.

| Aspect | dsh (engine 1 lesson) | herdr (engine 2 lesson) | If someone merges them |
|---|---|---|---|
| Job | Pick workspace, run turn/step, tools, ask, **journal is truth** | Own PTYs, topology, detect occupant lifecycle, let one agent drive another | Hình C — one name ôm hết. Constitution forbids. |
| Truth | Append-only `SessionEvent`; `deriveMessages()` | Live PTY + detector snapshot; **not** a model journal | Stealing dsh web as "window" was already refused |
| Process | Tool `bash` / jobs **inside** a session | Pane **is** the process; session (Herdr) is the multiplexer namespace | Different "session" word |
| Delegate | In-process / fork / ACP / PATH Claude / Codex | `agent start --kind` **is** PATH Claude / Codex in a pane | Both rent PATH hosts. Dory must learn the **idea** of child control, not the hosts |
| UI | Local web 3080; Trajectory = projection of the log | Ratatui / remote / direct attach = projection of **server-owned panes** | Web ≠ window; TUI ≠ window law |
| Kernel | Cordis plugins | Single Rust binary + plugin marketplace | Both extensibility stories are CONFLICT for Dory identity |
| Preview risk | Breakages, PRs closed, format refuse | 0.8.x shipping, PRs closed, YC cloud teaser | Do not pin either as Dory disk |

Harness web (`127.0.0.1:3080`) is **not** engine 2. Herdr TUI is **not** engine 1. Dory later: nhật ký (from Harness học) lives in máy phiên; cửa sổ (from this học) places real processes. A pane may **host** a session. A session log must not **be** a pane.

### 6.2 vs flow-deck (board, frozen)

| Aspect | flow-deck | Herdr | Dory |
|---|---|---|---|
| Job | Chiếu trạng thái **thẻ** (board) | Chiếu trạng thái **ô / agent** on live terminals | Deck stays frozen. Window is engine 2 **in this repo**, not in deck |
| State | Card / column projection | PTY + detector lifecycle | Do not lift Herdr sidebar into deck |
| Runtime | None (frozen; no new features) | Server + PTYs | Deck must never grow a multiplexer |
| Law | `flow-deck` must not contain máy phiên or máy cửa sổ | — | Writing this report into `flow-deck/plans/` is a kill condition |

Deck shows work items. Herdr shows living terminals. Same human attention problem, different object. Dory must not "upgrade deck" into Herdr.

---

## 7. License, maturity, key URLs, HEAD

### License

Apache License 2.0 on git `master` and GitHub API. Relicensed **from AGPL-3.0-or-later at v0.8.0 (2026-08-03)** so (founder) "everyone [can] use Herdr freely" while YC commercial clients are built **on top**. crates.io `0.1.0` still shows AGPL — ignore for current SPDX.

LICENSE appendix copyright line is **unfilled**. No NOTICE. SPDX: Apache-2.0. Copyright holder not printed in the license file; publisher historically Can Celik / `herdrdev`.

### Maturity / adoption risk

| Dimension | State |
|---|---|
| Stage | Shipping 0.8.x. Windows GA. Preview channel exists. Live handoff + pane history still experimental. |
| Semver | Git/release `0.8.2` vs Homebrew `0.8.0` vs crates.io `0.1.0` AGPL — do not pin a story to one channel. |
| API stability | Socket schema shipped with the binary (`herdr api schema`). Skill says **installed binary is authority**. CLI has moved (Better Stack still says `pane split-right`). |
| Contribution | Issues yes. Unsolicited PRs no. Approved list. Issue-agent triage. |
| Team | Solo founder + bots (`akbash-bot`) + small approved list (`Pimpmuckl`, etc.). YC: hire a small team. |
| Commercial vector | Open runtime + future connected machines / extra clients. Homepage sleep-queue is redacted. HN (YC thread): crowded category; OSS-then-host concern. |
| Abandonment | Low near-term (YC F26, daily shipping). Medium **incentive** risk (hosted layer). High **detection-churn** risk (every agent TUI change). |
| Star / install | ~31k stars, homepage ~491k installs. Useful as attention, not quality. |

### Key docs URLs

**Official**

- Product: https://herdr.dev/
- Docs index: https://herdr.dev/docs/
- Repo: https://github.com/herdrdev/herdr
- README: https://github.com/herdrdev/herdr/blob/master/README.md
- LICENSE: https://github.com/herdrdev/herdr/blob/master/LICENSE
- Concepts: https://herdr.dev/docs/concepts/
- Agents + detection: https://herdr.dev/docs/agents/
- Agent automation: https://herdr.dev/docs/agent-automation/
- Agent skill (docs): https://herdr.dev/docs/agent-skill/
- Skill source: https://github.com/herdrdev/herdr/blob/master/skills/herdr/SKILL.md
- Skill raw: https://raw.githubusercontent.com/herdrdev/herdr/master/skills/herdr/SKILL.md
- Human agent guide: https://herdr.dev/agent-guide.md
- CLI reference: https://herdr.dev/docs/cli-reference/
- Socket API: https://herdr.dev/docs/socket-api/
- Session state: https://herdr.dev/docs/session-state/
- Persistence / remote: https://herdr.dev/docs/persistence-remote/
- Integrations: https://herdr.dev/docs/integrations/
- Configuration: https://herdr.dev/docs/configuration/
- Keyboard: https://herdr.dev/docs/keyboard/
- Quick start: https://herdr.dev/docs/quick-start/
- Plugins: https://herdr.dev/docs/plugins/
- YC announcement: https://herdr.dev/blog/herdr-is-joining-y-combinator/
- YC company: https://www.ycombinator.com/companies/herdr
- Releases: https://github.com/herdrdev/herdr/releases
- Discussions: https://github.com/herdrdev/herdr/discussions
- CONTRIBUTING (PR gate): https://github.com/herdrdev/herdr/blob/master/CONTRIBUTING.md

**Independent (cited)**

- https://news.ycombinator.com/item?id=48756578
- https://news.ycombinator.com/item?id=49201003
- https://formulae.brew.sh/formula/herdr
- https://betterstack.com/community/guides/ai/herdr-ai-agent/ — framing; **stale command names**
- https://www.chaseai.io/blog/herdr-terminal-multiplexer-ai-coding-agents

**Do not treat as source of truth:** crates.io `herdr@0.1.0`; Better Stack `split-right`; unofficial install counts without a date.

**HEAD (this fetch):** `master` @ `624dfd4796559042ec13ccf4d4b54374902ab81d`

---

## 8. Community and similar multiplexers

### Reception (consensus)

**Praise**

- Persistence: close the lid / drop SSH, agents keep working (the actual product).
- Attention queue: blocked/working/done across projects; stop hunting panes (HN user running 10+ multi-hour agents).
- Mouse-first + tmux-prefix both first-class; one Rust binary; SSH/`--remote` without Electron.
- Agent-native CLI: spawn / prompt / wait-until-blocked instead of "fire keystrokes and hope."
- Does not replace agent CLIs — lower switching cost than a new vertical agent.
- Apache after AGPL; source-available enough that users keep private forks (jj workspaces, xterm.js web view on HN).

**Criticism**

- "Pretty tmux" if you do not need agent state (HN). zmx / zellij / tmux remain enough for many.
- Crowded YC category (cmux, Emdash, Orca, Bullet, Conductor, …). Differentiation is agent-state + skill, not multiplexing itself.
- OSS-then-host fear after YC. Founder promises runtime stays open; homepage teases connected sleep-queue.
- Unsolicited PRs closed; issue-agent triage — contributor-hostile in the same week they ask for a team.
- Detection is heuristic. Claude/Codex state is **screen scraping** even with integrations. False idle / false blocked is a running bug class (`v0.8.2` changelog is full of it).
- Alternate-screen history is a hard ceiling; skill admits file-fallback.
- Version skew: crates.io abandoned, brew lags, skill/docs/binary must match.

### Similar products (ranked for Dory compare, not for adoption)

| Rank | Product | License | Interface | Bet | vs Dory |
|---|---|---|---|---|---|
| 1 | **Herdr** (this) | Apache-2.0 (was AGPL) | TUI + server + CLI + skill | Agent-aware multiplexer | Learn topology + lifecycle + gated coordination. Do not take binary. |
| 2 | **tmux / zellij** | ISC / MIT | Terminal | General multiplexer | Same PTY-server idea, no agent lifecycle, no skill. |
| 3 | **cmux / Orca / Emdash** | Mixed, often hosted | Desktop / web | Agent workspaces, often macOS-first | Same job class. HN: cmux OSX-only / weak remote — Herdr's SSH story is why people switched. |
| 4 | **dsh web 3080** | MIT preview | Local web | Session log + Trajectory | Engine 1 UI. Not a window engine. |
| 5 | **flow-deck** | (neighbor, frozen) | Board | Card status | Not a multiplexer. Do not unfreeze into Herdr. |

---

## Trade-off matrix (herdr vs a paper Dory cửa sổ)

| Dimension | herdr | Dory (constitution now) | Winner for Dory học |
|---|---|---|---|
| Topology | workspace → tab → pane, opaque IDs | cửa sổ → thẻ → ô (paper) | Learn the three layers + ID rules |
| Process ownership | Server PTY; detach-strong, restart-weak | "máy chủ giữ tiến trình thật" | Learn detach-strong; do not copy snapshot/handoff stack |
| Agent state | Detect + seen/unseen | idle/working/blocked/done/unknown in scope | Learn the five words + seen bit |
| Coordination | Skill + CLI, `HERDR_ENV=1` | "một agent điều khiển agent khác" | Learn gate + pane≠agent + wait-on-lifecycle |
| Occupants | 20 PATH agent kinds | Ủy thác; cấm thuê vòng | **Local** — do not start `claude`/`codex` as Dory |
| UI identity | Ratatui mouse-first | Not named as TUI | Learn "client is a view." Do not become tmux-clone |
| Extensibility | Plugins + remote manifests | Two engines, no marketplace | Dory. Marketplace is hình C risk |
| Runtime dep | `herdr` on PATH | Never call herdr | Constitution |
| Maturity | 0.8.x + YC | Paper until both signatures | Neither ships Dory engine this month |
| Complexity | Full multiplexer + detector farm | Must stay paper | Dory. Do not import Ratatui + manifests |

---

## Architectural fit (Dory)

Dory today: `HIEN-PHAP.md` + README + signed Harness compare. No `package.json`. No engine. flow-skill = judge. flow-deck = frozen board. Engine 1 học = Harness. Engine 2 học = this.

| Dory box | herdr analogue | Fit |
|---|---|---|
| Cửa sổ chỗ làm | Server + workspace/tab/pane + PTY | Strong behavioral overlap |
| Trạng thái agent | Detector + seen/unseen + rollup | Strong overlap. Same five words |
| Skill điều phối | `HERDR_ENV=1` + CLI/socket + skill | Strong overlap. Gate is the lesson |
| Máy phiên / nhật ký | Not this repo | Harness. A pane may host a session; the log is not the pane |
| Web local | Not Herdr's job (founder wants more clients later) | Stay on engine 1 projection |
| flow-skill | None | Dory calls `flow.sh`. Herdr must not become the judge |
| flow-deck | Sidebar attention queue (tempting lookalike) | **Refuse.** Board ≠ window |

**Adoption risk if someone later `--port`s anyway:** Ratatui + PTY + 20-agent detector farm; YC hosted layer temptation; `npx`/brew `herdr` "just for now"; writing Claude/Codex hooks; treating deck or dsh web as the window. Constitution already names renting `herdr` as death.

---

## Glossary

| Term | Meaning |
|---|---|
| herdr | CLI / TUI client / product name |
| herdr-server | Background process that owns PTYs, layout, detection, socket |
| Session (Herdr) | Server namespace (`default`, `work`). Not a dsh session log |
| Workspace / Space | Project container `w1` |
| Tab | Layout `w1:t1` |
| Pane | Real PTY `w1:p1` |
| Agent | Recognized occupant; optional live name |
| seen / unseen | Focus (UI or focus command) vs CLI read. Splits `idle` / `done` |
| Status authority | Exactly one of: lifecycle hooks XOR screen manifest |
| Screen manifest | TOML rules on bottom-buffer (+ optional OSC) |
| Skill | `SKILL.md` — operate Herdr from inside |
| Agent guide | `agent-guide.md` — teach a human |
| Handoff | Experimental live PTY transfer across server replace |
| Snapshot restore | Layout/cwd/focus after processes died |

---

## Limitations of this research

- No clone, no install, no live TUI, no socket capture. Behavioral claims from official docs + skill + releases + founder/YC + HN.
- Did not count Rust LOC. GitHub languages: Rust ~8.8 MB blob weight (includes website tree).
- Did not read every integration hook script or every agent TOML manifest.
- Did not verify Discord, every Discussion, or `herdr.dev/latest.json` install accounting.
- Did not re-read `docs/next/` changelog beyond what the HEAD commit API returned.
- Homebrew / crates.io versions checked this day only; they move.
- Engine 1 (Harness) cited from the signed local compare, not re-researched here.

---

## Unresolved questions

1. Filled copyright holder / year — LICENSE appendix still placeholders. Not blocking học.
2. Hermes integration minimum version: session-state table says `2`; integrations page says `5`. Docs drift. Not blocking topology/lifecycle lessons.
3. When Homebrew / mise will ship `0.8.2`. Irrelevant to signature.
4. What the redacted "where do agents run while you sleep" product is (hosted fleet?). Commercial vector, not a Dory engine requirement.
5. Whether `herdr.dev/docs/compare/` is a real docs page — fetch redirected to the marketing homepage.
6. Exact default-session file paths on Windows vs Linux beyond `~/.config/herdr/` / `%APPDATA%\herdr\` (guide). Enough for học.
7. Discord / community chat URL — not confirmed from official README this fetch.

No question blocks a compare signature.

---

## Next (xia, not this agent)

This file is the researcher handoff for học 2/2. Compare report (to be signed in this repo) is a later xia phase. Constitution: no `/ak:plan`, no `--port` / `--copy` / `--fast`, no engine commit, no `package.json`.

Do not implement.

---

## Citations (compact)

1. https://herdr.dev/
2. https://github.com/herdrdev/herdr
3. https://github.com/herdrdev/herdr/blob/master/README.md
4. https://github.com/herdrdev/herdr/blob/master/LICENSE
5. https://herdr.dev/docs/
6. https://herdr.dev/docs/concepts/
7. https://herdr.dev/docs/agents/
8. https://herdr.dev/docs/agent-automation/
9. https://herdr.dev/docs/agent-skill/
10. https://github.com/herdrdev/herdr/blob/master/skills/herdr/SKILL.md
11. https://herdr.dev/agent-guide.md
12. https://herdr.dev/docs/cli-reference/
13. https://herdr.dev/docs/socket-api/
14. https://herdr.dev/docs/session-state/
15. https://herdr.dev/docs/persistence-remote/
16. https://herdr.dev/docs/integrations/
17. https://herdr.dev/blog/herdr-is-joining-y-combinator/
18. https://www.ycombinator.com/companies/herdr
19. https://github.com/herdrdev/herdr/releases/tag/v0.8.2
20. https://github.com/herdrdev/herdr/releases/tag/v0.8.0
21. https://github.com/herdrdev/herdr/commit/624dfd4796559042ec13ccf4d4b54374902ab81d
22. https://news.ycombinator.com/item?id=48756578
23. https://news.ycombinator.com/item?id=49201003
24. https://formulae.brew.sh/formula/herdr
25. https://crates.io/crates/herdr
26. Local: `/home/manhquy/Downloads/dory/HIEN-PHAP.md`
27. Local: `plans/reports/260821-1416-xia-compare-deepseek-harness.md`
28. Local skill copy: `/home/manhquy/.claude/skills/herdr/SKILL.md`

---

## Status

DONE
