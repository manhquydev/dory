# SHIP — paper only

You are `aoe5_ship`. **Paper commit only. Do not push.**

Use `git add --` pathspec. Never `git add -A` / `-u`. Never leftover 5. Never rust.

## Pathspec ONLY

```
scripts/dory-isolate-aoe5-flow-judge.sh
plans/260827-1122-aoe-5-isolate-flow-judge/
plans/reports/260827-1116-research-01-flow-judge.md
plans/reports/260827-1116-research-02-isolate-project.md
plans/reports/260827-1122-scout-aoe5.md
plans/reports/260827-1122-aoe5-roster.md
plans/reports/260827-1122-rt-security.md
plans/reports/260827-1122-rt-failure.md
plans/reports/260827-1122-rt-fold.md
plans/reports/260827-ensure-aoe5-flow-judge-*
```

`git diff --cached --name-only` must be ⊆ pathspec.

Message:

```
feat(isolate): fail-then-pass flow.sh gate

```

After commit: leftover 5 still ` M` mint; `git log -1 -- rust/` still `b544f5f`; sock connectable=0; PATH empty.

Write `plans/reports/260827-ensure-aoe5-flow-judge-ops.md`
Reply `SHIP_PASS` or `SHIP_FAIL` plus paper HEAD.
