---
phase: 2
title: "Independent no-spawn test"
status: pending
priority: P1
effort: "20m"
dependencies: [1]
---

# Phase 2: Independent no-spawn test

## Overview

Tab khác cook. Không tin cook receipt. Đo lại HEAD blob, leftover mint, PATH, sock, isolate test artifacts. Không gõ `dory` trên factory.

## Requirements

- Functional: `git show HEAD:rust/src/attach.rs` — `ensure_server` has no `arg("server")` spawn; has fail-closed eprintln.
- Functional: `git show HEAD:rust/src/main.rs` — no “Starts the server if needed”; still `Bare \`dory\` opens the desk`.
- Functional: `git show HEAD:rust/src/attach.rs` module/help — no “starts the daemon if needed” / “Starts \`dory server\` if needed”.
- Functional: `git show HEAD:README.md` sit sentence = server then desk.
- Functional: leftover 5 hash-object == mint; leftover ELF sha unchanged.
- Functional: isolate cargo already produced PASS (re-run only with **absolute** isolate `--manifest-path`). Isolate ELF strings: no spawn-fail text.
- Non-functional: factory `type -a dory` empty; `$XDG_RUNTIME_DIR/dory/default/dory.sock` connectable=0; leftover ELF sha `3ba0e3bc…`; no leftover-tree cargo; no leftover/isolate ELF exec.

## Architecture

Independent tab. Observe `git show HEAD`. Observe hashes. Observe **dory.sock** connectable. Optional re-run: `env -u DORY_SOCKET cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml p5_attach`. Never relative leftover cargo. Never factory `dory` / leftover ELF / isolate ELF.

## Related Code Files

- Read: HEAD blobs via `git show HEAD`
- Create: `plans/reports/260827-ensure-server-no-spawn-test.md`
- Do not modify rust / leftover 5 / ELF

## Implementation Steps

1. New `w13` tab ≠ cook tab, ≠ `t13`. cwd leftover tree. `--no-focus`.
2. Refuse dirty `DORY_*` / `PI_CODING_AGENT_DIR`. STOP leftover/isolate ELF argv on factory XDG.
3. `NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)`. `git show HEAD:rust/src/attach.rs` — no spawn in `ensure_server`; eprintln present.
4. USAGE + attach help + HEAD README `:15` as above.
5. Leftover 5 `git hash-object` mint **including untouched** `server.rs` `4de1554a`. Leftover ELF sha `3ba0e3bc…` + spawn-strings still present.
6. `hash -r`; `type -a dory` empty. `UnixStream::connect("$XDG_RUNTIME_DIR/dory/default/dory.sock")` connectable=0. Do not probe the session directory.
7. Isolate `rev-parse HEAD` == `$NEWHEAD` ≠ `5a60953`. Isolate ELF strings: no `dory: start server:` / `server did not come up`. Optional absolute isolate cargo only.
8. `git status --porcelain`: leftover 5 ` M` only (plus plan/receipts `??` allowed). Cached empty of leftover 5.
9. Write test receipt. Do not invoke factory `dory`. Do not exec leftover/isolate ELF on factory XDG.

## Success Criteria

- [ ] HEAD spawn gone; stderr locked
- [ ] USAGE/README land sentences match C
- [ ] Leftover 5 + leftover ELF mint
- [ ] PATH empty; `dory.sock` connectable=0
- [ ] Distinct `TEST_TAB` ≠ cook ≠ `t13`

## Risk Assessment

| Risk | Signal | Response |
|---|---|---|
| Test on cook pane | same tab id | FAIL. New tab. |
| Re-run cargo leftover | ELF sha drift | FAIL. |
| “PASS if leftover ELF still spawns” | claim doors held | FAIL paper. C = HEAD inode only. |

## Next Steps

Phase 3 review tab, different id.
