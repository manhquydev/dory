## Summary

<!-- What changed and why. English preferred; Vietnamese is welcome. -->

## Engine

- [ ] Desk (`rust/`)
- [ ] Lamp (`npm-wrapper/`)
- [ ] Docs / community files only

## Checks

- [ ] Tests added or updated when behavior changed
- [ ] `cargo test --manifest-path rust/Cargo.toml --locked` (if desk)
- [ ] `cd npm-wrapper && npm test` (if lamp)
- [ ] Lamp `bin` is still `dory-serve` only (no `dory`)
- [ ] No secrets, tokens, or `.env` files

## Notes for reviewers
