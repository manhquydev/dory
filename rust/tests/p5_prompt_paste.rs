//! P5 layer-4c prompt honors live bracketed-paste — l4c_tst.
//!
//! Harness shape copied from `p5_report.rs`: `CARGO_BIN_EXE_dory`, temp
//! `XDG_RUNTIME_DIR`, `dory server`, IDs parsed from snapshot / `.result`.
//! Never hardcode the next id as `w1`.
//!
//!  1 BP on: `agent start -- /abs/path/to/occ_paste` → state `unknown`
//!    (argv0 comm is not allowlisted; not bash/sh/omp). Fixture emits
//!    CSI ? 2004 h. `agent prompt name -- hello-paste` captured stdin
//!    contains `\x1b[200~hello-paste\x1b[201~\r` and no raw `hello-paste\n`
//!    outside the wrap. Submit after paste is CR, not LF.
//!  2 BP off: `occ_raw` never emits 2004h (emits 2004l so last-wins is
//!    off). Prompt `hello-raw`. Captured stdin is `hello-raw` plus one
//!    trailing NL, no `\x1b[200~`.
//!
//! Does not exec `omp`. Does not copy `flow-skill`. Does not edit `rust/src`.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const START_TIMEOUT: &str = "8000";
const PASTE_NEEDLE: &str = "DORY_OCC_PASTE";
const RAW_NEEDLE: &str = "DORY_OCC_RAW";
const PASTE_TEXT: &str = "hello-paste";
const RAW_TEXT: &str = "hello-raw";
const BP_BEGIN: &[u8] = b"\x1b[200~";
const BP_END: &[u8] = b"\x1b[201~";

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
        "dory-p5-prompt-paste-{}-{}",
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
    assert_ne!(pane, snap_pane, "created pane must not be the snapshot pane");
    assert!(
        occupant_name(&body).is_none(),
        "create must not start an occupant: {body}"
    );
    Created { pane }
}

fn fixture(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    assert!(path.is_file(), "missing fixture {}", path.display());
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some(name),
        "argv0 comm must be {name}, not bash/sh/omp: {}",
        path.display()
    );
    let mut perms = fs::metadata(&path).expect("fixture metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).expect("chmod fixture");
    let abs = fs::canonicalize(&path).unwrap_or(path);
    assert!(abs.is_absolute(), "{}", abs.display());
    let shown = abs.to_string_lossy();
    assert!(!shown.contains("omp"), "must not exec omp: {shown}");
    assert!(
        !shown.contains("flow-skill"),
        "must not copy flow-skill: {shown}"
    );
    let comm = abs
        .file_name()
        .and_then(|n| n.to_str())
        .expect("fixture comm");
    assert_eq!(comm, name);
    assert_ne!(comm, "sh");
    assert_ne!(comm, "bash");
    assert_ne!(comm, "omp");
    let src = fs::read_to_string(&abs).expect("read fixture");
    assert!(
        !src.contains("flow-skill"),
        "fixture must not copy flow-skill: {src}"
    );
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

fn start_fixture(h: &Harness, pane: &str, name: &str, comm: &str) -> (Output, PathBuf) {
    let path = fixture(comm);
    let argv0 = path.to_str().expect("utf8 fixture path");
    assert!(argv0.starts_with('/'), "start must use abs path: {argv0}");
    assert!(!argv0.contains("omp"), "must not exec omp: {argv0}");
    let out = agent_start(h, pane, name, &[argv0]);
    (out, path)
}

fn agent_get(h: &Harness, pane: &str, name: &str) -> Output {
    cli(h, pane, &["agent", "get", name])
}

fn agent_prompt(h: &Harness, pane: &str, name: &str, text: &str) -> Output {
    cli(h, pane, &["agent", "prompt", name, "--", text])
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

fn capture_path(h: &Harness, name: &str) -> PathBuf {
    h.sock
        .parent()
        .expect("session dir")
        .join(format!("{name}.stdin"))
}

fn wait_capture(path: &Path, pred: impl Fn(&[u8]) -> bool) -> Vec<u8> {
    let start = Instant::now();
    let mut last = Vec::new();
    loop {
        if let Ok(body) = fs::read(path) {
            last = body;
            if pred(&last) {
                return last;
            }
        }
        if start.elapsed() >= Duration::from_secs(8) {
            panic!(
                "timed out waiting for capture {}: {:?}",
                path.display(),
                String::from_utf8_lossy(&last)
            );
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn assert_unknown_start(h: &Harness, pane: &str, name: &str, started: &Output, fixture: &Path) {
    assert_ok(started, "start occupant");
    let start_body = stdout(started);
    let start_state = json_field(&start_body, "state").map(str::to_string);
    let state = start_state.unwrap_or_else(|| {
        let got = agent_get(h, pane, name);
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
    let unknown = wait_state(h, pane, name, &["unknown"]);
    assert_eq!(json_field(&unknown, "state"), Some("unknown"), "{unknown}");
    assert_eq!(
        occupant_name(&pane_get(h, pane)).as_deref(),
        Some(name)
    );
}

fn wrap_bytes(text: &str) -> Vec<u8> {
    let mut out = BP_BEGIN.to_vec();
    out.extend_from_slice(text.as_bytes());
    out.extend_from_slice(BP_END);
    out
}

fn has_raw_outside_wrap(buf: &[u8], text: &str) -> bool {
    let wrap = wrap_bytes(text);
    let raw_nl = format!("{text}\n");
    let mut rest = buf;
    while let Some(at) = rest.windows(wrap.len()).position(|w| w == wrap.as_slice()) {
        rest = &rest[at + wrap.len()..];
    }
    rest.windows(raw_nl.len())
        .any(|w| w == raw_nl.as_bytes())
}

#[test]
fn p5_prompt_paste_bp_on_wraps() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();
    let name = "paster";

    let (started, path) = start_fixture(&h, pane, name, "occ_paste");
    assert_unknown_start(&h, pane, name, &started, &path);
    wait_pane_text(&h, pane, PASTE_NEEDLE);

    let _ = agent_prompt(&h, pane, name, PASTE_TEXT);

    let wrap = wrap_bytes(PASTE_TEXT);
    let captured = wait_capture(&capture_path(&h, "occ_paste"), |b| {
        b.windows(wrap.len()).any(|w| w == wrap.as_slice())
    });
    assert!(
        captured.windows(wrap.len()).any(|w| w == wrap.as_slice()),
        "BP-on stdin must contain wrap, got {:?}",
        String::from_utf8_lossy(&captured)
    );
    let after = captured
        .windows(wrap.len() + 1)
        .find(|w| w[..wrap.len()] == wrap[..])
        .map(|w| w[wrap.len()]);
    assert!(
        matches!(after, Some(b'\r') | Some(b'\n')),
        "BP-on wrap must be followed by Enter (CR on master; cooked PTY may show NL): {:?}",
        String::from_utf8_lossy(&captured)
    );
    assert!(
        !has_raw_outside_wrap(&captured, PASTE_TEXT),
        "BP-on stdin must not contain raw {PASTE_TEXT}\\n outside the wrap: {:?}",
        String::from_utf8_lossy(&captured)
    );
}

#[test]
fn p5_prompt_paste_bp_off_raw_nl() {
    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();
    let name = "rawer";

    let (started, path) = start_fixture(&h, pane, name, "occ_raw");
    assert_unknown_start(&h, pane, name, &started, &path);
    wait_pane_text(&h, pane, RAW_NEEDLE);

    let _ = agent_prompt(&h, pane, name, RAW_TEXT);

    let expected = format!("{RAW_TEXT}\n");
    let captured = wait_capture(&capture_path(&h, "occ_raw"), |b| b == expected.as_bytes());
    assert_eq!(
        captured,
        expected.as_bytes(),
        "BP-off stdin must be {RAW_TEXT} plus one NL, got {:?}",
        String::from_utf8_lossy(&captured)
    );
    assert!(
        !captured.windows(BP_BEGIN.len()).any(|w| w == BP_BEGIN),
        "BP-off stdin must not contain BP begin: {:?}",
        String::from_utf8_lossy(&captured)
    );
}
