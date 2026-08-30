#!/bin/bash
# Live factory gate for the thin OMP hop. Not CI. Not every commit.
# Mint a fresh isolate, start four occupants, one hop, decode result.text.
# Exit 0 = decoded workspace list AND factory sock still connectable.
# Exit != 0 = missing list, factory desk dead, or stop aborted.
# No set -x. Do not print auth bodies.

umask 077

CACHE="${HOME}/.cache/dory-isolates"
FACTORY_HOME="${HOME}"
FACTORY_XDG="$(realpath "${XDG_RUNTIME_DIR:?}")"
FACTORY_SOCK="$(realpath "$FACTORY_XDG/dory/default/dory.sock" 2>/dev/null || echo none)"
FACTORY_AGENT_DIR="$FACTORY_HOME/.omp/agent"
LEFTOVER_ISO="$CACHE/flock.6yaatuxg"
NEEDLE='{"ok":true,"result":{"workspaces":}'

ISO_REAL=""
ISO_SOCK=""
HB_PID=""
HB_ALIVE=""
HB_FAIL=""
SNAP_OCC=""
STOPPED=0
HAS_LIST=0
FACTORY_OK=1

if [ -n "${PI_CODING_AGENT_DIR:-}" ] || [ -n "${DORY_SOCKET:-}" ] || [ -n "${DORY_ENV:-}" ] || [ -n "${DORY_RECYCLE:-}" ]; then
  echo "refuse: factory already has DORY_* or PI_CODING_AGENT_DIR" >&2
  exit 1
fi
if [ "$HOME" != "$FACTORY_HOME" ]; then
  echo "refuse: HOME != FACTORY_HOME" >&2
  exit 1
fi

sock_connectable() {
  python3 - "$1" <<'PY'
import socket, sys
s = socket.socket(socket.AF_UNIX)
s.settimeout(1)
try:
    s.connect(sys.argv[1])
except Exception:
    raise SystemExit(1)
finally:
    s.close()
PY
}

iso() {
  test -n "$ISO_SOCK" || return 1
  test "$(realpath "$ISO_SOCK")" = "$ISO_SOCK" || return 1
  case "$(realpath "$ISO_SOCK")" in
    "$ISO_REAL"/*) ;;
    *) return 1 ;;
  esac
  DORY_SOCKET=$ISO_SOCK "$@"
}

iso_mut() {
  iso env DORY_ENV=1 dory "$@"
}

iso_identity_ok() {
  test -n "$ISO_REAL" || return 1
  test -d "$ISO_REAL" || return 1
  test ! -L "$ISO_REAL" || return 1
  test "$(realpath "$ISO_REAL")" = "$ISO_REAL" || return 1
  test "$ISO_REAL" != "$FACTORY_XDG" || return 1
  test "$ISO_REAL" != "$LEFTOVER_ISO" || return 1
  case "$ISO_REAL" in
    "$FACTORY_XDG"/*|/tmp|/tmp/*) return 1 ;;
  esac
  return 0
}

compound_stop() {
  if [ "$STOPPED" = 1 ]; then
    return 0
  fi
  if ! iso_identity_ok; then
    echo "stop abort: ISO identity fail" >&2
    return 1
  fi
  if [ ! -e "$ISO_REAL/dory/default/dory.sock" ]; then
    STOPPED=1
    return 0
  fi
  if [ -L "$ISO_REAL/dory/default/dory.sock" ]; then
    echo "stop abort: isolate sock is symlink" >&2
    return 1
  fi
  local sock_rp
  sock_rp="$(realpath "$ISO_REAL/dory/default/dory.sock")"
  if [ "$sock_rp" != "$ISO_SOCK" ]; then
    echo "stop abort: sock realpath != ISO_SOCK" >&2
    return 1
  fi
  case "$sock_rp" in
    "$ISO_REAL"/*) ;;
    *)
      echo "stop abort: sock not under ISO_REAL" >&2
      return 1
      ;;
  esac
  # Compound 2357. Never iso() / DORY_SOCKET= on server stop.
  XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET dory server stop >/dev/null
  STOPPED=1
}

wipe_iso() {
  if ! iso_identity_ok; then
    echo "wipe skip: ISO identity fail" >&2
    return 1
  fi
  local _i
  for _i in 1 2 3 4 5 6; do
    rm -rf -- "$ISO_REAL" || true
    if [ ! -e "$ISO_REAL" ]; then
      return 0
    fi
    sleep 0.3
  done
  echo "wipe leftover: $ISO_REAL" >&2
  return 1
}

stop_hb() {
  if [ -n "$HB_ALIVE" ]; then
    rm -f "$HB_ALIVE"
  fi
  if [ -n "${HB_PID:-}" ]; then
    kill "$HB_PID" 2>/dev/null || true
    wait "$HB_PID" 2>/dev/null || true
    HB_PID=""
  fi
}

teardown() {
  stop_hb
  compound_stop || true
  if iso_identity_ok; then
    wipe_iso || true
  fi
  if ! sock_connectable "$FACTORY_SOCK"; then
    FACTORY_OK=0
  fi
}

fail() {
  echo "FAIL: $*" >&2
  teardown
  exit 1
}

if ! sock_connectable "$FACTORY_SOCK"; then
  echo "factory sock not connectable" >&2
  exit 1
fi

test ! -L "${HOME}/.cache"
test ! -L "$CACHE" 2>/dev/null || true
mkdir -p "$CACHE"
test ! -L "$CACHE" || fail "cache symlink"

SNAP_OCC="$(mktemp "$CACHE/hop-snap.XXXXXX")"
HB_ALIVE="$(mktemp "$CACHE/hop-hb.XXXXXX")"
HB_FAIL="$(mktemp "$CACHE/hop-fail.XXXXXX")"
rm -f "$HB_FAIL"
python3 - "$SNAP_OCC" "$FACTORY_XDG" "$FACTORY_HOME" <<'PY' || fail "default snap"
import json, os, subprocess, sys
snap_path, xdg, home = sys.argv[1:4]
assert os.environ["HOME"] == home
assert os.path.realpath(os.environ["XDG_RUNTIME_DIR"]) == xdg
env = os.environ.copy()
env["XDG_RUNTIME_DIR"] = xdg
env.pop("DORY_ENV", None)
env.pop("DORY_SOCKET", None)
snap = {}
for ws in ("w1", "w2"):
    p = subprocess.run(
        ["dory", "pane", "list", "--workspace", ws],
        env=env, capture_output=True, text=True, check=True,
    )
    data = json.loads(p.stdout)
    snap[ws] = {pane["id"]: pane.get("occupant") for pane in data["result"]["panes"]}
with open(snap_path, "w") as f:
    json.dump(snap, f)
PY

ISO="$(mktemp -d "$CACHE/flock.XXXXXX")" || fail "mktemp isolate"
ISO_REAL="$(realpath "$ISO")"
if [ -L "$ISO" ] || [ -L "$ISO_REAL" ]; then
  fail "ISO is symlink"
fi
if [ "$ISO_REAL" = "$FACTORY_XDG" ] || [ "$ISO_REAL" = "$LEFTOVER_ISO" ]; then
  fail "ISO pin collision"
fi
case "$ISO_REAL" in
  "$FACTORY_XDG"/*|/tmp|/tmp/*) fail "ISO under FACTORY_XDG or /tmp" ;;
esac
iso_identity_ok || fail "ISO identity fail after mint"

mkdir -p -m 0700 "$ISO_REAL/home"
if [ -f "$FACTORY_HOME/.grok/auth.json" ] && [ ! -L "$FACTORY_HOME/.grok/auth.json" ]; then
  mkdir -p -m 0700 "$ISO_REAL/home/.grok"
  cp -f "$FACTORY_HOME/.grok/auth.json" "$ISO_REAL/home/.grok/auth.json"
  if [ -L "$ISO_REAL/home/.grok/auth.json" ]; then
    fail "grok auth dest is symlink"
  fi
fi
if [ -e "$ISO_REAL/home/.omp" ] || [ -e "$ISO_REAL/home/.agents" ]; then
  fail "isolate home grew an omp store"
fi

setsid env -u DORY_SOCKET -u DORY_ENV -u DORY_PANE_ID -u DORY_TAB_ID \
  -u DORY_WORKSPACE_ID -u DORY_SIT_SHELL \
  DORY_BARE_SHELL=1 HOME="$ISO_REAL/home" \
  PI_CODING_AGENT_DIR="$FACTORY_AGENT_DIR" \
  XDG_RUNTIME_DIR="$ISO_REAL" \
  /bin/bash -c 'cd "$0" && exec dory server' "$ISO_REAL" \
  </dev/null >"$ISO_REAL/server.log" 2>&1 &

ISO_SOCK=""
for _i in $(seq 1 75); do
  if [ -S "$ISO_REAL/dory/default/dory.sock" ] && [ ! -L "$ISO_REAL/dory/default/dory.sock" ]; then
    cand="$(realpath "$ISO_REAL/dory/default/dory.sock")"
    if [ -n "$cand" ] && [ "$cand" = "$(realpath "$cand")" ] && [ "$cand" != "$FACTORY_SOCK" ]; then
      case "$cand" in
        "$ISO_REAL"/*)
          if sock_connectable "$cand"; then
            ISO_SOCK="$cand"
            break
          fi
          ;;
      esac
    fi
  fi
  sleep 0.2
done
[ -n "$ISO_SOCK" ] || fail "isolate sock never became connectable"

WS_JSON="$(iso dory workspace list)" || fail "iso workspace list failed"
read -r WS_ID COORD_PANE <<EOF
$(python3 -c 'import json,sys; d=json.loads(sys.stdin.read()); w=d["result"]["workspaces"][0]; print(w["workspace"]["id"], w["tabs"][0]["root_pane"]["id"])' <<<"$WS_JSON")
EOF

split_one() {
  local out
  out="$(iso_mut pane split --pane "$1" --direction down --no-focus)" || return 1
  python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["result"]["pane"]["id"])' <<<"$out"
}

TEST_PANE="$(split_one "$COORD_PANE")" || fail "split omptest"
PREV_PANE="$(split_one "$TEST_PANE")" || fail "split omprev"
GROK_PANE="$(split_one "$PREV_PANE")" || fail "split groktry"

start_omp() {
  iso_mut agent start "$1" --pane "$2" -- omp --no-session --no-skills --no-rules --no-extensions >/dev/null
}

start_omp coord "$COORD_PANE" || fail "start coord"
start_omp omptest "$TEST_PANE" || fail "start omptest"
start_omp omprev "$PREV_PANE" || fail "start omprev"
iso_mut agent start groktry --pane "$GROK_PANE" -- grok >/dev/null || true

heartbeat_loop() {
  while [ -f "$HB_ALIVE" ]; do
    if ! sock_connectable "$FACTORY_SOCK"; then
      printf '%s\n' "factory sock dead" >"$HB_FAIL"
      compound_stop || true
      return 1
    fi
    if ! python3 - "$SNAP_OCC" "$FACTORY_XDG" "$FACTORY_HOME" <<'PY'
import json, os, subprocess, sys
snap_path, xdg, home = sys.argv[1:4]
if os.environ.get("HOME") != home:
    raise SystemExit("HOME drift")
if os.path.realpath(os.environ.get("XDG_RUNTIME_DIR", "")) != xdg:
    raise SystemExit("XDG drift")
with open(snap_path) as f:
    snap = json.load(f)
env = os.environ.copy()
env["XDG_RUNTIME_DIR"] = xdg
env.pop("DORY_ENV", None)
env.pop("DORY_SOCKET", None)
for ws, expected in snap.items():
    p = subprocess.run(
        ["dory", "pane", "list", "--workspace", ws],
        env=env, capture_output=True, text=True,
    )
    if p.returncode != 0:
        raise SystemExit(f"pane list {ws} failed")
    data = json.loads(p.stdout)
    have = {pane["id"]: pane.get("occupant") for pane in data["result"]["panes"]}
    for pane_id, occ in expected.items():
        if pane_id not in have or have[pane_id] != occ:
            raise SystemExit(f"occupant drift {pane_id}")
    for pane_id, occ in have.items():
        if pane_id not in expected and occ is not None:
            raise SystemExit(f"new occupant {pane_id}")
PY
    then
      printf '%s\n' "default occupant drift" >"$HB_FAIL"
      compound_stop || true
      return 1
    fi
    sleep 30
  done
}

: >"$HB_ALIVE"
heartbeat_loop &
HB_PID=$!

ready_wait() {
  if [ -f "$HB_FAIL" ]; then
    fail "heartbeat: $(cat "$HB_FAIL")"
  fi
  iso_mut pane wait-output --pane "$1" --match "$2" --timeout 180000 >/dev/null || fail "ready timeout $3"
  if [ -f "$HB_FAIL" ]; then
    fail "heartbeat: $(cat "$HB_FAIL")"
  fi
}

ready_wait "$COORD_PANE" "Welcome back" "coord Welcome back"
ready_wait "$COORD_PANE" "╭──" "coord prompt"
ready_wait "$TEST_PANE" "Welcome back" "omptest Welcome back"
ready_wait "$TEST_PANE" "╭──" "omptest prompt"

if [ -f "$HB_FAIL" ]; then
  fail "heartbeat before hop: $(cat "$HB_FAIL")"
fi

iso_mut agent prompt coord --timeout 180000 -- "$(cat <<'EOF'
You are coord. Do not change DORY_SOCKET or XDG_RUNTIME_DIR. Do not run dory server stop. Do not run bare dory, attach, or herdr. Do not pass --wait or --timeout. Do not read cook skills or plan files. Do not read or write ~/.omp, agent.db, or PI_CODING_AGENT_DIR.
Run exactly: dory agent prompt omptest -- You are omptest. Do not change DORY_SOCKET or XDG_RUNTIME_DIR. Do not run dory server stop. Run: dory workspace list. Then: dory agent report --current --state idle
Then run: dory agent report --current --state idle
EOF
)" >/dev/null || fail "hop prompt cli failed"

deadline=$((SECONDS + 300))
while [ "$SECONDS" -lt "$deadline" ]; do
  if [ -f "$HB_FAIL" ]; then
    fail "heartbeat during hop: $(cat "$HB_FAIL")"
  fi
  READ_OUT="$(iso dory agent read omptest --source recent-unwrapped 2>/dev/null || true)"
  if python3 -c 'import json,re,sys
NEEDLE="{\"ok\":true,\"result\":{\"workspaces\":"
ANSI=re.compile(r"\x1b\[[0-9;?]*[ -/]*[@-~]|\x1b\][^\x07]*(?:\x07|\x1b\\)")
env=json.loads(sys.stdin.read())
text=(env.get("result") or {}).get("text")
if isinstance(text,str) and (NEEDLE in text or NEEDLE in ANSI.sub("", text)):
    raise SystemExit(0)
raise SystemExit(1)
' <<<"$READ_OUT"
  then
    HAS_LIST=1
    break
  fi
  sleep 5
done

teardown
rm -f "$SNAP_OCC" "$HB_ALIVE" "$HB_FAIL"

if [ "$HAS_LIST" != 1 ]; then
  echo "missing decoded workspace list" >&2
  exit 1
fi
if [ "$FACTORY_OK" != 1 ]; then
  echo "factory sock dead" >&2
  exit 1
fi
if [ "$STOPPED" != 1 ]; then
  echo "stop aborted" >&2
  exit 1
fi

echo "ISO_REAL=$ISO_REAL"
echo "FACTORY_SOCK=$FACTORY_SOCK"
echo "hop=PASS"
exit 0
