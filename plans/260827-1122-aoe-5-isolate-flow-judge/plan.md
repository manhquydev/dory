---
title: "AOE 5 isolate flow judge"
description: "New isolate script: occupant fills idea; real flow.sh gate 00-idea fail then pass. No rust. No default. Leftover 5 mint."
status: pending
priority: P1
effort: 3h
branch: main
tags: [dory, isolate, aoe5, flow, judge]
blockedBy: []
blocks: []
created: 2026-08-27
---

# AOE 5 isolate flow judge

## Contract

| Field | Closed |
|---|---|
| Outcome | Isolate: occupant viết `flow/00-idea.md` sạch; taxi `dory flow -- gate 00-idea` với **FLOW_BIN = abs flow.sh** (không `/bin/true`): lần 1 exit **1**, lần 2 exit **0**. Journal `bin` = flow.sh; sit footer `Flow 1. gate` rồi `Flow 0. gate`. Factory sock connectable=0. Leftover 5 mint. |
| Constraints | Script **mới**. Không exec/source 1910/0043/0227/0242/hop. Không rust. Không cargo leftover tree. Không start default. Không sit `t13`/`p2R`/`wP`. Không nút Flow trong Dory. Factory không viết file PASS. Stop = `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`. |
| Non-goals | Default occupancy. `prompt --wait` / `occ.report=Working`. Recook 1910/0043/0227/0242/C. Fold leftover 5. Timeout rust. Semantic `gate-rules.md`. Full 6-stage Flow project. Retarget PATH. `herdr server stop`. Close `wP`/`w15`/`t13`. |
| Acceptance | Script exit 0 hai lần độc lập (cook + test). Journal: hai `flow/result`, `bin` = abs `~/.claude/skills/flow/runner/flow.sh`, codes `1` rồi `0`, stdout có `GATE stage 00-idea`. Sit needles `Flow 1. gate` rồi `Flow 0. gate`. Leftover 5 mint. Sock connectable=0. PATH `dory` empty. Repo `.dory/` không đổi. Review critical 0. |

Nguồn: [1116 research-01](../reports/260827-1116-research-01-flow-judge.md) · [1116 research-02](../reports/260827-1116-research-02-isolate-project.md) · [1122 scout](../reports/260827-1122-scout-aoe5.md) · [1012 eval-aoe](../reports/260827-1012-eval-aoe.md) · north-star phase 5 · CHARTER hình B

## Scope Challenge

```
- Existing: 1910 true-chrome; 0242 đàn+prompt; glance paints last flow/result; C no-spawn on b544f5f; leftover 5 mint; sock dead; PATH gone
- Requested: open AOE 5 and continue the product — first paid slice: real Flow judge + project work inside isolate Dory
- Complexity: 1 new script + fixture mint + Herdr sit/flock. 0 rust. 3 phases
- Mode: HOLD (no --yagni). Full company Phase 5 (6-stage + default sit) is later, named unpaid — not a silent cut of this slice
```

## Approaches

| # | Approach | Verdict |
|---|---|---|
| **A** | New isolate script. Mint FAIL idea. Taxi `gate 00-idea` → 1. Occupant writes PASS idea. Taxi → 0. Real `FLOW_BIN`. Sit glances. | **Chọn.** |
| B | `FLOW_BIN` world-state script (not flow.sh) | Không phải hộp thẩm phán. |
| C | `doctor`/`status` + `hello.sh` | Không án. `status`/`doctor` rc=0 luôn / án máy. |
| D | Nút `next`/`card`/`check` trong Dory | **Cấm** CHARTER. |
| E | Bump timeout rust / recook glance | Không cần. Tiny fixture xong dưới 15s. 1012 cấm AOE rust. |
| F | Default sit | Unpaid. Cấm wave này. |

## Scout 11:22

- Taxi land: `git show HEAD:rust/src/flow.rs:26-102` — `DORY_ENV=1`, `FLOW_BIN`, journal `{DORY_WORKSPACE_DIR}/.dory/sessions/s1.jsonl`, return judge exit.
- Glance: `HEAD:rust/src/desk.rs:3450-3465` `Flow {n}. {arg0}` — **không** vẽ stdout. `true -- status` và `flow.sh -- status` cùng `Flow 0. status`.
- `/tmp` probe (no dory): template `gate 00-idea` rc=1; filled+checked rc=0 `clean`.
- 1910 `:308-309` `FLOW_BIN=/bin/true` — chrome, không án.
- 0242 occupants write via omp tools. `comm_allowlisted` = classify, không khóa Write.
- SIT_DORY = `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` (C ELF). Stat only until cook.

## Bẫy

1. Gọi 1910 `/bin/true` hoặc `status`/`doctor` rc=0 là AOE 5 — **cấm**. Án = `gate 00-idea` fail **rồi** pass.
2. Factory viết file PASS rồi taxi 2 — **cấm**. Occupant (omp tools) mới được lật `00-idea.md`.
3. Exec/source 1910/0043/0227/0242/hop — **cấm**. Copy law, file mới `scripts/dory-isolate-aoe5-flow-judge.sh`.
4. `PROJECT=$ISO_REAL` (không `hello/`). Glance `{world.cwd}/.dory/sessions/s1.jsonl` (`desk.rs:2729-2733`). Taxi journal = `DORY_WORKSPACE_DIR` (`flow.rs:105-118`). Pin `FLOW_PROJECT_ROOT=$ISO_REAL` trên **dòng taxi** (override inherit). Refuse factory `FLOW_PROJECT_ROOT`/`FLOW_BIN` lúc vào. Ancestor-walk `~/Downloads/flow` từ `~/.cache/dory-isolates/` **không** xảy ra (không cùng ancestor) — đừng lấy đó làm án.
5. Pin `FLOW_BIN=/home/manhquy/.claude/skills/flow/runner/flow.sh` sau `realpath`; basename phải `flow.sh`; `-x`. Refuse factory `FLOW_BIN` nếu set. Taxi 1 FAIL nếu journal thiếu `flow/result` mới hoặc stdout không có `GATE stage 00-idea` (herdr/dsh refuse cũng rc=1 — không tính án).
6. Sit pane/tab **exact**: ≠ `w13:t13` ≠ `w13:p2R` ≠ `*wP:*`. `herdr pane get` `tab_id` khớp `SIT_TAB`. Không `[ = t13 ]`. Close only wave tabs. Không `herdr server stop`.
7. Stop = copy 1910 `compound_stop` (`:69-100`): `iso_identity_ok` + sock tồn tại / không symlink / realpath=`ISO_SOCK` / dưới `ISO_REAL` **rồi** `XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET "$SIT_DORY" server stop`. Identity fail → abort, không stop. Cấm `iso()` / `DORY_SOCKET=`.
8. Factory `dory` / leftover ELF / isolate ELF trên factory XDG. Mọi dory = `"$SIT_DORY"` với isolate env.
9. Cargo `/home/manhquy/Downloads/flow/dory`. Fold leftover 5. `git add -u` / `commit -a`.
10. `prompt --wait` / `occ.report = Working` (0242 trap 10).
11. `herdr pane run` attach (TUI không exit). Attach = `send-text` + `enter` + `wait-output`. Pane id **trước** option (Herdr 0.7.5).
12. Claim “AOE 5 company xong” / default trống “đã sâu”. Slice này = isolate first slice. Default + 6-stage + semantic vẫn unpaid.
13. `assess` / `next` / `card` / PASS-path `check` (mutate harness) — không gọi.
14. Repo `/home/manhquy/Downloads/flow/dory/.dory` create/mtime/ino đổi = FAIL.
15. Factory `DORY_*` / `PI_CODING_AGENT_DIR` set lúc vào script = refuse. Isolate **server only** được `PI_CODING_AGENT_DIR=$FACTORY_HOME/.omp/agent`.
16. Không `export HOME` script-scope. Entry: `HOME` == `FACTORY_HOME`. Server + taxi prefix only: `HOME="$ISO_REAL/home"` (0242 `:348`, không `$ISO/home`).
17. Không rust hunk. Không isolate `reset --hard`. Không cargo leftover. Isolate ELF đã có — **không** cargo trừ ELF missing (mặc định: FAIL, không rebuild).
18. Sock probe = AF_UNIX `…/dory/default/dory.sock`. Không probe thư mục session.
19. Attach = 1910 `:331` verbatim: `cd "$ISO_REAL" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE "$SIT_DORY" attach`. Thiếu isolate XDG → ping factory sock chết (C no-spawn) hoặc leftover ELF đẻ default.
20. Server start = 0242 `:346-353` `setsid` + `XDG_RUNTIME_DIR="$ISO_REAL"` + `HOME="$ISO_REAL/home"` + `PI_CODING_AGENT_DIR` **chỉ** dòng đó. Sau bind: isolate sock connectable **và** factory_must_dead.
21. Taxi 2 **không** chạy ngay sau `agent prompt` (prompt xong khi coord idle, không khi file sạch). Poll `$ISO_REAL/flow/00-idea.md` khớp PASS bytes (timeout ~180s). Rồi mới taxi 2. Cấm `--wait`.
22. Journal đúng **hai** `flow/result`, codes `[1,0]`, cùng `bin`. Copy journal → receipt **trước** `wipe_iso`.
23. Leftover mint **bảng** (không “snap rồi khớp snap”):
    README `68190a5ffa073c082aa318aad5ed032e13cc90e3`
    attach `602479094e84d31ad6f017775a3d55aeb485c644`
    main `373d688636ff7315ccd665f450069d8284eb47ff`
    server `4de1554ad56e248cdcf42f02111b7389b08dae82`
    p5_attach `9c28fc3e0f3666498a8952411242d5301f7911de`
    `desk.rs` worktree == `HEAD:rust/src/desk.rs` `4c788562e4fdda10c8edd2878ed1fdd46050c218`
24. Copy-law **cấm** 1910 taxi `:308-309` (`FLOW_BIN=/bin/true`). Script `case $0` refuse paid names; `rg` self for `source`/`exec` những script đó.
25. Ship = `git add --` pathspec. **Không** `git add -A` / `ak:git` nếu nó add -A. `rv_left` ≠ “worktree rust clean” (leftover 5 **là** dirty rust). `git log -1 -- rust/` vẫn `b544f5f`.
26. Commit subject không pretends company Phase 5. Dùng `feat(isolate): fail-then-pass flow.sh gate`.

## Herdr

Sau gate sạch: tab mới trên `w13`, `--no-focus`, cwd `/home/manhquy/Downloads/flow/dory`. Mỗi tab 4–6 OMP. Không split `t13`. Không ngồi factory. Close only wave tabs (kể cả sit tab script mint).

| Tab | Job |
|---|---|
| sit | Shell sạch. `SIT_PANE`/`SIT_TAB`. **Không** `agent start`. |
| cook | Script + chạy 1 lần + cook receipt |
| test | Chạy lại độc lập + test receipt |
| review | Judge / leftover / sit-door / fold |
| ship | Pathspec script+plan+reports. Never leftover 5. Không push. |

OMP: `herdr agent start NAME --kind omp --pane ID --timeout 180000 -- --no-session --no-skills --no-rules --no-extensions` trừ khi skill ON được ghi trong prompt (ship = ak:git).

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [New isolate AOE 5 script](./phase-01-start.md) | Pending |
| 2 | [Independent AOE 5 test](./phase-02-independent-aoe-5-test.md) | Pending |
| 3 | [Review and ship](./phase-03-review-and-ship.md) | Pending |

## Success Criteria

- [ ] New script `scripts/dory-isolate-aoe5-flow-judge.sh` exit 0; does not exec paid scripts
- [ ] Taxi 1: exit 1, journal `bin` = abs flow.sh, stdout `GATE stage 00-idea` + unchecked/FILL
- [ ] Occupant (not factory) writes PASS `00-idea.md`
- [ ] Taxi 2: exit 0, same `bin`, stdout `clean`
- [ ] Sit visible `Flow 1. gate` then `Flow 0. gate`
- [ ] Leftover 5 `git hash-object` = mint; sock connectable=0; PATH `dory` empty
- [ ] No rust diff; no leftover-tree cargo; no factory `dory` argv
- [ ] Repo `.dory/` unchanged

## Remainder (named unpaid — not this cook)

- Default occupancy / sit default
- Founder `prompt --wait` / five states `working`/`blocked`
- Full 6-stage Flow project + semantic gate
- Leftover 5 fold / PATH retarget
- Isolate unlock `flow -- next` → sibling plan `plans/260827-1657-isolate-flow-next-unlock/` (not this cook)

## Red Team Review

### Round 1 — 2026-08-27 specialized (security / failure / fold)

| # | Finding | Sev | Disposition |
|---|---|---|---|
| S1 | Attach thiếu isolate XDG | Critical | **Accept** — 1910 `:331` verbatim |
| S2 | Stop one-liner không identity | Critical | **Accept** — 1910 `compound_stop` `:69-100` |
| S3 | Server start thiếu `XDG_RUNTIME_DIR` | Critical | **Accept** — 0242 `:346-353` |
| S4 | Ancestor-walk Downloads/flow sai địa lý; `FLOW_*` inherit | High | **Accept paper** — pin `FLOW_PROJECT_ROOT` trên taxi; refuse factory `FLOW_*` |
| S5 | `FLOW_BIN=herdr` executable → taxi1 rc=1 giả | High | **Accept** — pin abs `flow.sh`; journal GATE bắt buộc |
| S6 | Refuse `t13` không khớp `w13:t13` | High | **Accept** |
| S7 | `HOME=$ISO/home` vs `$ISO_REAL/home` | High | **Accept** |
| S8 | PI_CODING leak `~/.omp` | High | **Accept paper** — 0242 prompt ban `~/.omp`/`agent.db`. Không FAIL hard nếu `~/.omp` mtime (wP đang omp) |
| S9 | Leftover snap không nêu path | Medium | **Accept** — bảng mint trap 23 |
| F1 | Taxi 2 ngay sau prompt (coord idle ≠ file sạch) | Critical | **Accept** — poll PASS bytes |
| F2 | Attach XDG | Critical | **Accept** (S1) |
| F3 | Factory có thể viết PASS | High | **Accept** — poll file; factory không Write |
| F4 | Wipe xóa journal trước test đọc | High | **Accept** — copy journal trước wipe |
| F5 | first/last ≠ đúng hai result | High | **Accept** — count==2 codes `[1,0]` |
| F6 | Needle `Flow 0. gate` = true chrome | High | **Accept paper** — journal `bin` là bằng; sit phụ |
| F7 | Vá F1 bằng `--wait` | High | **Reject cook** — cấm `--wait` |
| F8 | Không `rg` source paid scripts | High | **Accept** |
| F9 | Ancestor-walk /hello leftover | Medium | **Accept** (S4) |
| L1 | Mint không có bảng SHA | Critical | **Accept** |
| L2 | `rv_left` no rust diff vs b544f5f | Critical | **Accept** — leftover 5 được dirty; land rust commit vẫn `b544f5f` |
| L3 | `ak:git` = `add -A` | Critical | **Accept** — pathspec `git add --` only |
| L4 | Isolate cargo door | High | **Accept** — ELF có thì không cargo |
| L5 | Copy-law gồm 1910 `:308` true | High | **Accept** — exclude taxi chrome |
| L6 | p5 flow-skill drift | High | **Accept** — pin skill `flow.sh` basename |
| L7 | Subject pretends company AOE 5 | High | **Accept** — đổi subject |
| L8 | Recook paid nếu 2–5 đứng | High | **Accept paper** — refuse exec |

Rejected as cook: `--wait`, rust timeout, default sit, factory-write PASS, `git add -A`, `rv_left` = worktree rust clean, FAIL-on-`~/.omp` mtime.

### Whole-Plan Consistency Sweep

- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas: PROJECT=ISO_REAL; attach 1910; stop compound_stop; server 0242 setsid; poll PASS file; journal copy-before-wipe; leftover SHA table; ship pathspec; subject isolate slice
- Unresolved contradictions: 0
- Reconciled stale: `$ISO/home`, `hello/` journal, refuse bare `t13`, `rv_left` rust-clean, `feat(isolate): AOE 5`

## Validation Log

### Session 1 — 2026-08-27
**Trigger:** User opened AOE 5 and ordered continue product (plan → red-team → validate → cook). Settled by 1116 research + R1.
**Questions asked:** 0 live (no material fork left)

#### Questions & Answers

1. **[Architecture]** Judge verb?
   - Options: `gate 00-idea` fail-then-pass (Recommended) | `doctor`/`status` | custom world-state bin
   - **Answer:** `gate 00-idea` fail-then-pass
   - **Rationale:** research-01: only `gate`/`check` fail-path is án; doctor/status not judge.

2. **[Scope]** Company Phase 5 vs isolate first slice?
   - Options: isolate first slice (Recommended) | default sit + 6-stage now
   - **Answer:** isolate first slice
   - **Rationale:** HOLD. Remainder named unpaid. Subject must not claim company.

3. **[Risks]** Taxi 2 timing?
   - Options: poll PASS file bytes (Recommended) | `prompt --wait` | factory Write PASS
   - **Answer:** poll file
   - **Rationale:** F1/F7. `--wait` = 0242 trap. Factory-write = fake AOE 5.

#### Confirmed Decisions
- A = new script, `PROJECT=$ISO_REAL`, real skill `flow.sh`, occupant PASS bytes, journal `[1,0]`
- Copy 1910 attach+stop, 0242 server setsid; never 1910 taxi true
- No rust, no default, leftover mint table
- Ship pathspec only; subject `feat(isolate): fail-then-pass flow.sh gate`

#### Action Items
- None remaining on the plan

#### Verification Results
- **Tier:** Standard (3 phases)
- **Claims checked:** 10
- **Verified:** 10 | **Failed:** 0 | **Unverified:** 0
- Taxi `flow.rs:26-102` — VERIFIED
- Glance `desk.rs:3450-3465` + journal path `:2729-2733` — VERIFIED
- C no-spawn `attach.rs:326-332` — VERIFIED
- `forbidden_name` `flow.rs:132-146` — VERIFIED
- `comm_allowlisted` HEAD `server.rs:1208-1209` — VERIFIED
- 1910 taxi true `:308-309` + attach `:331` + stop `:69-100` — VERIFIED
- 0242 server `:346-353` — VERIFIED
- `/tmp` gate fail-then-pass — VERIFIED this session
- Leftover 5 mint live MATCH — VERIFIED prior measure
- `ak plan validate` format valid — VERIFIED

#### Whole-Plan Consistency Sweep
- Files reread: plan.md, phase-01, phase-02, phase-03
- Decision deltas checked: 12
- Reconciled stale references: `$ISO/home` in phase-01 Architecture
- Unresolved contradictions: 0
