# Research — PATH / spawn door

Date: 2026-08-27 09:40. Factory `w13:t13` / `w13:p2R`. Did not invoke `dory`.

## Live

- `~/.local/bin/dory` is a symlink (23 Aug 19:33) → `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory`.
- `type -a dory` repeats only that path (PATH lists `~/.local/bin` many times). No second binary on PATH.
- Default sock `/run/user/1000/dory/default/dory.sock` absent.
- Leftover target strings: `workspace_live_cwd` ×59. Isolate `land-4b70f79` debug: ×0.

## HEAD spawn door

`git show HEAD:rust/src/attach.rs:332-370` `ensure_server`: if `ping()` miss, `Command::new(current_exe).arg("server")` + `setsid` + spawn. No named `spawn_server` on HEAD. Leftover `attach.rs` line numbers differ — do not cite leftover as land.

Bare factory `dory` / `dory attach` (factory XDG, `DORY_SOCKET` unset, sock absent) mints default. **Cook must never invoke `dory`.**

## Isolate

`scripts/dory-isolate-flock-prompt.sh:210-214` refuses `SIT_DORY` equal to `realpath ~/.local/bin/dory`. After unlink, that realpath is empty — refuse stays inert (good). Flock still pins `ISO_REAL/bin/dory` → `SIT_DORY`.

Hop `scripts/dory-flock-hop.sh` still PATH `dory` and requires factory sock connectable. Do not exec hop.

## Conclusion

Unlink the symlink. Do not retarget. Do not run leftover target by full path. Do not cargo leftover tree.
