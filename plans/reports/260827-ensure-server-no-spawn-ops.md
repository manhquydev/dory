---
type: ops
date: 2026-08-27
plan: 260827-1032-ensure-server-no-auto-spawn
phase: paper-ship
verdict: SHIP_PASS
rust_head: b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
paper_head: 0ead6ff9a9d34d06f89937b8b9b3722b07cfbe3b
pushed: no
---

# Ops — paper commit ensure_server no auto-spawn

**Verdict: SHIP_PASS**

Paper only. `ak:git` cm. Did not push. Did not `git add -u` / `git add -A`. Did not add leftover 5. Did not invoke factory `dory` / leftover ELF / isolate ELF. Did not `herdr server stop`. Did not sit `t13`.

```
0ead6ff docs(plan): record ensure_server no auto-spawn
```

Rust C remains HEAD parent `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2`.

## Cached before commit (⊆ pathspec)

21 files, all paper:

```
plans/260827-1032-ensure-server-no-auto-spawn/phase-01-start.md
plans/260827-1032-ensure-server-no-auto-spawn/phase-02-independent-no-spawn-test.md
plans/260827-1032-ensure-server-no-auto-spawn/phase-03-review-and-ship.md
plans/260827-1032-ensure-server-no-auto-spawn/plan.md
plans/reports/260827-1027-research-01-spawn.md
plans/reports/260827-1027-research-02-leftover.md
plans/reports/260827-1032-c-roster.md
plans/reports/260827-ensure-server-no-spawn-cook-prompt.md
plans/reports/260827-ensure-server-no-spawn-cook.md
plans/reports/260827-ensure-server-no-spawn-review-fold.md
plans/reports/260827-ensure-server-no-spawn-review-fold.txt
plans/reports/260827-ensure-server-no-spawn-review-left.md
plans/reports/260827-ensure-server-no-spawn-review-left.txt
plans/reports/260827-ensure-server-no-spawn-review-spawn.md
plans/reports/260827-ensure-server-no-spawn-review-spawn.txt
plans/reports/260827-ensure-server-no-spawn-review-usage.md
plans/reports/260827-ensure-server-no-spawn-review-usage.txt
plans/reports/260827-ensure-server-no-spawn-review.md
plans/reports/260827-ensure-server-no-spawn-ship-prompt.md
plans/reports/260827-ensure-server-no-spawn-test-prompt.md
plans/reports/260827-ensure-server-no-spawn-test.md
```

Leftover 5 not in cached. Secret scan: 0 hits.

## After commit

| check | result |
|---|---|
| paper HEAD | `0ead6ff9a9d34d06f89937b8b9b3722b07cfbe3b` |
| subject | `docs(plan): record ensure_server no auto-spawn` |
| leftover 5 porcelain | ` M` unstaged (all five) |
| leftover cached | empty |
| push | no |

### Leftover 5 mint

| path | `git hash-object` | mint | |
|---|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5f…` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `60247909…` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d6886…` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554a…` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e…` | MATCH |

Leftover ELF sha `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` held (not exec'd).

### PATH

`hash -r`; `type -a dory` → `type: dory not found` TYPE_EXIT=1.

### sock

AF_UNIX `UnixStream.connect` on `$XDG_RUNTIME_DIR/dory/default/dory.sock` = `/run/user/1000/dory/default/dory.sock` (not the session dir).

lexists=False. connectable=0. `FileNotFoundError: [Errno 2] No such file or directory`. timeout 1s. Did not start default.

`DORY_SOCKET` `DORY_ENV` `DORY_RECYCLE` `PI_CODING_AGENT_DIR` UNSET.

SHIP_PASS
