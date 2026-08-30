# Release checklist — `@manhquy/dory` npm-wrapper

Copied from flow-skill traps. Do not invent a token path.

## Trusted Publisher (one-time, npmjs dashboard, npm account `@manhquy`)

Same four fields as flow-skill, only `repo` changes:

| Field | Value |
|---|---|
| owner | `manhquydev` |
| repo | `dory` |
| workflow | `publish-npm-wrapper.yml` |
| environment | `npm-publish` |

Allowed action: **publish** (required after 2026-05-20).

Repo must stay **public** (OIDC audience). Environment `npm-publish` has required reviewer `manhquydev`.

## Do not

- Laptop `npm publish --provenance` (unsupported CI).
- `NPM_TOKEN` / committed `.npmrc`.
- Guard `NODE_AUTH_TOKEN` empty (setup-node OIDC fills it — flow run 29554226397).
- `always-auth: true` on setup-node.
- Tag `v*` expecting npm (wrong shape). Use `npm@X.Y.Z`.
- Pre-release → dist-tag `latest`.
- `npm dist-tag add` in GHA (OIDC → E401). Promote from your shell with a granular bypass-2FA token, then revoke. Never paste the token into chat.
- Bind bin `dory` (desk SKU). This package is `dory-serve` only.
- Publish from repo root `package.json` (`private: true`, name `dory` taken).

## Publish

1. `npm-wrapper/package.json` version == tag semver.
2. Push tag `npm@0.1.0-next.0` → workflow dist-tag `next`.
3. Or Actions → workflow_dispatch → `dry_run=true` first.
4. After first live publish: `npm view @manhquy/dory dist-tags --json`.
