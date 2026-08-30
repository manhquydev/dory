---
type: research
date: 2026-08-30
time: 10:46
topic: npm-oidc-connect
head: 06f9c7f
remote: https://github.com/manhquydev/dory
visibility: public
---

# Connect Dory npm publish to Flow’s live system

GitHub MCP is **not registered** in this Cursor session. Live checks used `gh` against the same GitHub API.

## Contract

| Field | Closed |
|---|---|
| **Outcome** | Repo public like flow-skill/flow-deck. Publish env + workflow copy Flow’s OIDC shape. First live `npm publish` waits for Trusted Publisher on npmjs. |
| **Constraints** | No leftover 5. No `git add -A`. No `NPM_TOKEN`. No bin `dory`. Root `package.json` stays `private: true`. No laptop publish this wave. |
| **Non-goals** | Live npm publish. Dist-tag promote. Desk ELF via cargo-npm. Leftover fold. |
| **Acceptance** | `visibility=PUBLIC`. Env `npm-publish` + required reviewer. Workflow filename `publish-npm-wrapper.yml`. Package `@manhquy/dory` / bin `dory-serve`. Wrapper tests pass. Leftover README still `68190a5f…`. |

## What Flow actually runs (live)

`manhquydev/flow-skill` PUBLIC. Last success: tag `npm@0.7.1-next.0` → workflow `publish-npm-wrapper.yml` → env `npm-publish` (required reviewer `manhquydev`) → `npm publish --provenance --access public`.

Package: `@manhquy/flow-skill`. Dist-tags live: `latest` 0.7.0, `next` 0.7.1-next.0.

Trusted Publisher (checklist, confirmed rc.2+):

`owner=manhquydev` `repo=flow-skill` `workflow=publish-npm-wrapper.yml` `environment=npm-publish`

Traps Flow already paid for (do not re-learn):

| Trap | Fail |
|---|---|
| Node 20 / npm 10 | OIDC 404 |
| `always-auth: true` | Breaks OIDC |
| Guard `NODE_AUTH_TOKEN` empty | Blocks every run (setup-node fills it) |
| Tag `v*` | Does not publish |
| `promote_to` / `npm dist-tag add` in GHA | E401 |
| Laptop `--provenance` | Unsupported CI |
| Pre-release → `latest` | Workflow FAIL |

Account `@manhquy` is passkey-only. Dist-tag still needs a granular bypass-2FA token **in your shell**, then revoke. Never paste into chat.

## Dory wiring (this wave)

| Piece | Value |
|---|---|
| Repo | PUBLIC `https://github.com/manhquydev/dory` |
| Env | `npm-publish` + required reviewer `manhquydev` |
| Workflow | `.github/workflows/publish-npm-wrapper.yml` |
| Package | `@manhquy/dory@0.1.0-next.0` (404 before first publish — name free) |
| Bin | `dory-serve` only |
| Dispatch default | `dry_run=true` |

## Operator — one click left (npmjs)

npmjs → `@manhquy` → Trusted Publisher for **new** package `@manhquy/dory`:

1. owner `manhquydev`
2. repo `dory`
3. workflow `publish-npm-wrapper.yml`
4. environment `npm-publish`
5. action **publish**

Then Actions → Publish npm-wrapper → `dry_run=true` / version `0.1.0-next.0`. After dry-run green, `dry_run=false` **or** push tag `npm@0.1.0-next.0`.

Do not `npm publish` from the leftover worktree.

## Unresolved

- First-package create: if npmjs requires the package to exist before TP binds, first publish may need the same laptop granular-token bootstrap Flow used for `rc.1`. Try OIDC dry-run first.
- GitHub MCP still absent here; re-check with the plugin if you want the same reads inside MCP.
