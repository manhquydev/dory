//! P5 occupant wait — ag_tst.
//!
//! Proves cook-brief cases 1–13 through the CLI. Harness shape copied from
//! `pane_io.rs`: `CARGO_BIN_EXE_dory`, temp `XDG_RUNTIME_DIR`, `dory server`,
//! IDs parsed from snapshot / `.result`. Never hardcode the next id as `w1`.
//!
//!  1 missing pane → exit 1, pane count unchanged
//!  2 busy pane → exit 1, pane count unchanged
//!  3 `--kind` on `agent start` → exit 2, no occupant
//!  4 invalid names refused; `a` is valid
//!  5 two-agent = split then start; layout ids unchanged
//!  6 `agent read` does not flip `seen` or `done`→`idle`
//!  7 `agent focus` turns `done`→`idle` and `seen=true`
//!  8 `unknown` is not a default `agent wait` success
//!  9 `prompt` while `blocked` refuses and does not append
//! 10 default `prompt --wait` / `agent wait` accept idle|done|blocked only
//! 11 refuse `herdr` / `dsh` argv (no exec)
//! 12 outside `DORY_ENV` → exit 1
//! 13 A drives B: start fixture, prompt, wait settles

use std::collections::BTreeSet;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const READY_NEEDLE: &str = "DORY_OCC_READY";
const BLOCKED_NEEDLE: &str = "DORY_OCC_BLOCKED";
const READY_FIXTURE: &str = concat!(
    "printf '%s\\n' DORY_OCC_READY; ",
    "while IFS= read -r line; do ",
    "printf 'got:%s\\n' \"$line\"; ",
    "printf '%s\\n' DORY_OCC_READY; ",
    "done"
);
const BLOCKED_FIXTURE: &str = concat!(
    "printf '%s\\n' DORY_OCC_BLOCKED; ",
    "while IFS= read -r line; do printf 'got:%s\\n' \"$line\"; done"
);
const BLOCKED_PROMPT: &str = "P5_BLOCKED_PROMPT_260822";
const DRIVE_PROMPT: &str = "P5_DRIVE_PROMPT_260822";
const START_TIMEOUT: &str = "8000";
const SETTLE_TIMEOUT: &str = "8000";
const NEG_WAIT_TIMEOUT: &str = "1500";

struct Harness {
    xdg: PathBuf,
    sock: PathBuf,
    server: Child,
}

impl Drop for Harness {
    fn drop(&mut self) {
        let _ = Command::new(bin())
            .args(["server", "stop"])
            .env("XDG_RUNTIME_DIR", &self.xdg)
            .output();
        let _ = self.server.kill();
        let _ = self.server.wait();
        let _ = fs::remove_dir_all(&self.xdg);
    }
}

struct Created {
    ws: String,
    tab: String,
    pane: String,
    snap_pane: String,
}

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn temp_xdg() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!("dory-p5-occ-{}-{}", std::process::id(), nanos));
    fs::create_dir_all(&path).unwrap();
    path
}

fn session_sock(xdg: &Path) -> PathBuf {
    xdg.join("dory").join("default").join("dory.sock")
}

fn start() -> Harness {
    let xdg = temp_xdg();
    let mut server = Command::new(bin())
        .arg("server")
        .env("XDG_RUNTIME_DIR", &xdg)
        .current_dir(&xdg)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn dory server");
    let sock = session_sock(&xdg);
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if UnixStream::connect(&sock).is_ok() {
            return Harness { xdg, sock, server };
        }
        if let Ok(Some(status)) = server.try_wait() {
            let mut err = String::new();
            if let Some(mut stderr) = server.stderr.take() {
                let _ = stderr.read_to_string(&mut err);
            }
            panic!("dory server exited {status}: {err}");
        }
        thread::sleep(Duration::from_millis(20));
    }
    let _ = server.kill();
    panic!("dory server did not bind {}", sock.display());
}

fn cli(h: &Harness, pane: &str, args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_SOCKET", &h.sock)
        .env("DORY_ENV", "1")
        .env("DORY_PANE_ID", pane)
        .output()
        .expect("dory cli")
}

fn cli_no_env(h: &Harness, args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_SOCKET", &h.sock)
        .env_remove("DORY_ENV")
        .env_remove("DORY_PANE_ID")
        .output()
        .expect("dory cli")
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn json_field<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":\"");
    let rest = json.split_once(&pat)?.1;
    Some(rest.split_once('"')?.0)
}

fn nested_id<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":{{");
    let rest = json.split_once(&pat)?.1;
    let after = rest.split_once("\"id\":\"")?.1;
    Some(after.split_once('"')?.0)
}

fn result_id(json: &str, key: &str) -> String {
    nested_id(json, key)
        .or_else(|| json_field(json, key))
        .unwrap_or_else(|| panic!("missing .result {key} in {json}"))
        .to_string()
}

fn optional_result_pane(json: &str) -> Option<String> {
    nested_id(json, "pane")
        .or_else(|| json_field(json, "pane"))
        .map(str::to_string)
}
fn json_string_value(json: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":\"");
    let rest = json.split_once(&pat)?.1;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                other => out.push(other),
            }
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    None
}

fn seen_token(json: &str) -> Option<String> {
    let rest = json.split_once("\"seen\":")?.1.trim_start();
    if rest.starts_with('"') {
        let inner = rest.get(1..)?.split_once('"')?.0;
        return Some(format!("\"{inner}\""));
    }
    let tok: String = rest
        .chars()
        .take_while(|c| !c.is_whitespace() && *c != ',' && *c != '}')
        .collect();
    if tok.is_empty() { None } else { Some(tok) }
}

fn seen_bool(json: &str) -> Option<bool> {
    match seen_token(json)?.as_str() {
        "true" | "\"true\"" => Some(true),
        "false" | "\"false\"" => Some(false),
        _ => None,
    }
}

fn rpc(h: &Harness, line: &str) -> String {
    let mut stream = UnixStream::connect(&h.sock).expect("rpc connect");
    writeln!(stream, "{line}").unwrap();
    let _ = stream.flush();
    let mut reply = String::new();
    BufReader::new(stream).read_line(&mut reply).unwrap();
    reply
}

fn snapshot(h: &Harness) -> String {
    rpc(h, r#"{"op":"snapshot"}"#)
}

fn pane_get(h: &Harness, pane: &str) -> String {
    rpc(h, &format!(r#"{{"op":"pane.get","pane":"{pane}"}}"#))
}

fn occupant_name(json: &str) -> Option<String> {
    let rest = json.split_once("\"occupant\":")?.1.trim_start();
    if rest.starts_with("null") {
        return None;
    }
    if rest.starts_with('{') {
        return json_field(rest, "name").map(str::to_string);
    }
    if rest.starts_with('"') {
        return rest.get(1..)?.split_once('"').map(|(s, _)| s.to_string());
    }
    None
}

fn workspace_list(h: &Harness, pane: &str) -> String {
    let out = cli(h, pane, &["workspace", "list"]);
    assert!(out.status.success(), "workspace list: {}", stderr(&out));
    stdout(&out)
}

fn extract_quoted_ids(json: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let bytes = json.as_bytes();
    let needle = b"\"id\":\"";
    let mut i = 0;
    while i + needle.len() < bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let start = i + needle.len();
            if let Some(rel) = json[start..].find('"') {
                let id = &json[start..start + rel];
                if id.starts_with('w') {
                    ids.insert(id.to_string());
                }
                i = start + rel + 1;
                continue;
            }
        }
        i += 1;
    }
    ids
}

fn pane_seq(id: &str) -> Option<u64> {
    id.rsplit_once(":p")?.1.parse().ok()
}

fn known_pane_ids(created: &Created, extra: &[&str]) -> Vec<String> {
    let mut ids = vec![created.snap_pane.clone(), created.pane.clone()];
    for id in extra {
        if !ids.iter().any(|have| have == id) {
            ids.push((*id).to_string());
        }
    }
    ids
}

fn next_split_seq(known: &[String]) -> u64 {
    known
        .iter()
        .filter_map(|id| pane_seq(id))
        .max()
        .expect("known panes")
        + 1
}

fn created_layout(h: &Harness) -> Created {
    let snap = snapshot(h);
    assert!(snap.contains("\"live\":true"), "{snap}");
    let snap_pane = json_field(&snap, "pane")
        .expect("snapshot pane")
        .to_string();
    assert!(!snap_pane.is_empty(), "{snap}");
    assert_ne!(
        snap_pane, "w1",
        "snapshot pane is a pane id, not workspace w1"
    );

    let created = cli(h, &snap_pane, &["workspace", "create"]);
    assert!(
        created.status.success(),
        "workspace create: {}",
        stderr(&created)
    );
    let body = stdout(&created);
    assert!(body.contains("\"ok\":true"), "{body}");
    let ws = result_id(&body, "workspace");
    let tab = result_id(&body, "tab");
    let pane = result_id(&body, "root_pane");
    assert_ne!(ws, "w1", "next workspace must come from .result, not w1");
    assert_ne!(
        pane, snap_pane,
        "created pane must not be the snapshot pane"
    );
    assert!(
        occupant_name(&body).is_none(),
        "create must not start an occupant: {body}"
    );
    Created {
        ws,
        tab,
        pane,
        snap_pane,
    }
}

fn split_pane(h: &Harness, caller: &str) -> String {
    let out = cli(
        h,
        caller,
        &[
            "pane",
            "split",
            "--pane",
            caller,
            "--direction",
            "right",
            "--no-focus",
        ],
    );
    assert!(out.status.success(), "pane split: {}", stderr(&out));
    let body = stdout(&out);
    assert!(body.contains("\"ok\":true"), "{body}");
    let pane = result_id(&body, "pane");
    assert_ne!(pane, caller, "split must return a new pane");
    assert!(
        occupant_name(&body).is_none(),
        "split must not start an occupant: {body}"
    );
    pane
}

fn assert_pane_count_unchanged(h: &Harness, caller: &str, known: &[String]) {
    let expected = next_split_seq(known);
    let new_pane = split_pane(h, caller);
    let got = pane_seq(&new_pane).unwrap_or_else(|| panic!("pane id {new_pane}"));
    assert_eq!(
        got, expected,
        "pane count changed (start minted a pane); known={known:?} next={new_pane}"
    );
}

fn agent_start(h: &Harness, pane: &str, name: &str, argv: &[&str]) -> Output {
    let mut args = vec![
        "agent",
        "start",
        name,
        "--pane",
        pane,
        "--timeout",
        START_TIMEOUT,
        "--",
    ];
    args.extend_from_slice(argv);
    cli(h, pane, &args)
}

fn agent_get(h: &Harness, pane: &str, name: &str) -> Output {
    cli(h, pane, &["agent", "get", name])
}

fn agent_read(h: &Harness, pane: &str, name: &str) -> Output {
    cli(
        h,
        pane,
        &["agent", "read", name, "--source", "recent-unwrapped"],
    )
}

fn agent_focus(h: &Harness, pane: &str, name: &str) -> Output {
    cli(h, pane, &["agent", "focus", name])
}

fn agent_wait(h: &Harness, pane: &str, name: &str, timeout_ms: &str) -> Output {
    cli(h, pane, &["agent", "wait", name, "--timeout", timeout_ms])
}

fn agent_prompt(h: &Harness, pane: &str, name: &str, text: &str) -> Output {
    cli(h, pane, &["agent", "prompt", name, "--", text])
}

fn agent_prompt_wait(h: &Harness, pane: &str, name: &str, text: &str) -> Output {
    cli(
        h,
        pane,
        &[
            "agent",
            "prompt",
            name,
            "--wait",
            "--timeout",
            SETTLE_TIMEOUT,
            "--",
            text,
        ],
    )
}

fn start_ready(h: &Harness, pane: &str, name: &str) -> Output {
    agent_start(h, pane, name, &["/bin/sh", "-c", READY_FIXTURE])
}

fn start_blocked(h: &Harness, pane: &str, name: &str) -> Output {
    agent_start(h, pane, name, &["/bin/sh", "-c", BLOCKED_FIXTURE])
}

fn start_working(h: &Harness, pane: &str, name: &str) -> Output {
    agent_start(h, pane, name, &["/bin/sleep", "120"])
}

fn start_unknown(h: &Harness, pane: &str, name: &str) -> Output {
    agent_start(h, pane, name, &["/usr/bin/tail", "-f", "/dev/null"])
}

fn assert_ok(out: &Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} exit={:?} stdout={} stderr={}",
        out.status.code(),
        stdout(out),
        stderr(out)
    );
    let body = stdout(out);
    assert!(body.contains("\"ok\":true"), "{what}: {body}");
}

fn assert_exit(out: &Output, code: i32, what: &str) {
    assert_eq!(
        out.status.code(),
        Some(code),
        "{what} exit={:?} stdout={} stderr={}",
        out.status.code(),
        stdout(out),
        stderr(out)
    );
}

fn wait_pane_match(h: &Harness, pane: &str, needle: &str) {
    let out = cli(
        h,
        pane,
        &[
            "pane",
            "wait-output",
            "--pane",
            pane,
            "--match",
            needle,
            "--timeout",
            "8000",
        ],
    );
    assert!(
        out.status.success(),
        "wait-output {needle}: {}",
        stderr(&out)
    );
}

fn wait_state(h: &Harness, pane: &str, name: &str, allowed: &[&str]) -> String {
    let start = Instant::now();
    let mut last;
    loop {
        let out = agent_get(h, pane, name);
        if out.status.success() {
            last = stdout(&out);
            if let Some(state) = json_field(&last, "state") {
                if allowed.iter().any(|w| *w == state) {
                    return last;
                }
            }
        } else {
            last = format!("stdout={} stderr={}", stdout(&out), stderr(&out));
        }
        if start.elapsed() >= Duration::from_secs(8) {
            panic!("timed out waiting for {name} in {allowed:?}: {last}");
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn wait_agent_text(h: &Harness, pane: &str, name: &str, needle: &str) -> String {
    let start = Instant::now();
    let mut last;
    loop {
        let out = agent_read(h, pane, name);
        if out.status.success() {
            last = stdout(&out);
            let text = json_string_value(&last, "text").unwrap_or_else(|| last.clone());
            if text.contains(needle) {
                return last;
            }
        } else {
            last = format!("stdout={} stderr={}", stdout(&out), stderr(&out));
        }
        if start.elapsed() >= Duration::from_secs(8) {
            panic!("timed out waiting for {needle} in agent read: {last}");
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn assert_settled_state(json: &str) {
    let state = json_field(json, "state").unwrap_or("");
    assert!(
        matches!(state, "idle" | "done" | "blocked"),
        "expected idle|done|blocked, got {state} in {json}"
    );
}

fn wait_settled_body(h: &Harness, pane: &str, name: &str, out: &Output, what: &str) -> String {
    assert_ok(out, what);
    let body = stdout(out);
    if json_field(&body, "state").is_some() {
        assert_settled_state(&body);
        return body;
    }
    let got = agent_get(h, pane, name);
    assert_ok(&got, &format!("{what} get"));
    let g = stdout(&got);
    assert_settled_state(&g);
    g
}

fn assert_no_occupant(h: &Harness, pane: &str) {
    let got = pane_get(h, pane);
    assert!(
        occupant_name(&got).is_none(),
        "expected no occupant on {pane}: {got}"
    );
}

#[test]
fn p5_01_missing_pane_exit_one_count_unchanged() {
    let h = start();
    let created = created_layout(&h);
    let before = extract_quoted_ids(&workspace_list(&h, &created.pane));
    let known = known_pane_ids(&created, &[]);
    let missing = format!("{}-missing", created.pane);

    let out = agent_start(&h, &missing, "ghost", &["/bin/sleep", "5"]);
    assert_exit(&out, 1, "missing pane start");
    let err = format!("{}{}", stdout(&out), stderr(&out));
    assert!(
        !err.contains("unknown op"),
        "server must implement agent.start: {err}"
    );
    assert_no_occupant(&h, &created.pane);

    let after = extract_quoted_ids(&workspace_list(&h, &created.pane));
    assert_eq!(before, after, "workspace/tab ids must stay put");
    assert_pane_count_unchanged(&h, &created.pane, &known);
}

#[test]
fn p5_02_busy_pane_exit_one_count_unchanged() {
    let h = start();
    let created = created_layout(&h);
    let first = start_working(&h, &created.pane, "alice");
    assert_ok(&first, "first start");
    let _ = wait_state(&h, &created.pane, "alice", &["working", "unknown"]);

    let known = known_pane_ids(&created, &[]);
    let before = extract_quoted_ids(&workspace_list(&h, &created.pane));
    let busy = agent_start(&h, &created.pane, "bob", &["/bin/sleep", "5"]);
    assert_exit(&busy, 1, "busy pane start");

    let got = agent_get(&h, &created.pane, "alice");
    assert_ok(&got, "alice still live");
    assert_eq!(json_field(&stdout(&got), "name"), Some("alice"));
    let bob = agent_get(&h, &created.pane, "bob");
    assert!(
        !bob.status.success(),
        "bob must not occupy the busy pane: {}",
        stdout(&bob)
    );

    let after = extract_quoted_ids(&workspace_list(&h, &created.pane));
    assert_eq!(before, after, "workspace/tab ids must stay put");
    assert_pane_count_unchanged(&h, &created.pane, &known);
}

#[test]
fn p5_03_kind_is_usage_no_occupant() {
    let h = start();
    let created = created_layout(&h);
    let known = known_pane_ids(&created, &[]);
    let pane = created.pane.as_str();
    let kind_after_name = [
        "agent",
        "start",
        "kinded",
        "--kind",
        "sh",
        "--pane",
        pane,
        "--",
        "/bin/true",
    ];
    let kind_after_pane = [
        "agent",
        "start",
        "kinded",
        "--pane",
        pane,
        "--kind",
        "sh",
        "--",
        "/bin/true",
    ];
    let kind_before_name = [
        "agent",
        "start",
        "--kind",
        "sh",
        "kinded",
        "--pane",
        pane,
        "--",
        "/bin/true",
    ];
    for args in [
        kind_after_name.as_slice(),
        kind_after_pane.as_slice(),
        kind_before_name.as_slice(),
    ] {
        let out = cli(&h, pane, args);
        assert_exit(&out, 2, &format!("--kind {:?}", args));
        assert_no_occupant(&h, pane);
    }
    assert_pane_count_unchanged(&h, &created.pane, &known);
}

#[test]
fn p5_04_invalid_names_and_valid_a() {
    let h = start();
    let created = created_layout(&h);
    for name in ["Alice", "1bad", "a@b", ""] {
        let out = agent_start(&h, &created.pane, name, &["/bin/true"]);
        assert!(
            matches!(out.status.code(), Some(1) | Some(2)),
            "invalid name {name:?} exit={:?} stderr={}",
            out.status.code(),
            stderr(&out)
        );
        assert_no_occupant(&h, &created.pane);
    }

    let ok = start_working(&h, &created.pane, "a");
    assert_ok(&ok, "name a is valid");
    let got = wait_state(&h, &created.pane, "a", &["working", "unknown"]);
    assert_eq!(json_field(&got, "name"), Some("a"));
    assert_eq!(
        occupant_name(&pane_get(&h, &created.pane)).as_deref(),
        Some("a")
    );
}

#[test]
fn p5_05_two_agent_split_then_start_layout_ids_unchanged() {
    let h = start();
    let created = created_layout(&h);
    let sibling = split_pane(&h, &created.pane);
    let known = known_pane_ids(&created, &[&sibling]);
    let layout_before = extract_quoted_ids(&workspace_list(&h, &created.pane));
    assert!(layout_before.contains(&created.ws));
    assert!(layout_before.contains(&created.tab));

    let start_a = start_working(&h, &sibling, "alpha");
    assert_ok(&start_a, "start alpha on split pane");
    let start_a_body = stdout(&start_a);
    if let Some(pane) = optional_result_pane(&start_a_body) {
        assert_eq!(pane, sibling, "start must not mint a pane");
    }

    let start_b = start_unknown(&h, &created.pane, "beta");
    assert_ok(&start_b, "start beta on caller");
    let start_b_body = stdout(&start_b);
    if let Some(pane) = optional_result_pane(&start_b_body) {
        assert_eq!(pane, created.pane, "start must not mint a pane");
    }

    let layout_after = extract_quoted_ids(&workspace_list(&h, &created.pane));
    assert_eq!(
        layout_before, layout_after,
        "workspace/tab ids changed across start"
    );

    let a = agent_get(&h, &sibling, "alpha");
    assert_ok(&a, "alpha get");
    assert_eq!(json_field(&stdout(&a), "pane"), Some(sibling.as_str()));
    let b = agent_get(&h, &created.pane, "beta");
    assert_ok(&b, "beta get");
    assert_eq!(json_field(&stdout(&b), "pane"), Some(created.pane.as_str()));

    assert_eq!(
        occupant_name(&pane_get(&h, &sibling)).as_deref(),
        Some("alpha")
    );
    assert_eq!(
        occupant_name(&pane_get(&h, &created.pane)).as_deref(),
        Some("beta")
    );
    assert_pane_count_unchanged(&h, &created.pane, &known);
}

#[test]
fn p5_06_agent_read_does_not_flip_seen_or_done() {
    let h = start();
    let created = created_layout(&h);
    assert_ok(&start_ready(&h, &created.pane, "reader"), "start reader");
    wait_pane_match(&h, &created.pane, READY_NEEDLE);
    let before = wait_state(&h, &created.pane, "reader", &["done"]);
    assert_eq!(
        seen_bool(&before),
        Some(false),
        "done starts unseen: {before}"
    );

    let read = agent_read(&h, &created.pane, "reader");
    assert_ok(&read, "agent read");
    let text = json_string_value(&stdout(&read), "text").unwrap_or_default();
    assert!(text.contains(READY_NEEDLE), "{}", stdout(&read));

    let after = agent_get(&h, &created.pane, "reader");
    assert_ok(&after, "get after read");
    let body = stdout(&after);
    assert_eq!(json_field(&body, "state"), Some("done"), "{body}");
    assert_eq!(
        seen_bool(&body),
        Some(false),
        "read must not set seen: {body}"
    );
}

#[test]
fn p5_07_agent_focus_done_to_idle() {
    let h = start();
    let created = created_layout(&h);
    assert_ok(&start_ready(&h, &created.pane, "focusme"), "start focusme");
    wait_pane_match(&h, &created.pane, READY_NEEDLE);
    let before = wait_state(&h, &created.pane, "focusme", &["done"]);
    assert_eq!(seen_bool(&before), Some(false), "{before}");

    let focus = agent_focus(&h, &created.pane, "focusme");
    assert_ok(&focus, "agent focus");
    let focused = stdout(&focus);
    let state_src = if json_field(&focused, "state").is_some() {
        focused
    } else {
        let got = agent_get(&h, &created.pane, "focusme");
        assert_ok(&got, "get after focus");
        stdout(&got)
    };
    assert_eq!(json_field(&state_src, "state"), Some("idle"), "{state_src}");
    assert_eq!(seen_bool(&state_src), Some(true), "{state_src}");
}

#[test]
fn p5_08_unknown_is_not_default_wait_success() {
    let h = start();
    let created = created_layout(&h);
    assert_ok(&start_unknown(&h, &created.pane, "unk"), "start unknown");
    let got = wait_state(&h, &created.pane, "unk", &["unknown"]);
    assert_eq!(json_field(&got, "state"), Some("unknown"), "{got}");

    let wait = agent_wait(&h, &created.pane, "unk", NEG_WAIT_TIMEOUT);
    assert!(
        !wait.status.success(),
        "unknown must not satisfy default wait: stdout={} stderr={}",
        stdout(&wait),
        stderr(&wait)
    );
    let still = agent_get(&h, &created.pane, "unk");
    assert_ok(&still, "still unknown");
    assert_eq!(json_field(&stdout(&still), "state"), Some("unknown"));
}

#[test]
fn p5_09_prompt_refuses_blocked_does_not_append() {
    let h = start();
    let created = created_layout(&h);
    assert_ok(
        &start_blocked(&h, &created.pane, "blocker"),
        "start blocker",
    );
    wait_pane_match(&h, &created.pane, BLOCKED_NEEDLE);
    let _ = wait_state(&h, &created.pane, "blocker", &["blocked"]);

    let prompt = agent_prompt(&h, &created.pane, "blocker", BLOCKED_PROMPT);
    assert_exit(&prompt, 1, "prompt while blocked");

    let read = agent_read(&h, &created.pane, "blocker");
    assert_ok(&read, "read after refused prompt");
    let text = json_string_value(&stdout(&read), "text").unwrap_or_else(|| stdout(&read));
    assert!(
        !text.contains(BLOCKED_PROMPT),
        "blocked prompt must not append: {text}"
    );

    let pane_read = cli(
        &h,
        &created.pane,
        &[
            "pane",
            "read",
            "--pane",
            &created.pane,
            "--source",
            "recent-unwrapped",
        ],
    );
    assert_ok(&pane_read, "pane read after refused prompt");
    let pane_text =
        json_string_value(&stdout(&pane_read), "text").unwrap_or_else(|| stdout(&pane_read));
    assert!(
        !pane_text.contains(BLOCKED_PROMPT),
        "blocked prompt leaked into pane: {pane_text}"
    );
}

#[test]
fn p5_10_default_wait_accepts_idle_done_blocked_only() {
    let h = start();
    let created = created_layout(&h);
    let blocked_pane = split_pane(&h, &created.pane);
    let working_pane = split_pane(&h, &created.pane);

    assert_ok(
        &start_ready(&h, &created.pane, "readybot"),
        "start readybot",
    );
    assert_ok(
        &start_blocked(&h, &blocked_pane, "blockbot"),
        "start blockbot",
    );
    assert_ok(
        &start_working(&h, &working_pane, "workbot"),
        "start workbot",
    );

    wait_pane_match(&h, &created.pane, READY_NEEDLE);
    wait_pane_match(&h, &blocked_pane, BLOCKED_NEEDLE);
    let _ = wait_state(&h, &created.pane, "readybot", &["done"]);
    let _ = wait_state(&h, &blocked_pane, "blockbot", &["blocked"]);
    let work = wait_state(&h, &working_pane, "workbot", &["working"]);
    assert_eq!(json_field(&work, "state"), Some("working"), "{work}");

    let wait_done = agent_wait(&h, &created.pane, "readybot", SETTLE_TIMEOUT);
    let wait_done_body = wait_settled_body(&h, &created.pane, "readybot", &wait_done, "wait done");
    assert_eq!(json_field(&wait_done_body, "state"), Some("done"));

    let wait_blocked = agent_wait(&h, &blocked_pane, "blockbot", SETTLE_TIMEOUT);
    let _ = wait_settled_body(&h, &blocked_pane, "blockbot", &wait_blocked, "wait blocked");

    let wait_working = agent_wait(&h, &working_pane, "workbot", NEG_WAIT_TIMEOUT);
    assert!(
        !wait_working.status.success(),
        "working must not satisfy default wait: stdout={} stderr={}",
        stdout(&wait_working),
        stderr(&wait_working)
    );

    let focus = agent_focus(&h, &created.pane, "readybot");
    assert_ok(&focus, "focus readybot");
    let idle = wait_state(&h, &created.pane, "readybot", &["idle"]);
    assert_eq!(json_field(&idle, "state"), Some("idle"), "{idle}");

    let wait_idle = agent_wait(&h, &created.pane, "readybot", SETTLE_TIMEOUT);
    let _ = wait_settled_body(&h, &created.pane, "readybot", &wait_idle, "wait idle");

    let prompted = agent_prompt_wait(&h, &created.pane, "readybot", "ping-ready");
    let _ = wait_settled_body(&h, &created.pane, "readybot", &prompted, "prompt --wait");
}

#[test]
fn p5_11_refuse_herdr_dsh_argv() {
    let h = start();
    let created = created_layout(&h);
    let known = known_pane_ids(&created, &[]);
    for argv in [
        &["herdr"][..],
        &["/usr/bin/herdr"][..],
        &["dsh"][..],
        &["/opt/bin/dsh"][..],
    ] {
        let out = agent_start(&h, &created.pane, "refused", argv);
        assert_exit(&out, 1, &format!("refuse {argv:?}"));
        assert_no_occupant(&h, &created.pane);
        let err = format!("{}{}", stdout(&out), stderr(&out));
        assert!(
            err.contains("refuse") || err.contains("herdr") || err.contains("dsh"),
            "expected refuse text for {argv:?}: {err}"
        );
    }
    assert_pane_count_unchanged(&h, &created.pane, &known);
}

#[test]
fn p5_12_outside_dory_env_exit_one() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();
    let start_args = [
        "agent",
        "start",
        "outsider",
        "--pane",
        pane,
        "--",
        "/bin/true",
    ];
    let prompt_args = ["agent", "prompt", "outsider", "--", "hi"];
    let wait_args = ["agent", "wait", "outsider"];
    let focus_args = ["agent", "focus", "outsider"];
    let keys_args = ["agent", "send-keys", "outsider", "enter"];
    for args in [
        start_args.as_slice(),
        prompt_args.as_slice(),
        wait_args.as_slice(),
        focus_args.as_slice(),
        keys_args.as_slice(),
    ] {
        let out = cli_no_env(&h, args);
        assert_exit(&out, 1, &format!("no DORY_ENV {:?}", args));
        let err = stderr(&out);
        assert!(
            err.contains("\"ok\":false"),
            "gate must be JSON runtime error: {err}"
        );
        assert!(
            err.contains("I am not running inside a Dory-managed pane"),
            "stderr={err}"
        );
    }
    assert_no_occupant(&h, &created.pane);
}

#[test]
fn p5_13_agent_a_drives_b_via_cli_wait() {
    let h = start();
    let created = created_layout(&h);
    let b_pane = split_pane(&h, &created.pane);
    let known = known_pane_ids(&created, &[&b_pane]);
    let layout_before = extract_quoted_ids(&workspace_list(&h, &created.pane));

    assert_ok(
        &start_working(&h, &created.pane, "driver"),
        "start driver A",
    );
    assert_ok(&start_ready(&h, &b_pane, "target"), "start target B");
    wait_pane_match(&h, &b_pane, READY_NEEDLE);
    let _ = wait_state(&h, &b_pane, "target", &["done", "idle"]);

    let prompted = agent_prompt_wait(&h, &b_pane, "target", DRIVE_PROMPT);
    let _ = wait_settled_body(&h, &b_pane, "target", &prompted, "A prompts B --wait");

    let body = wait_agent_text(&h, &b_pane, "target", DRIVE_PROMPT);
    let text = json_string_value(&body, "text").unwrap_or(body);
    assert!(text.contains(DRIVE_PROMPT), "{text}");

    let wait = agent_wait(&h, &b_pane, "target", SETTLE_TIMEOUT);
    let _ = wait_settled_body(&h, &b_pane, "target", &wait, "wait after drive");

    let layout_after = extract_quoted_ids(&workspace_list(&h, &created.pane));
    assert_eq!(layout_before, layout_after);
    assert_eq!(
        occupant_name(&pane_get(&h, &b_pane)).as_deref(),
        Some("target")
    );
    assert_eq!(
        occupant_name(&pane_get(&h, &created.pane)).as_deref(),
        Some("driver")
    );
    assert_pane_count_unchanged(&h, &created.pane, &known);
}
