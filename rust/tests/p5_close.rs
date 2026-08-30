//! Close pane/tab/workspace. Isolated XDG. Last live pane stays.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
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
    let path = std::env::temp_dir().join(format!(
        "dory-p5-close-{}-{}",
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

fn cli(h: &Harness, args: &[&str], env: bool) -> std::process::Output {
    let mut cmd = Command::new(bin());
    cmd.args(args)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .current_dir(&h.xdg);
    if env {
        cmd.env("DORY_ENV", "1");
    } else {
        cmd.env_remove("DORY_ENV");
    }
    cmd.output().unwrap()
}

fn root_pane(h: &Harness) -> String {
    let snap = rpc(h, r#"{"op":"snapshot"}"#);
    json_field(&snap, "pane")
        .expect("pane")
        .to_string()
}

fn snap_focused(h: &Harness) -> String {
    let snap = rpc(h, r#"{"op":"snapshot"}"#);
    json_field(&snap, "focused")
        .unwrap_or_else(|| panic!("focused missing in {snap}"))
        .to_string()
}

fn nested_id(body: &str, key: &str) -> String {
    let pat = format!("\"{key}\":{{\"id\":\"");
    body.split(&pat)
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect(key)
        .to_string()
}

fn split_right(h: &Harness, pane: &str) -> String {
    let split = cli(
        h,
        &[
            "pane",
            "split",
            "--pane",
            pane,
            "--direction",
            "right",
            "--no-focus",
        ],
        true,
    );
    assert!(
        split.status.success(),
        "{}",
        String::from_utf8_lossy(&split.stderr)
    );
    json_field(&String::from_utf8_lossy(&split.stdout), "id")
        .expect("split id")
        .to_string()
}

#[test]
fn close_split_leaf_keeps_sibling() {
    let h = start();
    let root = root_pane(&h);
    let new_pane = split_right(&h, &root);
    let closed = rpc(
        &h,
        &format!(r#"{{"op":"pane.close","pane":"{new_pane}"}}"#),
    );
    assert!(closed.contains("\"ok\":true"), "{closed}");
    assert!(closed.contains("\"retired\":true"), "{closed}");
    let lay = rpc(&h, r#"{"op":"desk.layout","cols":80,"rows":22}"#);
    assert!(lay.contains(&root), "{lay}");
    assert!(!lay.contains(&new_pane), "{lay}");
    let get = rpc(&h, &format!(r#"{{"op":"pane.get","pane":"{new_pane}"}}"#));
    assert!(!get.contains("\"ok\":true"), "{get}");
}

#[test]
fn refuse_last_live_pane() {
    let h = start();
    let root = root_pane(&h);
    let closed = rpc(&h, &format!(r#"{{"op":"pane.close","pane":"{root}"}}"#));
    assert!(closed.contains("last live pane"), "{closed}");
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    assert!(snap.contains(&root), "{snap}");
}

#[test]
fn workspace_close_other_and_refuse_last() {
    let h = start();
    let first = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "workspace")
        .expect("ws")
        .to_string();
    let created = rpc(&h, r#"{"op":"workspace.create"}"#);
    assert!(created.contains("\"ok\":true"), "{created}");
    let second = nested_id(&created, "workspace");
    let closed = rpc(
        &h,
        &format!(r#"{{"op":"workspace.close","workspace":"{second}"}}"#),
    );
    assert!(closed.contains("\"ok\":true"), "{closed}");
    assert!(closed.contains("\"retired\":true"), "{closed}");
    let refuse = rpc(
        &h,
        &format!(r#"{{"op":"workspace.close","workspace":"{first}"}}"#),
    );
    assert!(refuse.contains("last live pane"), "{refuse}");
    assert_eq!(snap_focused(&h).split(':').next(), Some(first.as_str()));
}

#[test]
fn cli_close_requires_env() {
    let h = start();
    let root = root_pane(&h);
    let out = cli(&h, &["pane", "close", "--pane", &root], false);
    assert_eq!(out.status.code(), Some(1), "{}", String::from_utf8_lossy(&out.stderr));
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    assert!(snap.contains(&root), "{snap}");
    let ws = json_field(&snap, "workspace").expect("ws").to_string();
    let ws_out = cli(&h, &["workspace", "close", &ws], false);
    assert_eq!(
        ws_out.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&ws_out.stderr)
    );
    assert!(rpc(&h, r#"{"op":"snapshot"}"#).contains(&root));
}

#[test]
fn close_split_root_focus_stays_on_sibling() {
    let h = start();
    let ws = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "workspace")
        .expect("ws")
        .to_string();
    let root = root_pane(&h);
    let sibling = split_right(&h, &root);
    assert_eq!(snap_focused(&h), root);
    let closed = rpc(&h, &format!(r#"{{"op":"pane.close","pane":"{root}"}}"#));
    assert!(closed.contains("\"ok\":true"), "{closed}");
    assert_eq!(snap_focused(&h), sibling);
    let got = rpc(&h, &format!(r#"{{"op":"workspace.get","workspace":"{ws}"}}"#));
    assert!(got.contains(&format!("\"id\":\"{sibling}\"")), "{got}");
    assert!(!got.contains(&format!("\"id\":\"{root}\"")), "{got}");
}

#[test]
fn close_last_pane_on_tab_keeps_other_tab() {
    let h = start();
    let first = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "workspace")
        .expect("ws")
        .to_string();
    let root = root_pane(&h);
    let created = rpc(
        &h,
        &format!(r#"{{"op":"tab.create","workspace":"{first}"}}"#),
    );
    assert!(created.contains("\"ok\":true"), "{created}");
    let extra = nested_id(&created, "root_pane");
    let tab = nested_id(&created, "tab");
    let focused = rpc(&h, &format!(r#"{{"op":"pane.focus","pane":"{extra}"}}"#));
    assert!(focused.contains("\"ok\":true"), "{focused}");
    assert_eq!(snap_focused(&h), extra);
    let closed = rpc(&h, &format!(r#"{{"op":"pane.close","pane":"{extra}"}}"#));
    assert!(closed.contains("\"ok\":true"), "{closed}");
    assert_eq!(snap_focused(&h), root);
    let listed = rpc(
        &h,
        &format!(r#"{{"op":"tab.list","workspace":"{first}"}}"#),
    );
    assert!(!listed.contains(&tab), "{listed}");
    assert!(listed.contains(":t"), "{listed}");
}

#[test]
fn close_pane_on_second_workspace_keeps_focus_there() {
    let h = start();
    let first = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "workspace")
        .expect("ws")
        .to_string();
    let created = rpc(&h, r#"{"op":"workspace.create"}"#);
    let second = nested_id(&created, "workspace");
    let pane = nested_id(&created, "root_pane");
    let focused = rpc(&h, &format!(r#"{{"op":"pane.focus","pane":"{pane}"}}"#));
    assert!(focused.contains("\"ok\":true"), "{focused}");
    let sibling = split_right(&h, &pane);
    let closed = rpc(&h, &format!(r#"{{"op":"pane.close","pane":"{pane}"}}"#));
    assert!(closed.contains("\"ok\":true"), "{closed}");
    assert_eq!(snap_focused(&h), sibling);
    assert!(sibling.starts_with(&format!("{second}:")));
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    assert_eq!(json_field(&snap, "workspace"), Some(first.as_str()));
    assert_eq!(json_field(&snap, "focused"), Some(sibling.as_str()));
}
