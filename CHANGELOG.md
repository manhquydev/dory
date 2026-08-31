# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Product versions follow [SemVer](https://semver.org/). Lamp npm dist-tag `latest` only points at a version **without** a hyphen. Preview uses `next`.

## [Unreleased]

Desk binary release is not shipped yet.

## [0.1.1] — 2026-08-31

Docs-only lamp. Same CLI as `0.1.0`. The tarball README now matches the proven commands.

### Changed

- README (EN/VI) says `http://127.0.0.1:7380/` exists only while the lamp is running.
- Uninstall does not stop a listening lamp. `cargo` does not put `dory` on `PATH`.
- Pin is `npx @manhquy/dory@0.1.1`.

## [0.1.0] — 2026-08-31

First stable lamp. `npx @manhquy/dory` follows this version.

### Added

- Default lamp start: current directory, no `serve` verb required.
- GitHub Release for this semver (lamp only).

### Changed

- Install command is `npx @manhquy/dory`. `--workspace /abs` is an override.
- `serve` remains an accepted alias.

### Lamp preview history (not `latest`)

- `0.1.0-next.2` — honest CLI/README; still a prerelease.
- `0.1.0-next.1` — first OIDC + provenance.
- `0.1.0-next.0` — first registry row; README taught a command that exits 2. Spent.

[Unreleased]: https://github.com/manhquydev/dory/compare/npm@0.1.1...HEAD
[0.1.1]: https://github.com/manhquydev/dory/releases/tag/npm@0.1.1
[0.1.0]: https://github.com/manhquydev/dory/releases/tag/npm@0.1.0
