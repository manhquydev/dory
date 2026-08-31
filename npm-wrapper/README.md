# `@manhquy/dory`

[English](README.md) · [Tiếng Việt](README.vi.md)

Journal **lamp** for [Dory](https://github.com/manhquydev/dory) (Session OS). This package is **not** the desk.

[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

| | |
|---|---|
| Bin | `dory-serve` only |
| Node | `>=22.14.0` (you install Node; this package does not) |
| Desk command | `dory` — Rust binary, built from the repo, not this tarball |

Typing `dory` after `npm install` will not open the desk. Do not install the unscoped npm name `dory`; that is a different product.

## Install

```bash
npx @manhquy/dory
```

Run from the session directory. Open `http://127.0.0.1:7380/`. Another folder: `--workspace /abs`. Pin: `npx @manhquy/dory@0.1.0`. Preview: `@next`.

Need Node first? [nodejs.org](https://nodejs.org/en/download) or [fnm](https://github.com/Schniz/fnm).

## Verify and uninstall

```bash
npm view @manhquy/dory name version bin
# bin must be { "dory-serve": "bin/dory-serve.js" } only

npm uninstall -g @manhquy/dory
```

From a clone of the repo:

```bash
bash scripts/dory-lamp-doctor.sh
```

The doctor detects Node and PATH collisions. It never installs Node and never binds `dory`.

## Repository

Source, desk build, contributing, and security policy:

**https://github.com/manhquydev/dory**

- [Contributing](https://github.com/manhquydev/dory/blob/main/CONTRIBUTING.md)
- [Security](https://github.com/manhquydev/dory/blob/main/SECURITY.md)
- [Code of Conduct](https://github.com/manhquydev/dory/blob/main/CODE_OF_CONDUCT.md)
- [Contributors](https://github.com/manhquydev/dory/graphs/contributors)

## License

[MIT](https://github.com/manhquydev/dory/blob/main/LICENSE) © 2026 manhquy and Dory contributors.
