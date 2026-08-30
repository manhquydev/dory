#!/bin/bash
# Isolate flock: mint isolate, four named occupants, sit desk, sibling prompt.
# Exit 0 iff visible has coord/omptest/omprev/groktry AND omptest decoded
# workspace list AND factory sock not connectable throughout.
# No set -x. Do not source or exec scripts/dory-flock-hop.sh
# or scripts/dory-isolate-flow-sit.sh.

umask 077

CACHE="${HOME}/.cache/dory-isolates"
FACTORY_HOME="${HOME}"
FACTORY_XDG="$(realpath "${XDG_RUNTIME_DIR:?}")"
FACTORY_SOCK="$(realpath "$FACTORY_XDG/dory/default/dory.sock" 2>/dev/null || echo none)"
FACTORY_AGENT_DIR="$FACTORY_HOME/.omp/agent"
LEFTOVER_ISO="$CACHE/flock.6yaatuxg"
REPO_DORY=/home/manhquy/Downloads/flow/dory/.dory
REPO_ROOT=/home/manhquy/Downloads/flow/dory
REPO_TARGET=/home/manhquy/Downloads/flow/dory/rust/target
ISO_REAL=""
ISO_SOCK=""
STOPPED=0
TORN=0
SNAP_DORY=""
HASH_SNAP=""
HAS_LIST=0
WS_ID=""
COORD_PANE=""
TEST_PANE=""
PREV_PANE=""
GROK_PANE=""

if [ "${HERDR_ENV:-}" != 1 ]; then
  echo "refuse: HERDR_ENV!=1" >&2
  exit 1
fi
if [ -z "${SIT_PANE:-}" ] || [ -z "${SIT_TAB:-}" ] || [ -z "${SIT_DORY:-}" ]; then
  echo "refuse: SIT_PANE SIT_TAB SIT_DORY required" >&2
  exit 1
fi
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
  iso env DORY_ENV=1 "$SIT_DORY" "$@"
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
  XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET "$SIT_DORY" server stop >/dev/null
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

factory_must_dead() {
  if sock_connectable "$FACTORY_XDG/dory/default/dory.sock"; then
    echo "FAIL: factory sock connectable" >&2
    return 1
  fi
  return 0
}

repo_dory_stat() {
  if [ ! -e "$REPO_DORY" ]; then
    printf 'ABSENT\n'
    return 0
  fi
  stat -c 'ino=%i mtime=%Y birth=%W' "$REPO_DORY"
}

leftover_hash_snap() {
  git -C "$REPO_ROOT" hash-object \
    README.md rust/src/attach.rs rust/src/main.rs rust/src/server.rs rust/tests/p5_attach.rs \
    rust/src/desk.rs
}

pane_fields() {
  herdr pane get "$1" | python3 -c '
import json, sys
d = json.loads(sys.stdin.read())
p = (d.get("result") or {}).get("pane") or {}
agent = p.get("agent")
if agent is None:
    agent = ""
print(p.get("tab_id") or "")
print(agent)
print(p.get("pane_id") or "")
'
}

teardown() {
  if [ "$TORN" = 1 ]; then
    return 0
  fi
  TORN=1
  compound_stop || true
  if iso_identity_ok; then
    wipe_iso || true
  fi
}

fail() {
  echo "FAIL: $*" >&2
  teardown
  if sock_connectable "$FACTORY_XDG/dory/default/dory.sock"; then
    echo "FAIL: factory sock connectable" >&2
  fi
  exit 1
}

trap 'teardown' EXIT

SIT_DORY="$(realpath "$SIT_DORY")"
if [ ! -x "$SIT_DORY" ]; then
  echo "refuse: SIT_DORY not executable" >&2
  exit 1
fi
LOCAL_DORY="$(realpath "$HOME/.local/bin/dory" 2>/dev/null || true)"
if [ -n "$LOCAL_DORY" ] && [ "$SIT_DORY" = "$LOCAL_DORY" ]; then
  echo "refuse: SIT_DORY is ~/.local/bin/dory" >&2
  exit 1
fi
case "$SIT_DORY" in
  "$REPO_TARGET"/*)
    echo "refuse: SIT_DORY is leftover rust/target" >&2
    exit 1
    ;;
esac
if [ ! -d "$FACTORY_AGENT_DIR" ]; then
  echo "refuse: FACTORY_AGENT_DIR missing" >&2
  exit 1
fi
if ! command -v omp >/dev/null || ! command -v grok >/dev/null; then
  echo "refuse: omp or grok missing on factory PATH" >&2
  exit 1
fi

if [ "$SIT_PANE" = "${HERDR_PANE_ID:-}" ]; then
  fail "SIT_PANE is this cook pane"
fi
if [ "$SIT_PANE" = "w13:p2R" ]; then
  fail "SIT_PANE is factory p2R"
fi
case "$SIT_PANE" in
  *wP:*) fail "SIT_PANE is wP" ;;
esac
if [ "$SIT_TAB" = "w13:t13" ]; then
  fail "SIT_TAB is factory t13"
fi

if sock_connectable "$FACTORY_XDG/dory/default/dory.sock"; then
  fail "factory sock connectable"
fi
if [ -e "$FACTORY_XDG/dory/default/dory.sock" ]; then
  echo "warn: factory sock exists but not connectable" >&2
fi

mapfile -t _PF < <(pane_fields "$SIT_PANE")
GOT_TAB="${_PF[0]}"
GOT_AGENT="${_PF[1]}"
GOT_PANE="${_PF[2]}"
if [ "$GOT_TAB" != "$SIT_TAB" ]; then
  fail "pane tab_id $GOT_TAB != SIT_TAB $SIT_TAB"
fi
if [ "$GOT_TAB" = "w13:t13" ]; then
  fail "sit tab_id is w13:t13"
fi
if [ -n "$GOT_AGENT" ]; then
  fail "sit pane has agent $GOT_AGENT"
fi
if [ -n "$GOT_PANE" ] && [ "$GOT_PANE" != "$SIT_PANE" ]; then
  fail "pane get id $GOT_PANE != SIT_PANE $SIT_PANE"
fi

visible_has_roster() {
  case "$1" in
    *coord*|*omptest*|*omprev*|*groktry*|*Agents*) return 0 ;;
  esac
  return 1
}

PRE="$(herdr pane read "$SIT_PANE" --source visible)" || fail "pane read preflight"
if visible_has_roster "$PRE"; then
  SPLIT_JSON="$(herdr pane split --pane "$SIT_PANE" --direction down --no-focus)" || fail "split dirty sit pane"
  NEW_PANE="$(printf '%s' "$SPLIT_JSON" | python3 -c '
import json, sys
d = json.loads(sys.stdin.read())
r = d.get("result") or {}
p = r.get("pane") or r
print(p.get("pane_id") or p.get("id") or r.get("pane_id") or "")
')"
  [ -n "$NEW_PANE" ] || fail "split returned no pane id"
  SIT_PANE="$NEW_PANE"
  mapfile -t _PF < <(pane_fields "$SIT_PANE")
  GOT_TAB="${_PF[0]}"
  GOT_AGENT="${_PF[1]}"
  GOT_PANE="${_PF[2]}"
  if [ "$GOT_TAB" != "$SIT_TAB" ] || [ "$GOT_TAB" = "w13:t13" ] || [ -n "$GOT_AGENT" ]; then
    fail "split pane tab/agent invalid tab=$GOT_TAB agent=$GOT_AGENT"
  fi
  PRE="$(herdr pane read "$SIT_PANE" --source visible)" || fail "pane read after split"
  if visible_has_roster "$PRE"; then
    fail "visible still contains roster chrome after split"
  fi
fi

SNAP_DORY="$(repo_dory_stat)"
HASH_SNAP="$(leftover_hash_snap)" || fail "leftover hash mint"

test ! -L "${HOME}/.cache"
mkdir -p -m 0700 "$CACHE"
test ! -L "$CACHE" || fail "cache symlink"

ISO="$(mktemp -d "$CACHE/roster.XXXXXX")" || fail "mktemp isolate"
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

mkdir -p -m 0700 "$ISO_REAL/bin"
ln -sfn "$SIT_DORY" "$ISO_REAL/bin/dory"
if [ "$(realpath "$ISO_REAL/bin/dory")" != "$SIT_DORY" ]; then
  fail "isolate bin/dory is not SIT_DORY"
fi

setsid env -u DORY_SOCKET -u DORY_ENV -u DORY_PANE_ID -u DORY_TAB_ID \
  -u DORY_WORKSPACE_ID -u DORY_SIT_SHELL \
  DORY_BARE_SHELL=1 HOME="$ISO_REAL/home" \
  PATH="$ISO_REAL/bin:$PATH" \
  PI_CODING_AGENT_DIR="$FACTORY_AGENT_DIR" \
  XDG_RUNTIME_DIR="$ISO_REAL" \
  /bin/bash -c 'cd "$0" && exec "$1" server' "$ISO_REAL" "$SIT_DORY" \
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
factory_must_dead || fail "factory sock connectable after isolate start"

XDG_RUNTIME_DIR="$ISO_REAL" env -u DORY_SOCKET DORY_ENV=1 "$SIT_DORY" workspace list >/dev/null \
  || fail "isolate workspace list miss; no attach"

WS_JSON="$(iso_mut workspace list)" || fail "iso workspace list failed"
read -r WS_ID COORD_PANE <<EOF
$(python3 -c 'import json,sys; d=json.loads(sys.stdin.read()); w=d["result"]["workspaces"][0]; print(w["workspace"]["id"], w["tabs"][0]["root_pane"]["id"])' <<<"$WS_JSON")
EOF
[ -n "$WS_ID" ] && [ -n "$COORD_PANE" ] || fail "parse WS_ID/COORD_PANE"

split_one() {
  local out
  out="$(iso_mut pane split --pane "$1" --direction down --no-focus)" || return 1
  python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["result"]["pane"]["id"])' <<<"$out"
}

TEST_PANE="$(split_one "$COORD_PANE")" || fail "split omptest"
PREV_PANE="$(split_one "$TEST_PANE")" || fail "split omprev"
GROK_PANE="$(split_one "$PREV_PANE")" || fail "split groktry"
factory_must_dead || fail "factory sock connectable after split"

start_omp() {
  iso_mut agent start "$1" --pane "$2" -- omp --no-session --no-skills --no-rules --no-extensions >/dev/null
}

start_omp coord "$COORD_PANE" || fail "start coord"
start_omp omptest "$TEST_PANE" || fail "start omptest"
start_omp omprev "$PREV_PANE" || fail "start omprev"
iso_mut agent start groktry --pane "$GROK_PANE" -- grok >/dev/null || fail "start groktry"
factory_must_dead || fail "factory sock connectable after agent start"

LIST_JSON="$(iso_mut pane list --workspace "$WS_ID")" || fail "iso pane list failed"
python3 - "$LIST_JSON" <<'PY' || fail "roster names missing on isolate pane list"
import json, sys
data = json.loads(sys.argv[1])
want = {"coord", "omptest", "omprev", "groktry"}
have = set()
for pane in data["result"]["panes"]:
    occ = pane.get("occupant")
    if isinstance(occ, dict) and occ.get("name"):
        have.add(occ["name"])
if have != want:
    raise SystemExit(f"have={sorted(have)} want={sorted(want)}")
PY

ready_wait() {
  if ! iso_mut pane wait-output --pane "$1" --match "$2" --timeout 180000 >/dev/null; then
    echo "ready miss $3:" >&2
    iso_mut pane read --pane "$1" --source recent-unwrapped >&2 || true
    fail "ready timeout $3"
  fi
}

# "Welcome back" is flaky (omp/grok splash). Prompt box is the ready signal.
ready_wait "$COORD_PANE" "╭──" "coord prompt"
ready_wait "$TEST_PANE" "╭──" "omptest prompt"

ATTACH_CMD="cd \"$ISO_REAL\" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \"$SIT_DORY\" attach"
case "$ATTACH_CMD" in
  *coord*|*omptest*|*omprev*|*groktry*|*Agents*) fail "ATTACH_CMD contains roster needle" ;;
esac

herdr pane send-text "$SIT_PANE" "$ATTACH_CMD" || fail "send-text attach"
herdr pane send-keys "$SIT_PANE" enter || fail "send-keys enter"

for name in coord omptest omprev groktry; do
  if ! herdr pane wait-output "$SIT_PANE" --match "$name" --source visible --timeout 20000 >/dev/null; then
    echo "visible after miss ($name):" >&2
    herdr pane read "$SIT_PANE" --source visible >&2 || true
    fail "wait-output miss $name"
  fi
done

VISIBLE="$(herdr pane read "$SIT_PANE" --source visible)" || fail "pane read after match"
for name in coord omptest omprev groktry; do
  case "$VISIBLE" in
    *"$name"*) ;;
    *) fail "visible missing $name after wait-output" ;;
  esac
done
factory_must_dead || fail "factory sock connectable after sit"

iso_mut agent prompt coord --timeout 180000 -- "$(cat <<'EOF'
You are coord. Do not change DORY_SOCKET or XDG_RUNTIME_DIR. Do not run dory server stop. Do not run bare dory, attach, or herdr. Do not pass --wait or --timeout. Do not read cook skills or plan files. Do not read or write ~/.omp, agent.db, or PI_CODING_AGENT_DIR.
Run exactly: dory agent prompt omptest -- You are omptest. Do not change DORY_SOCKET or XDG_RUNTIME_DIR. Do not run dory server stop. Run: dory workspace list. Then: dory agent report --current --state idle
Then run: dory agent report --current --state idle
EOF
)" >/dev/null || fail "sibling prompt cli failed"

deadline=$((SECONDS + 300))
while [ "$SECONDS" -lt "$deadline" ]; do
  factory_must_dead || fail "factory sock connectable during sibling"
  READ_OUT="$(iso_mut agent read omptest --source recent-unwrapped 2>/dev/null || true)"
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
TORN=1
if sock_connectable "$FACTORY_XDG/dory/default/dory.sock"; then
  echo "FAIL: factory sock connectable at end" >&2
  exit 1
fi
if [ "$(repo_dory_stat)" != "$SNAP_DORY" ]; then
  echo "FAIL: repo .dory changed at end" >&2
  exit 1
fi
if [ "$(leftover_hash_snap)" != "$HASH_SNAP" ]; then
  echo "FAIL: leftover hashes changed" >&2
  exit 1
fi
if [ "$HAS_LIST" != 1 ]; then
  echo "FAIL: missing decoded workspace list" >&2
  exit 1
fi

echo "SIT_DORY=$SIT_DORY"
echo "SIT_PANE=$SIT_PANE"
echo "SIT_TAB=$SIT_TAB"
echo "WS_ID=$WS_ID"
echo "COORD_PANE=$COORD_PANE"
echo "TEST_PANE=$TEST_PANE"
echo "PREV_PANE=$PREV_PANE"
echo "GROK_PANE=$GROK_PANE"
echo "FACTORY_SOCK=$FACTORY_SOCK"
echo "FACTORY_CONNECTABLE=0"
echo "REPO_DORY_STAT=$SNAP_DORY"
echo "VISIBLE_NAMES=1"
echo "SIBLING_LIST=1"
echo "flock=PASS"
exit 0
