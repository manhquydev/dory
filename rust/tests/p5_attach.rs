//! Sit-down attach: human client on a live pane. Detach ≠ kill.
//! Isolated XDG. No `:7380`. No `herdr`.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
    let path =
        std::env::temp_dir().join(format!("dory-p5-attach-{}-{}", std::process::id(), nanos));
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
        thread::sleep(Duration::from_millis(20));
    }
    let _ = server.kill();
    panic!("server socket did not appear");
}

fn rpc(h: &Harness, line: &str) -> String {
    let mut stream = UnixStream::connect(&h.sock).expect("rpc connect");
    writeln!(stream, "{line}").unwrap();
    let _ = stream.flush();
    let mut reply = String::new();
    BufReader::new(stream).read_line(&mut reply).unwrap();
    reply
}

fn json_field<'a>(body: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{key}\"");
    let idx = body.find(&needle)?;
    let mut rest = body[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(&rest[..end])
}

fn json_u32(body: &str, key: &str) -> Option<u32> {
    let needle = format!("\"{key}\"");
    let idx = body.find(&needle)?;
    let mut rest = body[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn cli(h: &Harness, args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env_remove("DORY_ENV")
        .stdin(Stdio::null())
        .output()
        .expect("cli")
}

fn cli_env(h: &Harness, args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_ENV", "1")
        .stdin(Stdio::null())
        .output()
        .expect("cli env")
}

#[test]
fn attach_handshake_writes_and_detach_leaves_pty() {
    let h = start();
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    assert!(snap.contains("\"live\":true"), "{snap}");
    let pane = json_field(&snap, "pane").expect("pane").to_string();
    assert!(pane.contains(':'), "{pane}");

    let desk = rpc(&h, r#"{"op":"desk.snapshot"}"#);
    assert!(desk.contains("\"ok\":true"), "{desk}");
    assert!(desk.contains(&pane), "{desk}");
    assert!(desk.contains("workspace / tab / pane"), "{desk}");
    assert!(!desk.contains(":7380"), "{desk}");

    let mut stream = UnixStream::connect(&h.sock).expect("attach");
    writeln!(stream, r#"{{"op":"pane.attach","pane":"{pane}"}}"#).unwrap();
    let _ = stream.flush();
    let mut ack = String::new();
    BufReader::new(stream.try_clone().unwrap())
        .read_line(&mut ack)
        .unwrap();
    assert!(ack.contains("\"ok\":true"), "{ack}");
    assert!(ack.contains(&pane), "{ack}");

    stream.write_all(b"echo ATTACH_MARK\n").expect("write pane");
    let _ = stream.flush();

    let start = Instant::now();
    let mut body = String::new();
    let mut err = String::new();
    while start.elapsed() < Duration::from_secs(5) {
        let out = cli(&h, &["pane", "read", "--pane", &pane, "--source", "recent"]);
        body = String::from_utf8_lossy(&out.stdout).into_owned();
        err = String::from_utf8_lossy(&out.stderr).into_owned();
        if body.contains("ATTACH_MARK") {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    assert!(
        body.contains("ATTACH_MARK"),
        "pane read after attach stdout={body} stderr={err}"
    );

    drop(stream);
    thread::sleep(Duration::from_millis(50));
    let ping = rpc(&h, r#"{"op":"ping"}"#);
    assert!(ping.contains("\"ok\":true"), "{ping}");
    let snap2 = rpc(&h, r#"{"op":"snapshot"}"#);
    assert!(snap2.contains("\"live\":true"), "{snap2}");
    let pid = json_u32(&snap2, "pid").expect("pid");
    assert!(
        Path::new(&format!("/proc/{pid}")).exists(),
        "detach must not kill pid {pid}"
    );
}

#[test]
fn neighbor_walks_split_panes() {
    let h = start();
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    let root = json_field(&snap, "pane").expect("pane").to_string();
    let split = cli_env(
        &h,
        &[
            "pane",
            "split",
            "--pane",
            &root,
            "--direction",
            "right",
            "--no-focus",
        ],
    );
    let body = String::from_utf8_lossy(&split.stdout);
    assert!(
        split.status.success(),
        "{}",
        String::from_utf8_lossy(&split.stderr)
    );
    assert!(body.contains("\"ok\":true"), "{body}");
    let new_pane = json_field(&body, "id").expect("split id").to_string();
    assert_ne!(new_pane, root);

    let next = rpc(
        &h,
        &format!(r#"{{"op":"desk.neighbor","from":"{root}","step":"next"}}"#),
    );
    assert!(next.contains("\"ok\":true"), "{next}");
    assert!(next.contains(&new_pane), "{next}");
}

#[test]
fn bare_dory_without_tty_starts_server() {
    let xdg = temp_xdg();
    let out = Command::new(bin())
        .env("XDG_RUNTIME_DIR", &xdg)
        .env_remove("DORY_ENV")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("bare dory");
    assert_eq!(
        out.status.code(),
        Some(1),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("needs a tty"), "{err}");

    let list = Command::new(bin())
        .args(["workspace", "list"])
        .env("XDG_RUNTIME_DIR", &xdg)
        .output()
        .expect("list");
    let body = String::from_utf8_lossy(&list.stdout);
    assert!(
        list.status.success(),
        "{}",
        String::from_utf8_lossy(&list.stderr)
    );
    assert!(body.contains("\"workspaces\""), "{body}");
    assert!(body.contains("w"), "{body}");

    let _ = Command::new(bin())
        .args(["server", "stop"])
        .env("XDG_RUNTIME_DIR", &xdg)
        .output();
    let _ = fs::remove_dir_all(&xdg);
}

#[test]
fn attach_help_and_usage_name_sit_down() {
    let out = Command::new(bin()).args(["--help"]).output().expect("help");
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(body.contains("dory attach"), "{body}");
    assert!(body.contains("Bare `dory` opens the desk"), "{body}");
    assert!(body.contains("--plain"), "{body}");
    assert!(!body.contains("/workplace"), "{body}");
}

#[test]
fn desk_tree_lists_split_siblings() {
    let h = start();
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    let root = json_field(&snap, "pane").expect("pane").to_string();
    let split = cli_env(
        &h,
        &[
            "pane",
            "split",
            "--pane",
            &root,
            "--direction",
            "right",
            "--no-focus",
        ],
    );
    assert!(
        split.status.success(),
        "{}",
        String::from_utf8_lossy(&split.stderr)
    );
    let new_pane = json_field(&String::from_utf8_lossy(&split.stdout), "id")
        .expect("split id")
        .to_string();

    let tree = rpc(&h, r#"{"op":"desk.tree"}"#);
    assert!(tree.contains("\"ok\":true"), "{tree}");
    assert!(tree.contains("\"k\":\"w\""), "{tree}");
    assert!(tree.contains("\"k\":\"t\""), "{tree}");
    assert!(tree.contains("\"k\":\"p\""), "{tree}");
    assert!(tree.contains(&root), "{tree}");
    assert!(tree.contains(&new_pane), "{tree}");
    assert!(!tree.contains(":7380"), "{tree}");
}
