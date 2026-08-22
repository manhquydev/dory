---
title: "Workplace skill mux"
description: "Herdr-like Dory loop: skill → CLI → socket → server-held PTY + DORY_ENV=1, plus flow.sh taxi."
status: pending
priority: P1
effort: "6-12w"
tags: [dory, workplace, rust]
created: 2026-08-22
---

# Workplace skill mux

## Overview

Build Dory’s Workplace OS as a **Herdr-like invocable loop**, not a Herdr clone and not the Node HTTP spike.

```text
skills/dory/SKILL.md  →  dory <group>  →  Unix socket  →  daemon-owned PTY
                              ↑
                    DORY_ENV=1 injected at spawn
dory flow -- <args>   →  foreign flow.sh / FLOW_BIN + journal receipt
```

Authority: `CHARTER.md`, `plans/reports/260822-skill-cli-socket-contract.md` (accepted), `plans/reports/260822-stack-decision.md` (Rust), `plans/reports/brainstorm-260822-impl-direction.md`.

Do **not** implement against `src/workplace/http.js`. Do **not** Xia `--copy` Herdr. Do **not** write `dory` into flow-skill.

Successor slice (do **not** flip phases below from paper): `plans/260823-0011-close-coding-occupancy/` — factory coding occupancy. Contract §11 stays after that slice, not a Phase 6 gate.

## Goals

| # | Goal | Priority |
|---|------|----------|
| 1 | One Rust `dory` binary owns PTY masters and the control socket | P1 |
| 2 | Occupant inside a pane can split / run / start / prompt / wait via CLI | P1 |
| 3 | Skill stops unless `DORY_ENV=1` | P1 |
| 4 | `dory flow --` taxis `flow.sh`; flow-skill stays dory-free | P1 |

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Toolchain](./phase-01-start.md) | Completed |
| 2 | [Daemon PTY master](./phase-02-daemon-pty-master.md) | Pending |
| 3 | [Env and CLI groups](./phase-03-env-and-cli-groups.md) | Pending |
| 4 | [Skill gate](./phase-04-skill-gate.md) | Pending |
| 5 | [Occupant wait](./phase-05-occupant-wait.md) | Pending |
| 6 | [Flow taxi](./phase-06-flow-taxi.md) | Pending |

## Success Criteria

- [ ] `rustc`/`cargo` available; crate exists in this repo only
- [ ] Detach leaves the PTY up; `server stop` kills and leftover snapshot is labeled not-live
- [ ] `test "${DORY_ENV:-}" = 1` is the skill’s first command
- [ ] JSON IDs parsed from responses; closed IDs never reused
- [ ] `rg -i dory` on flow-skill is empty
- [ ] Shipped spawn path has no `herdr` / `dsh`
- [ ] `dory flow --` taxis foreign `flow.sh`; receipts + refuse-list hold; flow-skill has zero `dory` bytes
- [ ] Product-done (contract §11, **after** this plan): a stranger inside `DORY_ENV=1` finishes a real external card — not a Phase 6 gate

## Refuse

Node `/workplace` as the skill. `X-Dory-Inside`. `script` as production PTY. `node-pty`. Ratatui-as-identity. Marketplace. `--kind` farm. Windows host in v1.

<!-- slug: workplace-skill-mux -->
