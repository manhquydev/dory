---
type: redteam
lens: security
date: 2026-08-28
plan: 260828-1612-isolate-flow-scope-unlock
---

# Red-team R1 — security

Keep N S1–S8 (land hash, PI, sit PATH, ISO/bin, refuse any FLOW_*, sit ids, compound stop).

**O-S1** `flow.sh:289-294` `:660-671` `:1027` — unlock-2 fires `gate_durable_hook 01-research` unless `FLOW_HARNESS_DISABLE`. **Accept** — taxi pin + self-rg.

**O-S2** N `scripts/dory-isolate-aoe5-flow-next.sh:44-46` `:263-276` omits itself. **Accept** — O `$0` + regex add `dory-isolate-aoe5-flow-next`.

No factory ELF exec. No default sock.
