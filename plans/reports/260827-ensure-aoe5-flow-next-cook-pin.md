# COOK PIN — trap 17/23 after ck_impl write

**Verdict: PIN_PASS**

Gate: `scripts/dory-isolate-aoe5-flow-next.sh` exists (29307 bytes). Cook receipt not written yet. No cargo. No dory argv. Leftover 5 not folded.

## Leftover 5 path+sha mint (trap 23)

| Path | `git hash-object` | Mint |
|---|---|---|
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

Porcelain still ` M` ×5. `desk.rs` worktree == `HEAD:rust/src/desk.rs` `4c788562e4fdda10c8edd2878ed1fdd46050c218`. `git log -1 -- rust/` = `b544f5f`.

## ELF pins (trap 17, stat only)

| Binary | sha256 | Mint |
|---|---|---|
| `SIT_DORY` land inode | `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3` | MATCH |
| leftover `rust/target/debug/dory` | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` | MATCH unchanged |

mtime/size unchanged vs pre-write baseline. No cargo leftover. No cargo isolate.

## Script pin (hashes env, refuses leftover target)

- `LAND_SHA=2ef20730…` then `SIT_DORY_SHA="$(sha256sum "$SIT_DORY")"` vs pin (`:24`, `:535-538`). Also hashes `ISO_REAL/bin/dory` (`:714-715`).
- `case "$SIT_DORY" in "$REPO_TARGET"/*)` → `refuse: SIT_DORY is leftover rust/target` (`:17`, `:545-548`).
- Leftover ELF `sha256sum` vs `LEFTOVER_ELF_SHA=3ba0e3bc…` (`leftover_elf_stat_ok`, called start + after taxis).
- No hardcoded path string `land-4b70f79`.
- Leftover mint table is path+sha (`leftover_mint_ok`), not snap-then-MATCH.

Doors held: no cargo, no dory invoke, leftover 5 still unstaged mint.
