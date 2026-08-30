#!/bin/bash
# Isolate sit: mint isolate, taxi unique token, attach sit pane, wait footer.
# Exit 0 iff visible contains Flow 0. $FLOW_TOKEN AND factory sock not connectable throughout.
# No set -x. Do not source or exec scripts/dory-flock-hop.sh.

umask 077

CACHE="${HOME}/.cache/dory-isolates"
FACTORY_HOME="${HOME}"
FACTORY_XDG="$(realpath "${XDG_RUNTIME_DIR:?}")"
FACTORY_SOCK="$(realpath "$FACTORY_XDG/dory/default/dory.sock" 2>/dev/null || echo none)"
LEFTOVER_ISO="$CACHE/flock.6yaatuxg"
REPO_DORY=/home/manhquy/Downloads/flow/dory/.dory
REPO_TARGET=/home/manhquy/Downloads/flow/dory/rust/target

ISO_REAL=""
ISO_SOCK=""
STOPPED=0
TORN=0
SNAP_DORY=""
FLOW_TOKEN=""
NEEDLE=""
JOURNAL=""

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

FLOW_TOKEN="sit-${RANDOM}${RANDOM}"
NEEDLE="Flow 0. $FLOW_TOKEN"

PRE="$(herdr pane read "$SIT_PANE" --source visible)" || fail "pane read preflight"
case "$PRE" in
  *"$NEEDLE"*)
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
    case "$PRE" in
      *"$NEEDLE"*) fail "visible still contains NEEDLE after split" ;;
    esac
    ;;
esac

SNAP_DORY="$(repo_dory_stat)"

test ! -L "${HOME}/.cache"
mkdir -p -m 0700 "$CACHE"
test ! -L "$CACHE" || fail "cache symlink"

ISO="$(mktemp -d "$CACHE/sit.XXXXXX")" || fail "mktemp isolate"
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

setsid env -u DORY_SOCKET -u DORY_ENV -u DORY_PANE_ID -u DORY_TAB_ID \
  -u DORY_WORKSPACE_ID -u DORY_SIT_SHELL -u PI_CODING_AGENT_DIR \
  DORY_BARE_SHELL=1 HOME="$ISO_REAL/home" \
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

WS="$ISO_REAL"
(cd "$WS" && DORY_SOCKET="$ISO_SOCK" DORY_ENV=1 DORY_WORKSPACE_DIR="$WS" FLOW_BIN=/bin/true \
  "$SIT_DORY" flow -- "$FLOW_TOKEN") >/dev/null || fail "taxi flow failed"

JOURNAL="$WS/.dory/sessions/s1.jsonl"
[ -f "$JOURNAL" ] || fail "taxi journal missing"
python3 - "$JOURNAL" <<'PY' || fail "journal missing flow/result code 0"
import json, sys
ok = False
with open(sys.argv[1], encoding="utf-8") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        d = json.loads(line)
        if d.get("type") == "flow/result" and d.get("code") == 0:
            ok = True
sys.exit(0 if ok else 1)
PY
if [ "$(repo_dory_stat)" != "$SNAP_DORY" ]; then
  fail "repo .dory changed after taxi"
fi
factory_must_dead || fail "factory sock connectable after taxi"

ATTACH_CMD="cd \"$ISO_REAL\" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \"$SIT_DORY\" attach"
case "$ATTACH_CMD" in
  *"$NEEDLE"*|*"Flow 0."*) fail "ATTACH_CMD contains needle" ;;
esac

herdr pane send-text "$SIT_PANE" "$ATTACH_CMD" || fail "send-text attach"
herdr pane send-keys "$SIT_PANE" enter || fail "send-keys enter"

# Live herdr 0.7.5: pane id first (help lists it last; that argv is unknown-option).
if ! herdr pane wait-output "$SIT_PANE" --match "$NEEDLE" --source visible --timeout 20000; then
  echo "visible after miss:" >&2
  herdr pane read "$SIT_PANE" --source visible >&2 || true
  echo "journal:" >&2
  cat "$JOURNAL" >&2 || true
  fail "wait-output miss"
fi

VISIBLE="$(herdr pane read "$SIT_PANE" --source visible)" || fail "pane read after match"
case "$VISIBLE" in
  *"$NEEDLE"*) ;;
  *) fail "visible missing NEEDLE after wait-output" ;;
esac
factory_must_dead || fail "factory sock connectable after sit"

JOURNAL_SNAP="$(cat "$JOURNAL")"
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

echo "SIT_DORY=$SIT_DORY"
echo "SIT_PANE=$SIT_PANE"
echo "SIT_TAB=$SIT_TAB"
echo "FLOW_TOKEN=$FLOW_TOKEN"
echo "NEEDLE=$NEEDLE"
echo "FACTORY_SOCK=$FACTORY_SOCK"
echo "FACTORY_CONNECTABLE=0"
echo "REPO_DORY_STAT=$SNAP_DORY"
echo "JOURNAL_OK=1"
echo "VISIBLE_MATCH=1"
echo "sit=PASS"
exit 0
