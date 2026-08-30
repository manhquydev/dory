---
type: advise
date: 2026-08-30
time: 13:41
topic: user-install
confirmed: prior sitting (npm first) + 2026-08-30 continue
research: 260830-1341-research-user-install.md
---

# Advise — user install on non-ideal machines

## Reframing (confirmed)

**Problem:** Users will not have a factory Node/PATH. They will type `npm i -g dory` or expect one script to “just work.” Lamp is on npm; desk is not a released ELF. Installing Node for them is not a Dory feature.

**Exact requirements**

1. Honest two-SKU install page: lamp = `@manhquy/dory` / `dory-serve`; desk = Rust `dory`.
2. Verify install without claiming desk.
3. Uninstall lamp without touching leftover ELF / isolate sit.
4. When Node missing or `<22.14.0`, fail with nodejs.org / fnm — do not install Node.
5. Refuse unscoped `dory` and any `bin.dory` on the lamp package.

**Goals:** A stranger can get the lamp running if they already have Node 22.14+, or know exactly what to install if they do not. PATH never shows lamp as `dory`.

**Non-goals:** cargo-dist desk release this sitting. Node SEA. Binding Trusted Publisher (user dashboard). Leftover fold. Windows desk. Darwin occupant classify. `latest` promote debate.

**Constraints:** leftover 5 stay `M`. No `git add -A`. No token in chat/repo. Two engines stay two. Factory no default sock / no sit `t13`.

## 1. Verdict

The “setup hệ thống cài Node giúp user” idea is the wrong ceiling. Best we can do honestly: a **doctor** (detect / install-lamp-only / verify / uninstall) plus docs. Shipping a Node bootstrapper would make us a worse nvm and a supply-chain liability. Desk “one curl” waits on GitHub release artifacts we do not have.

## 2. What you should do

1. Keep lamp default as pinned `npx @manhquy/dory@0.1.0-next.0` (no `-g`).
2. Ship `scripts/dory-lamp-doctor.sh`: node version, registry, PATH collisions, optional `--install` / `--uninstall` of **only** `@manhquy/dory`.
3. Point missing Node at https://nodejs.org/en/download or fnm — print, do not run their installer.
4. Bind Trusted Publisher on the existing package page (you). Next lamp bump is `0.1.0-next.1`.
5. Later cook: cargo-dist `shell` for desk ELF, separate from this package.

## 3. What you shouldn't do

- `curl | sh` NodeSource / fnm from a Dory script.
- `npm i -g dory` (unscoped, other product).
- Map lamp `bin` to `dory`.
- `npm i -g` on this factory (cursor-agent prefix).
- cargo-dist npm installer as the lamp package (would fetch desk ELF under lamp name).
- Treat 3-minute packument 404 as unpublished.

## 4. What could be better / more efficient

1. **Doctor + docs** — this wave. Highest impact, lowest risk.
2. **Pin version in every public command** — avoids npx engines fallback later.
3. **cargo-dist desk** — when we want users without Node. Needs release CI.
4. **Node SEA** — lamp without system Node. New SKU, still not desk. Defer.

## 5. My take and how to get there

Do A now: doctor + npm-wrapper README + docs route. Do not start cargo-dist until a named cook owns GH release ELF. You only open npmjs Settings for TP.

## 6. Benefits

- Matches confirmed “npm first, then install UX.”
- Users without Node get a clear fail, not a half-installed runtime.
- Uninstall/verify is mechanical (`npm uninstall -g @manhquy/dory`, `type -a`).
- Desk SKU stays `dory`.

## 7. Trade-offs

- We will **not** auto-install Node. Users on empty VMs still do one official install. Switch away later = write a bootstrapper and inherit their Node forever.
- First npm publish set `latest` = `0.1.0-next.0`. `npx @manhquy/dory` without pin works today; still pin in docs.
- Recommendation stops being right when a **desk** ELF is the product users download (then cargo-dist, not this doctor).

## 8. Work checklist & success metrics

- [x] Research how far an installer can go (this file’s sibling).
- [x] Brainstorm contract: doctor now, cargo-dist later, never Node bootstrap.
- [x] `scripts/dory-lamp-doctor.sh` (no Node install, no `bin.dory`).
- [x] npm-wrapper + docs/README install door.
- [ ] User: bind TP on https://www.npmjs.com/package/@manhquy/dory
- [ ] Later cook: cargo-dist desk (not this wave).

**Success metrics**

- `bash scripts/dory-lamp-doctor.sh` exit 0 on a machine with Node `>=22.14.0` and live `@manhquy/dory`.
- Same script exit 1 when `node` missing; stdout contains `nodejs.org`; does not download Node.
- `npm view @manhquy/dory bin` = `{ 'dory-serve': 'bin/dory-serve.js' }` only.
- `type -a dory` after lamp install still not a lamp shim (no `bin.dory`).
- Leftover README hash-object still `68190a5ffa073c082aa318aad5ed032e13cc90e3`.
