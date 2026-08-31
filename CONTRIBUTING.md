# Contributing to Dory

Thank you for wanting to help. English is the source language for new docs and code comments. [Tiếng Việt: CONTRIBUTING.vi.md](CONTRIBUTING.vi.md).

Please read the [Code of Conduct](CODE_OF_CONDUCT.md). Security issues go to [SECURITY.md](SECURITY.md), not public issues.

## Two engines

Dory is one product with two engines. Keep them separate.

| Engine | What users type | Source |
|---|---|---|
| **Desk** (Workplace OS) | `dory` | `rust/` — windows → tabs → live PTY panes |
| **Lamp** (Session OS) | `dory-serve` | `npm-wrapper/` — journal at `http://127.0.0.1:7380/` |

Do **not** add a `dory` bin to the npm package. `@manhquy/dory` is lamp-only. The unscoped npm name `dory` is someone else's package.

## Ways to help

- Bug reports and reproductions (desk or lamp — say which)
- Docs, translations (`README.vi.md`, `CONTRIBUTING.vi.md`)
- Tests for an existing verb or pane behavior
- Lamp UX and accessibility on the journal page
- Desk keybindings, layout, and occupant CLI (no `--kind` flag)

A small, tested change beats a large redesign. Open an issue before a large refactor.

## Development

### Desk

You need a recent stable Rust toolchain (CI uses `dtolnay/rust-toolchain@stable`; crate edition is 2024).

```bash
git clone https://github.com/manhquydev/dory.git
cd dory
cargo test --manifest-path rust/Cargo.toml --offline --locked
cargo build --manifest-path rust/Cargo.toml --release
```

On macOS, short socket paths matter (`TMPDIR=/tmp` in CI). Some occupant `done`/`idle` tests are Linux-only.

### Lamp

Node `>=22.14.0`.

```bash
cd npm-wrapper
npm ci
npm test
```

The doctor script checks a machine; it never installs Node:

```bash
bash scripts/dory-lamp-doctor.sh
```

## Pull requests

1. Fork and branch from `main`.
2. Keep the change focused. Do not commit secrets, tokens, or `.env` files.
3. Add or update tests when you change behavior.
4. Run the desk and/or lamp tests that match the files you touched.
5. Fill in the PR template.

Maintainers will not merge a change that maps lamp `bin` to `dory`, publishes the unscoped name `dory`, or adds a language-toolchain installer.

## License

By contributing, you agree that your work is licensed under the [MIT License](LICENSE) of this repository.
