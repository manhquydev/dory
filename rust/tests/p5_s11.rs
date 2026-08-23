//! Contract §11 — named occupant *inside* the first pane drives
//! split / start / prompt / wait / `flow -- status` via `$DORY_BIN`
//! after opening `skills/dory/SKILL.md`, against a **clone** of the
//! locked spec-kit repo. Not a copy of `flow-skill/flow/`.
//!
//! Test process may: start server, `workspace create`, `agent start` the
//! named driver, then only `pane read` / `agent get`. It does **not**
//! `pane run` the loop, start the peer, prompt, wait the peer, or `dory flow`.
//!
//! Do not block `pane wait-output` while the driver still calls `$DORY_BIN`
//! (single-threaded server). Poll `pane read`.
//!
//! Driver argv: `/bin/bash /abs/occ_s11 <clone> <SKILL.md> <FLOW_BIN>`.
//! Does not exec `omp`. Does not edit `rust/src`.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const DRIVER: &str = "s11driver";
const PEER: &str = "s11peer";
const START_TIMEOUT: &str = "15000";
const SKILL_PHRASE: &str =
    "After the env gate, a coding occupant inside the pane that is ready for prompts must run";
const DONE: &str = "S11_FINISH";
const SPEC_KIT: &str = "/home/manhquy/Downloads/spec-kit";
const FLOW_BIN: &str = "/home/manhquy/Downloads/flow/flow-skill/skills/flow/runner/flow.sh";
const FLOW_SKILL_ROOT: &str = "/home/manhquy/Downloads/flow/flow-skill";
const FLOW_DECK: &str = "/home/manhquy/Downloads/flow/flow-deck";

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

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn skill_md() -> PathBuf {
    crate_dir().join("../skills/dory/SKILL.md")
}

fn temp_xdg() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!(
        "dory-p5-s11-{}-{}",
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

fn created_layout(h: &Harness) -> String {
    let snap = snapshot(h);
    assert!(snap.contains("\"live\":true"), "{snap}");
    let snap_pane = json_field(&snap, "pane").expect("snapshot pane").to_string();
    assert_ne!(snap_pane, "w1");

    let created = cli(h, &snap_pane, &["workspace", "create"]);
    assert_ok(&created, "workspace create");
    let body = stdout(&created);
    let ws = result_id(&body, "workspace");
    let pane = result_id(&body, "root_pane");
    assert_ne!(ws, "w1");
    assert_ne!(pane, snap_pane);
    assert!(
        occupant_name(&body).is_none(),
        "create must not start an occupant: {body}"
    );
    pane
}

fn pane_read_text(h: &Harness, pane: &str) -> String {
    let out = cli(
        h,
        pane,
        &["pane", "read", "--pane", pane, "--source", "recent-unwrapped"],
    );
    assert_ok(&out, "pane read");
    let body = stdout(&out);
    json_string_value(&body, "text").unwrap_or_else(|| panic!("missing text in {body}"))
}

fn wait_driver_text(h: &Harness, pane: &str, needle: &str) -> String {
    let start = Instant::now();
    loop {
        let text = pane_read_text(h, pane);
        if text.contains(needle) {
            return text;
        }
        if start.elapsed() >= Duration::from_secs(60) {
            panic!("timed out waiting for {needle} in driver recent: {text}");
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn occ_s11() -> PathBuf {
    let path = crate_dir().join("tests/fixtures/occ_s11");
    assert!(path.is_file(), "missing {}", path.display());
    let mut perms = fs::metadata(&path).expect("fixture metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).expect("chmod fixture");
    let abs = fs::canonicalize(&path).unwrap_or(path);
    assert!(!abs.to_string_lossy().contains("omp"));
    let src = fs::read_to_string(&abs).expect("read occ_s11");
    assert!(
        src.contains("SKILL") || src.contains("skills/dory/SKILL.md"),
        "fixture must open SKILL.md: {src}"
    );
    assert!(
        !src.contains("copy_tree") && !src.contains("flow-skill/flow"),
        "fixture must not copy flow-skill/flow: {src}"
    );
    abs
}

fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn clone_spec_kit() -> Scratch {
    let src = Path::new(SPEC_KIT);
    assert!(
        src.join(".git").is_dir(),
        "locked spec-kit missing or not git: {}",
        src.display()
    );
    assert!(
        !src.join("flow").exists(),
        "spec-kit must not already be a flow tree: {}",
        src.display()
    );
    assert!(
        !src.join(".dory").exists(),
        "original spec-kit must not have .dory before the test: {}",
        src.display()
    );

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dest = std::env::temp_dir().join(format!(
        "dory-s11-clone-{}-{}",
        std::process::id(),
        nanos
    ));
    let out = Command::new("git")
        .args([
            "clone",
            "--no-hardlinks",
            "--depth",
            "1",
            "--",
            src.to_str().unwrap(),
            dest.to_str().unwrap(),
        ])
        .output()
        .expect("git clone");
    assert!(
        out.status.success(),
        "git clone spec-kit: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dest.join(".git").exists(), "clone must be git");
    assert!(!dest.join("flow").exists(), "clone must not gain a flow/");
    Scratch { path: dest }
}

fn assert_foreign_project(project: &Path) {
    let canon_project = canon(project);
    let shown = canon_project.to_string_lossy();
    assert!(!canon_project.starts_with(&canon(&crate_dir())));
    assert_ne!(canon_project, canon(Path::new(FLOW_SKILL_ROOT)));
    assert!(!canon_project.starts_with(&canon(Path::new(FLOW_SKILL_ROOT))));
    assert!(!shown.contains("eval/phase5-project"));
    assert_ne!(canon_project, canon(Path::new(SPEC_KIT)));
    if Path::new(FLOW_DECK).exists() {
        assert_ne!(canon_project, canon(Path::new(FLOW_DECK)));
    }
}

fn assert_houses_clean() {
    assert!(
        !Path::new(FLOW_SKILL_ROOT).join(".dory").exists(),
        "flow-skill must not gain .dory"
    );
    assert!(
        !Path::new(SPEC_KIT).join(".dory").exists(),
        "original spec-kit must not gain .dory"
    );
    let rg = Command::new("rg")
        .args(["-i", "dory", "--glob", "!.git/**"])
        .current_dir(FLOW_SKILL_ROOT)
        .output();
    if let Ok(out) = rg {
        let hits = String::from_utf8_lossy(&out.stdout);
        assert!(
            hits.trim().is_empty(),
            "flow-skill must have zero dory bytes: {hits}"
        );
    }
}

fn session_journal(ws: &Path) -> PathBuf {
    ws.join(".dory").join("sessions").join("s1.jsonl")
}

fn journal_body(ws: &Path) -> String {
    fs::read_to_string(session_journal(ws)).unwrap_or_default()
}

#[test]
fn p5_s11_named_driver_flow_on_spec_kit_clone() {
    assert_ne!(FLOW_BIN, "/bin/true");
    assert!(!FLOW_BIN.contains("eval/phase5-project"));
    assert!(Path::new(FLOW_BIN).is_file(), "missing FLOW_BIN {FLOW_BIN}");
    assert_houses_clean();

    let skill = fs::read_to_string(skill_md()).expect("SKILL.md");
    assert!(skill.contains(SKILL_PHRASE));
    assert!(skill.contains("dory pane split --current --direction right --no-focus"));
    assert!(skill.contains("dory flow -- status"));

    let project = clone_spec_kit();
    assert_foreign_project(&project.path);

    let h = start();
    let pane = created_layout(&h);
    let fixture = occ_s11();
    let skill_path = fs::canonicalize(skill_md()).unwrap();

    let started = cli(
        &h,
        &pane,
        &[
            "agent",
            "start",
            DRIVER,
            "--pane",
            &pane,
            "--timeout",
            START_TIMEOUT,
            "--",
            "/bin/bash",
            fixture.to_str().unwrap(),
            project.path.to_str().unwrap(),
            skill_path.to_str().unwrap(),
            FLOW_BIN,
        ],
    );
    assert_ok(&started, "start s11 driver");
    let start_body = stdout(&started);
    assert!(
        start_body.contains(&format!("\"name\":\"{DRIVER}\"")),
        "{start_body}"
    );
    assert!(
        !stdout(&started).contains("omp") && !stderr(&started).contains("--kind"),
        "no --kind / omp: stdout={start_body} stderr={}",
        stderr(&started)
    );

    let text = wait_driver_text(&h, &pane, DONE);
    assert!(
        text.contains(SKILL_PHRASE),
        "driver must print the skill phrase: {text}"
    );
    assert!(
        text.contains("printenv DORY_ENV") || text.contains("DORY_ENV="),
        "driver must observe DORY_ENV: {text}"
    );
    assert!(
        text.contains("DORY_ENV=1"),
        "driver must print DORY_ENV=1: {text}"
    );
    assert!(
        text.contains("pane split --current --direction right --no-focus"),
        "driver must type split from the skill: {text}"
    );
    assert!(
        text.contains("agent start") && text.contains(PEER),
        "driver must start the peer: {text}"
    );
    assert!(
        text.contains("agent prompt") && text.contains("S11_PEER_PROMPT"),
        "driver must prompt the peer: {text}"
    );
    assert!(
        text.contains("agent wait") && text.contains(PEER),
        "driver must wait the peer: {text}"
    );
    assert!(
        text.contains("flow -- status"),
        "driver must taxi flow -- status: {text}"
    );
    assert!(
        !text.contains("eval/phase5-project"),
        "must not use eval/phase5-project: {text}"
    );
    assert!(
        occupant_name(&rpc(
            &h,
            &format!(r#"{{"op":"pane.get","pane":"{pane}"}}"#)
        ))
        .as_deref()
            == Some(DRIVER),
        "root pane occupant must remain the named driver"
    );

    let journal = journal_body(&project.path);
    assert!(
        journal.contains("\"type\":\"flow/invoke\""),
        "missing flow/invoke in {}: {journal}",
        session_journal(&project.path).display()
    );
    assert!(
        journal.contains("\"type\":\"flow/result\""),
        "missing flow/result: {journal}"
    );
    let clone_s = project.path.to_string_lossy();
    assert!(
        journal.contains(clone_s.as_ref()),
        "journal cwd must be the clone {clone_s}: {journal}"
    );
    assert!(
        !journal.contains(&format!("{SPEC_KIT}/.dory")),
        "journal must not target original spec-kit: {journal}"
    );

    assert_houses_clean();
    assert!(
        !Path::new(SPEC_KIT).join(".dory").exists(),
        "original spec-kit gained .dory"
    );
}
