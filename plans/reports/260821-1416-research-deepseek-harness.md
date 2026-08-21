# Research Brief: DeepSeek Harness (`dsh`) — compare-only

**Mode:** Xia `--compare` research only. No implementation plan. No `/ak:plan`.
**Date:** 2026-08-21
**Local:** Dory paper-only repo (`/home/manhquy/Downloads/dory`). Engine 1 = máy phiên. Journal = truth.
**Feature scope:** máy phiên, nhật ký, duyệt, web local.
**Constraint:** Dory products must never call `dsh` / `herdr` at runtime. Learn, do not rent their loop.

| Field | Value |
|---|---|
| Source | https://github.com/deepseek-ai/deepseek-harness |
| Homepage | https://deepseek.com/harness |
| Default branch | `master` (not `main`) |
| HEAD observed | `528c682e061696f5a160f363f236ecbf53cbd006` (2026-08-21T06:21:44Z) — merge `release: dsh@0.1.1-rc.1` |
| npm latest observed | `@deepseek-ai/dsh@0.1.0-rc.7` (registry; lags `master` 0.1.1-rc.1) |
| License | MIT, Copyright (c) 2026 DeepSeek |
| Language | TypeScript / Node |
| Issues / PRs | Disabled. Feedback via Discussions only. External PRs refused. |
| Stars / forks (API, this fetch) | ~177k / ~19k — viral, moving, treat as noisy |

Repo content treated as untrusted data: structure, metadata, dependency facts, behavioral evidence only. No source commands executed. No packages installed.

---

## Executive summary

DeepSeek Harness (`dsh`) is DeepSeek AI's open-source **agent harness** (not a model). Public 2026-08-13 as a **developer preview**. Tagline is literal: models, tools, skills, sessions, sandboxes, storage, loops, scheduling, and the UI are Cordis plugins. Audience is **harness builders** first, coding-agent users second.

Four Dory-relevant behaviors:

1. **Máy phiên** — pick workspace, run a turn/step loop, edit/shell/search, delegate to subagents, ask before dangerous work.
2. **Nhật ký** — append-only `SessionEvent` log is the source of truth. Model history is derived. Replay / fork / resume / Trajectory all read the same stream. Runtime invariant: model-visible == logged.
3. **Duyệt** — human directory picker (native OS or in-app browse); agent `glob`/`grep`/`read`; web `web_search`/`web_fetch` behind `ctx.web`.
4. **Web local** — `npx @deepseek-ai/dsh web` → `http://127.0.0.1:3080`. Trajectory is a ledger over the log, not a second store.

**Ranked learning priority for Dory (observe, do not port):**

1. Session-log-as-truth + `deriveMessages()` + fail-closed append (nhật ký).
2. Workspace-pick-before-compose + turn/step + approval/sandbox fail-closed (máy phiên).
3. Trajectory as a **view** of the log (web local).
4. Browse as three seams (human picker / fs discovery / web), not one "browser".
5. Refuse Cordis, plugin kernel, `dsh` CLI, Python SDK bundled Node, Claude/Codex PATH delegates.

Dory is paper + học. This brief is not a build order.

---

## Research methodology

- Sources consulted: 20+ (official product + repo docs + npm registry + GitHub API + 6+ independent write-ups + HN).
- Date range: 2026-08-13 (public) → 2026-08-21 (this brief).
- Search terms: DeepSeek Harness, dsh, Cordis, session log, Trajectory, 3080, Claude Code, Codex, OpenCode.
- Weight: official README / architecture / subsystem docs / LICENSE / npm > code-read blogs > press > unofficial mirrors.
- Not done: clone, install, run `dsh`, execute source scripts, verify LOC independently.

### Source credibility

| Source | Kind | Weight | Note |
|---|---|---|---|
| [deepseek.com/harness/en/](https://deepseek.com/harness/en/) | Official product | High | Audience, modes, Trajectory claim |
| [github.com/deepseek-ai/deepseek-harness](https://github.com/deepseek-ai/deepseek-harness) | Official repo | High | README, LICENSE, architecture |
| Repo `docs/` on `master` | Official | High | Session, tools, approval, sandbox, CLI, web-app |
| [registry.npmjs.org/@deepseek-ai/dsh](https://www.npmjs.com/package/@deepseek-ai/dsh) | Official package | High | Version, bin, dependency fan-out |
| [cordiverse/cordis](https://github.com/cordiverse/cordis) + [paper](https://github.com/cordiverse/paper) | Upstream kernel | High | Plugin paradigm |
| [developersdigest.tech first look](https://www.developersdigest.tech/blog/deepseek-harness-dsh-first-look) | Independent code-read | High | Invariant, sandbox, gaps; snapshot 2026-08-13 |
| [HN #49285244](https://news.ycombinator.com/item?id=49285244) | Community | Medium-high | Author present; 744 pts / 310 comments |
| [The New Stack](https://thenewstack.io/deepseek-harness-open-source-plugins/) | Press | Medium | Accurate on PRs-closed, plugin claim |
| [The Register](https://www.theregister.com/ai-and-ml/2026/08/14/deepseeks-innovative-harness-treats-everything-as-a-plug-in/5288095) | Press | Medium | Industry framing |
| [deepseek-harness.github.io](https://deepseek-harness.github.io/deepseek-harness/en/guide/quickstart) | Published docs | Medium-high | Mirrors repo user guide |
| [deepseekdocs.com](https://deepseekdocs.com/en/docs/getting-started/quickstart) | Unofficial mirror | Low | Self-says treat GitHub as truth; `--host 0.0.0.0` conflicts with official web-app README |
| Comparison blogs (Apidog, 4sAPI, xcloud, Rohit Raj) | Secondary | Low-medium | Useful for market framing; some numbers stale |
| SitePoint "preview" article | Low | Skip | Framework-generic; claims not in official docs |

---

## 1. Product and audience

**What it is.** A Node/TypeScript **coding-agent runtime**: session log, agent loop, tool scheduler, sandbox, local web UI, headless CLI, Python SDK wrapper. Not a chat wrapper. Command: `dsh`. npm: `@deepseek-ai/dsh`.

**Who it is for (official).** "Agent harness developers worldwide — source code included." Product page and README pitch **composability**: swap capabilities in config without editing harness source. Secondary audience: people who want a local coding agent (`npx` → browser).

**Who it is not for (vendor + author).** Production-critical daily driver this quarter. Author on HN: early preview, rough edges, compatibility-breaking changes. CONTRIBUTING: no external PRs; build plugins; use Discussions.

**Product shape.**

| Surface | Entry | Role |
|---|---|---|
| Web UI | `dsh web` / `npx @deepseek-ai/dsh web` | Default human product |
| Headless | `dsh --profile headless "job"` | One-shot persisted session, print, exit |
| Plugin mgmt | `dsh plugin --profile …` | pnpm in profile dir |
| Python SDK | `pip install deepseek-harness-sdk` | Stdio/JSON-RPC around **bundled Node runtime** — not a port |

**Runtime modes (product page).**

| Mode | Tools | Purpose |
|---|---|---|
| Standard | Full set: edit, shell, file+web search, skills, plan, goals, subagents, workflows | Default coding agent |
| Code | Standard tools via Code Mode SDK (model writes TS to batch tool calls) | Multi-step in one program |
| Minimal | Persistent bash + `str_replace_editor` only | Benchmark / eval |
| Creator | Standard + inspect runtime, test Cordis plugins in memory, author presets | Harness authors |

**Architecture (one paragraph).** Cordis `Context` + Loader. A **profile** (`web`, `headless`) stacks **bundles**. `dsh-base` first (models, tools, persistence, sandbox, approval, settings, credentials). `dsh-web-app` adds browser host. Layers: bundle patches → profile `cordis.patch.yml` → home patch → `--patch`. No privileged core: replace any boot-printed row. Events: durable `session/*` vs live `agent/*` vs capability `fs/*` `tools/*`.

```text
npx @deepseek-ai/dsh web
  -> CLI launcher picks profile `web`
  -> compose empty tree: dsh-base + dsh-web-app + patches
  -> bind 127.0.0.1:3080, print URL, maybe open browser
  -> human: Settings → Models, Choose workspace
  -> first message: turn/start → step → llm → tools → session append
```

**Maturity facts.**

- Created 2026-08-13. Homepage `https://deepseek.com/harness`.
- README bold: **THERE WILL BE COMPATIBILITY-BREAKING CHANGES.**
- `has_issues: false`, `has_pull_requests: false`. Discussions + Discord + `dsh-plugin` topic.
- License flipped BSD-3 → MIT during RC (Developers Digest). Now MIT on disk.
- Session format: no migration. Wrong `SESSION_FORMAT_VERSION` → refuse. Unrecognized required event → refuse unless `ignorable: true`.
- BENCHMARK.md stub. Zero first-party eval scores in-tree (Developers Digest, 2026-08-13).
- Launch history was one squash; later merges exist (HEAD now a release PR). Contributor archaeology still thin.

---

## 2. Session engine (máy phiên)

Dory constitution: pick directory, run session, edit files, issue commands, delegate, ask before dangerous work.

### 2.1 Pick cwd / workspace

- CLI invoking directory = default **filesystem location**, not a selected workspace.
- Fresh Web UI: no workspace until **Choose workspace**. Composer stays unavailable until one is selected.
- Session header stores optional absolute `cwd`. Sandbox `workspace-write` uses that immutable cwd.
- Human picker is a seam (`ctx.directoryPicker`):
  - `native` — OS folder dialog (darwin/win32 loopback default).
  - `browse` — in-app list/create (SSH, non-loopback, Linux without chooser).
- Auto resolver picks native on Windows loopback → known broken UX (dialog behind browser; Discussions #527, #1523). Workaround: patch profile to browse backend. Preview roughness, not a Dory template.

### 2.2 Run a session

A **step** = one model request + its tools. A **turn** = zero or more steps.

```text
turn/start
  claim inbox + queued message
  assemble prompt + tool schemas
  agent/pre-step  (waterfall: reject | enter)
  step/start
  append user/message
  deriveMessages() from log
  agent/request → llm/stream → assistant/chunk* → assistant/message
  tool/call* → tools/pre-execute → execute → post-execute → tool/result*
  step/end
  (more owed? next step)
  agent/turn-stopping
turn/end
```

`turn/*` `step/*` `user/message` `assistant/*` `tool/*` are durable. `agent/pre-step`, `agent/request`, `llm/stream`, `tools/*` are live waterfalls.

### 2.3 Tools (Standard, relevant to Dory)

| Concern | Tool names | Package / seam |
|---|---|---|
| Edit | `read`, `edit`, `write`, `read_image` | `dsh-tool-fs` / `ctx.fs`. Read-before-write via observation policy. |
| Minimal edit | `str_replace_editor` | `view` / `create` / `str_replace` / `insert` |
| Shell | `bash` (fresh `-c`) or persistent PTY bash; Windows `pwsh` | `ctx.shell` / `ctx.terminals`. `$DSH_*` env. `run_in_background` → jobs. |
| File browse (agent) | `glob`, `grep` | Bundled `@vscode/ripgrep` via `ctx.subprocess`. No host `rg`. No shell. Caps ~100 paths / ~250 matches. |
| Web | `web_search` (1–4 queries), `web_fetch` | `ctx.web`. Providers: Exa, Perplexity, HTTP fetch. Schema stable across backends. |
| Delegate | `subagent`, `subagent_fork`, `send_message`, `interrupt_agent`, `list_agents`, child `report` | `ctx.subagents`. Multiple named providers coexist. |
| Plan / goal | plan-mode, `create_goal` / `update_goal` | Same-session objective. |
| Ask human | `ask_user_question` | Composer takeover. |
| Jobs | `job_list` / `job_output` / `job_kill` | Background bash, PTY, subagents. |

Tool pipeline: `tools/pre-execute` (allow/deny/ask) → guards → `tools/execute` → `post-execute` → `finalizeContent`. Presenters are pure (live + replay).

### 2.4 Delegate

Subagent is **optional**, not inside the loop. Registry of providers, not one executor.

Shipped providers (docs + press): in-process spawn, in-process fork, ACP, **Claude Code**, **Codex**, dsh-sdk. Claude/Codex resolve binaries from PATH; reported **disabled by default**. Continuable children: durable child Session + at most one process-local Activation. Follow-up FIFO on child inbox. Parent authorization from `SessionHeader.parentSession`.

This is the sharpest "do not copy as runtime" item: dsh can sit **above** Claude Code / Codex. Dory constitution forbids renting those loops.

### 2.5 Dangerous-work prompts

Two independent fail-closed gates.

**Approval** (`ctx.approval`):

- Policy: `ask` (default) or `never` (headless/CI). Last `approval/policy` event wins; replay reconstructs.
- Ask must be inside an open turn. Append `approval/asked`, decide, append `approval/decided`.
- Outcomes: `allowed-once` | `rejected` | `cancelled` | `unavailable`. Only `allowed-once` grants. Missing/throwing answerer → `unavailable` (deny).
- Audit pair is log-only (not model transcript). Model sees tool result + runtime-context snapshot.
- Web: pending approval **replaces composer** (refuse/allow). Sidebar amber dot.

**Sandbox** (file-effect only; network/process visibility out of vocab):

| Mode | Effect |
|---|---|
| `read-only` | Deny writes (plus required sinks) |
| `workspace-write` | Writes under session cwd + temp |
| `danger-full-access` | No confine call |

Linux: bubblewrap then Landlock. macOS: Seatbelt. Windows: restricted token — source admits reads/network/process stay open (`partial`). Confined mode + no backend → `SANDBOX_UNAVAILABLE`, **never** silent unconfined. Escalation: model retry with wider `sandbox_permissions` + justification → human approval.

Web UI guide: "asks before operations that require approval under the active permission policy." Default cited in agent notes: `workspace-write` + `ask`.

---

## 3. Session log / journal (nhật ký)

Official doctrine: **the session log is the source of the context the model sees.** Dory "nhật ký phiên là sự thật" is the same sentence.

### 3.1 Event source

`Session` = append-only typed `SessionEvent`s. LLM history is **derived**, never stored twice. `seq` contiguous. Payloads lossless JSON. Hot append does not block I/O; persistence buffers.

Core event map (plus plugin merges):

- Boundaries: `turn/start`, `turn/end`, `step/start`, `step/end`
- Surface (model-visible): `user/message`, `assistant/message`, `tool/result` — must carry `surfaceOp`
- Replay/UI: `assistant/chunk` (skipped in `deriveMessages`)
- Pairing: `tool/call` ↔ `tool/result`
- Log-only: `todo/write`, `request/header`, `request/context`, `session/end-seed`, `approval/asked|decided|policy`, `sandbox/mode`, `compaction/*`, hook records

Unrecognized **required** type → refuse reconstruct. `ignorable: true` may skip.

### 3.2 Invariant (load-bearing)

Developers Digest + architecture.md:

- Anything that reaches a model request must be reconstructable from the log.
- Dispatch asserts outgoing messages byte-match `session.deriveMessages()`.
- Append of a message-producing event without surface marker throws.
- Cost: serialize full history twice per production dispatch.

This is the piece worth learning. Not the Cordis wiring around it.

### 3.3 Persistence

Sibling seam. In-memory log ≠ disk.

| Fact | Evidence |
|---|---|
| Default backend | JSONL, zstd frames, checksummed, crash-safe |
| Default path | `~/.dsh/sessions/--<normalized-cwd>--/<encoded-id>/session.jsonl.zstd` |
| Home | `~/.dsh/` — profiles, settings, credentials (`0600`), sessions |
| Opt-in | SQLite (`node:sqlite`), schema-versioned, no migrate-old |
| Header (beside log) | version, id, createdAt, cwd, parentSession, seedLength, origin, delegationDepth, agentPreset |
| Flush | batched (~200ms window); `session/flush` drains |
| Crash | open `turn/start` without end → synthetic `turn/end { interrupted }`. Do not truncate. Live load refuses synthetic close. |
| Format | refuse foreign version. No upgrader. |

### 3.4 Replay / resume / fork

| Operation | Mechanism |
|---|---|
| Replay | `Session.create(id, seed)` / `fromRestore`. Seed events validated, frozen. `session/end-seed` marks first live seq. |
| Resume | `ctx.agents.resume({ resumeSessionId })`. Header `agentPreset` required so tools/prompt match history. |
| Fork | `ctx.sessions.fork(source, boundary?, childSessionId?)`. Prefix must end **outside** an open turn. Child inherits cwd + `parentSession` + `seedLength`. |
| Test replay | Recorded JSONL as both mock-model script and expected output (Developers Digest). |

Trajectory / search / resume all project the same stream. Compaction uses `surfaceOp: replace` so derived history shrinks without rewriting old bytes.

---

## 4. Browse / file browsing / web search (duyệt)

Three different products. Do not collapse.

### 4.1 Human directory browse

`ui-workspace` + `directoryPicker`. Pick/create a workspace root. Not a full IDE file tree as the primary agent surface. Agent sees the tree via tools.

### 4.2 Agent file discovery

`glob` / `grep` spawn packaged ripgrep (`--no-config`, argv only). `read` / `edit` / `write` go through `ctx.fs`. Search result cards: `shape: 'paths' | 'matches'`, truncated flags. Spill store for over-cap lists.

### 4.3 Web search / fetch

`ctx.web` is the seam. Tools do not name Exa/Perplexity. Selection: explicit provider id or exactly one usable provider; else structured `WebError` (missing / unavailable / ambiguous). Fetch: non-2xx is a result, not a throw. No PDF arm. No Firecrawl-style extract (deferred).

Session-query tools (`session_search`, `session_event_search`, …) search **prior session logs** in the workspace — browse of nhật ký, not of the web.

---

## 5. Local web UI

**Launch (official README):**

```sh
npx @deepseek-ai/dsh web
```

- Default: `http://127.0.0.1:3080`, open default browser on local launch.
- SSH: print host URL only (`SSH_CONNECTION` / `SSH_TTY`).
- `--no-open` — server, no browser.
- `--port` — web-app flag (after launcher flags).
- `--dump-config` — composed plugin tree.

**Bind policy (official `dsh-web-app` README, higher weight than unofficial quickstarts):** CLI **rejects `--host 0.0.0.0`**. All-interfaces binding not supported yet. Unofficial `deepseekdocs.com` saying `dsh web --host 0.0.0.0` is stale/wrong. Loopback-only is the documented safety posture (API key + filesystem).

**After boot:**

1. Settings → Models (DeepSeek key; catalog Anthropic/OpenAI/Bedrock/Vertex/Azure; custom OpenAI-compatible). Keys in `$DSH_HOME/.credentials.yaml`.
2. Choose workspace.
3. Send a task. Approval panel takes the composer when needed.

**Trajectory** (`@deepseek-ai/dsh-client-ui-trajectory`):

- Turn-aware ledger: User / Assistant / Tool / nested Subtool.
- Inspect by source; token usage, duration, timing overview.
- Virtualized tail; page older records; search/fold in loaded window.
- Resume/fork/search/replay = same event stream (product page).
- Renders session data in the browser. Nothing here enters a model request.
- Session window keeps raw Events; Trajectory keeps assembled State. Does not mutate the chat snapshot.

**Web stack (facts):** `dsh-web-app` mounts webserver, API gateway, workspace, projection cache, frontend-static. Frontend dist must be built. Client plugins HMR. `DSH_WEB_URL` injected into bash env when `surfaceContext` is true.

---

## 6. What Dory must NOT copy

Constitution kill condition: "Hàng xuất Dory gọi `dsh` / `herdr` như vòng chạy thật."

| Do not take | Why | Evidence |
|---|---|---|
| Cordis kernel / plugin runtime | Dory engine-commit gate forbids plugin kernel until both xia reports signed **and** staffing/burn letter. Even after: transplanting a vendored Cordis fork is renting their architecture. | architecture.md; vendor/cordis (Digest: 4.0.0-rc.7 + local patches, `@deepseek-ai` scope) |
| `dsh` / `@deepseek-ai/dsh` as runtime dep | Product would call their CLI/loop. Forbidden. | HIEN-PHAP.md; npm bin `dsh` |
| Python `deepseek-harness-sdk` | Official SDK **bundles Node runtime**. Still `dsh` at runtime. | docs/user/guide/python-sdk.md |
| Subagent providers that exec Claude Code / Codex / another `dsh` | Same rent. Disabled-by-default in source is irrelevant if Dory ships it. | subagent.md; New Stack / xcloud |
| Everything-is-a-plugin as product identity | Dory is two engines, not a plugin marketplace. flow-skill stays the judge; deck stays the board. | HIEN-PHAP.md |
| 200+ package monorepo / Creator `cordis_*` tools | Preview-scale complexity. Dory is paper. | Digest 219 pkgs / ~453k LOC (2026-08-13 clone; not re-counted here) |
| Their format versions / `~/.dsh` layout | No migrate; tying Dory journal to dsh JSONL is a hidden runtime coupling. | persistence docs |
| MCP-as-identity | dsh is MCP **client only**. Do not become a dsh plugin host. | Digest; tool-catalog |

**Ideas that are legal to learn (still not a port ticket):**

- Journal is the only model-visible history; UI is a projection.
- Ask-before-danger as logged ask/decide pairs; fail closed.
- Workspace selected before compose.
- Fork only at a closed-turn boundary.
- Browse split: picker / fs search / web seam.

---

## 7. License, maturity, key URLs

### License

MIT. Copyright (c) 2026 DeepSeek. Third-party: `THIRD_PARTY_NOTICES.md`. SPDX on GitHub API: MIT.

### Maturity / adoption risk

| Dimension | State |
|---|---|
| Stage | Developer preview. Vendor + author: breaking changes. |
| Semver | npm `0.1.0-rc.7` vs git `0.1.1-rc.1` same day — do not pin a story to one RC. |
| API stability | Session format refuse-not-migrate. Plugin IDs / Cordis APIs will move. |
| Contribution | No external PRs. Small team. Discussions may go unanswered. |
| Support surface | Issues off. Bug reports are Discussions. |
| Windows | Directory picker discussions: native dialog hidden/crash; browse UI may not mount. |
| Eval | No first-party harness scores. Minimal mode is what their model cards named. |
| Abandonment | Low org-abandon risk (DeepSeek-owned, tied to V4 agent story). High **API-churn** risk. |
| Star count | 8-day viral (~27k → ~89k → ~170k+). Useful as attention, not as quality. |

### Key docs URLs

**Official**

- Product: https://deepseek.com/harness/en/
- Repo: https://github.com/deepseek-ai/deepseek-harness
- README: https://github.com/deepseek-ai/deepseek-harness/blob/master/README.md
- Architecture: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/architecture.md
- Session: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/session.md
- Persistence (published): https://deepseek-harness.github.io/deepseek-harness/en/reference/subsystems/persistence
- Tools: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/tools.md
- Tool catalog: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/tool-catalog.md
- Approval: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/approval.md
- Sandbox: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/sandbox.md
- Subagent: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/subagent.md
- Web UI guide: https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/user/guide/index.md
- Published guide: https://deepseek-harness.github.io/deepseek-harness/en/guide/quickstart
- CLI: https://github.com/deepseek-ai/deepseek-harness/blob/master/apps/cli/README.md
- Web bundle: https://github.com/deepseek-ai/deepseek-harness/blob/master/packages/bundle/web-app/README.md
- Trajectory: https://github.com/deepseek-ai/deepseek-harness/blob/master/packages/client/ui-trajectory/README.md
- Cordis tutorial: https://deepseek-harness.github.io/deepseek-harness/en/develop/cordis-tutorial/
- Cordis: https://github.com/cordiverse/cordis
- Cordis paper: https://github.com/cordiverse/paper
- Discussions: https://github.com/deepseek-ai/deepseek-harness/discussions
- npm: https://www.npmjs.com/package/@deepseek-ai/dsh
- Models key: https://platform.deepseek.com/

**Independent (cited)**

- https://www.developersdigest.tech/blog/deepseek-harness-dsh-first-look
- https://thenewstack.io/deepseek-harness-open-source-plugins/
- https://news.ycombinator.com/item?id=49285244
- https://www.theregister.com/ai-and-ml/2026/08/14/deepseeks-innovative-harness-treats-everything-as-a-plug-in/5288095

**Do not treat as source of truth:** https://deepseekdocs.com/ — community course; conflicts official bind flags.

---

## 8. Community and similar harnesses

### Reception (consensus)

**Praise**

- Opened the **loop**, not another closed vertical (Claude Code / Codex). MIT + source.
- Traceable run: append-only log + Trajectory. HN: "killer feature"; US traces often obfuscated.
- Plugin claim survives contact with source (no privileged core).
- Fail-closed sandbox + approval + log invariant = disciplined runtime engineering (Digest).
- Model-agnostic adapters; can sit above Claude/Codex via PATH (also a political/architecture smell for Dory).

**Criticism**

- Plugin fatigue (HN). "Everything is a plugin" read as no batteries / ecosystem rot risk. Counter: batteries **are** included; plugins are the **kernel**, not an empty app store.
- Preview: breaking changes, RC versioning, Windows picker bugs, no external PRs.
- JS/Node for a filesystem+shell agent. Terminal users prefer Crush / OpenCode / Claude Code CLI.
- No eval harness / scores shipped with the runtime.
- Squash-launch + issues-off = contributor-hostile.
- Web UI "OK for prototyping" (HN), not a daily terminal.

### Similar products (ranked for Dory compare, not for adoption)

| Rank | Product | License | Interface | Bet | vs Dory |
|---|---|---|---|---|---|
| 1 | **dsh** (this) | MIT preview | Local web 3080 + headless | Plugin kernel + log-as-truth | Learn nhật ký + máy phiên. Do not take kernel. |
| 2 | **Claude Code** | Proprietary | CLI, IDE, desktop, web | Vertical, hooks, skills, mature | Same job. Closed loop. Dory must not call it. |
| 3 | **OpenCode** | MIT | Terminal / IDE, huge adoption | Polished open daily driver | Closer "just works". Weaker public log-invariant story. |
| 4 | **Codex CLI** | Closed core | CLI + ChatGPT surfaces | Kernel sandbox, OpenAI stack | Strong confine. Ecosystem-bound. |
| 5 | **Pi / Prime / others** | Mixed | Often tiny cores | Minimal tools, user builds rails | Opposite of dsh's maximal plugin tree. |

Press one-liners (secondary): Claude Code = production default; OpenCode = open daily driver; dsh = architecture lab / second agent. Fits vendor's own preview warning.

---

## Trade-off matrix (dsh vs a paper Dory máy phiên)

| Dimension | dsh | Dory (constitution now) | Winner for Dory học |
|---|---|---|---|
| Session truth | Append-only events + runtime byte-match | Nhật ký is truth (paper) | Learn dsh invariant; keep Dory's own log shape |
| Extensibility | Cordis plugins, ~200 packages | Two engines, no plugin kernel yet | Dory. Kernel is a kill condition. |
| Dangerous work | Approval log + sandbox ladder | "Hỏi trước việc nguy hiểm" | Learn fail-closed, not their backends |
| Browse | Picker + rg + ctx.web | Duyệt in scope | Learn the **split**, not Exa/ripgrep packaging |
| UI | Local web + Trajectory | Web local in scope | Learn "UI = projection" |
| Runtime dep | `dsh` / bundled Node SDK | Never call dsh/herdr | Dory constitution |
| Maturity | Preview, breakages | Paper until two signed compares | Neither ships engine this month |
| Complexity | High | Must stay paper | Dory. Do not import monorepo. |
| Cost | DeepSeek-cheap models + self-host UI | N/A | Irrelevant until engines exist |
| Community | Viral, PRs closed | Internal học | Do not wait on their ecosystem |

---

## Architectural fit (Dory)

Dory today: `HIEN-PHAP.md` + README. No `package.json`. No engine. flow-skill = judge. flow-deck = frozen board. Dory = máy phiên + cửa sổ (engine 2 = Herdr compare, later).

| Dory box | dsh analogue | Fit |
|---|---|---|
| Máy phiên | Agent loop + tools + approval + sandbox + cwd | Strong behavioral overlap |
| Nhật ký | `Session` + persistence + deriveMessages | Strongest overlap. Same doctrine. |
| Duyệt | directoryPicker + glob/grep + web | Split three ways in dsh |
| Web local | `dsh web` + Trajectory | Same product shape; different stack |
| Cửa sổ (engine 2) | Not this repo | Herdr compare. Do not steal dsh web as window engine. |

**Adoption risk if someone later `--port`s anyway:** preview churn, format refuse, Cordis vendor-fork merge tax, Windows picker, Python-is-Node, temptation to `npx dsh` "just for now". Constitution already names that as death.

---

## Glossary

| Term | Meaning |
|---|---|
| dsh | DeepSeek Harness CLI / product |
| Cordis | Plugin/event context runtime. Spatiotemporal composability paper. |
| Profile | Named plugin stack (`web`, `headless`) under `~/.dsh/profiles/` |
| Bundle | npm package of Cordis config rows + code |
| SessionEvent | One append-only log row |
| Surface | Ordered model-visible projection of the log |
| Trajectory | Web ledger over session events |
| Approval | Per-action ask; `allowed-once` only grant |
| SandboxMode | File-effect confine: read-only / workspace-write / danger-full-access |

---

## Limitations of this research

- No clone, no `npx`, no live Trajectory screenshot. Behavioral claims from docs + one code-read blog + discussions.
- LOC / package counts from Developers Digest 2026-08-13 (`47f9438`); tree has moved (`528c682`).
- Star counts unstable; possible inflation + antifraud drops.
- Did not enumerate every tool in `tool-catalog.md` (~2200 lines).
- Did not verify Discord, every Discussion, or PyPI `deepseek-harness-sdk` current version.
- SitePoint / some SEO blogs discarded or down-weighted.
- Engine 2 (Herdr / cửa sổ) out of scope.

---

## Unresolved questions

1. Exact default permission preset in current `dsh-base` patch (agent notes say workspace-write+ask; not re-read on HEAD).
2. Whether npm will publish `0.1.1-rc.1` before the Herdr compare is signed.
3. Official "Developer docs" href on deepseek.com/harness (page fetch did not expose the link target; github.io + repo `docs/` are the working set).
4. How much of Trajectory "inspect by source" is shipped vs marketed (package README is UI-dense; product page is one paragraph).
5. Whether in-app browse backend is now default on Windows after #527/#1523 (unverified on HEAD).

---

## Next (xia, not this agent)

This file is the researcher handoff. Compare report (signed, in this repo) is a later xia phase. Constitution: no `/ak:plan`, no `--port`/`--copy`/`--fast`, no engine commit.

Do not implement.

---

## Citations (compact)

1. https://deepseek.com/harness/en/
2. https://github.com/deepseek-ai/deepseek-harness
3. https://github.com/deepseek-ai/deepseek-harness/blob/master/README.md
4. https://github.com/deepseek-ai/deepseek-harness/blob/master/LICENSE
5. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/architecture.md
6. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/session.md
7. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/tools.md
8. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/approval.md
9. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/sandbox.md
10. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/subagent.md
11. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/user/guide/index.md
12. https://github.com/deepseek-ai/deepseek-harness/blob/master/apps/cli/README.md
13. https://github.com/deepseek-ai/deepseek-harness/blob/master/packages/bundle/web-app/README.md
14. https://github.com/deepseek-ai/deepseek-harness/blob/master/packages/client/ui-trajectory/README.md
15. https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/tool-catalog.md
16. https://github.com/deepseek-ai/deepseek-harness/blob/master/CONTRIBUTING.md
17. https://deepseek-harness.github.io/deepseek-harness/en/reference/subsystems/persistence
18. https://github.com/cordiverse/cordis
19. https://github.com/cordiverse/paper
20. https://www.npmjs.com/package/@deepseek-ai/dsh
21. https://www.developersdigest.tech/blog/deepseek-harness-dsh-first-look
22. https://thenewstack.io/deepseek-harness-open-source-plugins/
23. https://news.ycombinator.com/item?id=49285244
24. https://github.com/deepseek-ai/deepseek-harness/discussions/527
25. Local: `/home/manhquy/Downloads/dory/HIEN-PHAP.md`
