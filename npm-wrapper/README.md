# `@manhquy/dory`

[English](README.md) · [Tiếng Việt](README.vi.md)

Browser journal for [Dory](https://github.com/manhquydev/dory). This package is the **lamp**. It is **not** the desk.

[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

| | |
|---|---|
| Command | `dory-serve` only |
| Node | `>=22.14.0` (you install Node; this package does not) |
| Desk | `dory` — Rust binary, built from the GitHub repo, not this tarball |

After `npm install`, typing `dory` will not open the desk. Do not install the unscoped npm name `dory`; that is a different product.

## Run

In the session folder:

```bash
npx @manhquy/dory
```

The process prints a line that includes `http://127.0.0.1:7380/`. Open that URL **while the command is still running**. The page is not there until then. Stop with Ctrl-C. If the port stays open, end the Node process that owns `7380`.

Another folder (absolute path): `--workspace /abs`. Pin this release: `npx @manhquy/dory@0.1.1`. Preview: `@next`.

Need Node first? [nodejs.org](https://nodejs.org/en/download) or [fnm](https://github.com/Schniz/fnm).

## Check and uninstall

```bash
npm view @manhquy/dory name version bin
# bin must be { "dory-serve": "bin/dory-serve.js" } only
```

If you once installed the package globally:

```bash
npm uninstall -g @manhquy/dory
```

That only removes a global install. It does not stop a lamp that is already running.

From the repo root of a clone (not this directory):

```bash
bash scripts/dory-lamp-doctor.sh
```

The script checks Node and PATH collisions. It never installs Node and never binds `dory`. A global install of `@manhquy/dory` is typed as `dory-serve`, never `dory`.

## Repository

Source, desk build, contributing, and security policy:

**https://github.com/manhquydev/dory**

- [Contributing](https://github.com/manhquydev/dory/blob/main/CONTRIBUTING.md)
- [Security](https://github.com/manhquydev/dory/blob/main/SECURITY.md)
- [Code of Conduct](https://github.com/manhquydev/dory/blob/main/CODE_OF_CONDUCT.md)
- [Contributors](https://github.com/manhquydev/dory/graphs/contributors)

## License

[MIT](https://github.com/manhquydev/dory/blob/main/LICENSE) © 2026 manhquy and Dory contributors.
