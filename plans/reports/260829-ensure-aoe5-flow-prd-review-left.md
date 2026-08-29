---
type: review
lens: leftover
date: 2026-08-29
wave: dory-aoe5p
result: ACCEPT
critical: 0
writer: pr_left
---

# rv_left — leftover mint

Accept iff leftover 5 `git hash-object` mint MATCH and `git log -1 -- rust/` = `b544f5f`. Did not fold leftover.

| File | live hash-object | mint | vs HEAD |
|---|---|---|---|
| README.md | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH `68190a5f` | dirty `5ac82b10…` |
| rust/src/attach.rs | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH `60247909` | dirty `62f09a95…` |
| rust/src/main.rs | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH `373d6886` | dirty `5fc70ad5…` |
| rust/src/server.rs | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH `4de1554a` | dirty `dfca2ac5…` |
| rust/tests/p5_attach.rs | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH `9c28fc3e` | dirty `fa44bfbb…` |

`git log -1 -- rust/` = `b544f5f fix(attach): do not auto-start server on sit` (`b544f5ff75a3c085ae6ab51ccafb3b58fa551db2`).

Porcelain leftover 5 still unstaged ` M` ×5. `git diff --cached --name-only` leftover 5 + `rust/` = empty. Worktree rust dirty leftover = pass.

`desk.rs` worktree == HEAD `4c788562e4fdda10c8edd2878ed1fdd46050c218`. Leftover ELF `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` size `18568240` inode `2490742` mtime_epoch `1787716801` (stat-only). No leftover cargo.

This pane did not `git add` leftover 5, checkout/restore leftover, cargo leftover, or exec leftover ELF.

Critical: 0
