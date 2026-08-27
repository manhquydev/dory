---
type: test
date: 2026-08-27
plan: 260827-1032-ensure-server-no-auto-spawn
phase: 02
verdict: TEST_PASS
commit: b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
---

# Independent no-spawn test — increment C

TEST_TAB=w13:t23
TEST_PANE=w13:p69
COOK_TAB=w13:t26
COOK_PANE=w13:p6F
cwd=/home/manhquy/Downloads/flow/dory
when=2026-08-27
phase=2 independent re-measure (do not trust cook)

## Verdict

HEAD `ensure_server` fail-closed. USAGE/attach help have no auto-start sentences. Leftover 5 + leftover ELF mint. PATH empty. factory `dory.sock` connectable=0. Isolate HEAD == NEWHEAD. Isolate ELF spawn-fail strings gone. Isolate `--test p5_attach` 5 passed.

TEST_TAB ≠ COOK_TAB ≠ t13. TEST_PANE ≠ COOK_PANE. Not a split of cook.

C = HEAD inode / isolate rebuild. Leftover ELF still contains spawn strings. Do not claim leftover doors held.

## Env refuse

| var | state |
|---|---|
| DORY_SOCKET | UNSET |
| DORY_ENV | UNSET |
| DORY_RECYCLE | UNSET |
| PI_CODING_AGENT_DIR | UNSET |
| DORY_* | NONE |
| XDG_RUNTIME_DIR | `/run/user/1000` |
| HERDR_ENV | `1` (tab identity only) |

No refuse. Proceeded.

## 1 NEWHEAD

```
NEWHEAD=b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
```

`git -C /home/manhquy/Downloads/flow/dory log -1 --oneline`:

```
b544f5f fix(attach): do not auto-start server on sit
```

## 2 HEAD `ensure_server` (`git show HEAD:rust/src/attach.rs`)

```
326:pub fn ensure_server() -> Result<(), i32> {
327:    if ping() {
328:        return Ok(());
329:    }
330:    eprintln!("dory: server not running; start with `dory server`");
331:    Err(1)
332:}
```

| needle in fn | present |
|---|---|
| `arg("server")` | no |
| `Command::new` | no |
| `spawn(` | no |
| `setsid` | no |
| `dory: start server:` | no |
| `server did not come up` | no |
| `ping()` | yes |
| `server not running; start with` | yes |

`sit` still calls `ensure_server` (`:130`).

## 3 USAGE + attach help

HEAD USAGE `:50`: `Bare \`dory\` opens the desk (sidebar + tiled live panes).`

No USAGE line contains “Starts the server if needed”. Sole HEAD `main.rs` hit is negative assert `:705`:

```
assert!(!super::USAGE.contains("Starts the server if needed"));
```

HEAD attach module `:3-4`: `Bare \`dory\` / \`dory attach\` sits at the desk`. Help `:101-126`: no “starts the daemon if needed”, no “Starts `dory server` if needed”, no “Starts the server if needed”.

HEAD `p5_attach.rs` hits are `assert!(!…contains(…))` only.

## 4 HEAD README sit sentence

`git show HEAD:README.md`

- `:15` `Chạy \`dory server\` rồi \`dory\` để mở **desk**…`
- `:31` `| \`Ctrl-b b\` | Thu sidebar 22↔4↔0 |` (unchanged)

## 5 Leftover 5 mint

`git hash-object` on working tree (leftover bytes, not HEAD blobs):

| path | hash-object | mint | |
|---|---|---|---|
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |

`server.rs` prefix `4de1554a` held. Untouched leftover file.

Cached names: empty (no leftover 5 staged).

Porcelain ` M` = leftover 5 only. Extra `??` are pre-existing `plans/` plus `.claude/` `eval/` `scripts/` `src/workplace/` `test/phase*.test.js` — not leftover-5 drift.

## 6 Leftover ELF sha + spawn-strings

path=`/home/manhquy/Downloads/flow/dory/rust/target/debug/dory` (not exec'd)

| field | value |
|---|---|
| sha256 | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| expect | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| size | 18568240 |
| inode | 2490742 |
| mtime_epoch | 1787716801 |

Byte needles (read, not exec):

| needle | leftover ELF |
|---|---|
| `dory: start server:` | yes ×1 |
| `server did not come up` | yes ×1 |
| `server not running; start with` | no |

Leftover ELF still the spawn binary. C did not rebuild it.

## 7 PATH + factory sock

`hash -r`; `type -a dory`:

```
type: dory not found
TYPE_EXIT:1
```

PATH walk `*/dory` (lexists, no exec): COUNT=0. `~/.local/bin/dory` lexists=False.

Sock probe = AF_UNIX `UnixStream.connect` on **file** `$XDG_RUNTIME_DIR/dory/default/dory.sock` (not the session dir). After isolate cargo too:

| field | value |
|---|---|
| path | `/run/user/1000/dory/default/dory.sock` |
| lexists | False |
| connectable | 0 |
| err | `FileNotFoundError: [Errno 2] No such file or directory` |
| timeout | 1s |

## 8 Isolate HEAD + ELF strings

`git -C /home/manhquy/.cache/dory-isolates/land-4b70f79 rev-parse HEAD`

```
b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
```

== `$NEWHEAD`. ≠ `5a60953`.

Isolate ELF `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` (not exec'd on factory XDG):

| needle | isolate ELF |
|---|---|
| `dory: start server:` | no |
| `server did not come up` | no |
| `server not running; start with` | yes ×1 |
| `Starts the server if needed` | no |

sha256 `2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3` (distinct from leftover).

## 9 Isolate cargo (optional, absolute manifest)

cwd=`/tmp`. Not leftover-tree cargo.

```
env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \
  cargo test --manifest-path /home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml --test p5_attach
```

Name filter `p5_attach` without `--test` is not this run. `--test p5_attach` is the integration crate.

```
running 5 tests
test attach_help_and_usage_name_sit_down ... ok
test bare_dory_without_server_fails_closed ... ok
test attach_handshake_writes_and_detach_leaves_pty ... ok
test neighbor_walks_split_panes ... ok
test desk_tree_lists_split_siblings ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s
```

HEAD invert `bare_dory_without_server_fails_closed` (`:214`): every `Command` `env_remove("DORY_SOCKET")` + `DORY_ENV`; no daemon → exit 1 + new stderr; `workspace list` fail; temp XDG sock connect fail; then `start()` + bare `dory` still `needs a tty`.

Factory sock still connectable=0 after this run. Leftover ELF sha unchanged.

## Forbidden argv (this test)

No factory `dory`. No leftover ELF exec. No isolate ELF argv on factory XDG. No leftover-tree cargo. No sit `t13`. No `herdr server stop`. No start default sock. No `git add` leftover 5. No factory `reset --hard`.

## Must-prove

| # | check | result |
|---|---|---|
| 1 | NEWHEAD | `b544f5ff…` PASS |
| 2 | HEAD `ensure_server` no spawn; fail-closed stderr | PASS |
| 3 | USAGE + attach help no auto-start sentences | PASS |
| 4 | HEAD README sit = server then dory; `:31` `22↔4↔0` | PASS |
| 5 | leftover 5 mint incl. `server.rs` `4de1554a` | PASS |
| 6 | leftover ELF sha `3ba0e3bc…` + spawn-strings present | PASS |
| 7 | `type -a dory` empty; sock connectable=0 | PASS |
| 8 | isolate HEAD == NEWHEAD ≠ `5a60953`; isolate strings no spawn-fail | PASS |
| 9 | isolate `--test p5_attach` 5 passed | PASS |

TEST_PASS
