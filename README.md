# Dory

[English](README.md) · [Tiếng Việt](README.vi.md)

A local workplace for multi-agent software. The desk command is `dory` (Rust binary, not the npm package). The journal **lamp** is a different command.

[![CI](https://github.com/manhquydev/dory/actions/workflows/ci.yml/badge.svg)](https://github.com/manhquydev/dory/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Dory is not a new version of [flow-skill](https://github.com/manhquydev/flow-skill) (the judge) or flow-deck (the wall board). Those stay their own houses. Dory is the place you **do the work**.

**This project is public and open to contribution.** Issues, translations, tests, and small patches are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) and the [Code of Conduct](CODE_OF_CONDUCT.md).

## Two parts

| Part | Command | What it does | Needs |
|---|---|---|---|
| **desk** | `dory` | Windows → tabs → live terminal panes | A real terminal and a Rust binary |
| **lamp** | `dory-serve` | Session journal in the browser | Node `>=22.14.0` |

The desk command is `dory`. The lamp does not replace the desk. The npm package **does not** put `dory` on your `PATH`.

```
desk keeps processes running when you close the UI.   lamp only shows the session journal.
Leaving the UI ≠ stopping work.
```

## Install

### Journal in the browser (npm)

Published as [`@manhquy/dory`](https://www.npmjs.com/package/@manhquy/dory). Run this **in the session folder**:

```bash
npx @manhquy/dory
```

The process prints a line that includes `http://127.0.0.1:7380/`. Open that URL **while the command is still running**. The page is not there until then. Stop with Ctrl-C. If the port stays open, end the Node process that owns `7380`.

Another folder (absolute path): `--workspace /abs`. Pin this release: `npx @manhquy/dory@0.1.1`. Preview: `@next`.

From the repo root of a clone (this script is not inside the npm tarball):

```bash
# checks Node, registry, and PATH collisions — does not install Node
bash scripts/dory-lamp-doctor.sh
```

A global install of `@manhquy/dory` is typed as `dory-serve`, never `dory`.

If you once installed the package globally:

```bash
npm uninstall -g @manhquy/dory
```

That only removes a global install. It does not stop a lamp that is already running.

Do **not** run `npm i -g dory` (no `@manhquy`). That name is another product.

Need Node first? Install it yourself from [nodejs.org](https://nodejs.org/en/download) or [fnm](https://github.com/Schniz/fnm). Dory does not install Node or Rust for you.

Package docs: [`npm-wrapper/README.md`](npm-wrapper/README.md) · [Tiếng Việt](npm-wrapper/README.vi.md)

### Desk (build from source)

There is no binary download yet. You need a recent Rust toolchain.

```bash
git clone https://github.com/manhquydev/dory.git
cd dory
cargo build --manifest-path rust/Cargo.toml --release
```

The binary is `rust/target/release/dory`. It is **not** on `PATH` until you put it there. Do not type `dory` until that is done.

Once `dory` is on your `PATH`: `dory server` then `dory` shows Spaces and Agents on the left, tab chips, and live panes on the right. Empty panes use `$SHELL` (with rc). A new tab follows the focused pane’s cwd; a new window follows the directory where you typed `dory`. If an old server still uses `--norc`: `dory server stop` then `dory`.

## Desk keys

The prefix is `Ctrl-b`. Do not press `x` / `1` / `w` alone.

| Key / mouse | Action |
|---|---|
| Click card / agent / chip / pane | Focus |
| Drag the split | Resize |
| Drag-select ≥ 2 cells on a tile | Copy (OSC 52; footer `copied` = sent) |
| `Ctrl-b h/j/k/l` | Move focus |
| `Ctrl-b n` / `p` / `1..9` | Tabs in this window |
| `Ctrl-b c` | New tab |
| `Ctrl-b v` / `-` | Split right / below |
| `Ctrl-b z` | Zoom focused pane; siblings stay alive |
| `Ctrl-b w` | Window picker (does not create) |
| `Ctrl-b Shift-n` | New window |
| `Ctrl-b b` | Sidebar 26↔4↔0 |
| `Ctrl-b Ctrl-b` | Send `C-b` into the pane |
| `Ctrl-b x` | Close pane (confirm if last on the tab) |
| `Ctrl-b Shift-x` | Close tab |
| `Ctrl-b Shift-d` | Close window |
| `Ctrl-b q` or `Ctrl-b d` | Leave the UI; PTYs stay up |
| `Ctrl-b ?` | Key chart |

`dory attach --plain` is a PTY client with no sidebar. Inside a pane (`DORY_ENV=1`) the occupant CLI talks to the server; it does not open the desk UI. `dory server stop` is what actually stops work.

The Rust binary treats `dory serve` as a reminder that the browser journal is Node, not the desk.

## Status

Early public tree. The desk is Rust. The lamp is the npm package `@manhquy/dory` (`dory-serve` only). Windows desk CI only posts a notice; it does not test the desk. Darwin occupant `done`/`idle` classification is not implemented yet.

| Area | State | Owner |
|---|---|---|
| Product why | Closed | [CHARTER.md](CHARTER.md) |
| Skill / CLI / socket | Accepted | `skills/dory/` · `rust/` |
| Journal on npm | `@manhquy/dory@0.1.1` | `npm-wrapper/` |
| Desk binary download | Not yet | build from `rust/` |

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) ([Tiếng Việt](CONTRIBUTING.vi.md)).

- [Open an issue](https://github.com/manhquydev/dory/issues/new/choose)
- [Open a pull request](https://github.com/manhquydev/dory/pulls)
- Security: [SECURITY.md](SECURITY.md) — private report, not a public issue

By contributing you agree your work is licensed under MIT.

## Contributors

This project exists because people show up. The GitHub [contributors graph](https://github.com/manhquydev/dory/graphs/contributors) is the live list. First-time PRs are welcome — docs and tests count.

## Security

See [SECURITY.md](SECURITY.md). Do not paste tokens into issues or pull requests.

## License

[MIT](LICENSE) © 2026 manhquy and [Dory contributors](https://github.com/manhquydev/dory/graphs/contributors).

Vendored `rust/vendor/portable-pty` keeps its own license file.
