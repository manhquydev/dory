#!/usr/bin/env bash
# Lamp (Session OS) only. Never installs Node. Never binds desk `dory`.
# Never sudo. Never curl|sh a toolchain.
set -euo pipefail

PKG="@manhquy/dory"
BIN="dory-serve"
NODE_MIN="22.14.0"
UNSCOPED_TRAP="dory"

usage() {
  cat <<'EOF'
usage: dory-lamp-doctor.sh [--install | --uninstall | --help]

diagnose (default)
  node >= 22.14.0, live @manhquy/dory, PATH collisions.
  prints the npx lamp command. does not install anything.

--install
  npm install -g @manhquy/dory  (lamp bin dory-serve only)
  refused if node is missing/old, or npm prefix looks like cursor-agent.

--uninstall
  npm uninstall -g @manhquy/dory
  does not remove leftover/isolate Rust ELF.

this is not desk. gõ dory is the Rust SKU. do not npm i -g dory
EOF
}

node_ge_min() {
  node -e '
    const need = process.argv[1].split(".").map(Number);
    const have = process.versions.node.split(".").map(Number);
    const ok = have[0] > need[0]
      || (have[0] === need[0] && have[1] > need[1])
      || (have[0] === need[0] && have[1] === need[1] && have[2] >= need[2]);
    process.exit(ok ? 0 : 1);
  ' "$NODE_MIN"
}

print_node_help() {
  cat <<'EOF'
Node >= 22.14.0 is required for the lamp. Dory will not install Node.
  https://nodejs.org/en/download
  or fnm: https://github.com/Schniz/fnm
Then: npx @manhquy/dory@0.1.0-next.0 dory-serve -- serve --workspace /abs
EOF
}

prefix_is_agent() {
  case "$(npm prefix -g 2>/dev/null || true)" in
    *cursor-agent*|*cursor/|*versions/20*) return 0 ;;
    *) return 1 ;;
  esac
}

cmd_path() {
  command -v "$1" 2>/dev/null || true
}

diagnose() {
  local rc=0
  echo "pkg $PKG  bin $BIN  node_min $NODE_MIN"

  if ! command -v node >/dev/null 2>&1; then
    echo "FAIL: node not on PATH"
    print_node_help
    return 1
  fi
  echo "node $(node -v) at $(cmd_path node)"
  if ! node_ge_min; then
    echo "FAIL: node $(node -v) < $NODE_MIN"
    print_node_help
    return 1
  fi

  if ! command -v npm >/dev/null 2>&1; then
    echo "FAIL: npm not on PATH"
    print_node_help
    return 1
  fi
  echo "npm $(npm -v) prefix-g $(npm prefix -g 2>/dev/null || echo '?')"
  if prefix_is_agent; then
    echo "note: npm prefix-g is an agent runtime. do not --install here; use npx."
  fi

  if command -v npm >/dev/null 2>&1; then
    if ! npm view "$PKG" name version bin --json >/tmp/dory-lamp-view.json 2>/tmp/dory-lamp-view.err; then
      echo "FAIL: npm view $PKG"
      tail -n 8 /tmp/dory-lamp-view.err || true
      rm -f /tmp/dory-lamp-view.json /tmp/dory-lamp-view.err
      return 2
    fi
    node -e '
      const fs = require("node:fs");
      let p = JSON.parse(fs.readFileSync("/tmp/dory-lamp-view.json", "utf8"));
      if (Array.isArray(p)) p = p[0];
      const bin = p.bin || {};
      if (bin.dory) { console.error("FAIL: registry bin.dory steals desk SKU"); process.exit(2); }
      if (!bin["dory-serve"]) { console.error("FAIL: registry missing dory-serve"); process.exit(2); }
      console.log("registry", p.name, p.version, "bin", JSON.stringify(bin));
    '
    rm -f /tmp/dory-lamp-view.json /tmp/dory-lamp-view.err
  fi

  local dory_p serve_p
  dory_p="$(cmd_path "$UNSCOPED_TRAP")"
  serve_p="$(cmd_path "$BIN")"
  if [ -n "$dory_p" ]; then
    echo "note: PATH dory=$dory_p  (desk SKU or a stranger — not this lamp)"
    case "$dory_p" in
      *node_modules/dory/*|*node_modules/@clidey/*|*node_modules/@getdory/*)
        echo "FAIL: PATH dory looks like an npm lamp/CLI collision. do not use it as desk."
        rc=3
        ;;
    esac
  else
    echo "PATH dory: (none)"
  fi
  if [ -n "$serve_p" ]; then
    echo "PATH dory-serve=$serve_p"
  else
    echo "PATH dory-serve: (none) — expected unless you --install. use npx."
  fi

  echo "run: npx ${PKG}@0.1.0-next.0 ${BIN} -- serve --workspace /abs"
  echo "uninstall: npm uninstall -g $PKG"
  echo "never: npm i -g $UNSCOPED_TRAP"
  return "$rc"
}

do_install() {
  if ! command -v node >/dev/null 2>&1 || ! node_ge_min; then
    echo "FAIL: --install needs node >= $NODE_MIN"
    print_node_help
    return 1
  fi
  if prefix_is_agent; then
    echo "FAIL: npm prefix-g is an agent runtime. use npx, not --install."
    return 1
  fi
  npm install -g "$PKG"
  echo "installed. verify: command -v $BIN"
}

do_uninstall() {
  npm uninstall -g "$PKG" || true
  echo "uninstalled $PKG (if it was global). leftover/isolate ELF not touched."
}

case "${1:-diagnose}" in
  --help|-h) usage ;;
  --install) diagnose && do_install ;;
  --uninstall) do_uninstall ;;
  diagnose|"") diagnose ;;
  *) usage >&2; exit 4 ;;
esac
