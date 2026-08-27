---
type: cook
date: 2026-08-27
plan: 260827-1032-ensure-server-no-auto-spawn
phase: 01
verdict: COOK_PASS
commit: b544f5ff75a3c085ae6ab51ccafb3b58fa551db2
---

# Cook receipt — ensure server no auto-spawn (phase 01)

**Verdict: COOK_PASS**

Land method 0130: KEEP four leftover files → `git checkout HEAD --` attach/main/`p5_attach`/README → hunk on HEAD blobs → commit those four → restore leftover bytes. Cargo only isolate `land-4b70f79`. No leftover-tree cargo. No factory `dory` / leftover ELF / isolate ELF argv. No sit `t13`. No `herdr server stop`.

```
b544f5f fix(attach): do not auto-start server on sit
```

## Identity

| Field | Value |
|---|---|
| NEWHEAD | `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2` |
| isolate HEAD | `b544f5ff75a3c085ae6ab51ccafb3b58fa551db2` (== NEWHEAD, ≠ `5a60953`) |
| COOK_TAB | `w13:t26` |
| COOK_PANE | `w13:p6F` |
| caller | `w13:t22` / `w13:p68` (not `t13`) |

## HEAD cite (`git show HEAD`, not leftover WT)

`git show HEAD:rust/src/attach.rs` `ensure_server`:

```
pub fn ensure_server() -> Result<(), i32> {
    if ping() {
        return Ok(());
    }
    eprintln!("dory: server not running; start with `dory server`");
    Err(1)
}
```

No `cmd.arg("server")`, no `setsid`, no `cmd.spawn()`, no 5s poll. `ping()` kept.

USAGE: `Bare \`dory\` opens the desk` remains. `"Starts the server if needed"` gone.

HEAD README `:15`: `Chạy \`dory server\` rồi \`dory\` để mở **desk**`. `:31` still `22↔4↔0`.

## Leftover 5 after KEEP→WT restore

| path | `git hash-object` |
|---|---|
| `rust/src/attach.rs` | `602479094e84d31ad6f017775a3d55aeb485c644` |
| `rust/src/main.rs` | `373d688636ff7315ccd665f450069d8284eb47ff` |
| `rust/src/server.rs` | `4de1554ad56e248cdcf42f02111b7389b08dae82` (untouched; never checkout/restore) |
| `rust/tests/p5_attach.rs` | `9c28fc3e0f3666498a8952411242d5301f7911de` |
| `README.md` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` |

Cached names at commit: exactly those four (no `server.rs`). Cached after commit: empty.

## Isolate cargo

Absolute manifest `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/Cargo.toml`.

`env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE cargo test … p5_attach` is a **name filter**. After rename it ran **0 tests** (5 filtered in `tests/p5_attach.rs`).

`env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE cargo test --manifest-path … --test p5_attach`:

```
running 5 tests
test attach_help_and_usage_name_sit_down ... ok
test bare_dory_without_server_fails_closed ... ok
test attach_handshake_writes_and_detach_leaves_pty ... ok
test desk_tree_lists_split_siblings ... ok
test neighbor_walks_split_panes ... ok
test result: ok. 5 passed; 0 failed
```

Same three `-u` on `cargo build` same isolate manifest: `Finished dev` 0.03s.

## ELF / sock / PATH

| Check | Result |
|---|---|
| leftover ELF sha | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` (held) |
| leftover strings | `dory: start server:` **yes**; `server did not come up` **yes** |
| isolate strings | `dory: start server:` **no**; `server did not come up` **no**; new stderr **yes** |
| `$XDG_RUNTIME_DIR/dory/default/dory.sock` | connectable=0 (`FileNotFoundError`; AF_UNIX on `dory.sock`, not the session dir) |
| `type -a dory` | empty (`type: dory not found`) |

No factory `reset --hard`. No leftover cargo. `DORY_SOCKET` / `DORY_ENV` / `DORY_RECYCLE` / `PI_CODING_AGENT_DIR` unset throughout.

COOK_PASS
