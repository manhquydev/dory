//! Combined P5 live loop — occupant wait then `dory flow -- status`.
//!
//! Harness shape copied from `p5_occupant.rs` / `pane_io.rs`. IDs parsed from
//! snapshot / `.result`. Never hardcode the next id as `w1`.
//!
//! One process, `DORY_ENV=1`: workspace create → pane split → ready fixture on
//! the new pane → `agent prompt --wait` → `agent wait` → taxi `flow -- status`.
//! Judge is the foreign flow-skill `flow.sh`, not `eval/phase5-project` or
//! `/bin/true`. cwd / `FLOW_PROJECT_ROOT` is a temp dir outside the dory crate.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const READY_NEEDLE: &str = "DORY_OCC_READY";
const READY_FIXTURE: &str = concat!(
    "printf '%s\\n' DORY_OCC_READY; ",
    "while IFS= read -r line; do ",
    "printf 'got:%s\\n' \"$line\"; ",
    "printf '%s\\n' DORY_OCC_READY; ",
    "done"
);
const LIVE_PROMPT: &str = "P5_LIVE_LOOP_PROMPT_260822";
const START_TIMEOUT: &str = "8000";
const SETTLE_TIMEOUT: &str = "8000";
const FLOW_BIN: &str = "/home/manhquy/Downloads/flow/flow-skill/skills/flow/runner/flow.sh";

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

struct Scratch {
    path: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

struct Created {
    pane: String,
}

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn temp_dir(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!(
        "dory-p5-live-{tag}-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

fn scratch(tag: &str) -> Scratch {
    Scratch {
        path: temp_dir(tag),
    }
}

fn session_sock(xdg: &Path) -> PathBuf {
    xdg.join("dory").join("default").join("dory.sock")
}

fn start() -> Harness {
    let xdg = temp_dir("xdg");
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

fn agent_wait(h: &Harness, pane: &str, name: &str, timeout_ms: &str) -> Output {
    cli(h, pane, &["agent", "wait", name, "--timeout", timeout_ms])
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

fn session_journal(ws: &Path) -> PathBuf {
    ws.join(".dory").join("sessions").join("s1.jsonl")
}

fn journal_body(ws: &Path) -> String {
    fs::read_to_string(session_journal(ws)).unwrap_or_default()
}

fn journal_types(ws: &Path) -> Vec<String> {
    journal_body(ws)
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let rest = line.split_once("\"type\":\"")?.1;
            Some(rest.split_once('"')?.0.to_string())
        })
        .collect()
}

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn assert_foreign_project(project: &Path) {
    let crate_root = crate_dir();
    let canon_project = project
        .canonicalize()
        .unwrap_or_else(|_| project.to_path_buf());
    let canon_crate = crate_root
        .canonicalize()
        .unwrap_or_else(|_| crate_root.clone());
    assert!(
        !canon_project.starts_with(&canon_crate),
        "FLOW_PROJECT_ROOT must not be the dory crate: {} under {}",
        canon_project.display(),
        canon_crate.display()
    );
    let shown = canon_project.to_string_lossy();
    assert!(
        !shown.contains("eval/phase5-project"),
        "FLOW_PROJECT_ROOT must not be eval/phase5-project: {shown}"
    );
}

fn assert_foreign_judge() {
    assert_ne!(FLOW_BIN, "/bin/true", "judge must not be /bin/true");
    assert!(
        !FLOW_BIN.contains("eval/phase5-project"),
        "judge must not be eval/phase5-project: {FLOW_BIN}"
    );
    assert!(
        Path::new(FLOW_BIN).is_file(),
        "missing foreign FLOW_BIN {FLOW_BIN}"
    );
}

fn result_code(journal: &str) -> Option<i32> {
    let line = journal
        .lines()
        .find(|l| l.contains("\"type\":\"flow/result\""))?;
    let rest = line.split_once("\"code\":")?.1.trim_start();
    if rest.starts_with("null") {
        return None;
    }
    let tok: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    tok.parse().ok()
}

fn dory_flow_status(h: &Harness, pane: &str, project: &Path) -> Output {
    Command::new(bin())
        .args(["flow", "--", "status"])
        .current_dir(project)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_SOCKET", &h.sock)
        .env("DORY_ENV", "1")
        .env("DORY_PANE_ID", pane)
        .env("FLOW_BIN", FLOW_BIN)
        .env("FLOW_PROJECT_ROOT", project)
        .env_remove("DORY_WORKSPACE_DIR")
        .output()
        .expect("dory flow -- status")
}

#[test]
fn p5_live_loop_occupant_then_flow_status() {
    assert_foreign_judge();

    let h = start();
    let created = created_layout(&h);
    let new_pane = split_pane(&h, &created.pane);

    assert_ok(
        &start_ready(&h, &new_pane, "loop"),
        "start ready on new pane",
    );
    wait_pane_match(&h, &new_pane, READY_NEEDLE);

    let prompted = agent_prompt_wait(&h, &new_pane, "loop", LIVE_PROMPT);
    let _ = wait_settled_body(&h, &new_pane, "loop", &prompted, "prompt --wait");

    let wait = agent_wait(&h, &new_pane, "loop", SETTLE_TIMEOUT);
    let _ = wait_settled_body(&h, &new_pane, "loop", &wait, "agent wait");

    let project = scratch("project");
    assert_foreign_project(&project.path);

    let taxi = dory_flow_status(&h, &new_pane, &project.path);
    let types = journal_types(&project.path);
    let body = journal_body(&project.path);
    let preserved = result_code(&body).unwrap_or(1);
    assert_eq!(
        taxi.status.code(),
        Some(preserved),
        "dory must preserve judge exit={preserved} stdout={} stderr={} journal={body}",
        stdout(&taxi),
        stderr(&taxi)
    );

    assert!(
        types.iter().any(|t| t == "flow/invoke"),
        "missing flow/invoke in {body}"
    );
    assert!(
        types.iter().any(|t| t == "flow/result"),
        "missing flow/result in {body}"
    );
    assert!(
        body.contains(&format!("\"bin\":\"{FLOW_BIN}\"")),
        "journal must record foreign FLOW_BIN: {body}"
    );
    assert!(
        body.contains("\"args\":[\"status\"]"),
        "journal must record status argv: {body}"
    );
    assert!(
        !body.contains("phase5-project") && !body.contains("/bin/true"),
        "journal must not record a fake judge: {body}"
    );
}
