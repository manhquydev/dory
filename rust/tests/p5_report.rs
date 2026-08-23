//! P5 layer-3 self-report — l3_tst.
//!
//! Stranger comm outside the allowlist still lets `agent wait` close after
//! a pane-env `report`. Harness shape copied from `p5_occupant.rs`:
//! `CARGO_BIN_EXE_dory`, temp `XDG_RUNTIME_DIR`, `dory server`, IDs parsed
//! from snapshot / `.result`. Never hardcode the next id as `w1`.
//!
//!  1 `agent start -- /abs/path/to/occ_reporter` → state `unknown`
//!    (argv0 comm is not allowlisted; not bash/sh)
//!  2 same-pane env: `"$DORY_BIN" agent report --current --state idle`
//!    then `agent wait` settles `idle` or `done`
//!  3 report without `DORY_ENV=1` → exit 1, state unchanged
//!  4 `--state done` and `--state unknown` → exit 2
//!  5 omit `--current` and `--pane` → exit 2
//!  6 `--kind` on `agent start` → exit 2
//!
//! Does not exec `omp`. Does not copy `flow-skill`.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const AGENT: &str = "reporter";
const START_TIMEOUT: &str = "8000";
const SETTLE_TIMEOUT: &str = "8000";
const READY_NEEDLE: &str = "DORY_OCC_REPORTER";

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
    pane: String,
}

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn temp_xdg() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path =
        std::env::temp_dir().join(format!("dory-p5-report-{}-{}", std::process::id(), nanos));
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
        .env("DORY_BIN", bin())
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
    Created { pane }
}

fn occ_reporter() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("occ_reporter");
    assert!(path.is_file(), "missing fixture {}", path.display());
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some("occ_reporter"),
        "argv0 comm must be occ_reporter, not bash/sh: {}",
        path.display()
    );
    let mut perms = fs::metadata(&path).expect("fixture metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).expect("chmod occ_reporter");
    let abs = fs::canonicalize(&path).unwrap_or(path);
    assert!(abs.is_absolute(), "{}", abs.display());
    let comm = abs
        .file_name()
        .and_then(|n| n.to_str())
        .expect("fixture comm");
    assert_eq!(comm, "occ_reporter");
    assert_ne!(comm, "sh");
    assert_ne!(comm, "bash");
    assert_ne!(comm, "omp");
    abs
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

fn start_reporter(h: &Harness, pane: &str, name: &str) -> (Output, PathBuf) {
    let fixture = occ_reporter();
    let argv0 = fixture.to_str().expect("utf8 fixture path");
    assert!(argv0.starts_with('/'), "start must use abs path: {argv0}");
    assert!(!argv0.contains("omp"), "must not exec omp: {argv0}");
    let out = agent_start(h, pane, name, &[argv0]);
    (out, fixture)
}

fn agent_get(h: &Harness, pane: &str, name: &str) -> Output {
    cli(h, pane, &["agent", "get", name])
}

fn agent_wait(h: &Harness, pane: &str, name: &str, timeout_ms: &str) -> Output {
    cli(h, pane, &["agent", "wait", name, "--timeout", timeout_ms])
}

fn agent_report(h: &Harness, pane: &str, args: &[&str]) -> Output {
    let mut all = vec!["agent", "report"];
    all.extend_from_slice(args);
    cli(h, pane, &all)
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

fn assert_no_occupant(h: &Harness, pane: &str) {
    let got = pane_get(h, pane);
    assert!(
        occupant_name(&got).is_none(),
        "expected no occupant on {pane}: {got}"
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

fn wait_pane_text(h: &Harness, pane: &str, needle: &str) {
    let start = Instant::now();
    let mut last;
    loop {
        let out = cli(
            h,
            pane,
            &[
                "pane",
                "read",
                "--pane",
                pane,
                "--source",
                "recent-unwrapped",
            ],
        );
        if out.status.success() {
            last = stdout(&out);
            if last.contains(needle) {
                return;
            }
        } else {
            last = format!("stdout={} stderr={}", stdout(&out), stderr(&out));
        }
        if start.elapsed() >= Duration::from_secs(8) {
            panic!("timed out waiting for {needle} in pane {pane}: {last}");
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn assert_settled_state(json: &str) {
    let state = json_field(json, "state").unwrap_or("");
    assert!(
        matches!(state, "idle" | "done"),
        "expected idle|done, got {state} in {json}"
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

fn state_of(h: &Harness, pane: &str, name: &str) -> String {
    let got = agent_get(h, pane, name);
    assert_ok(&got, "agent get");
    json_field(&stdout(&got), "state").unwrap_or("").to_string()
}

#[test]
fn p5_report_stranger_comm_unknown_then_idle_wait() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();

    let (started, fixture) = start_reporter(&h, pane, AGENT);
    assert_ok(&started, "start occ_reporter");
    let start_body = stdout(&started);
    let start_state = json_field(&start_body, "state").map(str::to_string);
    let state = start_state.unwrap_or_else(|| {
        let got = agent_get(&h, pane, AGENT);
        assert_ok(&got, "get after start");
        json_field(&stdout(&got), "state")
            .expect("state after start")
            .to_string()
    });
    assert_eq!(
        state, "unknown",
        "stranger comm must start unknown: start={start_body}"
    );
    assert!(
        !fixture.to_string_lossy().contains("omp"),
        "must not exec omp: {}",
        fixture.display()
    );
    wait_pane_text(&h, pane, READY_NEEDLE);
    let unknown = wait_state(&h, pane, AGENT, &["unknown"]);
    assert_eq!(json_field(&unknown, "state"), Some("unknown"), "{unknown}");
    assert_eq!(occupant_name(&pane_get(&h, pane)).as_deref(), Some(AGENT));

    let reported = agent_report(&h, pane, &["--current", "--state", "idle"]);
    assert_ok(&reported, "report --current --state idle");

    let wait = agent_wait(&h, pane, AGENT, SETTLE_TIMEOUT);
    let settled = wait_settled_body(&h, pane, AGENT, &wait, "wait after report");
    assert!(
        matches!(json_field(&settled, "state"), Some("idle") | Some("done")),
        "{settled}"
    );
}

#[test]
fn p5_report_without_env_exit_one_state_unchanged() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();

    let (started, _) = start_reporter(&h, pane, AGENT);
    assert_ok(&started, "start occ_reporter");
    let before = wait_state(&h, pane, AGENT, &["unknown"]);
    assert_eq!(json_field(&before, "state"), Some("unknown"), "{before}");

    let out = cli_no_env(&h, &["agent", "report", "--current", "--state", "idle"]);
    assert_exit(&out, 1, "report without DORY_ENV");
    let err = stderr(&out);
    assert!(
        err.contains("\"ok\":false"),
        "gate must be JSON runtime error: {err}"
    );
    assert!(
        err.contains("I am not running inside a Dory-managed pane"),
        "stderr={err}"
    );

    let after = agent_get(&h, pane, AGENT);
    assert_ok(&after, "get after refused report");
    assert_eq!(
        json_field(&stdout(&after), "state"),
        Some("unknown"),
        "state must not change: {}",
        stdout(&after)
    );
}

#[test]
fn p5_report_state_done_or_unknown_is_usage() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();

    let (started, _) = start_reporter(&h, pane, AGENT);
    assert_ok(&started, "start occ_reporter");
    let _ = wait_state(&h, pane, AGENT, &["unknown"]);
    let before = state_of(&h, pane, AGENT);
    assert_eq!(before, "unknown");

    for state in ["done", "unknown"] {
        let out = agent_report(&h, pane, &["--current", "--state", state]);
        assert_exit(&out, 2, &format!("report --state {state}"));
    }

    assert_eq!(
        state_of(&h, pane, AGENT),
        "unknown",
        "usage must not store done/unknown"
    );
}

#[test]
fn p5_report_omit_current_and_pane_is_usage() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();

    let out = agent_report(&h, pane, &["--state", "idle"]);
    assert_exit(&out, 2, "report without --current/--pane");
}

#[test]
fn p5_report_kind_on_start_is_usage() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();
    let fixture = occ_reporter();
    let argv0 = fixture.to_str().unwrap();
    let kind_after_name = [
        "agent", "start", "kinded", "--kind", "sh", "--pane", pane, "--", argv0,
    ];
    let kind_after_pane = [
        "agent", "start", "kinded", "--pane", pane, "--kind", "sh", "--", argv0,
    ];
    let kind_before_name = [
        "agent", "start", "--kind", "sh", "kinded", "--pane", pane, "--", argv0,
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
}
