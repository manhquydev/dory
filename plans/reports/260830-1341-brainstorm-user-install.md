---
type: brainstorm-accept
date: 2026-08-30
time: 13:41
advice: 260830-1341-advise-user-install.md
research: 260830-1341-research-user-install.md
---

# Brainstorm accept — lamp doctor, not a Node installer

## Contract

| Field | Closed |
|---|---|
| **Outcome** | A user with Node `>=22.14.0` can install, verify, and uninstall the **lamp** (`@manhquy/dory` / `dory-serve`) from public docs + `scripts/dory-lamp-doctor.sh`. A user without Node gets a directed fail (nodejs.org / fnm), not a silent toolchain install. Desk stays “build/release ELF later.” |
| **Constraints** | Two engines. No `bin.dory` on the lamp package. No leftover 5. No `git add -A`. No token in repo/chat. No sudo. No `curl \| sh` Node. Factory: no default sock, no sit `t13`. Do not `-g` onto cursor-agent prefix as the documented path. |
| **Non-goals** | cargo-dist / GitHub desk release. Node SEA. Trusted Publisher dashboard (user). Leftover fold. Windows desk. Darwin occupant classify. Dist-tag `latest` policy fight. |
| **Acceptance** | Doctor `--help` documents diagnose / `--install` / `--uninstall`. Diagnose exit 0 here (Node + live package). Diagnose mentions `nodejs.org` in the missing-Node branch (code path exists). `npm view @manhquy/dory bin` has `dory-serve` only. Leftover README hash unchanged. |

## Approaches

| # | Approach | Load-bearing assumption | Fails first | Verdict |
|---|---|---|---|---|
| **A** | Doctor + docs. Detect/direct. Optional lamp `--install`. | Users can install Node themselves once told. | Empty VM user wanted zero steps. | **Choose.** Cheapest to abandon. |
| B | Doctor also installs fnm + Node LTS. | We may own their Node. | Existing nvm/apt Node; curl\|sh. | Reject. |
| C | cargo-dist desk + lamp in one curl. | Release ELF + checksum exist. | No GH desk artifacts today. | Next cook, not this wave. |

## Chosen direction

A. Implement doctor + evergreen install door on `npm-wrapper/README.md` and `docs/README.md`. Defer C until a named cook owns release CI.

## Evidence

- Live: `@manhquy/dory@0.1.0-next.0`, bin `dory-serve`, tags `next` + `latest`.
- This machine: no PATH `dory` / `dory-serve`; Node v24.5.0; leftover 5 still mint hashes.
- Research: Microsoft installer pattern = detect, do not bootstrap language runtimes. cargo-dist 0.32 = fetch ELF from releases we do not publish yet.

## Unresolved risks

- npmjs TP unbound → OIDC live still E404 until user saves the form.
- Factory `npm prefix -g` is cursor-agent — doctor must refuse `--install` there.
- Land README (leftover 5) still says `node bin/dory.js serve` — cannot fold this wave. Docs route owns the public lamp command.
