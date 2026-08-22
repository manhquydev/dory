---
type: factory
date: 2026-08-23
time: 00:17
status: fail
cause: submit
authority:
  - plans/260823-0011-close-coding-occupancy/phase-01-start.md
  - plans/reports/260823-0009-brainstorm-next-occupancy-scope.md
---

# Layer 4f factory — `omp --no-session` after 4c+4e (không CI)

Chấm **fail** từ file bằng chứng. Một nguyên nhân: **submit**.

## Verdict table

| Check | Result | Source |
|---|---|---|
| start state | `unknown` | `/tmp/dory-l4f-evidence/start.json` |
| `--kind` absent | yes (`-- omp --no-session`) | start argv |
| classifier leak | no | start stayed `unknown` |
| prompt CLI | `dory agent prompt` `ok:true` | `/tmp/dory-l4f-evidence/prompt.json` |
| `agent_prompt_stalled` | no (4e holds) | prompt.json + empty prompt.err |
| wait | `timeout` | `/tmp/dory-l4f-evidence/wait.err` |
| final state | `unknown` `seen=false` | `/tmp/dory-l4f-evidence/get-final.json` |
| compose holds prompt | yes | `/tmp/dory-l4f-evidence/read.txt` |
| model turn / report | no | get-final still `unknown` |
| no rust edit | yes | this journal |
| no cargo omp test | yes | this journal |
| no flow-skill bytes | yes | this journal |

## Evidence (quoted paths only)

- `/tmp/dory-l4f-evidence/FAIL`
- `/tmp/dory-l4f-evidence/start.json`
- `/tmp/dory-l4f-evidence/prompt.json`
- `/tmp/dory-l4f-evidence/wait.err`
- `/tmp/dory-l4f-evidence/get-final.json`
- `/tmp/dory-l4f-evidence/read.txt`

Do not treat the first FAIL draft’s `has_report=True` / `has_reported=True` as occupant action. Those strings were in the **sent prompt**.

## Sequence on transcript

1. `bash-5.2$ omp --no-session`
2. Prompt bytes appear as raw line **before** TUI splash (prompted too early).
3. `omp` splash (“Welcome back”, MCP connecting).
4. Same prompt text sits in the **compose box** (cyan `╭──` editor) for the whole 180s wait.
5. No assistant reply. No `dory agent report` execution. State stays `unknown`.

4c wrap delivered text into the editor. Trailing `\\n` after `CSI 201 ~` did **not** submit the turn. This is not stall, not classifier leak, not skill-load (skill file never needed — occupant never left compose).

## Allowed next hole (plan phase 2)

`submit`: after BP wrap, Enter as `\\r` and/or `send-keys enter`. Not allowlist. Not `--kind`. Not 1a.

This is **not** contract §11.
