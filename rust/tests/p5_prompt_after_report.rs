//! P5 prompt after `agent report --state idle` — must not stall.
//!
//! Harness shape copied from `p5_prompt_unknown.rs`.
//! Never hardcode the next id as `w1`.
//!
//!  1 `agent start -- /abs/path/to/occ_reporter` → state `unknown`
//!  2 `agent report --pane <id> --state idle` → wait settles `idle` or `done`
//!  3 `agent prompt name -- ping-after-report` exits 0, body `"ok":true`.
//!    Must not contain `agent_prompt_stalled`.
//!
//! Does not exec `omp`. Does not copy `flow-skill`.
//! Does not edit leftover `p5_attach.rs`.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const AGENT: &str = "stranger";
const START_TIMEOUT: &str = "8000";
const SETTLE_TIMEOUT: &str = "8000";
const READY_NEEDLE: &str = "DORY_OCC_REPORTER";
const PROMPT_TEXT: &str = "ping-after-report";

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
    let path = std::env::temp_dir().join(format!(
        "dory-p5-prompt-unknown-{}-{}",
        std::process::id(),
        nanos
    ));
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

fn agent_prompt(h: &Harness, pane: &str, name: &str, text: &str) -> Output {
    cli(h, pane, &["agent", "prompt", name, "--", text])
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

#[test]
fn p5_prompt_after_report_idle_does_not_stall() {
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

    let reported = agent_report(&h, pane, &["--pane", pane, "--state", "idle"]);
    assert_ok(&reported, "report --pane --state idle");
    let wait = agent_wait(&h, pane, AGENT, SETTLE_TIMEOUT);
    let settled = wait_settled_body(&h, pane, AGENT, &wait, "wait after report");
    assert!(
        matches!(json_field(&settled, "state"), Some("idle") | Some("done")),
        "{settled}"
    );

    let prompted = agent_prompt(&h, pane, AGENT, PROMPT_TEXT);
    assert_eq!(
        prompted.status.code(),
        Some(0),
        "prompt after report must exit 0: stdout={} stderr={}",
        stdout(&prompted),
        stderr(&prompted)
    );
    let prompt_out = stdout(&prompted);
    let prompt_err = stderr(&prompted);
    let combined = format!("{prompt_out}{prompt_err}");
    assert!(
        prompt_out.contains("\"ok\":true"),
        "prompt body must be ok: stdout={prompt_out} stderr={prompt_err}"
    );
    assert!(
        !combined.contains("agent_prompt_stalled"),
        "report then prompt must not stall: stdout={prompt_out} stderr={prompt_err}"
    );
}
