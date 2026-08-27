# SHIP — paper only (ak:git cm, not full ak:ship PR)

You are `c_ship`. Skills ON (ak:git). **Paper commit only. Do not push.**

Rust C already on HEAD `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2`.

## Pathspec ONLY

```
plans/260827-1032-ensure-server-no-auto-spawn/
plans/reports/260827-ensure-server-no-spawn-*
plans/reports/260827-1027-research-01-spawn.md
plans/reports/260827-1027-research-02-leftover.md
plans/reports/260827-1032-c-roster.md
```

## STOP

- leftover 5 (`README.md` `rust/src/attach.rs` `rust/src/main.rs` `rust/src/server.rs` `rust/tests/p5_attach.rs`)
- `git add -u` / `git add -A` / leftover ELF
- push / PR / `herdr server stop` / sit t13 / invoke dory

Before commit: `git diff --cached --name-only` must be ⊆ pathspec above.
After commit: leftover 5 still ` M` unstaged. Leftover hashes mint. sock connectable=0. PATH empty.

Message:

```
docs(plan): record ensure_server no auto-spawn

```

Write `/home/manhquy/Downloads/flow/dory/plans/reports/260827-ensure-server-no-spawn-ops.md`
Reply `SHIP_PASS` or `SHIP_FAIL` plus paper HEAD.
