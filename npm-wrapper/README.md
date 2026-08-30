# `@manhquy/dory`

Journal **lamp** (Session OS). Not the desk. Needs Node `>=22.14.0`. This package does **not** install Node.

```bash
# pin; npx without a version follows latest
npx @manhquy/dory@0.1.0-next.1 dory-serve -- serve --workspace /abs
```

Then open `http://127.0.0.1:7380/`. Gõ `dory` is the Rust desk (not this package). Bin here is **`dory-serve` only**.

## Verify / uninstall

```bash
bash scripts/dory-lamp-doctor.sh          # detect; never installs Node
npm view @manhquy/dory name version bin
npm uninstall -g @manhquy/dory           # lamp only; does not touch desk ELF
```

Never `npm i -g dory` (unscoped name is another product). Never expect this package to put `dory` on PATH.

Missing Node: https://nodejs.org/en/download or [fnm](https://github.com/Schniz/fnm). The doctor prints that and exits; it does not bootstrap a toolchain.

Publish: GitHub Actions OIDC — same shape as `@manhquy/flow-skill`. See `RELEASE_CHECKLIST.md`.
