#!/bin/bash
# Isolate AOE 5 flow next: mint FAIL idea, taxi flow -- next rc=1,
# occupant writes PASS, taxi rc=0, unlock flow/01-research.md.
# Real FLOW_BIN=flow.sh. No /bin/true. No set -x.
# Do not source or exec hop, 1910, 0043, 0227, 0242, or judge scripts.

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
FLOW_SKILL=/home/manhquy/.claude/skills/flow/runner/flow.sh
IDEA_TEMPLATE=/home/manhquy/.claude/skills/flow/_templates/00-idea.md
RESEARCH_TEMPLATE=/home/manhquy/.claude/skills/flow/_templates/01-research.md
JOURNAL_COPY_TMP=/tmp/dory-aoe5-flow-next-journal.jsonl
JOURNAL_COPY_RECEIPT="$REPO_ROOT/plans/reports/260827-ensure-aoe5-flow-next-journal.jsonl"
RESEARCH_SHA_RECEIPT="$REPO_ROOT/plans/reports/260827-ensure-aoe5-flow-next-01.sha256"
LAND_SHA=2ef2073031d5ddfe23f5cbc906342bd0c6143137494b91df18b0518a1ff2f6b3
LEFTOVER_ELF_SHA=3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14

ISO_REAL=""
ISO_SOCK=""
STOPPED=0
TORN=0
SNAP_DORY=""
JOURNAL=""
PASS_WANT=""
WS_ID=""
COORD_PANE=""
TEST_PANE=""
PROMPTED=0

if [ "${HERDR_ENV:-}" != 1 ]; then
  echo "refuse: HERDR_ENV!=1" >&2
  exit 1
fi
case "$0" in
  *dory-flock-hop.sh|*dory-isolate-flow-sit.sh|*dory-isolate-flock-roster.sh|*dory-isolate-flock-report.sh|*dory-isolate-flock-prompt.sh|*dory-isolate-aoe5-flow-judge.sh)
    echo "refuse: this script is dory-isolate-aoe5-flow-next.sh" >&2
    exit 1
    ;;
esac
if [ -z "${SIT_PANE:-}" ] || [ -z "${SIT_TAB:-}" ] || [ -z "${SIT_DORY:-}" ]; then
  echo "refuse: SIT_PANE SIT_TAB SIT_DORY required" >&2
  exit 1
fi
if [ -n "${PI_CODING_AGENT_DIR:-}" ] || [ -n "${DORY_SOCKET:-}" ] || [ -n "${DORY_ENV:-}" ] || [ -n "${DORY_RECYCLE:-}" ]; then
  echo "refuse: factory already has DORY_* or PI_CODING_AGENT_DIR" >&2
  exit 1
fi
if python3 -c 'import os,sys; sys.exit(0 if any(k.startswith("FLOW_") for k in os.environ) else 1)'; then
  echo "refuse: factory already has FLOW_*" >&2
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

factory_flow_absent() {
  if [ -e "$REPO_ROOT/flow" ]; then
    echo "FAIL: factory flow/ exists" >&2
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

leftover_mint_ok() {
  python3 - "$REPO_ROOT" <<'PY'
import subprocess, sys
root = sys.argv[1]
want = [
    ("README.md", "68190a5ffa073c082aa318aad5ed032e13cc90e3"),
    ("rust/src/attach.rs", "602479094e84d31ad6f017775a3d55aeb485c644"),
    ("rust/src/main.rs", "373d688636ff7315ccd665f450069d8284eb47ff"),
    ("rust/src/server.rs", "4de1554ad56e248cdcf42f02111b7389b08dae82"),
    ("rust/tests/p5_attach.rs", "9c28fc3e0f3666498a8952411242d5301f7911de"),
]
out = subprocess.check_output(
    ["git", "-C", root, "hash-object", *[p for p, _ in want]],
    text=True,
)
got = out.splitlines()
if len(got) != len(want):
    raise SystemExit(f"hash-object count {len(got)}")
bad = []
for (path, sha), g in zip(want, got):
    if g != sha:
        bad.append(f"{path} {g} != {sha}")
if bad:
    raise SystemExit("; ".join(bad))
PY
}

desk_head_ok() {
  python3 - "$REPO_ROOT" <<'PY'
import subprocess, sys
root = sys.argv[1]
want = "4c788562e4fdda10c8edd2878ed1fdd46050c218"
work = subprocess.check_output(
    ["git", "-C", root, "hash-object", "rust/src/desk.rs"], text=True
).strip()
head = subprocess.check_output(
    ["git", "-C", root, "rev-parse", "HEAD:rust/src/desk.rs"], text=True
).strip()
if work != want:
    raise SystemExit(f"desk worktree {work} != mint {want}")
if head != want:
    raise SystemExit(f"desk HEAD {head} != mint {want}")
PY
}

leftover_elf_stat_ok() {
  local p="$REPO_TARGET/debug/dory"
  if [ ! -e "$p" ]; then
    echo "FAIL: leftover ELF missing (stat only; do not cargo)" >&2
    return 1
  fi
  local got
  got="$(sha256sum "$p" | awk '{print $1}')"
  if [ "$got" != "$LEFTOVER_ELF_SHA" ]; then
    echo "FAIL: leftover ELF sha $got != $LEFTOVER_ELF_SHA" >&2
    return 1
  fi
  return 0
}

path_dory_empty() {
  if type -a dory >/dev/null 2>&1; then
    echo "FAIL: PATH dory is not empty" >&2
    type -a dory >&2 || true
    return 1
  fi
  return 0
}

flow_unset_keys() {
  python3 -c 'import os
for k in os.environ:
    if k.startswith("FLOW_"):
        print(k)
'
}

self_refuse_paid() {
  python3 - "$0" <<'PY'
import re, sys
text = open(sys.argv[1], encoding="utf-8").read()
pat = re.compile(
    r"^\s*(source|\.|exec)\s+.*?(dory-isolate-flow-sit|dory-isolate-flock-prompt|"
    r"dory-isolate-flock-roster|dory-isolate-flock-report|dory-flock-hop|"
    r"dory-isolate-aoe5-flow-judge)",
    re.M,
)
if pat.search(text):
    raise SystemExit("self sources/execs paid script")
if "FLOW_BIN=" + "/bin/true" in text:
    raise SystemExit("self contains true chrome FLOW_BIN")
if "prompt --" + "wait" in text:
    raise SystemExit("self contains prompt wait flag")
if "flow -- " + "gate" in text:
    raise SystemExit("self contains flow gate taxi")
PY
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

copy_journal() {
  if [ -z "$JOURNAL" ] || [ ! -f "$JOURNAL" ]; then
    return 1
  fi
  cp -f "$JOURNAL" "$JOURNAL_COPY_TMP" || return 1
  mkdir -p "$REPO_ROOT/plans/reports"
  cp -f "$JOURNAL" "$JOURNAL_COPY_RECEIPT" || return 1
}

copy_research_sha() {
  local f="$ISO_REAL/flow/01-research.md"
  if [ -z "$ISO_REAL" ] || [ ! -f "$f" ]; then
    return 1
  fi
  mkdir -p "$REPO_ROOT/plans/reports"
  {
    sha256sum "$f"
    sha256sum "$RESEARCH_TEMPLATE"
    stat -c 'path=%n ino=%i size=%s mtime=%Y' "$f"
  } > "$RESEARCH_SHA_RECEIPT" || return 1
}

taxi() {
  local -a ue=()
  local k
  while IFS= read -r k; do
    [ -n "$k" ] || continue
    ue+=(-u "$k")
  done < <(flow_unset_keys)
  (cd "$ISO_REAL" && \
    HOME="$ISO_REAL/home" DORY_SOCKET="$ISO_SOCK" DORY_ENV=1 \
    DORY_WORKSPACE_DIR="$ISO_REAL" \
    env "${ue[@]}" \
    FLOW_PROJECT_ROOT="$ISO_REAL" \
    FLOW_BIN="$FLOW_BIN" FLOW_LOG_DISABLE=1 DO_NOT_TRACK=1 \
    "$SIT_DORY" flow -- next)
}

idea_still_fail() {
  python3 - "$ISO_REAL/flow/00-idea.md" <<'PY'
from pathlib import Path
import sys
text = Path(sys.argv[1]).read_text(encoding="utf-8")
if "- [ ]" in text or "[FILL" in text:
    raise SystemExit(0)
raise SystemExit("00-idea.md is no longer FAIL")
PY
}

journal_taxi1() {
  python3 - "$JOURNAL" "$FLOW_BIN" "$ISO_REAL" <<'PY'
import json, sys
path, flow_bin, iso = sys.argv[1], sys.argv[2], sys.argv[3]
rows = []
with open(path, encoding="utf-8") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        d = json.loads(line)
        if d.get("type") == "flow/result":
            rows.append(d)
if len(rows) != 1:
    raise SystemExit(f"taxi1 flow/result count={len(rows)}")
r = rows[0]
if r.get("bin") != flow_bin:
    raise SystemExit(f"taxi1 bin={r.get('bin')!r} want={flow_bin!r}")
if r.get("args") != ["next"]:
    raise SystemExit(f"taxi1 args={r.get('args')!r} want=['next']")
if r.get("code") != 1:
    raise SystemExit(f"taxi1 code={r.get('code')!r}")
if r.get("cwd") != iso:
    raise SystemExit(f"taxi1 cwd={r.get('cwd')!r} want={iso!r}")
stdout = r.get("stdout") or ""
if not isinstance(stdout, str):
    stdout = str(stdout)
if "FAIL: gate for stage 00-idea is not clean" not in stdout:
    raise SystemExit("taxi1 stdout missing FAIL gate 00-idea not clean")
if "unlocked stage" in stdout:
    raise SystemExit("taxi1 stdout has unlocked stage")
if "GATE stage" in stdout:
    raise SystemExit("taxi1 stdout has GATE stage")
PY
}

journal_taxi2() {
  python3 - "$JOURNAL" "$FLOW_BIN" "$ISO_REAL" <<'PY'
import json, sys
path, flow_bin, iso = sys.argv[1], sys.argv[2], sys.argv[3]
rows = []
with open(path, encoding="utf-8") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        d = json.loads(line)
        if d.get("type") == "flow/result":
            rows.append(d)
if len(rows) != 2:
    raise SystemExit(f"taxi2 flow/result count={len(rows)} want 2")
codes = [r.get("code") for r in rows]
if codes != [1, 0]:
    raise SystemExit(f"codes={codes} want [1, 0]")
for r in rows:
    if r.get("bin") != flow_bin:
        raise SystemExit(f"bin={r.get('bin')!r} want={flow_bin!r}")
    if r.get("args") != ["next"]:
        raise SystemExit(f"args={r.get('args')!r} want=['next']")
    if r.get("cwd") != iso:
        raise SystemExit(f"cwd={r.get('cwd')!r} want={iso!r}")
stdout = rows[1].get("stdout") or ""
if not isinstance(stdout, str):
    stdout = str(stdout)
needle = "unlocked stage 1 (flow/01-research.md)"
if needle not in stdout:
    raise SystemExit("taxi2 stdout missing unlocked stage 1")
if "already exists" in stdout:
    raise SystemExit("taxi2 stdout has already exists")
if "GATE stage" in stdout:
    raise SystemExit("taxi2 stdout has GATE stage")
if "unlocked stage 00" in stdout:
    raise SystemExit("taxi2 stdout has unlocked stage 00")
PY
}

research_sha_ok() {
  python3 - "$ISO_REAL/flow/01-research.md" "$RESEARCH_TEMPLATE" <<'PY'
import hashlib, pathlib, sys
def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
got, want = sha(sys.argv[1]), sha(sys.argv[2])
if got != want:
    raise SystemExit(f"01 sha {got} != template {want}")
text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
if "[FILL" not in text:
    raise SystemExit("01-research.md was filled")
PY
}

write_pass_want() {
  cat > "$PASS_WANT" <<'EOF'
# Stage 00 — Idea

## Gate — check ALL before `/flow next`
- [x] The pitch below is 3 sentences, no more
- [x] I can name at least ONE real person/group who has this pain (named below)
- [x] No FILL placeholders remain in this file

## Pitch (3 sentences: who, pain, what you'd build)

Operators sitting Dory cannot prove Flow unlocked research after a real idea gate.
AOE5 paid gate 00-idea; next must mint flow/01-research.md.
This fixture is a one-stage idea so Flow can fail then unlock.

## One real person/group with this pain

Founder sitting the Dory desk after isolate AOE5 gate.
EOF
}

coord_prompt_write_pass() {
  iso_mut agent prompt coord --timeout 180000 -- "$(python3 - "$ISO_REAL" "$PASS_WANT" <<'PY'
import sys
from pathlib import Path
iso = sys.argv[1]
want = Path(sys.argv[2]).read_text(encoding="utf-8")
path = iso + "/flow/00-idea.md"
print(
    "You are coord. Do not change DORY_SOCKET or XDG_RUNTIME_DIR. "
    "Do not run dory server stop. Do not run bare dory, attach, or herdr. "
    "Do not pass --wait or --timeout. Do not read cook skills or plan files. "
    "Do not read or write ~/.omp, agent.db, or PI_CODING_AGENT_DIR.\n"
    "Run exactly: dory agent prompt omptest -- You are omptest. "
    "Do not change DORY_SOCKET or XDG_RUNTIME_DIR. Do not run dory server stop. "
    "Do not run bare dory, attach, or herdr. Do not pass --wait or --timeout. "
    "Do not read or write ~/.omp, agent.db, or PI_CODING_AGENT_DIR. "
    "Overwrite this file with EXACTLY the following bytes and nothing else: "
    + path
    + "\n\n"
    + want
    + "\nThen run: dory agent report --current --state idle\n"
    "Then run: dory agent report --current --state idle"
)
PY
)"
}

poll_pass_file() {
  local deadline
  deadline=$((SECONDS + 180))
  while [ "$SECONDS" -lt "$deadline" ]; do
    factory_must_dead || return 1
    if [ -f "$ISO_REAL/flow/00-idea.md" ] && cmp -s "$ISO_REAL/flow/00-idea.md" "$PASS_WANT"; then
      return 0
    fi
    sleep 2
  done
  return 1
}

copy_artifacts() {
  local rc=0
  if [ -n "$JOURNAL" ] && [ -f "$JOURNAL" ]; then
    copy_journal || rc=1
  else
    rc=1
  fi
  copy_research_sha || true
  return "$rc"
}

teardown() {
  if [ "$TORN" = 1 ]; then
    return 0
  fi
  TORN=1
  copy_artifacts || true
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

self_refuse_paid || fail "self refuse paid scripts"

SIT_DORY="$(realpath "$SIT_DORY")"
if [ ! -x "$SIT_DORY" ]; then
  echo "refuse: SIT_DORY not executable" >&2
  exit 1
fi
SIT_DORY_SHA="$(sha256sum "$SIT_DORY" | awk '{print $1}')"
if [ "$SIT_DORY_SHA" != "$LAND_SHA" ]; then
  echo "refuse: SIT_DORY sha $SIT_DORY_SHA != land pin" >&2
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
  "$LEFTOVER_ISO"/*)
    echo "refuse: SIT_DORY is leftover isolate ELF" >&2
    exit 1
    ;;
  "$FACTORY_XDG"/*)
    echo "refuse: SIT_DORY is factory XDG ELF" >&2
    exit 1
    ;;
esac
if [ ! -d "$FACTORY_AGENT_DIR" ]; then
  echo "refuse: FACTORY_AGENT_DIR missing" >&2
  exit 1
fi
if ! command -v omp >/dev/null; then
  echo "refuse: omp missing on factory PATH" >&2
  exit 1
fi

FLOW_BIN="$(realpath "$FLOW_SKILL")"
if [ ! -x "$FLOW_BIN" ]; then
  echo "refuse: FLOW_BIN not executable" >&2
  exit 1
fi
if [ "$(basename "$FLOW_BIN")" != "flow.sh" ]; then
  echo "refuse: FLOW_BIN basename is not flow.sh" >&2
  exit 1
fi
if [ "$FLOW_BIN" = "/bin/true" ]; then
  echo "refuse: FLOW_BIN is /bin/true" >&2
  exit 1
fi
if [ ! -f "$IDEA_TEMPLATE" ]; then
  echo "refuse: idea template missing" >&2
  exit 1
fi
if [ ! -f "$RESEARCH_TEMPLATE" ]; then
  echo "refuse: research template missing" >&2
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

visible_has_needles() {
  case "$1" in
    *"Flow 1. next"*|*"Flow 0. next"*) return 0 ;;
  esac
  return 1
}

PRE="$(herdr pane read "$SIT_PANE" --source visible)" || fail "pane read preflight"
if visible_has_needles "$PRE"; then
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
  if visible_has_needles "$PRE"; then
    fail "visible still contains Flow needles after split"
  fi
fi

leftover_mint_ok || fail "leftover 5 != mint table"
desk_head_ok || fail "desk.rs != HEAD mint"
leftover_elf_stat_ok || fail "leftover ELF sha mismatch"
path_dory_empty || fail "PATH dory not empty at start"
factory_flow_absent || fail "factory flow/ present at start"

SNAP_DORY="$(repo_dory_stat)"

test ! -L "${HOME}/.cache"
mkdir -p -m 0700 "$CACHE"
test ! -L "$CACHE" || fail "cache symlink"

ISO="$(mktemp -d "$CACHE/aoe5n.XXXXXX")" || fail "mktemp isolate"
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
mkdir -p -m 0700 "$ISO_REAL/flow"
cp -f "$IDEA_TEMPLATE" "$ISO_REAL/flow/00-idea.md"
if [ -L "$ISO_REAL/flow/00-idea.md" ]; then
  fail "idea dest is symlink"
fi
idea_still_fail || fail "minted idea is not FAIL"
PASS_WANT="$ISO_REAL/.aoe5n-pass-want"
write_pass_want
if cmp -s "$ISO_REAL/flow/00-idea.md" "$PASS_WANT"; then
  fail "factory minted PASS idea"
fi
if [ -e "$ISO_REAL/home/.omp" ] || [ -e "$ISO_REAL/home/.agents" ]; then
  fail "isolate home grew an omp store"
fi
if [ -e "$ISO_REAL/flow/01-research.md" ]; then
  fail "factory minted 01-research.md"
fi

_flow_u=()
while IFS= read -r k; do
  [ -n "$k" ] || continue
  _flow_u+=(-u "$k")
done < <(flow_unset_keys)

mkdir -p -m 0700 "$ISO_REAL/bin"
ln -sfn "$SIT_DORY" "$ISO_REAL/bin/dory"
if [ "$(realpath "$ISO_REAL/bin/dory")" != "$SIT_DORY" ]; then
  fail "isolate bin/dory is not SIT_DORY"
fi
if [ "$(sha256sum "$(realpath "$ISO_REAL/bin/dory")" | awk '{print $1}')" != "$LAND_SHA" ]; then
  fail "isolate bin/dory sha != land pin"
fi

setsid env -u DORY_SOCKET -u DORY_ENV -u DORY_PANE_ID -u DORY_TAB_ID \
  -u DORY_WORKSPACE_ID -u DORY_SIT_SHELL \
  "${_flow_u[@]}" \
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
factory_must_dead || fail "factory sock connectable after split"

start_omp() {
  iso_mut agent start "$1" --pane "$2" -- omp --no-session --no-skills --no-rules --no-extensions >/dev/null
}

start_omp coord "$COORD_PANE" || fail "start coord"
start_omp omptest "$TEST_PANE" || fail "start omptest"
factory_must_dead || fail "factory sock connectable after agent start"

LIST_JSON="$(iso_mut pane list --workspace "$WS_ID")" || fail "iso pane list failed"
python3 - "$LIST_JSON" <<'PY' || fail "roster names missing on isolate pane list"
import json, sys
data = json.loads(sys.argv[1])
want = {"coord", "omptest"}
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

ready_wait "$COORD_PANE" "╭──" "coord prompt"
ready_wait "$TEST_PANE" "╭──" "omptest prompt"

report_idle() {
  iso_mut agent report --pane "$1" --state idle >/dev/null || fail "report idle $2"
}
report_idle "$COORD_PANE" coord
report_idle "$TEST_PANE" omptest

JOURNAL="$ISO_REAL/.dory/sessions/s1.jsonl"
TAXI1_RC=0
taxi >"$ISO_REAL/taxi1.out" 2>"$ISO_REAL/taxi1.err" || TAXI1_RC=$?
if [ "$TAXI1_RC" != 1 ]; then
  echo "taxi1 out:" >&2
  cat "$ISO_REAL/taxi1.out" >&2 || true
  echo "taxi1 err:" >&2
  cat "$ISO_REAL/taxi1.err" >&2 || true
  fail "taxi 1 rc=$TAXI1_RC want 1"
fi
[ -f "$JOURNAL" ] || fail "taxi 1 journal missing"
journal_taxi1 || fail "taxi 1 journal not fail-next"
idea_still_fail || fail "idea flipped PASS before occupant"
if [ -f "$ISO_REAL/flow/01-research.md" ]; then
  fail "01-research.md exists after taxi 1"
fi
if cmp -s "$ISO_REAL/flow/00-idea.md" "$PASS_WANT"; then
  fail "idea is PASS after taxi 1"
fi
if [ "$(repo_dory_stat)" != "$SNAP_DORY" ]; then
  fail "repo .dory changed after taxi 1"
fi
factory_must_dead || fail "factory sock connectable after taxi 1"
factory_flow_absent || fail "factory flow/ after taxi 1"

SIT_PATH_PROBE="$ISO_REAL/sit-path-probe"
SIT_PATH_DONE="$ISO_REAL/sit-path-probe.done"
rm -f "$SIT_PATH_PROBE" "$SIT_PATH_DONE"
herdr pane send-text "$SIT_PANE" "type -a dory >'$SIT_PATH_PROBE' 2>&1; echo \$? >'$SIT_PATH_DONE'" \
  || fail "send-text sit PATH probe"
herdr pane send-keys "$SIT_PANE" enter || fail "send-keys sit PATH probe"
_sit_path_i=0
while [ "$_sit_path_i" -lt 40 ]; do
  if [ -f "$SIT_PATH_DONE" ]; then
    break
  fi
  sleep 0.2
  _sit_path_i=$((_sit_path_i + 1))
done
[ -f "$SIT_PATH_DONE" ] || fail "sit PATH probe timeout"
if [ "$(tr -d '[:space:]' < "$SIT_PATH_DONE")" = 0 ]; then
  echo "sit PATH type rc=0:" >&2
  cat "$SIT_PATH_PROBE" >&2 || true
  fail "sit pane PATH has dory"
fi
if grep -q 'dory is ' "$SIT_PATH_PROBE" 2>/dev/null; then
  echo "sit PATH probe:" >&2
  cat "$SIT_PATH_PROBE" >&2 || true
  fail "sit pane PATH lists dory"
fi

ATTACH_CMD="cd \"$ISO_REAL\" && DORY_SKIP_ONBOARD=1 XDG_RUNTIME_DIR=$ISO_REAL env -u DORY_SOCKET -u DORY_ENV -u DORY_RECYCLE \"$SIT_DORY\" attach"
case "$ATTACH_CMD" in
  *"Flow 1. next"*|*"Flow 0. next"*) fail "ATTACH_CMD contains needle" ;;
esac

herdr pane send-text "$SIT_PANE" "$ATTACH_CMD" || fail "send-text attach"
herdr pane send-keys "$SIT_PANE" enter || fail "send-keys enter"

if ! herdr pane wait-output "$SIT_PANE" --match "Flow 1. next" --source visible --timeout 20000; then
  echo "visible after miss (Flow 1. next):" >&2
  herdr pane read "$SIT_PANE" --source visible >&2 || true
  fail "wait-output miss Flow 1. next"
fi
factory_must_dead || fail "factory sock connectable after attach"

PROMPT_OUT="$(coord_prompt_write_pass)" || {
  echo "$PROMPT_OUT" >&2
  fail "coord prompt cli failed"
}
PROMPTED=1

if ! poll_pass_file; then
  echo "pass file miss after first prompt; re-prompt once" >&2
  PROMPT_OUT="$(coord_prompt_write_pass)" || {
    echo "$PROMPT_OUT" >&2
    fail "coord re-prompt cli failed"
  }
  if ! poll_pass_file; then
    echo "00-idea.md after poll:" >&2
    cat "$ISO_REAL/flow/00-idea.md" >&2 || true
    fail "occupant did not write PASS bytes"
  fi
fi
if ! cmp -s "$ISO_REAL/flow/00-idea.md" "$PASS_WANT"; then
  fail "PASS file mismatch after poll"
fi

TAXI2_RC=0
taxi >"$ISO_REAL/taxi2.out" 2>"$ISO_REAL/taxi2.err" || TAXI2_RC=$?
if [ "$TAXI2_RC" != 0 ]; then
  echo "taxi2 out:" >&2
  cat "$ISO_REAL/taxi2.out" >&2 || true
  echo "taxi2 err:" >&2
  cat "$ISO_REAL/taxi2.err" >&2 || true
  fail "taxi 2 rc=$TAXI2_RC want 0"
fi
[ -f "$JOURNAL" ] || fail "taxi 2 journal missing"
journal_taxi2 || fail "taxi 2 journal not fail-then-unlock"
if [ ! -f "$ISO_REAL/flow/01-research.md" ]; then
  fail "01-research.md missing after taxi 2"
fi
research_sha_ok || fail "01 sha != template"
if [ "$(repo_dory_stat)" != "$SNAP_DORY" ]; then
  fail "repo .dory changed after taxi 2"
fi
factory_must_dead || fail "factory sock connectable after taxi 2"
factory_flow_absent || fail "factory flow/ after taxi 2"

if ! herdr pane wait-output "$SIT_PANE" --match "Flow 0. next" --source visible --timeout 20000; then
  echo "visible after miss (Flow 0. next):" >&2
  herdr pane read "$SIT_PANE" --source visible >&2 || true
  fail "wait-output miss Flow 0. next"
fi

leftover_mint_ok || fail "leftover 5 != mint table after taxis"
desk_head_ok || fail "desk.rs != HEAD after taxis"
leftover_elf_stat_ok || fail "leftover ELF sha mismatch after taxis"
if [ "$(repo_dory_stat)" != "$SNAP_DORY" ]; then
  fail "repo .dory changed before wipe"
fi
factory_must_dead || fail "factory sock connectable before stop"
path_dory_empty || fail "PATH dory not empty before stop"
factory_flow_absent || fail "factory flow/ before stop"

RESEARCH_SHA="$(sha256sum "$ISO_REAL/flow/01-research.md" | awk '{print $1}')"
TEMPLATE_SHA="$(sha256sum "$RESEARCH_TEMPLATE" | awk '{print $1}')"

copy_journal || fail "copy journal before wipe"
copy_research_sha || fail "copy 01 sha before wipe"
teardown
TORN=1
if sock_connectable "$FACTORY_XDG/dory/default/dory.sock"; then
  fail "factory sock connectable after teardown"
fi
if [ "$(repo_dory_stat)" != "$SNAP_DORY" ]; then
  fail "repo .dory changed after teardown"
fi
leftover_mint_ok || fail "leftover 5 != mint table after teardown"
desk_head_ok || fail "desk.rs != HEAD after teardown"
path_dory_empty || fail "PATH dory not empty after teardown"
factory_flow_absent || fail "factory flow/ after teardown"
[ -f "$JOURNAL_COPY_RECEIPT" ] || fail "journal receipt copy missing"
[ -f "$RESEARCH_SHA_RECEIPT" ] || fail "01 sha receipt copy missing"

echo "SIT_DORY=$SIT_DORY"
echo "SIT_DORY_SHA=$SIT_DORY_SHA"
echo "SIT_PANE=$SIT_PANE"
echo "SIT_TAB=$SIT_TAB"
echo "FLOW_BIN=$FLOW_BIN"
echo "WS_ID=$WS_ID"
echo "COORD_PANE=$COORD_PANE"
echo "TEST_PANE=$TEST_PANE"
echo "FACTORY_SOCK=$FACTORY_SOCK"
echo "FACTORY_CONNECTABLE=0"
echo "REPO_DORY_STAT=$SNAP_DORY"
echo "TAXI1_RC=1"
echo "TAXI2_RC=0"
echo "JOURNAL_CODES=1,0"
echo "NEEDLE1=Flow 1. next"
echo "NEEDLE2=Flow 0. next"
echo "JOURNAL_COPY=$JOURNAL_COPY_RECEIPT"
echo "RESEARCH_SHA=$RESEARCH_SHA"
echo "TEMPLATE_SHA=$TEMPLATE_SHA"
echo "RESEARCH_SHA_COPY=$RESEARCH_SHA_RECEIPT"
echo "VISIBLE_MATCH=1"
echo "aoe5-next=PASS"
exit 0
