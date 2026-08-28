---
type: redteam
lens: failure
date: 2026-08-28
plan: 260828-1612-isolate-flow-scope-unlock
---

# Red-team R1 — failure

Keep N F1–F9 retargeted to 01/02.

**O-F1** N journal needle `unlocked stage 1` would false-PASS recook. **Accept** — require `unlocked stage 2 (flow/02-scope.md)`; reject stage 1/00.

**O-F2** Factory writing PASS 01 skips occupant. **Accept** — factory PASS 00 only; refuse 01 == PASS_01 at mint.

**O-F3** `flow.sh:136-153` idx=1 if both files exist even when 00 dirty. **Accept** — assert 00 == PASS_00 before taxi1 and after taxi2.

**O-F4** Pre-existing 02 → `:1019-1022` already exists rc=0. **Accept** — refuse 02 at mint; reject already-exists in taxi2.

**O-F5** Missing `FLOW_HARNESS_DISABLE` can add harness lines to taxi2 stdout. **Accept** — pin; still require exact unlock needle.
