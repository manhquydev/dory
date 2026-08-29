---
type: redteam
lens: stale
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
head: 049e304
p_feat: f1c966c
critical: 0
---

# Red-team — stale paid claims

Findings only. Evergreen WHERE is `docs/README.md`. Land README is `git show HEAD:README.md`. Do not recook P.

| # | Claim | Where | Now | Sev | Disposition |
|---|---|---|---|---|---|
| D-S1 | Unlock-3 `03-prd.md` **unpaid — next isolate hunk** | `plans/reports/260829-0052-scout.md` paid-table (00:52) | P paid: `f1c966c` `feat(isolate): fail-then-pass flow.sh prd` + paper `049e304`. Owner `scripts/dory-isolate-aoe5-flow-prd.sh`. | Low | **Historical scout.** Snapshot paper was `45d32fb`; isolate scripts listed judge+next+scope only. Do not copy that row into evergreen. New docs must say P paid. |
| D-S1b | P is the next cook | `plans/reports/260829-0052-brainstorm-accept.md` | 0054 plan frontmatter `status: completed`. Journal: isolate P shipped. | Low | Historical accept for the cook that already landed. Not current unpaid remainder. |
| D-S1c | P still unpaid / unlock-3 missing | `docs/README.md`, CHARTER WHERE, `AGENTS.md`, 1204 journal | `docs/README.md`: "P unlock-3 is paid." Named unpaid does **not** list unlock-3. CHARTER has one WHERE link to docs. AGENTS: do not claim isolate N/O/P is company Phase 5. | — | **Pass.** Evergreen matches evidence. |
| D-S1d | HEAD README "Xong tới đâu" omits isolate taxi | `git show HEAD:README.md` | Desk how-to. Isolate owners live under docs. Working `README.md` is leftover 5 mint `68190a5f`. | — | Not a paid/unpaid lie. Approach B (edit land README) rejected this wave. |
| D-S1e | 0054 phase YAML/table still `pending` | `plans/260829-0054-isolate-flow-prd-unlock/plan.md` phases table + phase-0{1,2,3} frontmatter | Plan frontmatter `completed`. Phase success criteria `[x]`. Product remainder in that plan is company blob, not recook P. | Low | Stateful plan-body lag. Do not read as P unpaid. Do not recook P. |

Critical: 0

Falsifier for D-S1c: any new evergreen sentence that restates 0052 "unlock-3 unpaid" as current WHERE. Not observed.
