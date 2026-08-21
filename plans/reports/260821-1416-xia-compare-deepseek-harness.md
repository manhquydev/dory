---
type: xia-compare
date: 2026-08-21
status: signed
source: deepseek-ai/deepseek-harness
source_ref: master@528c682e061696f5a160f363f236ecbf53cbd006
local: dory
mode: --compare
scope: máy phiên, nhật ký, duyệt, web local
plan: forbidden-until-both-signatures
---

# Feature Comparison: Máy phiên · Nhật ký · Duyệt · Web local

## Source: deepseek-ai/deepseek-harness

## Local Project: Dory

Xia `--compare` only. Không kế hoạch cài. Không `--port` / `--copy` / `--fast`. Không `/ak:plan`. Không `package.json`. Không máy.

Hiến pháp bước 2–3: học Harness, viết báo cáo **trong repo này**, ký, dừng. Động cơ 2 (Herdr) là lượt sau.

---

## Source Manifest

| Field | Value |
|---|---|
| Repo | https://github.com/deepseek-ai/deepseek-harness |
| Homepage | https://deepseek.com/harness |
| Branch | `master` |
| SHA (observed 2026-08-21) | `528c682e061696f5a160f363f236ecbf53cbd006` — `dsh@0.1.1-rc.1` |
| npm (registry, same day) | `@deepseek-ai/dsh@0.1.0-rc.7` — lệch git; đừng ghim một RC |
| License | MIT, Copyright (c) 2026 DeepSeek |
| Language | TypeScript / Node, monorepo pnpm, ~200 packages |
| Stage | Developer preview. README: **THERE WILL BE COMPATIBILITY-BREAKING CHANGES.** |
| Issues / PRs | Tắt. Discussions only. Cấm PR ngoài. |
| Command | `dsh` · `npx @deepseek-ai/dsh web` → `http://127.0.0.1:3080` |
| Narrowed pack | 19 files / 58,141 tokens (README + architecture + session/persistence/approval/sandbox/fs/shell + user guide + package READMEs). Archive packed; **no source scripts executed**. |
| Security | Source treated as untrusted data. Structure, metadata, dependency facts, behavioral evidence only. |

## Local Map

| Field | Value |
|---|---|
| Repo | `/home/manhquy/Downloads/dory` |
| HEAD | `5c1e866` Lock product name as Dory. |
| Files | `README.md`, `HIEN-PHAP.md` |
| Engine | None. Paper until both compares signed **and** staffing/burn letter. |
| Engine 1 (this report) | Máy phiên: chọn thư mục, chạy phiên, sửa file, ra lệnh, ủy thác, hỏi trước việc nguy hiểm; **nhật ký phiên là sự thật**. |
| Engine 2 (not this report) | Cửa sổ chỗ làm. Học Herdr sau. |
| Neighbor | `flow-skill` giữ cổng và biên lai, không giữ runtime. Dory sau này gọi `flow.sh` như máy chủ lạ. |

---

## Source Anatomy

```text
npx @deepseek-ai/dsh web
  → profile `web` = bundle `dsh-base` + `dsh-web-app` + patches
  → bind 127.0.0.1:3080 (CLI từ chối --host 0.0.0.0)
  → người: Settings → Models, rồi Choose workspace
  → composer khóa đến khi có workspace
  → turn/start → agent/pre-step → step → llm → tools → session.append
  → Trajectory / resume / fork đọc cùng một log
```

| Layer | Source owner | Dory box |
|---|---|---|
| Kernel | Cordis Context + Loader. Everything is a plugin. | **Refuse.** Not Dory identity. |
| Session log | `packages/core/session` — append-only `SessionEvent`; `deriveMessages()` | Nhật ký |
| Persistence | Sibling seam. JSONL (+ zstd) default; SQLite opt-in; refuse foreign format, no migrate | Nhật ký (later, own shape) |
| Agent loop | `core/agent` + `core/agent-loop`. Turn = 0+ steps. Step = 1 model call + its tools | Máy phiên |
| Tools | `ctx.fs` / `ctx.shell` / `ctx.tools`. `read` `edit` `write` `bash` `glob` `grep` | Sửa file, ra lệnh, duyệt |
| Delegate | `ctx.subagents`: in-process, fork, ACP, Claude Code, Codex, dsh-sdk | Ủy thác — **do not rent PATH backends** |
| Ask | `ctx.approval`: `ask`/`never`; only `allowed-once` grants; fail closed | Hỏi trước việc nguy hiểm |
| Confine | `ctx.sandbox`: `read-only` / `workspace-write` / `danger-full-access`; silent unconfined forbidden | Not named in constitution; learn fail-closed |
| Human browse | `ctx.directoryPicker`: native OS vs in-app browse | Chọn thư mục / duyệt |
| Web search | `ctx.web`: `web_search` / `web_fetch` | Duyệt (third seam) |
| Local web | `dsh-web-app` + Trajectory as ledger over the log | Web local — **projection, not engine 2** |

---

## Head-to-Head

| Aspect | Source (dsh) | Local (Dory, giấy) | Recommendation |
|---|---|---|---|
| Product | Agent **runtime**. Model + tools + log + UI. | Một gia đình, hai động cơ. Chưa có máy. | Học hành vi. Đừng thành `dsh`. |
| Kernel | Cordis. Mọi thứ là plugin. Không privileged core. | Hai động cơ. Cấm nhân plugin trước cổng chết. | **Local.** Kernel = CONFLICT. |
| Chọn thư mục | Invoking cwd ≠ workspace. Composer chết đến khi Choose workspace. `ctx.directoryPicker`. | «Chọn thư mục» là câu đầu của máy phiên. | Học: workspace trước compose. |
| Chạy phiên | Turn/step loop. Inbox một đường. `agent/pre-step` quyết model thấy gì. | «Chạy phiên» — chưa có vòng. | Học vòng turn/step. Đừng lấy driver Cordis. |
| Sửa file | `read`/`edit`/`write` qua `ctx.fs`; read-before-write. | «Sửa file» — giấy. | Học seam, không package. |
| Ra lệnh | `bash` / persistent PTY; Windows `pwsh`; jobs. | «Ra lệnh» — giấy. | Học: lệnh đi qua policy, không raw spawn. |
| Ủy thác | Subagent optional. Provider registry. Claude Code / Codex trên PATH, mặc định tắt. | «Ủy thác». Hàng xuất **cấm** gọi `dsh`/`herdr`. | Học in-process child + journal riêng. **Cấm** PATH rent. |
| Hỏi nguy hiểm | Approval log-only `asked`/`decided`. Chỉ `allowed-once` mở. Thiếu answerer → `unavailable` (deny). Sandbox fail-closed. | «Hỏi trước việc nguy hiểm». | Học fail-closed + cặp hỏi/quyết trong nhật ký. |
| Nhật ký | Log = nguồn context model. `deriveMessages()`. Dispatch assert model-visible == logged. | «Nhật ký phiên là sự thật». | **Học mạnh nhất.** Giữ hình Dory, không `~/.dsh`. |
| Replay / fork | Seed + `session/end-seed`. Fork chỉ ngoài turn mở. Crash: đóng `interrupted`, không truncate. | Chưa có. | Học ranh giới turn và không xóa log. |
| Duyệt | Ba khe: picker người; `glob`/`grep` (ripgrep đóng gói); `ctx.web`. | Một chữ «duyệt». | Học **tách ba**, đừng gộp một «browser». |
| Web local | `127.0.0.1:3080`. Trajectory = sổ cái trên log, không store hai. Từ chối bind mọi interface. | «Web local» trong list học. Cửa sổ chỗ làm = động cơ 2 (Herdr). | Học UI = chiếu nhật ký. **Đừng** lấy web dsh làm cửa sổ. |
| Quan hệ flow | dsh ôm loop + approval + UI. | flow giữ cổng; Dory giữ máy; không ngược. | Dory gọi `flow.sh`. Đừng nhét cổng vào máy. |
| Runtime dep | `dsh` / Python SDK **bó Node**. | Cấm gọi `dsh` lúc chạy. | **Local.** Thuê vòng = giết hiến pháp. |
| Format đĩa | JSONL (+ zstd), header cạnh log, refuse version lạ, không migrate. | Chưa có. | Sau này format **của Dory**. Đừng khóa JSONL dsh. |
| Chín muồi | Preview, phá tương thích, PR đóng, star ảo. | Giấy. Tháng này không viết động cơ. | Không ghim preview. |
| Độ phức tạp | ~200 gói, Creator/cordis tools. | Phải ở giấy. | Đừng nhập monorepo. |

---

## Execution Path (máy phiên)

Thought 1 — Dory so hành vi, không so gói.

Thought 2 — Câu nhật ký Dory và câu session-log dsh là **cùng một luật**: model chỉ thấy cái đã ghi.

Thought 3 — Máy phiên dsh = chọn workspace + vòng turn/step + tool + hỏi + sandbox. Nhân là Cordis. Dory cần bốn hành vi, không cần nhân.

Thought 4 — Duyệt không phải một sản phẩm. Picker ≠ `grep` ≠ `web_fetch`.

Thought 5 — Web 3080 là **sản phẩm người** của dsh. Cửa sổ Dory là động cơ 2. Gộp hai cái là hình C (một tên ôm hết) — hiến pháp cấm.

Thought 6 — `subagent-claude-code` / `subagent-codex` / Python-SDK-bó-Node là thuê vòng. Hiến pháp đã đặt tên cái chết này.

Thought 7 [FINAL] — Học bốn ý. Từ chối kernel, CLI, format, PATH delegates, web-as-window. Ký. Dừng. Không plan.

```text
người chọn workspace
        │
        ▼
   mở phiên (header: cwd, preset)
        │
        ▼
   turn/start ──► pre-step ──► step (llm + tools)
        │                         │
        │                         ├─ fs / shell  ─► sandbox + approval
        │                         ├─ glob/grep/web
        │                         └─ subagent? (child Session)
        │
        ▼
   mọi sự thật ──► append SessionEvent
        │
        ├─ deriveMessages() ──► model request (assert khớp log)
        ├─ Trajectory / web  ──► chiếu, không ghi thứ hai
        └─ persistence       ──► đĩa; crash = interrupted, không cắt
```

Partial failure: tool deny = `tool/result` + audit approval, turn có thể tiếp. Crash giữa turn = synthetic `turn/end { interrupted }`, giữ bytes đã append. Fork cắt im lặng bên trong turn **bị từ chối**.

---

## Dependency Matrix

`EXISTS` = luật giấy đã nói. `NEW` = sau này phải tự viết. `CONFLICT` = lấy là sai / giết.

| Source component | Local equivalent | Status |
|---|---|---|
| «Nhật ký là sự thật» | `HIEN-PHAP.md` máy phiên | EXISTS (law) / NEW (impl) |
| Chọn thư mục trước compose | «Chọn thư mục» | EXISTS / NEW |
| Sửa file, ra lệnh | same sentence | EXISTS / NEW |
| Ủy thác in-process + child log | «Ủy thác» | EXISTS / NEW |
| Hỏi trước việc nguy hiểm | same sentence | EXISTS / NEW |
| Duyệt tách ba khe | «Duyệt» (chưa tách) | NEW (split is the lesson) |
| Web = chiếu log | «Web local» | NEW — not engine 2 |
| Gọi `flow.sh` | Mũi tên hiến pháp | EXISTS in flow-skill |
| Cordis / plugin kernel | Cổng chết + hình A | **CONFLICT** |
| `@deepseek-ai/dsh` / `dsh` CLI | Cấm thuê vòng | **CONFLICT** |
| Python SDK (bundled Node) | same ban | **CONFLICT** |
| Claude Code / Codex PATH | same ban | **CONFLICT** |
| `~/.dsh` / session format dsh | Own journal later | **CONFLICT** if adopted as Dory disk |
| dsh Web as cửa sổ chỗ làm | Engine 2 = Herdr | **CONFLICT** |
| Sandbox backends (bwrap/Seatbelt/ACL) | Unnamed | NEW later; learn fail-closed only |

Estimate (not a plan): this compare writes reports only. Engine files = 0 until both signatures + staffing/burn.

---

## Challenge

Hard gate. Năm câu trở lên. Mỗi câu: source / local / rủi ro nếu sai.

### 1. Necessity — cần sản phẩm hay chỉ ý?

- **Source:** Cả runtime: plugin kernel, loop, UI, 200 gói.
- **Local:** Cần ý nhật ký + máy phiên. Không cần `dsh`.
- **Risk if wrong:** Port kernel → phá cổng chết, ôm preview churn. Critical.

### 2. Simpler alternative — 80% với ít hơn?

- **Source:** Maximal plugin tree.
- **Local:** Sau này: log append-only + workspace lock + ask fail-closed + tool nhỏ. Đủ cho động cơ 1.
- **Risk if wrong:** Underbuild (mất reconstructability) hoặc overbuild (thành dsh thứ hai). Critical nếu overbuild.

### 3. Existing overlap — đã có phần nào?

- **Source:** Tự ôm approval + receipts trong loop.
- **Local:** `flow-skill` đã là thẩm phán / biên lai. Dory không được nuốt flow.
- **Risk if wrong:** Hai cổng, hoặc flow bị nhét chữ `dory` (giết). Critical.

### 4. Maintenance — ai nuôi sau khi «học»?

- **Source:** DeepSeek, PR đóng, phá API, format không migrate.
- **Local:** Nhà Dory phải tự nuôi mọi byte mình viết.
- **Risk if wrong:** Ghim RC hôm nay, mở không được log ngày mai. Critical.

### 5. Dependency chain — dep mới?

- **Source:** Node, Cordis vendor-fork, ripgrep đóng gói, Exa/Perplexity, optionally Claude/Codex trên PATH.
- **Local:** Cấm `dsh`/`herdr` lúc chạy. Học ≠ thuê.
- **Risk if wrong:** `npx dsh` «tạm» = hàng xuất gọi vòng người ta. Giết hiến pháp. Critical.

### 6. Architecture match — paradigm?

- **Source:** Spatiotemporal plugin composability.
- **Local:** Ba nhà, lịch chồng. Hai động cơ. Không marketplace.
- **Risk if wrong:** Hình C — một tên ôm hết. Hiến pháp cấm.

### 7. Blast radius — web dsh có phải cửa sổ Dory?

- **Source:** Web UI là mặt người của **cùng** runtime máy phiên.
- **Local:** Web local ∈ học động cơ 1. Cửa sổ / PTY / «một agent điều khiển agent khác» = động cơ 2 (Herdr).
- **Risk if wrong:** Học Herdr thành vô nghĩa, hoặc Dory biến thành clone UI dsh. Critical.

### 8. Browse — một khe hay ba?

- **Source:** Picker / fs search / web. Ba seam.
- **Local:** Một chữ duyệt.
- **Risk if wrong:** Nhét Exa vào «chọn thư mục», hoặc tưởng file tree IDE là máy phiên.

---

## Decision Matrix

| # | Decision | Source's way | Local way | Hybrid | Risk | Choice |
|---|---|---|---|---|---|---|
| 1 | Kernel | Cordis plugins | Two engines, no kernel | — | critical | **local** |
| 2 | Nhật ký | Event log + deriveMessages + assert | Nhật ký là sự thật | Same law, own events | medium | **hybrid — law yes, format no** |
| 3 | Persistence | JSONL zstd, refuse, no migrate | None yet | Own format when engines exist | critical if copied | **local later** |
| 4 | Workspace | Picker; composer locked | Chọn thư mục | Same rule | low | **learn source rule** |
| 5 | Approval | allowed-once, fail closed, log pair | Hỏi trước việc nguy hiểm | Same rule, own events | medium | **learn source rule** |
| 6 | Delegate | Provider registry incl. PATH hosts | Ủy thác; cấm thuê vòng | In-process only | critical | **local — no PATH hosts** |
| 7 | Duyệt | Three seams | One word | Split the word | low | **learn the split** |
| 8 | Web local | 3080 + Trajectory | Projection of journal | View ≠ window engine | critical if merged | **learn projection; Herdr owns window** |
| 9 | flow | Loop contains policy | flow.sh is foreign judge | Dory calls flow, never reverse | critical | **local arrow** |
| 10 | Runtime | `dsh` / bundled Node SDK | Never call dsh/herdr | — | critical | **local** |

---

## Risk Score

| Count of critical challenges | Band | Action |
|---|---|---|
| 6 (kernel, overbuild, flow collision, maintenance pin, runtime rent, web=window) | **High** if someone `--port`s | Stay `--compare`. Do not plan. |
| This report itself | **Low** | Paper + học. No engine commit. |

Critical = sai là mất dữ liệu, thủng bảo mật, hoặc >2 ngày đập đi xây lại — hoặc **giết hiến pháp**.

---

## Recommendation

**Ký báo cáo này như học 1/2. Dừng. Không nấu. Không plan.**

Học, giữ:

1. Nhật ký là nguồn context model. UI / transcript / replay là chiếu. Model-visible phải reconstruct được từ log.
2. Fail-closed: không answerer = deny. Cặp hỏi/quyết nằm trong nhật ký, không chỉ toast UI.
3. Chọn workspace trước khi gõ. Cwd process ≠ workspace phiên.
4. Fork / resume chỉ ở ranh ngoài turn mở. Crash đóng `interrupted`, không xóa.
5. Duyệt = ba khe (người chọn thư mục / agent tìm file / web), không một browser.
6. Web local = sổ cái trên nhật ký. Không phải động cơ cửa sổ.

Từ chối, kể cả «tạm»:

- Cordis, plugin kernel, monorepo dsh
- `dsh` / `@deepseek-ai/dsh` / Python SDK bó Node như runtime
- Claude Code / Codex / `herdr` như vòng chạy hàng xuất
- Format `~/.dsh` / JSONL dsh như đĩa Dory
- Web dsh như cửa sổ chỗ làm
- `/ak:plan`, `--port`, `--copy`, `--fast`, `package.json`

Lượt sau (hiến pháp bước 4): `/ak:xia https://github.com/herdrdev/herdr.git cửa sổ thẻ ô, trạng thái agent, skill điều phối --compare` — **sau chữ ký tờ này**.

---

## Evidence (primary)

- https://github.com/deepseek-ai/deepseek-harness/blob/master/README.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/architecture.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/session.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/persistence.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/approval.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/sandbox.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/user/guide/index.md
- https://github.com/deepseek-ai/deepseek-harness/blob/master/packages/bundle/web-app/README.md
- Local: `HIEN-PHAP.md`, `README.md`
- Researcher: `plans/reports/260821-1416-research-deepseek-harness.md`
- Scout: `plans/reports/260821-1416-scout-dory.md`
- Pack (untrusted extract): `/tmp/dory-xia-harness/harness-scope.md` — 19 files, not committed

Did not: clone into this repo, `pnpm install`, run `dsh`, execute source scripts.

---

## Unresolved Questions

1. Đường đĩa JSONL chính xác trên HEAD (`~/.dsh/sessions/…`) — researcher nêu; pack persistence.md không in template path. Không chặn chữ ký.
2. Default permission preset trên `dsh-base` HEAD (notes nói `workspace-write` + `ask`) — không re-read patch file.
3. Windows directory-picker: native vs browse đã đổi default chưa (Discussions #527 / #1523).
4. npm sẽ publish `0.1.1-rc.1` khi nào — không liên quan chữ ký.
5. Trajectory «inspect by source» — marketing vs package README; không chạy UI sống.

Không câu nào chặn ký học.

---

## Chữ ký

Báo cáo học 1/2 (Harness). Chưa đủ để mở cổng chết động cơ.

| Vai | Tên | Kết luận | Ngày | Chữ |
|---|---|---|---|---|
| Xia (học) | agent, cửa sổ `dory` | Compare xong. Operator đã ký. Không plan. Không máy. | 2026-08-21 | đã viết |
| Operator | cửa sổ `dory` | Chấp nhận tờ này là học Harness đã ký | 2026-08-21 | **ký** |

Đã ký qua cổng xia. Học 1/2 xong. Dừng. Đừng mở `/ak:plan`. Lượt sau (khi mở cửa sổ mới): Herdr `--compare`.
