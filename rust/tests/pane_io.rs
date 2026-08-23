//! P3-11…P3-14: `pane run` / `pane read` / `wait-output` through the CLI.
//! IDs come from snapshot or `.result`. Never hardcode `w1` as the next input.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MARKER: &str = "P3IO_MARKER_260822";

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

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn temp_xdg() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!("dory-pane-io-{}-{}", std::process::id(), nanos));
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

fn strip_json_string_field(json: &str, key: &str) -> String {
    let pat = format!("\"{key}\":\"");
    let Some((head, rest)) = json.split_once(&pat) else {
        return json.to_string();
    };
    let bytes = rest.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i = i.saturating_add(2);
            continue;
        }
        if bytes[i] == b'"' {
            return format!("{head}{pat}\"{tail}", tail = &rest[i..]);
        }
        i += 1;
    }
    json.to_string()
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
/// Snapshot the auto-minted pane, then create a new workspace and use that pane.
fn created_pane(h: &Harness) -> String {
    let mut stream = UnixStream::connect(&h.sock).expect("snapshot connect");
    writeln!(stream, r#"{{"op":"snapshot"}}"#).unwrap();
    let _ = stream.flush();
    let mut snap = String::new();
    BufReader::new(stream).read_line(&mut snap).unwrap();
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
    assert_ne!(
        ws, "w1",
        "P3-3: next workspace must come from .result, not w1"
    );
    assert_ne!(
        pane, snap_pane,
        "created pane must not be the snapshot pane"
    );
    pane
}

fn run_marker(h: &Harness, pane: &str) {
    let out = cli(
        h,
        pane,
        &["pane", "run", "--current", &format!("echo {MARKER}")],
    );
    assert!(out.status.success(), "pane run: {}", stderr(&out));
    assert!(stdout(&out).contains("\"ok\":true"), "{}", stdout(&out));
}

fn read_recent(h: &Harness, pane: &str) -> Output {
    cli(
        h,
        pane,
        &["pane", "read", "--current", "--source", "recent"],
    )
}
fn wait_read_contains(h: &Harness, pane: &str, needle: &str) -> String {
    let start = Instant::now();
    loop {
        let out = read_recent(h, pane);
        assert!(out.status.success(), "pane read: {}", stderr(&out));
        let body = stdout(&out);
        if let Some(text) = json_string_value(&body, "text") {
            if text.contains(needle) {
                return body;
            }
        } else if body.contains(needle) {
            return body;
        }
        if start.elapsed() >= Duration::from_secs(5) {
            panic!("timed out waiting for {needle} in pane read: {body}");
        }
        thread::sleep(Duration::from_millis(40));
    }
}
fn assert_no_status_words(json: &str) {
    let meta = strip_json_string_field(json, "text");
    for word in ["idle", "done", "blocked"] {
        assert!(
            !meta.contains(&format!("\"status\":\"{word}\"")),
            "P3-13: status {word} in {json}"
        );
        assert!(
            !meta.contains(&format!("\"status\": \"{word}\"")),
            "P3-13: status {word} in {json}"
        );
        assert!(
            !meta.contains(&format!("\"{word}\"")),
            "P3-13: {word} classified in {json}"
        );
    }
}

#[test]
fn p3_11_run_then_read_recent_contains_text() {
    let h = start();
    let pane = created_pane(&h);
    run_marker(&h, &pane);
    let body = wait_read_contains(&h, &pane, MARKER);
    let text = json_string_value(&body, "text").unwrap_or(body.clone());
    assert!(text.contains(MARKER), "{body}");
    assert!(body.contains("\"ok\":true"), "{body}");
    let read_pane = result_id(&body, "pane");
    assert_eq!(read_pane, pane);
}

#[test]
fn p3_12_read_does_not_change_seen() {
    let h = start();
    let pane = created_pane(&h);
    run_marker(&h, &pane);
    let first = wait_read_contains(&h, &pane, MARKER);
    let second_out = read_recent(&h, &pane);
    assert!(
        second_out.status.success(),
        "second read: {}",
        stderr(&second_out)
    );
    let second = stdout(&second_out);
    assert!(second.contains("\"ok\":true"), "{second}");

    let first_meta = strip_json_string_field(&first, "text");
    let second_meta = strip_json_string_field(&second, "text");
    match (seen_token(&first_meta), seen_token(&second_meta)) {
        (None, None) => {}
        (Some(a), Some(b)) => {
            assert_eq!(a, b, "P3-12: seen must stay unchanged\n{first}\n{second}")
        }
        other => panic!("P3-12: seen appeared on only one read: {other:?}"),
    }
}

#[test]
fn p3_13_read_json_has_no_idle_done_blocked_status() {
    let h = start();
    let pane = created_pane(&h);
    run_marker(&h, &pane);
    let body = wait_read_contains(&h, &pane, MARKER);
    assert_no_status_words(&body);
    let wait = cli(
        &h,
        &pane,
        &[
            "pane",
            "wait-output",
            "--current",
            "--match",
            MARKER,
            "--timeout",
            "2000",
        ],
    );
    assert!(wait.status.success(), "wait-output: {}", stderr(&wait));
    assert_no_status_words(&stdout(&wait));
}

#[test]
fn p3_14_wait_output_match_already_present() {
    let h = start();
    let pane = created_pane(&h);
    run_marker(&h, &pane);
    let _ = wait_read_contains(&h, &pane, MARKER);
    let start_at = Instant::now();
    let out = cli(
        &h,
        &pane,
        &[
            "pane",
            "wait-output",
            "--current",
            "--match",
            MARKER,
            "--timeout",
            "2000",
        ],
    );
    let elapsed = start_at.elapsed();
    assert!(out.status.success(), "P3-14 wait-output: {}", stderr(&out));
    let body = stdout(&out);
    assert!(body.contains("\"ok\":true"), "{body}");
    assert!(
        body.contains("\"matched\":true")
            || json_string_value(&body, "text").is_some_and(|t| t.contains(MARKER)),
        "{body}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "already-present match should not burn the timeout, took {elapsed:?}"
    );
}
