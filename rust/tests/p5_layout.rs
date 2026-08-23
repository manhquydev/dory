//! Spatial layout RPCs. Isolated XDG. No `:7380`. No `herdr`.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
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
    let path =
        std::env::temp_dir().join(format!("dory-p5-layout-{}-{}", std::process::id(), nanos));
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

#[test]
fn desk_layout_right_split_abuts_and_divider_moves() {
    let h = start();
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    let root = json_field(&snap, "pane").expect("pane").to_string();
    let split = Command::new(bin())
        .args([
            "pane",
            "split",
            "--pane",
            &root,
            "--direction",
            "right",
            "--no-focus",
        ])
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_ENV", "1")
        .output()
        .unwrap();
    assert!(
        split.status.success(),
        "{}",
        String::from_utf8_lossy(&split.stderr)
    );
    let new_pane = json_field(&String::from_utf8_lossy(&split.stdout), "id")
        .expect("split id")
        .to_string();

    let lay = rpc(&h, r#"{"op":"desk.layout","cols":80,"rows":22}"#);
    assert!(lay.contains("\"ok\":true"), "{lay}");
    assert!(lay.contains(&root), "{lay}");
    assert!(lay.contains(&new_pane), "{lay}");
    let w0 = json_u32(&lay, "w").expect("w");
    assert!(w0 > 0 && w0 < 80, "{lay}");
    // two exclusive cells: first w + second w == 80
    let first_w = w0;
    let after = lay.split("\"w\":").nth(2).unwrap_or("");
    let second_w: u32 = after
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    assert_eq!(first_w + second_w, 80, "{lay}");

    let next = rpc(
        &h,
        &format!(r#"{{"op":"desk.neighbor","from":"{root}","step":"next"}}"#),
    );
    assert!(next.contains(&new_pane), "{next}");

    let left = rpc(
        &h,
        &format!(
            r#"{{"op":"desk.neighbor","from":"{new_pane}","step":"left","cols":80,"rows":22}}"#
        ),
    );
    assert!(left.contains("\"ok\":true"), "{left}");
    assert!(left.contains(&root), "{left}");

    let none = rpc(
        &h,
        &format!(r#"{{"op":"desk.neighbor","from":"{root}","step":"left","cols":80,"rows":22}}"#),
    );
    assert!(none.contains("\"ok\":false"), "{none}");

    let focused_before = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "focused")
        .unwrap()
        .to_string();
    let mut stream = UnixStream::connect(&h.sock).unwrap();
    writeln!(
        stream,
        r#"{{"op":"pane.attach","pane":"{new_pane}","cols":20,"rows":10,"no_focus":true}}"#
    )
    .unwrap();
    let mut ack = String::new();
    BufReader::new(stream.try_clone().unwrap())
        .read_line(&mut ack)
        .unwrap();
    assert!(ack.contains("\"ok\":true"), "{ack}");
    drop(stream);
    let focused_after = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "focused")
        .unwrap()
        .to_string();
    assert_eq!(focused_before, focused_after);

    let before_x = json_u32(&rpc(&h, r#"{"op":"desk.layout","cols":80,"rows":22}"#), "w").unwrap();
    let div = rpc(
        &h,
        &format!(r#"{{"op":"desk.divider","a":"{root}","b":"{new_pane}","ratio":0.25}}"#),
    );
    assert!(div.contains("\"ok\":true"), "{div}");
    let after_w = json_u32(&rpc(&h, r#"{"op":"desk.layout","cols":80,"rows":22}"#), "w").unwrap();
    assert!(after_w < before_x, "before={before_x} after={after_w}");
}

#[test]
fn attach_default_still_focuses() {
    let h = start();
    let snap = rpc(&h, r#"{"op":"snapshot"}"#);
    let root = json_field(&snap, "pane").unwrap().to_string();
    let split = Command::new(bin())
        .args(["pane", "split", "--pane", &root, "--direction", "down"])
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_ENV", "1")
        .output()
        .unwrap();
    let new_pane = json_field(&String::from_utf8_lossy(&split.stdout), "id")
        .unwrap()
        .to_string();
    let mut stream = UnixStream::connect(&h.sock).unwrap();
    writeln!(stream, r#"{{"op":"pane.attach","pane":"{new_pane}"}}"#).unwrap();
    let mut ack = String::new();
    BufReader::new(stream.try_clone().unwrap())
        .read_line(&mut ack)
        .unwrap();
    assert!(ack.contains("\"ok\":true"), "{ack}");
    drop(stream);
    let snap2 = rpc(&h, r#"{"op":"snapshot"}"#);
    let focused = json_field(&snap2, "focused").unwrap();
    assert_eq!(focused, new_pane);
}

fn attach_read(sock: &Path, pane: &str, no_focus: bool) -> UnixStream {
    let mut stream = UnixStream::connect(sock).unwrap();
    let focus = if no_focus { ",\"no_focus\":true" } else { "" };
    writeln!(
        stream,
        r#"{{"op":"pane.attach","pane":"{pane}","cols":40,"rows":12{focus}}}"#
    )
    .unwrap();
    let _ = stream.flush();
    let mut ack = String::new();
    BufReader::new(stream.try_clone().unwrap())
        .read_line(&mut ack)
        .unwrap();
    assert!(ack.contains("\"ok\":true"), "{ack}");
    let _ = stream.set_nonblocking(true);
    stream
}

fn drain_contains(stream: &mut UnixStream, needle: &str, timeout: Duration) -> bool {
    let start = Instant::now();
    let mut body = String::new();
    let mut buf = [0u8; 4096];
    while start.elapsed() < timeout {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => body.push_str(&String::from_utf8_lossy(&buf[..n])),
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(20));
                continue;
            }
            Err(_) => break,
        }
        if body.contains(needle) {
            return true;
        }
    }
    body.contains(needle)
}

#[test]
fn two_attach_streams_live_after_split_detach_leaves() {
    let h = start();
    let root = json_field(&rpc(&h, r#"{"op":"snapshot"}"#), "pane")
        .unwrap()
        .to_string();
    let split = Command::new(bin())
        .args([
            "pane",
            "split",
            "--pane",
            &root,
            "--direction",
            "right",
            "--no-focus",
        ])
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_ENV", "1")
        .output()
        .unwrap();
    let sibling = json_field(&String::from_utf8_lossy(&split.stdout), "id")
        .unwrap()
        .to_string();
    let lay = rpc(&h, r#"{"op":"desk.layout","cols":80,"rows":22}"#);
    assert!(lay.contains(&root) && lay.contains(&sibling), "{lay}");

    let mut a = attach_read(&h.sock, &root, true);
    let mut b = attach_read(&h.sock, &sibling, true);
    let _ = rpc(
        &h,
        &format!(r#"{{"op":"pane.write","pane":"{root}","text":"echo GRID_A"}}"#),
    );
    let _ = rpc(
        &h,
        &format!(r#"{{"op":"pane.write","pane":"{sibling}","text":"echo GRID_B"}}"#),
    );
    assert!(
        drain_contains(&mut a, "GRID_A", Duration::from_secs(4)),
        "left attach missed GRID_A"
    );
    assert!(
        drain_contains(&mut b, "GRID_B", Duration::from_secs(4)),
        "right attach missed GRID_B"
    );
    drop(a);
    drop(b);
    thread::sleep(Duration::from_millis(50));
    let list = Command::new(bin())
        .args(["pane", "list", "--workspace", "w1"])
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .output()
        .unwrap();
    let body = String::from_utf8_lossy(&list.stdout);
    assert!(body.contains(&root), "{body}");
    assert!(body.contains(&sibling), "{body}");
}
