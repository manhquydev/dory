//! P5 layer-4 skill-using occupant — l4_tst.
//!
//! Stranger comm outside the allowlist opens `skills/dory/SKILL.md`, runs
//! the documented report verb, and lets `agent wait` close. Harness shape
//! copied from `p5_report.rs`: `CARGO_BIN_EXE_dory`, temp `XDG_RUNTIME_DIR`,
//! `dory server`, IDs parsed from snapshot / `.result`. Never hardcode `w1`.
//!
//!  1 `agent start -- /abs/path/occ_skill` → state `unknown`
//!    (argv0 comm is not allowlisted; not bash/sh/omp)
//!  2 pane transcript contains the SKILL.md occupant-first-action phrase
//!    printed by the fixture after opening that file
//!  3 `agent wait` settles `idle` or `done` (fixture reported; this test
//!    does not slave-report)
//!  4 fixture path does not contain `omp`
//!  5 no `flow-skill` copy
//!
//! Does not exec `omp`. Does not edit `rust/src`. Does not allowlist a comm.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const AGENT: &str = "skiller";
const START_TIMEOUT: &str = "8000";
const SETTLE_TIMEOUT: &str = "8000";
const SKILL_PHRASE: &str =
    "After the env gate, a coding occupant inside the pane that is ready for prompts must run";

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
        "dory-p5-skill-occ-{}-{}",
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

fn skill_md() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root above rust/")
        .join("skills")
        .join("dory")
        .join("SKILL.md");
    assert!(path.is_file(), "missing skill {}", path.display());
    let abs = fs::canonicalize(&path).unwrap_or(path);
    assert!(abs.is_absolute(), "{}", abs.display());
    let shown = abs.to_string_lossy();
    assert!(
        shown.contains("/skills/dory/SKILL.md"),
        "skill must be the repo skill, not a rust/ relative: {shown}"
    );
    assert!(
        !shown.contains("/rust/skills/"),
        "skill must not be a rust/ relative: {shown}"
    );
    abs
}

fn occ_skill() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("occ_skill");
    assert!(path.is_file(), "missing fixture {}", path.display());
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some("occ_skill"),
        "argv0 comm must be occ_skill, not bash/sh/omp: {}",
        path.display()
    );
    let mut perms = fs::metadata(&path).expect("fixture metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).expect("chmod occ_skill");
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
    assert_eq!(comm, "occ_skill");
    assert_ne!(comm, "sh");
    assert_ne!(comm, "bash");
    assert_ne!(comm, "omp");

    let src = fs::read_to_string(&abs).expect("read occ_skill");
    assert!(
        !src.contains("flow-skill"),
        "fixture must not copy flow-skill: {src}"
    );
    assert!(
        src.contains("DORY_SKILL") || src.contains("skills/dory/SKILL.md"),
        "fixture must open SKILL.md ($DORY_SKILL or absolute), not hardcode: {src}"
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

fn start_skiller(h: &Harness, pane: &str, name: &str) -> (Output, PathBuf) {
    let fixture = occ_skill();
    let argv0 = fixture.to_str().expect("utf8 fixture path");
    assert!(argv0.starts_with('/'), "start must use abs path: {argv0}");
    assert!(!argv0.contains("omp"), "must not exec omp: {argv0}");
    assert!(
        !argv0.contains("flow-skill"),
        "must not copy flow-skill: {argv0}"
    );
    let out = agent_start(h, pane, name, &[argv0]);
    (out, fixture)
}

fn agent_get(h: &Harness, pane: &str, name: &str) -> Output {
    cli(h, pane, &["agent", "get", name])
}

fn agent_wait(h: &Harness, pane: &str, name: &str, timeout_ms: &str) -> Output {
    cli(h, pane, &["agent", "wait", name, "--timeout", timeout_ms])
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
fn p5_skill_occ_unknown_then_skill_phrase_then_wait() {
    let skill = fs::read_to_string(skill_md()).expect("read SKILL.md");
    assert!(
        skill.contains(SKILL_PHRASE),
        "SKILL.md must contain the occupant-first-action phrase"
    );

    let h = start();
    let created = created_layout(&h);
    let pane = created.pane.as_str();

    let (started, fixture) = start_skiller(&h, pane, AGENT);
    assert_ok(&started, "start occ_skill");
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
    assert!(
        !fixture.to_string_lossy().contains("flow-skill"),
        "must not copy flow-skill: {}",
        fixture.display()
    );

    wait_pane_text(&h, pane, SKILL_PHRASE);
    assert_eq!(occupant_name(&pane_get(&h, pane)).as_deref(), Some(AGENT));

    let wait = agent_wait(&h, pane, AGENT, SETTLE_TIMEOUT);
    let settled = wait_settled_body(&h, pane, AGENT, &wait, "wait after skill report");
    assert!(
        matches!(json_field(&settled, "state"), Some("idle") | Some("done")),
        "{settled}"
    );
}
