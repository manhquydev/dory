---
type: research
date: 2026-08-30
time: 13:41
topic: user-install-nonideal-env
conducted: 2026-08-30
sources: 5 web + live npm view + HEAD npm-wrapper + 260830-1036-research-dory-npm-install.md
---

# Research Report: User install when the machine is not ideal

## Executive Summary

Lamp `@manhquy/dory@0.1.0-next.0` is on npmjs (bin `dory-serve` only). Desk SKU `dory` is still a Rust ELF with no GitHub release. A product installer **cannot honestly install Node** for the user — that fights fnm/volta/nvm/apt and is a supply-chain job we do not own. Ceiling this wave: **detect → tell → verify → uninstall lamp**. Ceiling later: **cargo-dist / cargo-npm for desk ELF**, still not a Node bootstrapper.

`npm i -g dory` remains a stranger’s Surge blog. Two engines stay two.

## Research Methodology

- Sources: 5 web searches (2026 Node managers; cargo-dist 0.32; npm engines/npx; curl|sh security; cargo-npm/esbuild optionalDeps) + live registry + land README via `git show HEAD:README.md`
- Date range: cargo-dist book (current) through cargo-dist 0.32.0 (2026-05-21)
- Terms: fnm volta nvm bootstrap; cargo-dist installers; engines-strict npx; curl|bash; optionalDependencies platform packages

## Key Findings

### 1. Technology Overview

| SKU | What the user types | Needs today |
|---|---|---|
| Lamp (Session OS) | `npx @manhquy/dory dory-serve -- --workspace /abs` then `serve` | Node `>=22.14.0` + network once |
| Desk (Workplace OS) | `dory` | Rust ELF on PATH; **no public binary** |

npm `engines.node` is `>=22.14.0` (lower bound only — good; tight upper bound would make `npx` silently pick an older unconstrained version).

### 2. Current State & Trends

- 2026 Node advice: **fnm** default personal; **Volta** team pin; official nodejs.org installer when no version manager. Distro `apt nodejs` is often stale.
- cargo-dist 0.32 (2026-05): shell / powershell / npm / homebrew **fetch** GitHub release ELF; msi **bundles**. npm installer dropped axios tree (still a downloader, not esbuild optionalDeps).
- cargo-npm / npmgen: platform `optionalDependencies` (esbuild pattern) — no postinstall download. Desk-on-npm later, **different bin** (`dory`), never overwrite lamp `dory-serve`.
- Microsoft one-line-installer pattern: detect missing toolchain, **do not** bootstrap Node/Python from your script.

### 3. Best Practices

1. Default lamp path = `npx @manhquy/dory@0.1.0-next.0` (pin version). No `-g` required.
2. Doctor script: check `node` / version / `npm view` / PATH collisions / `dory-serve`. Exit non-zero with the official Node URL. Never `curl | bash` NodeSource.
3. `--install` only installs **the lamp package**, never Node, never sudo, never `dory` bin.
4. Uninstall = `npm uninstall -g @manhquy/dory` + `type -a dory dory-serve`.
5. Desk later: cargo-dist `shell` + checksum, or cargo-npm optionalDeps. Host installer in this repo. Pin release URL.

### 4. Security Considerations

- `curl | sh` can serve different body to piped vs saved (idontplaydarts). If we ever ship a desk installer: download → sha256 → run; HTTPS; pin tag.
- Product installer that installs Node **conflicts** with existing nvm/fnm/volta and can clobber `/usr/local`.
- Token in chat is burned. No token in doctor.
- Unscoped `dory` on npm is not us. Doctor must refuse `npm i -g dory`.

### 5. Performance Insights

- `npx` downloads once per cache; no global prefix fight (this factory’s `npm prefix -g` is a cursor-agent dir — `-g` here is the wrong prefix).
- cargo-dist npm still fetches at install time unless we use optionalDeps.
- Git clone + `cargo build` remains the only honest desk install; leftover WT must not be the source.

## Comparative Analysis

| Path | Helps missing Node? | Desk? | Lamp? | Risk | Now |
|---|---|---|---|---|---|
| Docs + doctor (detect/direct) | Tells how | No | Yes | Low | **Do** |
| Doctor installs Node (fnm/NodeSource) | Yes | No | Yes | Fights user toolchain; curl|sh | Reject |
| Node SEA single binary of lamp | Yes | No | Yes | New SKU; still not desk | Defer |
| cargo-dist shell for desk | N/A (no Node) | Yes | No | Needs GH release ELF | Next cook |
| cargo-npm `bin.dory` | Needs Node to shim | Yes | Collision if same pkg | Must be **other package** | After ELF CI |
| `npm i -g dory` | — | Stolen | Stolen | Other product | Forbid |

## Implementation Recommendations

### Quick Start (honest, today)

```bash
# 1. Node 22.14+ (user’s job — fnm or nodejs.org)
node -v
# 2. lamp, no global
npx @manhquy/dory@0.1.0-next.0 dory-serve -- serve --workspace /abs
# 3. verify / remove
bash scripts/dory-lamp-doctor.sh
npm uninstall -g @manhquy/dory
```

Do not `npm i -g dory`. Do not claim this is desk.

### Common Pitfalls

| Pitfall | What happens |
|---|---|
| Installer bootstraps Node | Breaks nvm/fnm; we own their Node forever |
| `bin.dory` on lamp package | Steals desk SKU |
| cargo-dist npm as lamp | Downloads desk ELF through lamp name |
| Treat packument 404 for 3 min as fail | First-publish lag (paid 2026-08-30) |
| `-g` on cursor-agent npm prefix | Binary lands off user PATH |

## Resources & References

- https://nodejs.org/en/download
- https://axodotdev.github.io/cargo-dist/book/installers/index.html
- https://github.com/axodotdev/cargo-dist/releases/tag/v0.32.0
- https://github.com/abemedia/cargo-npm
- https://docs.npmjs.com/cli/v11/using-npm/config/ (engine-strict)
- https://hub.decision.ai/skills/microsoft/one-line-installer-patterns
- Sibling: `260830-1036-research-dory-npm-install.md` (pre-publish; scoped name now `@manhquy/dory`)

## Appendices

### A. Glossary

| Term | Meaning |
|---|---|
| Doctor | Detect/print/verify. Does not install Node. |
| Desk | Rust `dory` |
| Lamp | Node `dory-serve` |

### B. How far we can go

| Layer | Can we? |
|---|---|
| Detect Node / version / prefix / collisions | Yes |
| Install lamp via npm/npx | Yes (user has Node) |
| Verify + uninstall lamp | Yes |
| Install Node for user | **No** (not our package manager) |
| Install desk without Rust | Not until release ELF exists |
| One script for “any machine” | Lie if it claims Node+desk+lamp |

## Unresolved questions

- Trusted Publisher still unbound on the package page (user dashboard).
- When to cut first desk GitHub release (cargo-dist) — unpaid, needs CI release job.
- Whether `latest` stays on `0.1.0-next.0` (npm set both tags on first publish).
