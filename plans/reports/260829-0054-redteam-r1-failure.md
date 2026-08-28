---
type: redteam
lens: failure
date: 2026-08-29
plan: 260829-0054-isolate-flow-prd-unlock
---

# Red-team R1 — failure

**P-F1** Journal needle stage 2 false-PASSes recook O. **Accept** — require `unlocked stage 3 (flow/03-prd.md)`; reject stage 2/1/00.

**P-F2** Factory PASS 02 skips occupant. **Accept** — factory PASS 00+01 only.

**P-F3** Missing 02 → idx=1 recooks O. **Accept** — mint 02 template; refuse 03 at mint.

**P-F4** Dirty 00/01 + clean 02 still idx=2. **Accept** — assert 00+01 == PASS before taxi1 and after taxi2.
