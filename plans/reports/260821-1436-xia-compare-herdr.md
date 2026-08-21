---
type: xia-compare
date: 2026-08-21
status: signed
source: herdrdev/herdr
source_ref: master@624dfd4796559042ec13ccf4d4b54374902ab81d
local: dory
mode: --compare
scope: cửa sổ thẻ ô, trạng thái agent, skill điều phối
plan: forbidden-until-both-signatures
harness_compare: plans/reports/260821-1416-xia-compare-deepseek-harness.md
harness_status: signed
---

# Feature Comparison: Cửa sổ · Thẻ · Ô · Trạng thái agent · Skill điều phối

## Source: herdrdev/herdr

## Local Project: Dory

Xia `--compare` only. Không kế hoạch cài. Không `--port` / `--copy` / `--fast`. Không `/ak:plan`. Không `package.json`. Không `go.mod`. Không máy cửa sổ / PTY.

Hiến pháp bước 4–5: học Herdr, viết báo cáo **trong repo này**, ký, dừng. Học 1/2 (Harness) đã ký. Cổng chết động cơ vẫn đóng cho đến **cả hai** chữ ký **và** đủ người / thư đốt flow-skill.

Không ghi vào `flow-deck`.

---

## Source Manifest

| Field | Value |
|---|---|
| Repo | https://github.com/herdrdev/herdr |
| Homepage | https://herdr.dev |
| Docs | https://herdr.dev/docs/ |
| Branch | `master` |
| SHA (observed 2026-08-21) | `624dfd4796559042ec13ccf4d4b54374902ab81d` (2026-08-20) |
| Release | `v0.8.2` (Cargo.toml cùng số; Homebrew quan sát lệch 0.8.0) |
| License | Apache-2.0 (đổi từ AGPL-3.0-or-later ở v0.8.0). crates.io `0.1.0` vẫn AGPL — bỏ. |
| Language | Rust. Một binary. Server + client TUI (Ratatui). PTY vendored `portable-pty`. |
| Stage | Đang ship 0.8.x. Windows GA. Handoff / pane history còn experimental. |
| Issues / PRs | Issues + Discussions bật. PR implementation không mời bị đóng. |
| Command | `herdr` trần = attach TUI. Điều khiển = `herdr <group> …` qua socket local. |
| Narrowed pack | 69 files / ~257k tokens (README, AGENTS, skill, cli/workspace/pane/session/detect/api). Archive packed; **no source scripts executed**. Không `curl \| sh`. Không `cargo build`. Không chạy binary `herdr`. |
| Security | Source treated as untrusted data. Structure, metadata, dependency facts, behavioral evidence only. |

## Local Map

| Field | Value |
|---|---|
| Repo | `/home/manhquy/Downloads/dory` |
| Files | Giấy: `README.md`, `HIEN-PHAP.md` + reports học |
| Engine 1 | Máy phiên — học Harness, **đã ký** |
| Engine 2 (this report) | Cửa sổ chỗ làm: cửa sổ → thẻ → ô; máy chủ giữ tiến trình thật; một agent điều khiển agent khác |
| Neighbor board | `flow-deck` — chiếu **thẻ**, đóng băng, **không** phải multiplexer |
| Neighbor judge | `flow-skill` — cổng và biên lai |
| Ban | Hàng xuất Dory cấm gọi `dsh` / `herdr` lúc chạy |

---

## Source Anatomy

```text
human: herdr
  → connect hoặc spawn herdr-server (session namespace)
  → client TUI gắn vào; workspace từ cwd
  → pane = PTY thật (shell)
  → người hoặc CLI: agent start --kind claude|codex|…
  → detector: process foreground + (hooks XOR screen manifest)
  → sidebar: blocked / working / done / idle / unknown
  → detach (ctrl+b q): client mất; server + PTY còn
  → server stop: process chết; restore sau chỉ còn hình
```

| Layer | Source owner | Dory box |
|---|---|---|
| Session (Herdr) | Server namespace, socket riêng | Không phải «phiên» Harness |
| Workspace `w1` | Project container | Cửa sổ |
| Tab `w1:t1` | Một layout trong cửa sổ | Thẻ |
| Pane `w1:p1` | Terminal thật, sống khi detach | Ô |
| Agent name | Alias occupant, không phải ID layout | Trạng thái + điều phối |
| Detector | Hooks **hoặc** screen TOML (một authority / pane) | Trạng thái agent |
| Skill + CLI | `HERDR_ENV=1` rồi mới điều khiển | Skill điều phối |
| TUI / `--remote` | Client = chiếu server | Không lấy TUI làm danh tính |
| Plugin marketplace | Mở rộng pane | **Refuse** |

---

## Head-to-Head

| Aspect | Source (herdr) | Local (Dory, giấy) | Recommendation |
|---|---|---|---|
| Product | «Runtime agents live on.» Server ôm PTY. Không thay Claude/Codex — ôm terminal của chúng. | Động cơ 2: chỗ làm. Chưa viết. | Học hành vi. Đừng thành `herdr`. |
| Topology | session → workspace → tab → pane. ID mờ: `w1`, `w1:t1`, `w1:p1`. Đóng không tái sử dụng. | Cửa sổ → thẻ → ô. | Học ba lớp + ID không đoán. |
| Tạo | workspace/tab create luôn đẻ root pane. `pane split` không đẻ workspace. | Chưa có. | Học: layout trước process. |
| Máy chủ giữ process | Detach = mạnh (PTY sống). `server stop` = yếu (còn hình). | «Máy chủ giữ tiến trình thật». | Học hàng detach. Đừng nhầm restore hình với process sống. |
| Ô ≠ agent | Pane tồn tại không cần agent. `agent start` **cấm** tạo/split/move layout. | Một agent điều khiển agent khác. | Học ba primitive: layout / pane / agent. |
| Trạng thái | `working` `blocked` `done` `idle` `unknown` | Cùng năm chữ trong scope học. | Học. `unknown` ≠ xong. |
| Seen | `done` = idle chưa nhìn. Focus tab/ô mới thành `idle`. CLI read **không** đánh dấu seen. | Chưa có. | Học bit seen. Bẫy điều phối. |
| Detect | Claude/Codex: screen vẫn là authority dù có integration. Hooks XOR manifest. | Chưa có. | Học một authority. Đừng scrape làm sự thật máy phiên. |
| Wait | `agent prompt --wait` chờ lifecycle, không chờ từng turn. Đã `blocked` → không gửi. | Điều phối. | Học wait-on-state, không fire-and-hope. |
| Skill | `skills/herdr/SKILL.md`. Cổng `HERDR_ENV=1`. Ngoài Herdr thì dừng. | Skill điều phối. | Học cổng «phải đứng trong cửa sổ». |
| Occupants | `--kind` PATH: claude, codex, opencode, grok, … | Ủy thác; cấm thuê vòng. | Học ý child. **Cấm** PATH hosts làm hàng Dory. |
| UI | Ratatui chuột + prefix tmux. Client chiếu pane. | Không đặt tên TUI. | Học client = view. Đừng clone tmux. |
| vs máy phiên | Không phải nhật ký model. PTY là sự thật chỗ làm. | Harness học: log là sự thật phiên. | Hai sự thật, hai động cơ. Ô **chứa** phiên; log **không phải** ô. |
| vs deck | Sidebar agent trên terminal sống. | Deck chiếu **thẻ** build. Đóng băng. | Đừng nâng deck thành Herdr. |
| Runtime dep | Binary `herdr` + socket. | Cấm gọi `herdr` lúc chạy. | **Local.** Thuê vòng = giết. |
| Kernel | Một binary + marketplace plugin. | Hai động cơ, không chợ. | **Local.** Marketplace = hình C. |
| Chín muồi | 0.8.x + YC; detect hay lệch; PR đóng. | Giấy. | Không ghim binary họ. |

---

## Execution Path (cửa sổ chỗ làm)

Thought 1 — Dory so hành vi chỗ làm, không so Ratatui.

Thought 2 — «Máy chủ giữ tiến trình» = detach còn PTY, không = snapshot sau `server stop`.

Thought 3 — Ba primitive không gộp: chỗ (workspace/tab/pane) ≠ terminal thô ≠ occupant có lifecycle.

Thought 4 — `done`/`idle` là cùng trạng thái process + bit seen. CLI đọc không xóa `done`.

Thought 5 — Skill điều phối chỉ hợp lệ **từ trong** cửa sổ (`HERDR_ENV=1`). Agent ngoài không được lái session.

Thought 6 — `agent start --kind claude` là thuê vòng. Hiến pháp đã đặt tên cái chết.

Thought 7 — Web Harness ≠ cửa sổ. TUI Herdr ≠ máy phiên. Deck ≠ cả hai.

Thought 8 [FINAL] — Học topology + detach-strong + năm trạng thái + cổng skill. Từ chối binary, TUI-as-identity, marketplace, PATH hosts. Ký. Dừng. Không plan. Không máy.

```text
máy chủ (session namespace)
        │
        ▼
   cửa sổ w1 ──► thẻ w1:t1 ──► ô w1:p1 (PTY)
                                      │
                        ┌─────────────┴─────────────┐
                        │                           │
                   pane run / read            agent start / prompt
                   (process thường)           (occupant + lifecycle)
                        │                           │
                        └──────────┬────────────────┘
                                   ▼
                    detector → working|blocked|done|idle|unknown
                                   │
                    skill (chỉ khi HERDR_ENV=1)
                    một agent split ô khác, chờ blocked/idle
```

Partial failure: `agent_prompt_stalled` nếu không đổi lifecycle trong 5s. `agent_blocked` = không gửi. Move pane sang workspace khác: ID mới; wait đang chạy chết `agent_not_running`. Handoff sống cắt wait/subscription.

---

## Dependency Matrix

`EXISTS` = luật giấy. `NEW` = sau này tự viết. `CONFLICT` = lấy là sai / giết.

| Source component | Local equivalent | Status |
|---|---|---|
| Cửa sổ → thẻ → ô | `HIEN-PHAP.md` động cơ 2 | EXISTS (law) / NEW (impl) |
| Server ôm PTY, detach còn process | «Máy chủ giữ tiến trình thật» | EXISTS / NEW |
| Năm trạng thái + seen | «Trạng thái agent» | EXISTS / NEW |
| Một agent lái agent khác | cùng câu | EXISTS / NEW |
| Cổng «phải ở trong cửa sổ» | Skill điều phối | NEW (lesson) |
| layout ≠ pane ≠ agent | chưa tách chữ | NEW (split is the lesson) |
| Gọi `flow.sh` | Mũi tên hiến pháp | EXISTS in flow-skill |
| Nhật ký phiên (Harness) | Động cơ 1, đã ký | EXISTS (law) — khác hộp |
| Deck thẻ | `flow-deck`, đóng băng | EXISTS — **không** trộn |
| Binary `herdr` / socket họ | Cấm thuê vòng | **CONFLICT** |
| TUI Ratatui as identity | Không đặt | **CONFLICT** |
| `--kind claude\|codex\|…` | Cấm PATH rent | **CONFLICT** |
| Plugin marketplace | Hình C | **CONFLICT** |
| Screen-scrape làm nhật ký model | Động cơ 1 đã từ chối store hai | **CONFLICT** |
| dsh web 3080 as cửa sổ | Harness compare đã từ chối | **CONFLICT** |

Estimate: tờ này chỉ report. File động cơ = 0 cho đến hai chữ ký **và** đủ người / thư đốt.

---

## Challenge

### 1. Necessity — cần sản phẩm hay chỉ ý?

- **Source:** Cả multiplexer: TUI, detector 20 kind, marketplace, remote, handoff.
- **Local:** Cần ý server-ôm-PTY + ba lớp + lifecycle + cổng điều phối. Không cần `herdr`.
- **Risk if wrong:** Port binary / clone Ratatui → phá cổng chết, ôm detect farm. Critical.

### 2. Simpler alternative — 80% với ít hơn?

- **Source:** Full product.
- **Local:** Sau này: server giữ process, ID ba lớp, năm trạng thái + seen, skill chỉ chạy khi đứng trong cửa sổ.
- **Risk if wrong:** Underbuild (detach giết process) hoặc overbuild (thành herdr hai). Critical nếu overbuild.

### 3. Existing overlap — đã có phần nào?

- **Source:** Sidebar = attention queue trên terminal sống.
- **Local:** `flow-deck` đã là attention queue trên **thẻ**. Đóng băng. Không được lớn thành multiplexer.
- **Risk if wrong:** Nâng deck, hoặc ghi report vào `flow-deck/plans/`. Giết. Critical.

### 4. Maintenance — ai nuôi detect?

- **Source:** Solo + YC; screen-scrape Claude/Codex; remote manifest; changelog đầy false idle/blocked.
- **Local:** Nhà Dory phải tự nuôi mọi byte.
- **Risk if wrong:** Ghim heuristic họ. Mỗi lần TUI agent đổi là vỡ wait. Critical.

### 5. Dependency chain — `herdr` «tạm»?

- **Source:** Nhà máy hôm nay được dùng Herdr. Hàng xuất không được.
- **Local:** Học ≠ thuê. Cấm `herdr` lúc chạy.
- **Risk if wrong:** Dory gọi `herdr pane split` như runtime. Giết hiến pháp. Critical.

### 6. Architecture — TUI vs cửa sổ?

- **Source:** TUI là client mặc định. Founder: TUI là một client, runtime là server.
- **Local:** Hiến pháp nói cửa sổ/thẻ/ô, không nói Ratatui.
- **Risk if wrong:** Dory thành clone tmux. Mất chỗ cho client khác. Medium–critical.

### 7. Blast radius — điều phối = PATH Claude/Codex?

- **Source:** `agent start --kind` chính là spawn host trên PATH.
- **Local:** Ủy thác; Harness compare đã cấm PATH hosts.
- **Risk if wrong:** Động cơ 2 mở lại lỗ động cơ 1 vừa đóng. Critical.

### 8. Hai sự thật — PTY vs nhật ký?

- **Source:** Sự thật chỗ làm = PTY sống + detector snapshot.
- **Local:** Sự thật phiên = log append-only (Harness, đã ký).
- **Risk if wrong:** Gộp một store. Hình C. Ô phải chứa phiên; log không được là ô.

---

## Decision Matrix

| # | Decision | Source's way | Local way | Hybrid | Risk | Choice |
|---|---|---|---|---|---|---|
| 1 | Identity | TUI + marketplace | Hai động cơ, không chợ | Server-as-law, client-as-view | critical | **local** |
| 2 | Topology | w/t/p + opaque IDs | cửa sổ/thẻ/ô | Same three layers, own IDs | low | **learn source rule** |
| 3 | Process | Detach-strong, restart-weak | Máy chủ giữ tiến trình | Detach-strong only as law | medium | **learn detach row** |
| 4 | Agent state | Five words + seen | Same words | Same contract | medium | **learn source rule** |
| 5 | Detect | Hooks XOR screen | None | Own later; don't scrape as journal | critical | **local later** |
| 6 | Coordination | Skill + `HERDR_ENV=1` | Một agent lái agent khác | Gate + pane≠agent | medium | **learn gate** |
| 7 | Occupants | PATH 20 kinds | Cấm thuê vòng | In-house child only | critical | **local — no PATH hosts** |
| 8 | vs máy phiên | PTY truth | Log truth | Two truths, two engines | critical | **keep split** |
| 9 | vs deck | Sidebar terminals | Board cards, frozen | Never merge | critical | **local — deck stays board** |
| 10 | Runtime | `herdr` binary | Never call herdr | — | critical | **local** |

---

## Risk Score

| Count of critical challenges | Band | Action |
|---|---|---|
| 7 (product transplant, overbuild, deck merge, detect pin, runtime rent, PATH hosts, two-truths merge) | **High** if someone `--port`s | Stay `--compare`. Do not plan. |
| This report itself | **Low** | Paper + học. No engine commit. |

Critical = sai là mất process, thủng điều phối, hoặc **giết hiến pháp**.

---

## Recommendation

**Ký báo cáo này như học 2/2. Dừng. Không nấu. Không plan. Không máy.**

Học, giữ:

1. Ba lớp: cửa sổ → thẻ → ô. ID mờ, đóng không tái dùng, đọc JSON không đoán.
2. Máy chủ giữ PTY. Detach ≠ chết. `server stop` / restore hình ≠ process sống.
3. Layout ≠ pane (terminal thô) ≠ agent (occupant + lifecycle). `agent start` không tạo chỗ.
4. Năm trạng thái. `done`/`idle` = cùng nền + bit seen. CLI read không seen. `unknown` ≠ xong. `blocked` = không tự gửi.
5. Skill điều phối chỉ khi đứng trong cửa sổ. Agent ngoài dừng.
6. Ô chứa phiên. Nhật ký phiên (Harness) không phải ô. Deck chiếu thẻ, không chiếu PTY.

Từ chối, kể cả «tạm»:

- Binary `herdr`, socket họ, `curl | sh`
- TUI Ratatui / plugin marketplace như danh tính Dory
- `--kind claude|codex|…` như ủy thác hàng xuất
- Screen-scrape làm nhật ký model
- Web dsh như cửa sổ; Herdr như máy phiên; nâng `flow-deck`
- `/ak:plan`, `--port`, `--copy`, `--fast`, `package.json`, `go.mod`, PTY/engine commit

Sau chữ ký tờ này: **cả hai học đã ký**. Cổng chết **vẫn đóng** cho đến đủ người hoặc thư đốt flow-skill 6–12 tháng. Hiến pháp hình A: tháng này không viết động cơ.

---

## Evidence (primary)

- https://github.com/herdrdev/herdr/blob/master/README.md
- https://herdr.dev/docs/concepts/
- https://herdr.dev/docs/agent-automation/
- https://herdr.dev/docs/session-state/
- https://herdr.dev/docs/agent-skill/
- https://github.com/herdrdev/herdr/blob/master/skills/herdr/SKILL.md
- https://herdr.dev/docs/socket-api/
- Local: `HIEN-PHAP.md`, `README.md`
- Học 1/2 (signed): `plans/reports/260821-1416-xia-compare-deepseek-harness.md`
- Researcher: `plans/reports/260821-1435-research-herdr.md`
- Pack (untrusted extract): `/tmp/dory-xia-herdr/herdr-scope.md` — not committed

Did not: clone into this repo, install script, `cargo build`, run `herdr`, execute source, write into `flow-deck`.

---

## Unresolved Questions

1. Copyright line trong LICENSE để trống — không chặn ký học.
2. Homepage «agents while you sleep» bị che — commercial host sau này; không phải hành vi hôm nay.
3. Default detect authority trên từng kind có thể đổi theo remote manifest — học một-authority, không ghim bảng kind.
4. Homebrew/crates lệch version — đã ghi. Không chặn ký.

Không câu nào chặn ký học.

---

## Chữ ký

Báo cáo học 2/2 (Herdr). Sau khi ký: hai tờ học đủ điều kiện (1) của cổng chết. Điều kiện (2) — người / thư — **chưa** có. Vẫn cấm máy.

| Vai | Tên | Kết luận | Ngày | Chữ |
|---|---|---|---|---|
| Xia (học) | agent, cửa sổ `dory` | Compare xong. Operator đã ký. Không plan. Không máy. | 2026-08-21 | đã viết |
| Operator | cửa sổ `dory` | Chấp nhận tờ này là học Herdr đã ký | 2026-08-21 | **ký** |

Đã ký qua cổng xia. Học 2/2 xong. Hai tờ học đủ điều kiện (1). Điều kiện (2) người/thư chưa có. Dừng. Đừng mở `/ak:plan`. Không máy.
