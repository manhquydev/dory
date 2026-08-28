---
type: cook-pin
date: 2026-08-29
wave: dory-aoe5p
result: PIN_PASS
run: w13:p99
pin: w13:p9D pk_pin
iso_prefix: aoe5p
---

# COOK PIN — aoe5-prd=PASS

**Verdict: PIN_PASS**

Footer on `w13:p99` after `scripts/dory-isolate-aoe5-flow-prd.sh`:

```
aoe5-prd=PASS
```

ISO prefix `aoe5p` (`aoe5p.eGZMMi`, wiped). Land ELF sha `2ef20730…`. Rust log `b544f5f`. Leftover 5 not folded. No `git add -A`. No recook O/N. No sit `t13`/`p2R`. No default sock start. No dory argv from pin.

## Leftover 5 path+sha mint

| Path | `git hash-object` | Mint |
|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH `68190a5f` |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH `60247909` |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH `373d6886` |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH `4de1554a` |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH `9c28fc3e` |

Porcelain still ` M` ×5. `desk.rs` worktree == HEAD `4c788562e4fdda10c8edd2878ed1fdd46050c218`. `git log -1 -- rust/` = `b544f5f`.

## ELF pins (stat only)

| Binary | sha256 | Mint |
|---|---|---|
| land `SIT_DORY` | `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3` | MATCH `2ef20730` |
| leftover `rust/target/debug/dory` | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` | MATCH unchanged |

## Runner footer (p99)

- `SIT_DORY=/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory`
- `SIT_DORY_SHA=2ef20730…`
- `SIT_PANE=w13:p97` `SIT_TAB=w13:t20`
- `TAXI1_RC=1` `TAXI2_RC=0` `JOURNAL_CODES=1,0`
- `NEEDLE1=Flow 1. next` `NEEDLE2=Flow 0. next`
- `PRD_SHA=TEMPLATE_SHA=219c935006a657df295201d491cf2b7227fe0e06a646a386bcf69dd6b9e72cc4`
- `FACTORY_CONNECTABLE=0` `VISIBLE_MATCH=1`

Doors held: leftover 5 still unstaged mint. ISO prefix `aoe5p`.
