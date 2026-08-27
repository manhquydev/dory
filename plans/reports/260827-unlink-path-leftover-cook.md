COOK_PASS

# Unlink leftover PATH dory — cook receipt

COOK_TAB=w13:t1W
COOK_PANE=w13:p5C
cwd=/home/manhquy/Downloads/flow/dory
when=2026-08-27
phase=1 cook unlink only (no phase 2/3)

## Verdict

`rm` symlink path `/home/manhquy/.local/bin/dory` only. After `hash -r`, `type -a dory` empty. Leftover ELF kept, sha/mtime unchanged. Leftover 5 mint held before and after. Default sock not connectable on `$XDG_RUNTIME_DIR`.

## Gates before rm

### 1. Env refuse

| var | state |
|---|---|
| DORY_SOCKET | UNSET |
| DORY_ENV | UNSET |
| DORY_RECYCLE | UNSET |
| PI_CODING_AGENT_DIR | UNSET |
| XDG_RUNTIME_DIR | `/run/user/1000` |

No refuse.

### 2. Leftover 5 mint (researcher-02, BEFORE rm)

| File | hash-object | mint |
|---|---|---|
| README.md | `68190a5ffa073c082aa318aad5ed032e13cc90e3` | MATCH |
| rust/src/attach.rs | `602479094e84d31ad6f017775a3d55aeb485c644` | MATCH |
| rust/src/main.rs | `373d688636ff7315ccd665f450069d8284eb47ff` | MATCH |
| rust/src/server.rs | `4de1554ad56e248cdcf42f02111b7389b08dae82` | MATCH |
| rust/tests/p5_attach.rs | `9c28fc3e0f3666498a8952411242d5301f7911de` | MATCH |

### 3. `type -a dory` + PATH walk

`type -a dory` printed `/home/manhquy/.local/bin/dory` five times (`~/.local/bin` duplicated on PATH). PATH walk: one distinct realpath `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory`. No STOP.

### 4. Symlink target

`test -L /home/manhquy/.local/bin/dory` → 0.

```
lrwxrwxrwx 1 manhquy manhquy 56 Aug 23 19:33 /home/manhquy/.local/bin/dory -> /home/manhquy/Downloads/flow/dory/rust/target/debug/dory
```

realpath = leftover ELF. No STOP.

### 5. Leftover ELF snapshot (keep; not exec'd)

| field | value |
|---|---|
| path | `/home/manhquy/Downloads/flow/dory/rust/target/debug/dory` |
| sha256 | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` |
| mtime_epoch | `1787716801` |
| mtime | `2026-08-26 11:00:01.084955950 +0700` |
| size | `18568240` |
| inode | `2490742` |

### 6. Sock connectable

path=`$XDG_RUNTIME_DIR/dory/default/dory.sock` = `/run/user/1000/dory/default/dory.sock`

lexists=False exists=False connectable=False (FileNotFoundError, 1s AF_UNIX). Not a dead inode. Continue. Did not `dory server stop`. Did not `iso()` / `DORY_SOCKET=` on stop.

## Action

`test -L` held. `rm /home/manhquy/.local/bin/dory` → UNLINKED.

Not done: `rm "$(readlink -f …)"`, `rm` leftover ELF, `ln` isolate, cargo leftover tree, invoke `dory` / leftover ELF / isolate ELF, git add leftover 5, herdr server stop.

## After

### 9. PATH name gone

`hash -r`

`type -a dory` → `type: dory not found` TYPE_EXIT=1 (empty).

PATH walk COUNT=0.

`test ! -e /home/manhquy/.local/bin/dory` → 0

`test ! -L /home/manhquy/.local/bin/dory` → 0

### 10. ELF / mint / sock / porcelain

| check | result |
|---|---|
| leftover ELF exists | yes |
| sha256 | `3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14` unchanged |
| mtime_epoch | `1787716801` unchanged |
| mtime | `2026-08-26 11:00:01.084955950 +0700` unchanged |
| size | `18568240` unchanged |
| inode | `2490742` unchanged |
| leftover 5 hash-object | same 5 SHAs as researcher-02 |
| sock connectable | False (FileNotFoundError) |
| porcelain leftover 5 | still `M` those 5 files |
| porcelain `?? scripts/` | pre-existing, allowed |
| cached | empty |
| new `scripts/` or rust drift vs pre-rm snapshot | none |

## Forbidden argv (this cook)

No `dory`, `dory attach`, `dory server`, leftover ELF, isolate ELF. No `ln`. No cargo leftover. No sit `t13`. COOK_TAB ≠ t13.

## Known broken (not this increment)

Hop / USAGE / sit-child still PATH `dory` after unlink.
