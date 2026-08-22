---
type: brainstorm
date: 2026-08-23
time: 00:09
status: locked
feeds: plans/260823-close-coding-occupancy
authority:
  - CHARTER.md
  - plans/reports/260822-skill-cli-socket-contract.md
  - plans/reports/260822-1942-brainstorm-occupant-lock.md
  - plans/reports/260822-2011-layer4b-omp-factory.md
  - plans/reports/260822-2031-layer4d-omp-factory.md
  - plans/reports/260822-2022-layer4c-prompt-paste-brief.md
  - plans/reports/260822-2038-layer4e-prompt-stall-brief.md
---

# Brainstorm — phạm vi nấu tiếp (occupancy, không §11)

## Contract (reused — không mở lại)

| Field | Closed |
|---|---|
| Outcome (sản phẩm) | Occupant trong pane (`DORY_ENV=1`) điều phối chỗ làm; Flow taxi |
| Constraints | Ba nhà; không `herdr`/`dsh` runtime; không `--kind` farm; không Xia `--copy`; classifier `sleep\|cat\|sh\|bash\|true\|false` |
| Non-goals (vẫn) | Binary coding Dory; allowlist `omp`; TUI-as-identity; Flow gates trong Dory |
| Acceptance (sản phẩm) | Contract §11 — **không** phải cổng của slice này |

## Slice này (mới khóa)

| Field | Closed |
|---|---|
| Outcome | Nhà máy: `dory agent start coder --pane <id> -- omp --no-session` rồi `prompt` + `wait` đóng `idle\|done` vì occupant **mở skill và `report`**. Suite `cargo test --offline --locked` vẫn xanh; không test exec `omp`. |
| Constraints | Isolated `XDG_RUNTIME_DIR`; 4c BP + 4e no-stall-on-unknown giữ; không lật phase markdown của `260822-0847-workplace-skill-mux` từ giấy; không nấu 1a spec-kit |
| Non-goals | §11 Flow trên repo ngoài; fill §5 (`tab get`, `pane layout\|focus\|…`); `DORY_WORKSPACE_DIR` trừ khi factory chứng thiếu cwd; Node `/workplace` |
| Acceptance | Journal factory `status: pass` từ file bằng chứng (start `unknown`, wait `idle\|done`, transcript có phrase skill hoặc lệnh report). Nếu fail: journal ghi **một** nguyên nhân (submit / skill-load / occupant từ chối), không đoán ba thứ một lúc. |

HOLD SCOPE. Không `--yagni`.

## Evidence (hiện tại, không ý định)

| Mảnh | Trạng thái |
|---|---|
| Lớp 3 `report` + lớp 4 `occ_skill` | Ship. CI. |
| 4c live bracketed-paste | Ship. `p5_prompt_paste`. `agent_prompt` bọc khi drain có `CSI ? 2004 h`. |
| 4e stall | Ship. `unknown` không `agent_prompt_stalled`. `p5_prompt_unknown`. |
| Factory 4b / 4d | **FAIL.** 4d: start `unknown` (đúng); prompt `agent_prompt_stalled` (trước 4e); wait timeout; occupant không report. |
| Spawn env | `DORY_ENV`, `DORY_SOCKET`, `DORY_BIN`, workspace/tab/pane. **Không** `DORY_SKILL` / `DORY_WORKSPACE_DIR`. |
| Plan `260822-0847-workplace-skill-mux` | `status: pending`; phase 2–6 giấy `todo`. Không lật. |

4e **chưa** được factory-retry. Không được coi 4d stall là bằng chứng omp “không chịu report” sau 4e.

## Ba hướng (slice kế)

| | Cách | Giả định nặng | Gãy trước |
|---|---|---|---|
| **A** | Chỉ factory 4f, không plan/rust | Stall là blocker duy nhất; BP đủ submit; omp đọc prompt và `report` | Paste nằm compose; splash; omp không load `skills/dory`; 4f FAIL không có chỗ nấu |
| **B** | Mở 1a bash+spec-kit ngay (đóng bảng §11) | “Agent” = fixture bash đọc skill | Occupant-lock: 1a HOLD. Đo Herdr bằng Flow. Coding occupant vẫn `unknown` |
| **C** (chọn) | Plan mỏng: 4f → chỉ vá lỗ 4f gọi tên → factory lại | 4f có thể PASS không cần rust; nếu FAIL thì đúng một lỗ | Cook đoán allowlist / `--kind` / 1a khi journal mơ hồ |

**C** rẻ bỏ: xóa plan + journal. Không đụng classifier.

Không chọn B làm *next*. §11 là plan sau khi occupancy nhà máy pass.

## Quyết định (locked)

1. Plan mới, không viết đè `0847`. Không `blockedBy` 0847 (crate đã có).
2. Không nấu rust trước 4f trừ khi 4f đã chạy và journal chỉ **một** cause.
3. Hàng vá được phép *sau* FAIL (đúng một):
   - submit: Enter/`\\r` sau khối BP; không classifier.
   - skill-load: argv factory `omp --no-session --append-system-prompt @<SKILL.md>` trước; chỉ inject `DORY_SKILL` nếu argv không đủ và journal chứng occupant không thấy file.
   - occupant từ chối: ghi skill rõ hơn; không allowlist.
4. Cấm: `--kind`, allowlist `omp`, Xia `--copy`, `dory flow` trong slice, lật phase 0847, spawn `omp` trong `cargo test`.

## Cook

Plan files-first: `plans/<timestamp>-close-coding-occupancy/`.

## Unresolved

- omp có coi paste+NL là một turn sau 4e — chỉ 4f trả lời.
- Có cần `DORY_SKILL` trên spawn — không quyết trước 4f.
