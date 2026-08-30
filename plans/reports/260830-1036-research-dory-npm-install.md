---
type: research
date: 2026-08-30
time: 10:36
topic: dory-npm-install
head: fea7110
conducted: 2026-08-30
sources: 5 web + live npm view + HEAD package.json
---

# Research Report: Install Dory via npm

## Executive Summary

Today’s `package.json` is a **Node journal lamp**, not the desk. `bin.dory` → `bin/dory.js` → `src/cli.js` accepts only `serve --workspace`. Land README: gõ `dory` opens the **Rust desk**. `npm install -g` of this tree would put the lamp on PATH as `dory` and steal the SKU.

`private: true` blocks `npm publish`. Unscoped name `dory` is already 0.1.15 on npmjs (Surge blog). `@clidey/dory` and `@getdory/cli` both ship a `dory` bin. Publish later must be **scoped** (`@manhquydev/dory` or similar) and must not map `bin.dory` to the lamp.

Desk-via-npm is a later cook: prebuilt Rust per platform (`optionalDependencies`, cargo-npm / cargo-dist), not `npm i` of this private GitHub repo as-is.

## Research Methodology

- Sources consulted: 5 web searches + `npm view` + HEAD `package.json` / `bin/dory.js` / `src/cli.js` / land README
- Date range: npm docs v9 (current) through 2026-07 (`@clidey/dory`); cargo-dist npm installer book
- Key terms: package.json `bin` `private` `publishConfig`; rust CLI npm `optionalDependencies`; GitHub Packages vs `git+https`; private repo npm install

## Key Findings

### 1. Technology Overview

Two engines already in the repo:

| Engine | Entry | SKU |
|---|---|---|
| Workplace OS (desk) | Rust crate `dory` | Bare `dory` |
| Session OS (lamp) | `node bin/dory.js serve` | Explicit Node, not PATH `dory` |

npm only knows the second. GitHub `manhquydev/dory` (private) already has the first as source (`rust/`), not as a released binary.

### 2. Current State & Trends

| Name | Version | `bin` | Collision |
|---|---|---|---|
| `dory` (npmjs) | 0.1.15 | (blog platform) | **Unscoped name taken** |
| `@clidey/dory` | 1.0.3 | `dory` | PATH `dory` |
| `@getdory/cli` | 0.1.7 | `dory` | PATH `dory` |

Official npm: `"private": true` → publish refused. `bin` is a map of command → file. Scoped packages should set `bin` as an object (command `dory`, not `@scope/dory`). Public scoped packages need `publishConfig.access: public`.

Rust-on-npm trend: platform packages as `optionalDependencies` (esbuild pattern). `cargo-npm` generates those without postinstall download. `cargo-dist` npm installer historically downloads archives (axios tree); moving toward bundled binaries.

### 3. Best Practices

1. Keep `"private": true` until a scoped name and desk-vs-lamp bin contract are cooked.
2. Never `npm publish` unscoped `dory`.
3. If users need the **lamp** from git (private): `npm install git+ssh://git@github.com/manhquydev/dory.git` after SSH/gh auth. Do **not** `-g` if they also want desk `dory`.
4. If users need the **desk**: cargo/isolate ELF today; later scoped package with platform optionalDeps. Not this wave.
5. GitHub Packages (`npm.pkg.github.com`) needs `@scope`, `.npmrc` auth. Extra moving part vs git URL. Defer.
6. Never put a PAT in `package.json` or a committed `.npmrc`.

### 4. Security Considerations

- Private repo + `npm i git+https://TOKEN@...` leaks if the URL is committed.
- Fine-grained PAT, contents read, single repo — only in the installer machine env, not in git.
- Publishing leftover working `README.md` / dirty rust as the npm tarball would fold leftover. `files` allow-list required before any publish cook.
- `npm pack` / `npm publish --dry-run` must be run against a clean land tree, not leftover WT.

### 5. Performance Insights

- Git install clones the whole repo (`rust/vendor`, plans). Heavy for a lamp-only bin.
- Platform optionalDeps ship one ELF. cargo-dist downloader adds JS deps unless bundled.
- Local `npm link` of this tree is the worst SKU: global `dory` → lamp.

## Comparative Analysis

| Path | Installs | Desk? | Lamp? | Auth | Verdict now |
|---|---|---|---|---|---|
| `npm i -g` after flipping `private` + name `dory` | Impossible (name taken) + SKU theft | No | Yes | npmjs | Reject |
| `npm i -g @manhquydev/dory` lamp-only | Lamp on `dory` PATH | No | Yes | npmjs public later | Reject until bin ≠ desk |
| `npm i git+ssh://…manhquydev/dory.git` (local) | Source + lamp bin in that project | No | Yes | SSH/gh | OK for lamp spike only |
| GitHub Packages scoped | Same as npmjs scoped | Depends on bin | Depends | `.npmrc` | Defer |
| cargo-npm / cargo-dist later | Desk ELF | Yes | No | npmjs scoped | Next cook if named |
| Clone + `cargo build --manifest-path rust/Cargo.toml` | Desk | Yes | No | git | Current honest desk install; **not leftover tree** |

## Implementation Recommendations

### Quick Start (honest, today)

Desk (operator SKU):

```bash
git clone git@github.com:manhquydev/dory.git
# use HEAD rust, not leftover WT
cargo build --manifest-path rust/Cargo.toml --release
# put rust/target/release/dory on PATH — never leftover Downloads/flow/dory dirty ELF
```

Lamp (not desk):

```bash
# from a checkout you can read
node bin/dory.js serve --workspace /abs
# or, project-local only:
npm install git+ssh://git@github.com/manhquydev/dory.git
npx dory serve --workspace /abs
```

Do not `npm install -g` this package.

### Code Examples

HEAD `package.json` (do not publish):

```json
{
  "name": "dory",
  "private": true,
  "bin": { "dory": "./bin/dory.js" }
}
```

Later cook (not this wave) would look like:

```json
{
  "name": "@manhquydev/dory",
  "private": false,
  "publishConfig": { "access": "public" },
  "bin": { "dory": "./npm/run.js" },
  "optionalDependencies": {
    "@manhquydev/dory-linux-x64": "0.1.0"
  }
}
```

`npm/run.js` must exec the Rust desk ELF, not `src/cli.js`.

### Common Pitfalls

| Pitfall | What happens |
|---|---|
| `npm i -g` current tree | PATH `dory` = lamp; desk vanishes |
| Publish name `dory` | npmjs 409; name owned by other product |
| `git add -A` then pack | leftover 5 + eval in tarball |
| PAT in git URL | credential leak |
| cargo leftover dirty tree | fold / wrong ELF |

## Resources & References

### Official Documentation

- [npm package.json — bin, private, publishConfig](https://docs.npmjs.com/cli/v9/configuring-npm/package-json/)
- [GitHub: managing fine-grained PATs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
- [cargo-dist npm installer](https://axodotdev.github.io/cargo-dist/book/installers/npm.html)
- [cargo-npm (platform optionalDeps)](https://github.com/abemedia/cargo-npm)

### Further Reading

- Land README: `git show HEAD:README.md` — hai cửa (desk vs `node bin/dory.js serve`)
- CHARTER: lệnh / kho = `dory`; họ Flow
- npmjs: `dory@0.1.15`, `@clidey/dory`, `@getdory/cli`

## Appendices

### A. Glossary

| Term | Meaning |
|---|---|
| Desk | Rust TUI; SKU `dory` |
| Lamp | Node `:7380` journal projection |
| Leftover 5 | Dirty WT README + 4 rust files; mint; do not fold |
| Scoped name | `@user/pkg` on npmjs or GitHub Packages |

### B. Version Compatibility Matrix

| Surface | Now | npm-ready? |
|---|---|---|
| Node lamp | `private: true`, bin=lamp | Install from git only; not `-g` |
| Rust desk | source on GitHub; ELF local | No |
| Isolate taxi scripts | this wave ships missing paid scripts | Not an npm concern |

### C. Raw notes

Searches: npm name `dory`; package.json bin/private; rust-via-npm optionalDeps; GitHub Packages vs git+https; private repo npx/token. Live `npm view` 2026-08-30.

## Unresolved questions

- Exact scoped name (`@manhquydev/dory` vs `@flow-*/dory`) — unpaid until a publish cook.
- Whether lamp keeps a **different** bin (`dory-serve`) when desk ships on npm.
- Public vs private npmjs later (repo is private today).
