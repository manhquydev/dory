# Capacity freeze — flow-skill features / Dory calendar

- **Artifact:** founder capacity-freeze memo (feature freeze + resource reallocation)
- **Status:** In force
- **Effective:** 2026-08-22
- **Review:** 2027-02-22 (6 months). Outer bound 2027-08-22 (12 months).
- **Authority:** Operator grant, 2026-08-22 — *“tôi trao quyền cho bạn với vai trò CTO hãy tự viết và điều phối các nhân sự herdr… --auto”*
- **Author:** CTO (session agent), executing that grant. Not a silent self-unlock.

This file satisfies **CHARTER condition 2** (formerly HIEN-PHAP). It does not waive kill conditions.

## Decision

For **6–12 months** from the effective date:

1. **flow-skill** is on **maintenance**. No new product capabilities: no new stages, host adapters, orchestration surface, or feature train. Allowed: security, installer/version honesty (including the unpublished local `0.7.1` truth), critical regressions.
2. **flow-deck** stays frozen. No new profession.
3. **Dory** owns the primary calendar. North star: a local **Agent Operating Environment** (Session OS + Workplace OS, Flow as a foreign governance plane).
4. Factory (Herdr / Cursor / OMP) may be used to *build* Dory. Shipped Dory must not call `herdr` or `dsh` at runtime.

## Why this is a file

Chat is cheap. A dated memo in this repo is the opportunity-cost lock: one primary FTE cannot run two load-bearing products. The other door (>2 FTE or equivalent money) is not open.

## Unlock

- `/ak:plan` for Dory is allowed after this file exists.
- Engine commits (`go.mod`, `package.json`, PTY, session loop) are allowed **in this repo only**, under CHARTER kill conditions.
- First mile is Phase 1 Session OS. Phase 5 (operator-grade AOE) is the company. Do not sell Phase 1 as the destination.

## Kill if

Any of CHARTER kill conditions, or a Dory commit that pretends this memo was operator-typed body text. The grant is the quote above. The trade is this file.
