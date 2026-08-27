# TEST — independent AOE 5 re-run

You are `aoe5_test`. Skills OFF. Do **not** recook. Do not edit the script unless it is missing (it exists).

Run `/home/manhquy/Downloads/flow/dory/scripts/dory-isolate-aoe5-flow-judge.sh` once with:

```
SIT_PANE=<given>
SIT_TAB=<given>
SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory
```

Sit pane is a **new empty shell**. Do not sit t13. Do not invoke factory `dory`. Do not cargo leftover. Do not fold leftover 5.

Independent asserts (do not copy cook receipt as proof):

- Script exit 0
- Copied journal (script wipes ISO) has **exactly two** `flow/result`
- both `bin` == abs `/home/manhquy/.claude/skills/flow/runner/flow.sh`
- codes `[1, 0]`
- fail stdout has `GATE stage 00-idea`; pass has `clean`
- leftover 5 mint:
  README `68190a5ffa073c082aa318aad5ed032e13cc90e3`
  attach `602479094e84d31ad6f017775a3d55aeb485c644`
  main `373d688636ff7315ccd665f450069d8284eb47ff`
  server `4de1554ad56e248cdcf42f02111b7389b08dae82`
  p5_attach `9c28fc3e0f3666498a8952411242d5301f7911de`
- `desk.rs` hash == HEAD
- sock `/run/user/1000/dory/default/dory.sock` connectable=0
- `type -a dory` empty
- `git log -1 -- rust/` = `b544f5f`

Write `/home/manhquy/Downloads/flow/dory/plans/reports/260827-ensure-aoe5-flow-judge-test.md`
Reply `TEST_PASS` or `TEST_FAIL`.
