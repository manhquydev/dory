---
title: "Phase 3: Keys and close"
status: done
phase: 3
priority: P1
effort: 5h
dependencies: [2]
---

# Phase 3: Keys and close

## Overview

Bảng phím Herdr trên desk. Daemon đóng ô / cửa sổ. `tab.close` đã có.

## Requirements

- Functional (mọi chord **sau prefix**; `SHIFT` = `KeyModifiers::SHIFT`, không `'N'`/`'D'`):
  - `n`/`p` = thẻ kế/trước **cùng cửa sổ** (walk client, bounded)
  - `1..9` = thẻ đó
  - `w` = picker; `j/k`/`enter`/`esc` trong `Mode::Picker` (không `write_pty`)
  - `Shift+n` = tạo cửa sổ
  - `x` = đóng ô; `Shift+x` = đóng thẻ; `Shift+d` = đóng cửa sổ
  - `q` và `d` **không Shift** = detach. Không `prefix+D` detach.
  - **Cấm** `pane.close` / `tab.close` / `workspace.close` nếu còn đúng một ô sống trên session (phòng trống)
  - Ô không-cuối: `remove` khỏi `Tab.panes` + `remove_leaf` + retarget `root_pane` + retire. Ô cuối trên thẻ (còn cửa sổ khác hoặc còn thẻ khác): `tab.close`. Thẻ cuối trên cửa sổ không-cuối: `workspace.close` (phải `workspaces.remove` + retire `ws`)
- CLI mutate: mỗi arm `require_skill_env()` + `json_safe_id` / `pane_target` như `tab close`. Test: không env → exit 1, World nguyên. Socket RPC vẫn không cần env.
- Non-functional: CLI mutate vẫn `DORY_ENV=1`. Desk socket không cần env. ID đóng không tái dùng.

## Architecture

Một helper World: close pane = kill + retire + `panes.remove` + `remove_leaf` **hoặc** `close_tab` nếu last leaf — một `&mut World`. Không nhờ `ensure_layout`/`synthesize` thay `remove_leaf` (mất ratio).

`Mode::{Terminal, Prefix, Picker, Confirm}` trên `Desk` từ pha này. `handle_key` chỉ `write_pty` khi `Terminal`. Confirm bắt buộc trước `tab.close` / `workspace.close` (kể cả promote last-pane).

`desk.neighbor` `next`/`prev` **đóng băng** pane-global (attach `--plain` + `p5_attach`). Desk tab motion = walk client, không overload `next`.

Caps Lock: mint/close chỉ khi `SHIFT` bit, không khi `'N'`/`'D'`.

## Related Code Files

- Create tests: `rust/tests/p5_close.rs`
- Modify: `rust/src/layout.rs`, `server.rs`, `main.rs`, `desk.rs`, `skills/dory/SKILL.md` (P5 mới đủ USAGE — P5 docs cũng đụng)

## Implementation Steps

1. `remove_leaf` + unit layout.
2. `pane.close` / `workspace.close` RPC + CLI.
3. Desk prefix remap + picker overlay + confirm một dòng khi đóng thẻ/cửa sổ (`y`/`n`).
4. `p5_close.rs` isolated server: split → close leaf; close last pane on tab; refuse last workspace.
5. Suite.

## Todo

- [x] remove_leaf
- [x] RPC + CLI close
- [x] Desk keymap + picker
- [x] p5_close

## Success Criteria

- [x] `prefix+n` đổi thẻ cùng cửa sổ; `--plain` `n` vẫn ô
- [x] `prefix+w` không mint; `prefix+Shift+n` mint
- [x] `prefix+d` detach; `prefix+Shift+d` close workspace (sau confirm)
- [x] Close ô: sibling chiếm chỗ, ratio không synthesize; id retired
- [x] Close last live pane / last workspace: error; World còn ≥1 pane
- [x] CLI close không `DORY_ENV` → exit 1
- [x] Suite xanh

## Risk Assessment

Operator đã thuộc `w` = tạo. Signal: mint nhầm. Response: footer/overlay nói “picker”; README hàng đầu. Không giữ nghĩa cũ im lặng.
