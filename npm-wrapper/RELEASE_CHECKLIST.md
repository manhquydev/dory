# Release checklist — `@manhquy/dory` npm-wrapper

Copied from flow-skill traps. Do not invent a token path.

## Trusted Publisher (one-time, npmjs dashboard, npm account `@manhquy`)

Same four fields as flow-skill, only `repo` changes:

| Field | Value | Not |
|---|---|---|
| owner (GitHub) | `manhquydev` | npm account `manhquy` |
| repo | `dory` | |
| workflow | `publish-npm-wrapper.yml` | display name, or path `.github/workflows/…` |
| environment | `npm-publish` | blank |

npm scope `@manhquy` ≠ GitHub owner. flow-skill lives on this same pair. If the GitHub owner field is `manhquy`, OIDC does not match `manhquydev/dory` (PUT 404; Sigstore can still sign).

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

## First package (paid trap — 2026-08-30)

Live OIDC run https://github.com/manhquydev/dory/actions/runs/33295896563 : Sigstore signed, then `PUT` **E404**. Package did not exist.

**Door 1b closed 2026-08-30:** laptop `npm publish --access public --tag next` (no `--provenance`) as npm user `manhquy`. Registry: `@manhquy/dory@0.1.0-next.0`. Bin `dory-serve` only. First publish also set dist-tag `latest` (npm default). Packument GET stayed 404 ~3 min after version + tarball + dist-tags were live — do not treat that lag as “publish failed.”

That semver is **spent**. Next OIDC / live bump is `0.1.0-next.1`. Bind Trusted Publisher on the **package** page (`https://www.npmjs.com/package/@manhquy/dory` → Settings): owner `manhquydev`, repo `dory`, workflow `publish-npm-wrapper.yml`, environment `npm-publish`, action **publish**. Revoke any token used for 1b. Never paste tokens into chat.

## Publish

Order is a gate. Do not skip.

1. SHA on `main`. CI `all-checks-passed` green. Owner: `.github/workflows/ci.yml`.
2. `npm-wrapper/package.json` version == dispatch/tag semver. **Not** the environment name `npm-publish`.
3. Dispatch dry-run, then **Approve** environment `npm-publish`:

```
gh workflow run "Publish npm-wrapper to npm (trusted publishing)" \
  --repo manhquydev/dory --ref main \
  -f version=0.1.0-next.0 -f dist_tag=next -f dry_run=true -f promote_to=none
```

4. Dry-run must print `Publishing … (dry-run)` and `+ @manhquy/dory@…`. Bin stays `dory-serve`.
5. Dispatch the same inputs with `dry_run=false`. Approve `npm-publish` again (each run waits).
6. Or push tag `npm@0.1.0-next.0` (pre-release → dist-tag `next`).
7. After live: `npm view @manhquy/dory@0.1.0-next.0` and `npm view @manhquy/dory dist-tags --json`.
8. Promote `next` → `latest` only from your shell (`npm dist-tag add …`). Not GHA. Then revoke the token.

Bump `npm-wrapper/package.json` version **before** the next live. Same SHA cannot republish the same semver.
