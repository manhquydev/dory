# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Product versions follow [SemVer](https://semver.org/). Lamp npm dist-tag `latest` only points at a version **without** a hyphen. Preview uses `next`.

## [Unreleased]

Desk binary release is not shipped yet.

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

[Unreleased]: https://github.com/manhquydev/dory/compare/npm@0.1.0...HEAD
[0.1.0]: https://github.com/manhquydev/dory/releases/tag/npm@0.1.0
