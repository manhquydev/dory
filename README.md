# Dory

[English](README.md) · [Tiếng Việt](README.vi.md)

Local **Agent Operating Environment** for multi-agent software work. Type `dory` for the desk. The journal lamp is a separate engine.

[![CI](https://github.com/manhquydev/dory/actions/workflows/ci.yml/badge.svg)](https://github.com/manhquydev/dory/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@manhquy/dory.svg)](https://www.npmjs.com/package/@manhquy/dory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Dory is not a new version of [flow-skill](https://github.com/manhquydev/flow-skill) (the judge) or flow-deck (the wall board). Those stay their own houses. Dory is the place you **do the work**.

**This project is public and open to contribution.** Issues, translations, tests, and small patches are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) and the [Code of Conduct](CODE_OF_CONDUCT.md).

## Two engines

| Engine | Command | For | Needs |
|---|---|---|---|
| **Desk** (Workplace OS) | `dory` | Windows → tabs → live PTY panes | A real terminal, Rust binary |
| **Lamp** (Session OS) | `dory-serve` | Journal in the browser at `http://127.0.0.1:7380/` | Node `>=22.14.0` |

The desk is the default when you type `dory`. The lamp is a different door. The npm package **does not** put `dory` on your `PATH`.

```
Desk keeps processes alive.   Lamp projects the session journal.
Leaving the UI ≠ stopping work.
```

## Install

### Lamp (on npm today)

```bash
npx @manhquy/dory
```

Run from the session directory. Open `http://127.0.0.1:7380/`. Another folder: `--workspace /abs`. Pin: `npx @manhquy/dory@0.1.0`. Preview: `@next`.

```bash
# check Node / registry / PATH collisions — does not install Node
bash scripts/dory-lamp-doctor.sh

npm uninstall -g @manhquy/dory    # lamp only
```

Do **not** run `npm i -g dory` (unscoped). That name is another product.

Missing Node: install it yourself from [nodejs.org](https://nodejs.org/en/download) or [fnm](https://github.com/Schniz/fnm). Dory will not bootstrap a toolchain.

Package docs: [`npm-wrapper/README.md`](npm-wrapper/README.md) · [Tiếng Việt](npm-wrapper/README.vi.md)

### Desk (from source)

There is no binary release yet. Build the Rust crate:

```bash
git clone https://github.com/manhquydev/dory.git
cd dory
cargo build --manifest-path rust/Cargo.toml --release
# put rust/target/release/dory on your PATH
dory
```

`dory server` then `dory` opens the desk: Spaces and Agents on the left, tab chips, live panes on the right. Empty panes use `$SHELL` (with rc). A new tab follows the focused pane’s cwd; a new window follows the directory where you typed `dory`. If an old server still uses `--norc`: `dory server stop` then `dory`.

## Desk keys

Prefix is `Ctrl-b`. There are no bare `x` / `1` / `w` keys.

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

`dory attach --plain` is a raw PTY client (no sidebar). Inside a pane (`DORY_ENV=1`) the occupant uses the CLI; they do not sit the desk. `dory server stop` is what actually stops work.

The Rust binary treats `dory serve` as a reminder that the lamp is Node, not the desk.

## Status

Early public tree. The desk stack is Rust. The lamp is the published npm package `@manhquy/dory` (`dory-serve` only). Windows desk CI is a notice job; Darwin occupant `done`/`idle` classification is still unpaid.

| Area | State | Owner |
|---|---|---|
| Product why | Closed | [CHARTER.md](CHARTER.md) |
| Skill / CLI / socket | Accepted | `skills/dory/` · `rust/` |
| Lamp on npm | `@manhquy/dory@0.1.0` | `npm-wrapper/` |
| Desk binary release | Not yet | build from `rust/` |

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
