# TEST — increment C independent

You are `c_test` on a tab that is **not** cook. Skills ON (ak:test).

Read:
- `/home/manhquy/Downloads/flow/dory/plans/260827-1032-ensure-server-no-auto-spawn/phase-02-independent-no-spawn-test.md`
- cook receipt `/home/manhquy/Downloads/flow/dory/plans/reports/260827-ensure-server-no-spawn-cook.md`

Do **not** trust the cook receipt. Re-measure.

STOP: factory `dory` / leftover ELF / isolate ELF exec. leftover-tree cargo. sit t13. start default sock.

## Must prove

1. `NEWHEAD=$(git -C /home/manhquy/Downloads/flow/dory rev-parse HEAD)`
2. `git show HEAD:rust/src/attach.rs` — `ensure_server` has **no** `arg("server")` spawn; has `server not running; start with`
3. USAGE + attach help: no “Starts the server if needed” / “starts the daemon if needed”
4. `git show HEAD:README.md` sit = server then dory. Leave `:31` `22↔4↔0`
5. Leftover 5 mint including `server.rs` `4de1554a`
6. Leftover ELF sha `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` + spawn-strings still present
7. `type -a dory` empty. `UnixStream.connect($XDG_RUNTIME_DIR/dory/default/dory.sock)` connectable=0
8. Isolate worktree HEAD == `$NEWHEAD` ≠ `5a60953`. Isolate ELF strings: no `dory: start server:` / `server did not come up`
9. Optional: `env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml p5_attach`

Write `/home/manhquy/Downloads/flow/dory/plans/reports/260827-ensure-server-no-spawn-test.md`
End with `TEST_PASS` or `TEST_FAIL`.
