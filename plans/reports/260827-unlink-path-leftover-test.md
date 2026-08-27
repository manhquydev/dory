TEST_PASS

# Independent PATH test — leftover unlink A

TEST_TAB=w13:t1X
TEST_PANE=w13:p5G
COOK_TAB=w13:t1W
COOK_PANE=w13:p5C
cwd=/home/manhquy/Downloads/flow/dory
when=2026-08-27
phase=2 independent PATH re-measure (no unlink; no dory/ELF exec)

## Verdict

Five A asserts hold. PATH name `dory` gone. Leftover 5 mint unchanged. Leftover ELF kept at cook sha. Default sock not connectable.

TEST_TAB ≠ COOK_TAB ≠ t13. TEST_PANE ≠ COOK_PANE. Not a split of cook.

## Env refuse

| var | state |
|---|---|
| DORY_SOCKET | UNSET |
| DORY_ENV | UNSET |
| DORY_RECYCLE | UNSET |
| PI_CODING_AGENT_DIR | UNSET |
| DORY_* | NONE |
| XDG_RUNTIME_DIR | `/run/user/1000` |

No refuse. Proceeded.

## A asserts (all required)

### A1 `test ! -e` and `test ! -L` ~/.local/bin/dory

path=`/home/manhquy/.local/bin/dory`

| check | result |
|---|---|
| `test ! -e` exit | 0 |
| `test ! -L` exit | 0 |
| exists | False |
| lexists | False |
| is_symlink | False |

A1=PASS

### A2 `hash -r`; `type -a dory` empty; PATH walk 0

`hash -r` then `type -a dory`:

```
type: dory not found
TYPE_EXIT:1
```

PATH walk `*/dory` (lexists, no exec): COUNT=0 HITS=[]

A2=PASS

### A3 leftover 5 `git hash-object` == researcher-02 FULL SHA

| File | hash-object | mint | |
|---|---|---|---|
| README.md | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| rust/src/attach.rs | `602479094e84d31ad6f017775a3d55aeb485c644` | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| rust/src/main.rs | `373d688636ff7315ccd665f450069d8284eb47ff` | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| rust/src/server.rs | `4de1554ad56e248cdcf42f02111b7389b08dae82` | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| rust/tests/p5_attach.rs | `9c28fc3e0f3666498a8952411242d5301f7911de` | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

A3=PASS

### A4 `$XDG_RUNTIME_DIR/dory/default/dory.sock` not connectable (AF_UNIX 1s)

path=`/run/user/1000/dory/default/dory.sock`

| field | value |
|---|---|
| lexists | False |
| exists | False |
| connectable | False |
| err | `FileNotFoundError: [Errno 2] No such file or directory` |
| timeout | 1s |
| elapsed_s | 0.0001 |

A4=PASS

### A5 leftover ELF exists; sha256 == cook before-snapshot

path=`/home/manhquy/Downloads/flow/dory/rust/target/debug/dory`

| field | value |
|---|---|
| exists | True |
| sha256 | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| expect | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| size | 18568240 |
| inode | 2490742 |
| mtime_epoch | 1787716801 |

Not exec'd. sha matches cook before.txt + cook receipt.

A5=PASS

## Observe-only (not FAIL A)

| check | result |
|---|---|
| `desk.rs` hash-object | `4c788562e4fdda10c8edd2878ed1fdd46050c218` (still `4c788562…`) |
| isolate `land-4b70f79` debug | exists `/home/manhquy/.cache/dory-isolates/land-4b70f79/rust/target/debug/dory` size=18493816 inode=1063590 (not exec'd) |
| `git rev-parse HEAD` | `5a6095367f905a42ff1c38886ebffa0f0840977d` |

## Forbidden argv (this test)

No `dory`. No leftover ELF exec. No isolate ELF exec. No `ln`. No cargo. No `git add leftover`. No sit `t13`. No mutate PATH. No unlink.

## Known broken (not this increment)

Hop / USAGE / sit-child still PATH `dory` after unlink.
