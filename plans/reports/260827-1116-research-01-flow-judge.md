---
type: research
date: 2026-08-27
time: 11:16
researcher: researcher-01
kind: aoe-5-flow-judge
head: ee0350cf
desk_rs: 4c788562 (HEAD == worktree)
scope: what must be true for Flow to judge inside Dory vs 1910 FLOW_BIN=/bin/true chrome
---

# AOE 5 — Flow judge vs 1910 chrome

Land citations = `git show HEAD:path:line` (blob). Không cite leftover worktree `README.md` / `attach.rs` / `main.rs` / `server.rs` / `p5_attach.rs`.

`desk.rs` worktree hash `4c788562` == `HEAD:rust/src/desk.rs`. `flow.rs` cited from HEAD blob.

1012 eval HEAD was `53f3cff` / rust `5a60953`. This pane HEAD `ee0350cf`; last rust `b544f5f` (`fix(attach): do not auto-start server on sit`). Taxi + glance contract below unchanged.

## 1. Executive summary

**AOE 5 judge = mechanical `flow.sh` (or that same bin via `FLOW_BIN`) chạy trên một project tree, exit 0/1 theo file, journal `flow/result` ghi bin + argv + code + stdout.** Dory chỉ taxi. Dory không phải thẩm phán.

**AOE 5 judge không phải:** footer có chữ `Flow`. Taxi `dory flow --`. Isolate sit. `/bin/true` exit 0. `status` luôn 0. Một project “xong trong Dory”. Semantic layer (agent đọc `gate-rules.md`). Default sock. Nút `next`/`card`/`check` trong Rust.

North star phase 5: *“A real project is completed inside Dory; Flow judges; zero herdr/dsh on the runtime path.”* (`plans/reports/260822-north-star-aoe.md:33`). Đó là **công ty**. First increment chỉ trả **một** điều kiện cần: taxi mang **án cơ học thật**, isolate, không default occupancy.

Hai lớp Flow (`~/.claude/skills/flow/SKILL.md`):

| Lớp | Ai | Dory taxi được? |
|---|---|---|
| Mechanical | `flow.sh` exit 0/1 | Có. Đây là án Dory có thể chiếu. |
| Semantic | agent + `gate-rules.md` | Không. Cần occupant trong pane. Không phải slice 1. |

1910 đã trả **chrome**: `FLOW_BIN=/bin/true` → footer `Flow 0. sit-TOKEN`. 1012: đó **không** phải Flow judge (`plans/reports/260827-1012-eval-aoe.md:29,52`).

**Phải đúng (tối thiểu) để gọi là judge:**

1. `DORY_ENV=1` (không thì taxi chết trước khi exec).
2. `FLOW_BIN` = file `flow.sh` thật (abs path), **không** `/bin/true`.
3. cwd / `DORY_WORKSPACE_DIR` = project (hoặc inherit `FLOW_PROJECT_ROOT`). Taxi **không** set `FLOW_PROJECT_ROOT`.
4. argv là verb **án** (`gate` / `check` fail-path), không token giả, không chỉ `status`.
5. Journal `{cwd}/.dory/sessions/s1.jsonl`: `flow/invoke` + `flow/result`; `bin` = flow.sh; `code` theo file; `stdout` có `GATE`/`FAIL`/`PASS`.
6. Xong < 15s (hardcoded). Tiny fixture: có.
7. Không `herdr`/`dsh` trên **Dory** runtime (factory Herdr ngồi ngoài = hai ghế, không tính AOE 5).

**Khuyến nghị slice 1:** option **(A)** — isolate taxi `flow.sh gate 00-idea` (hoặc `check C-001` fail) trên fixture nhỏ. Journal là bằng. Không rust timeout. Không default. Không nút Flow trong Dory.

## 2. Taxi contract (HEAD)

Dory gọi flow.sh như máy lạ. Không bao giờ ngược (`CHARTER.md:30-32`). Skill: taxi only; cấm mọc cổng trong Dory (`skills/dory/SKILL.md:48,160,166`).

### argv

```
dory flow -- <args>
```

- Thiếu `--` → usage, exit 2 (`HEAD:rust/src/flow.rs:26-29`).
- Sau `--` rỗng → default `status` (`HEAD:rust/src/flow.rs:35-37`).
- Không parse `next`/`card`/`check`. Comment đất: *“No gate logic. No next/card/check.”* (`HEAD:rust/src/flow.rs:1-3`).
- USAGE: `dory flow -- <args>` (`HEAD:rust/src/main.rs:47`). Mutating group gồm `flow` cần `DORY_ENV=1` (`HEAD:rust/src/main.rs:49`).

### env

| Var | Bắt buộc? | Việc |
|---|---|---|
| `DORY_ENV=1` | Có | `require_skill_env` — khác `1` → stderr envelope *“I am not running inside a Dory-managed pane”*, exit 1 (`HEAD:rust/src/main.rs:542-550`). Taxi không chạy. |
| `FLOW_BIN` | Không | Nonempty → exec path đó. Rỗng/absent → `"flow.sh"` (`HEAD:rust/src/flow.rs:39-45,121-125`). |
| `DORY_WORKSPACE_DIR` | Không | Thắng cwd. Rỗng/absent → `current_dir` (`HEAD:rust/src/flow.rs:105-113`). |
| `FLOW_PROJECT_ROOT` | Không (flow.sh) | Taxi **không** inject. Child inherit nếu caller export. `flow.sh` ROOT = var này, else `$PWD` (+ ancestor walk nếu CWD không có `flow/`/`cards/`). Isolate phải `cd` project hoặc export var. |
| `DORY_SOCKET` / `XDG_RUNTIME_DIR` | Isolate | Taxi không đọc. 1910 set để RPC isolate, không phải án. |

### journal

- Path: `{cwd}/.dory/sessions/s1.jsonl` — `SESSION_ID` hardcoded `"s1"` (`HEAD:rust/src/flow.rs:16,115-118`). Desk cùng path (`HEAD:rust/src/desk.rs:2729-2733`).
- Trước exec: `flow/invoke` (bin, args, cwd). Fail ghi journal → exit 1, không exec (`HEAD:rust/src/flow.rs:67-85`).
- Sau: `flow/result` + `code` / `signal` / `stdout` / `stderr` / `error` (`HEAD:rust/src/flow.rs:87-100`).
- Taxi `println` envelope success của event; **return `result.code.unwrap_or(1)`** — giữ exit án (`HEAD:rust/src/flow.rs:101-102`). Skill: *“Preserve the judge exit code.”*

### timeout

- `DEFAULT_TIMEOUT = 15_000ms` **cứng**. Không `FLOW_TIMEOUT`. (`HEAD:rust/src/flow.rs:14,87`)
- Quá hạn: SIGTERM, grace 1s, SIGKILL. `code=null`, `signal=SIGTERM`, `error="timed out after 15000ms"` (`HEAD:rust/src/flow.rs:15,199-211`).
- stdin null; stdout/stderr piped, nhét journal (`HEAD:rust/src/flow.rs:162-167`).

### cấm herdr / dsh

`forbidden_name` trên **bin và mọi argv token**: basename `herdr` / `herdr.exe` / `dsh` / `dsh.exe`, hoặc token chứa `@deepseek-ai/dsh` (`HEAD:rust/src/flow.rs:46-59,132-146`). Charter giết hàng nếu runtime gọi `dsh`/`herdr` (`CHARTER.md:43-44,61`).

1910 taxi sống (`scripts/dory-isolate-flow-sit.sh:308-309`):

```
(cd "$WS" && DORY_SOCKET="$ISO_SOCK" DORY_ENV=1 DORY_WORKSPACE_DIR="$WS" FLOW_BIN=/bin/true \
  "$SIT_DORY" flow -- "$FLOW_TOKEN")
```

Đó là contract **chrome**: true + token giả + `code==0`. Không đọc project.

## 3. `/bin/true` không phải thẩm phán (1910)

`true` không mở file. Mọi argv → exit 0, stdout rỗng. Taxi vẫn journal + footer.

| Bằng | Nói gì |
|---|---|
| 1910 plan `:79` | `FLOW_BIN=/bin/true`. Needle `Flow 0. $FLOW_TOKEN` — **cố ý** không phải `Flow 0.` trần. |
| 1910 cook `:35-36,60` | `FLOW_TOKEN=sit-1447913329`, visible `Flow 0. sit-1447913329`, `JOURNAL_OK=1`. |
| Script `:308-325` | Assert `flow/result` `code==0` thôi. Không assert stdout án. |
| 1012 `:29,52,73` | *1910 `FLOW_BIN=/bin/true` ≠ Flow judge.* Cấm gọi true là AOE 5. |
| p5 land tests | `p5_inside.rs:19-21,387` / `p5_real_repo.rs:490` / `p5_live_loop.rs:393`: `assert_ne!(FLOW_BIN, "/bin/true")`. Đó là **cargo P5**, không phải isolate AOE 5 receipt. Bin của họ = bản `flow-skill` **khác** skill `~/.claude`. |

This-turn (không exec dory): `/bin/true check C-001` → rc=0, stdout rỗng. `flow.sh check C-001` trên card mỏng → rc=1, stdout liệt kê thiếu deps/sections. Cùng argv, án ngược. True **không thể** thua.

Glance **không** vẽ stdout. `true` + `dory flow -- status` và `flow.sh status` **cùng** footer `Flow 0. status`. Chrome 1910 dùng token (`sit-…`) để needle unique — đó là identity, không phải án.

## 4. Options ranked

| Rank | Option | Judge? | Footer ≠ 1910? | Rust cook? | Fit slice 1 |
|---|---|---|---|---|---|
| **1** | **(A) `flow.sh` verb nhanh** | Có nếu `gate`/`check` (fail). `status`/`doctor` = báo cáo, không án. | Có nếu code≠0 (`Flow 1. gate`). `status` thì **trùng** true. | Không | **Chọn.** Isolate + abs `FLOW_BIN`. |
| 2 | **(B) script world-state exit 0/1** | Án file, **không** phải Flow. Charter: thẩm phán = `flow-skill`. | Có nếu code≠0 | Không | Chỉ khi isolate **không** có `flow.sh`. Đừng gọi AOE 5. |
| 3 | **(C) bump timeout rust** | Không. Mở cửa `eval`/cây lớn. Tiny đã <200ms. | — | **Có** — 1012 cấm cook AOE rust | Không. Timeout cứng đủ. |
| 4 | **(D) nút next/card/check trong Dory** | **Cấm.** Đảo mũi tên. | — | Có + giết charter | **Forbidden.** |

### (A) verb nào

| Verb | <15s tiny? | Exit = án project? | Side effect | Slice 1? |
|---|---|---|---|---|
| `status` (default) | 45–83ms, rc=0 **luôn** | Không. Gate blocked vẫn 0. | Telemetry nhẹ | Chrome thật (stdout journal), **không** án. |
| `resume` | 47ms, rc=0 | Không | Đọc | Brief, không án. |
| `doctor` | 62ms | Án **máy**, không project | Không | Không. |
| `gate <stage>` | 22ms, rc=1 trên box trống | **Có.** 0 clean / 1 findings. Read-only, không unlock, không durable (`flow.sh` `cmd_gate`). | Không | **Best.** |
| `check C-NNN` noarg/missing | 17–18ms, rc=1 | Usage / not-found — yếu. | Không | Quá mỏng. |
| `check` card mỏng | 36ms, rc=1 | **Có.** FILL/status/sections. | Không (fail path) | OK, kém `gate` một bậc. |
| `check` card đủ hình todo | 163ms, rc=0 | Có. PASS path gọi `_graph_record` + harness story. | **Có** | Tránh slice 1. |
| `assess` lần 1 | 64ms, rc=0 | **Không.** Scaffold `00-inspect.md` rồi PASS. | **Mutate** | Cấm nhầm “án”. |
| `next` / `card` | không đo | Unlock / mint | **Mutate** | Không. Taxi được; 1012 + skill cấm mọc cổng; slice 1 đừng gọi. |
| `eval` | LLM, chậm | Semantic | Billable | Timeout 15s + (C). Không. |

### Trade-off

| | Perf | Complexity | Maintenance | Cost | Charter fit |
|---|---|---|---|---|---|
| A | <0.2s tiny | Thấp: đổi `FLOW_BIN` + argv + fixture | Skill `flow.sh` ≠ `flow-skill` copy (cmp DIFFERENT). Pin abs path. | Isolate script only | Đúng mũi tên |
| B | <10ms | Thấp | Script Dory giả Flow | Rẻ | Sai hộp thẩm phán |
| C | Mở chậm | Đụng land timeout | Lịch rust | Cook | Không cần |
| D | — | Cao | Gộp hộp | Đắt + kill | **Cấm** |

### Adoption / risk

- `flow.sh` зрелый (v0.25 skill). Mechanical ổn định. Hai bản trên disk: Claude skill 205886 (22/8) vs `flow-skill` 255864 (20/8). p5 pin bản cũ. Slice 1 pin `~/.claude/skills/flow/runner/flow.sh`.
- Breaking: verb `gate` là án read-only — đúng ý graph executor. Đừng phụ thuộc PASS-path `check` (harness).
- Abandon: flow-skill freeze 6–12 tháng (`CAPACITY-FREEZE.md`) — mechanical không chết.
- (C) timeout bump = land + test glance `15000ms` (`HEAD:rust/src/desk.rs:5035-5042`). Đắt, không trả AOE 5.
- (D) kill: `CHARTER.md:61`, `skills/dory/SKILL.md:166`, `HEAD:rust/src/flow.rs:3`.

## 5. Timeout evidence (this-turn, không dory)

Bin: `/home/manhquy/.claude/skills/flow/runner/flow.sh`.  
Root: `/tmp/dory-aoe5-flow-judge-1823056` (`FLOW_PROJECT_ROOT`, `FLOW_LOG_DISABLE=1`, `DO_NOT_TRACK=1`).  
Không ancestor-adopt `~/Downloads/flow`. Không factory XDG. Không ELF.

Fixture: `flow/00-idea.md` (`- [ ] box`), `cards/C-001.md` mỏng, `cards/C-002.md` đủ section todo.

| Cmd | rc | ms | Ý |
|---|---|---|---|
| `status` tiny | 0 | 82.8 | `NEXT -> fix gate: unchecked…` — báo cáo, không thua. |
| `status` empty | 0 | 44.6 | `planning: not started` — vẫn 0. |
| `resume` tiny | 0 | 47.4 | `gate: BLOCKED` — vẫn 0. |
| `doctor` | 0 | 62.3 | Env. |
| `check` (noarg) | 1 | 16.9 | usage. |
| `check C-999` | 1 | 18.4 | not found. |
| `check C-001` | 1 | 36.0 | **án** — thiếu deps + 5 section. |
| `check C-002` | 0 | 162.6 | PASS todo + harness. Vẫn ≪15s. |
| `gate 00-idea` | 1 | 21.8 | **án** — unchecked L2. |
| `assess` empty | 0 | 63.6 | **scaffold** `00-inspect.md`. Không phải án. |
| `/bin/true status` | 0 | 3.4 | stdout rỗng. |
| `/bin/true check C-001` | 0 | 2.6 | stdout rỗng. **Ngang hàng 1910.** |

**Kết:** `status` / `check` / `gate` / `doctor` / `resume` / `assess` trên fixture nhỏ **xong <15s**. (C) không cần. `eval` / cây lớn / semantic — ngoài slice.

`status` cwd=`/tmp` không `FLOW_PROJECT_ROOT`: 47ms, project=`/tmp` (không adopt Downloads/flow). Isolate vẫn phải chỉ ROOT — đừng taxi từ cwd lạ.

## 6. Footer / glance

Desk đọc **last** `flow/result` trên `{world.cwd}/.dory/sessions/s1.jsonl`. Không đọc stdout án.

`flow_glance_line` (`HEAD:rust/src/desk.rs:3450-3465`):

- `error` nonempty → `Flow lỗi. {err}`
- else `code=n` + arg0 → `Flow {n}. {arg0}`
- `code=n` + arg0 rỗng → `Flow {n}.`
- no code, no error → `Flow.` / `Flow. {arg0}`

arg0 = `args[0]` (depth-1). 1910 token = arg0.

`footer_line` Terminal + status rỗng → vẽ glance; overlay (Confirm/Prefix/Help/…) thắng (`HEAD:rust/src/desk.rs:3416-3424,5146-5180`).

| Taxi | Footer (Terminal, no overlay) | Journal stdout |
|---|---|---|
| 1910 `true -- sit-TOKEN` | `Flow 0. sit-TOKEN` (cook needle) | rỗng |
| `true -- status` | `Flow 0. status` | rỗng |
| `flow.sh -- status` (mọi fixture đo) | **`Flow 0. status`** — **trùng true** | `flow status` + NEXT/gate |
| `flow.sh -- gate 00-idea` (box trống) | `Flow 1. gate` | `GATE stage 00-idea:` + `[x]` |
| `flow.sh -- check C-001` (mỏng) | `Flow 1. check` | `FAIL: C-001 has gate violations` |
| `true -- check C-001` | `Flow 0. check` | rỗng |
| timeout | `Flow lỗi. timed out after 15000ms` (`HEAD:rust/src/desk.rs:5042`) | — |
| spawn miss / không `DORY_ENV` | không `flow/result` (env) hoặc `Flow lỗi. {io}` | — |

**Mắt founder:** `Flow 1. gate` / `Flow 1. check` ≠ 1910 `Flow 0. sit-*`.  
**Bằng kỹ thuật:** journal `bin` + `code` + `stdout`. Sit glance là slice sau — 1910 đã trả sơn footer.

## 7. Do-not

- Start `/run/user/1000/dory/default`. Sit `w13:t13` / `p2R` / `wP` / `w15`.
- Invoke factory `dory` / `dory attach` / `dory server`. Exec leftover ELF hoặc isolate ELF trên factory XDG.
- Recook / exec 1910 / 0043 / 0227 / 0242. Fold leftover 5.
- Cite leftover worktree blobs như land.
- Gọi 1910 `/bin/true` là AOE 5. Gọi `status` rc=0 là án. Gọi `assess` lần 1 là án.
- Option (D): mọc nút / gate `next`/`card`/`check` trong Dory.
- Option (C) slice 1. Cook rust AOE. Đụng `desk.rs` glance.
- `herdr`/`dsh` trên **Dory** runtime. (Factory Herdr ngồi = hai ghế, không phải AOE 5.)
- `FLOW_BIN=herdr|dsh`. Token argv cấm.
- Taxi từ cwd lạ không `FLOW_PROJECT_ROOT` / không `DORY_WORKSPACE_DIR`=project.
- Pin p5 `flow-skill/.../flow.sh` (khác skill). `eval` / semantic / “project completed”.
- Default occupancy. Claim full phase 5.

## 8. Recommended — first AOE 5 increment

**(A) isolate-only taxi, `FLOW_BIN` = abs `~/.claude/skills/flow/runner/flow.sh`, argv `gate 00-idea` (fallback `check C-001` trên card mỏng).**

Một lát:

1. Isolate XDG + binary isolate (không factory sock). Không attach default. Không bắt buộc sit — journal đủ.
2. Trong `DORY_WORKSPACE_DIR`: fixture `flow/00-idea.md` có `- [ ]` (copy pattern /tmp, không đụng repo).
3. `DORY_ENV=1` `FLOW_BIN=<abs flow.sh>` `"$ISO_DORY" flow -- gate 00-idea`.
4. Pass khi **cả bốn** đúng:
   - taxi exit **1** (true không làm được),
   - journal `bin` = abs flow.sh, không `/bin/true`,
   - `args` chứa `gate`,
   - `stdout` chứa `GATE stage 00-idea` + unchecked.
5. Factory sock vẫn chết. Repo `.dory/` không đổi. Không cargo leftover. Không recook 1910 body.

Không (B) trừ isolate thiếu `flow.sh`. Không (C). Không (D). Không default flock. Không ngồi w13 trừ khi founder muốn **mắt** `Flow 1. gate` — đó là chrome đã có, không phải điều kiện án.

Sau slice này AOE 5 vẫn **unpaid** (project thật xong trong Dory + semantic). Slice trả: *taxi mang án Flow, không phải true.*

## Methodology

- Sources: HEAD `flow.rs`/`desk.rs`/`main.rs`; `CHARTER.md`; north-star; `CAPACITY-FREEZE.md`; 1012; 1910 plan/cook/script; `skills/dory/SKILL.md`; `~/.claude/skills/flow/{SKILL.md,runner/flow.sh}`; p5 foreign-judge tests; timing /tmp 2026-08-27.
- Không web. Không factory dory. Không ELF. `assess` chỉ trên `/tmp/.../empty`.
- Credibility: land + charter > eval receipts > p5 tests (bin lệch) > this-turn timing (primary <15s).

## Unresolved

1. HEAD giờ `ee0350cf` / rust `b544f5f` (attach no auto-start). 1012 table cổ. Không re-gate AOE 0–4.
2. Sit `Flow 1. gate` — founder muốn mắt ở increment 1 hay journal-only?
3. Fixture đặt trong ISO home hay bind-mount project ngoài? Taxi không set `FLOW_PROJECT_ROOT`.
4. Semantic AOE 5 (agent trong pane) — ngoài scope.
5. `flow-skill` vs `~/.claude/skills/flow` drift — p5 còn pin bản cũ.

## Next (không làm ở pane này)

Một isolate script **mới** (không recook 1910). Pin abs `FLOW_BIN`. `gate 00-idea`. Bốn assert §8. Default sock chết.
