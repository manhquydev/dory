//! P3-19: second `workspace create` is a new `wN` and a second live master.
//!
//! Snapshot only reports the first workspace pid. The second master is proven
//! via `pane.get` on the parsed `.result.root_pane` (same newline-JSON RPC
//! the crate tests already use). No HTTP.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn dory_bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn temp_xdg() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!("dory-p3-19-{}-{}", std::process::id(), nanos));
    fs::create_dir_all(&path).unwrap();
    path
}

fn session_sock(xdg: &Path) -> PathBuf {
    xdg.join("dory").join("default").join("dory.sock")
}

fn start_server(xdg: &Path) -> Child {
    let mut child = Command::new(dory_bin())
        .arg("server")
        .env("XDG_RUNTIME_DIR", xdg)
        .current_dir(xdg)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn dory server");
    let sock = session_sock(xdg);
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if UnixStream::connect(&sock).is_ok() {
            return child;
        }
        if let Ok(Some(status)) = child.try_wait() {
            let mut err = String::new();
            if let Some(mut stderr) = child.stderr.take() {
                let _ = stderr.read_to_string(&mut err);
            }
            panic!("dory server exited {status}: {err}");
        }
        thread::sleep(Duration::from_millis(20));
    }
    let _ = child.kill();
    panic!("dory server did not bind {}", sock.display());
}

fn stop_server(xdg: &Path) -> String {
    let out = Command::new(dory_bin())
        .args(["server", "stop"])
        .env("XDG_RUNTIME_DIR", xdg)
        .output()
        .expect("dory server stop");
    assert!(
        out.status.success(),
        "stop failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn rpc(sock: &Path, line: &str) -> String {
    let mut stream = UnixStream::connect(sock).unwrap();
    writeln!(stream, "{line}").unwrap();
    let _ = stream.flush();
    let mut reply = String::new();
    BufReader::new(stream).read_line(&mut reply).unwrap();
    reply
}

fn json_field<'a>(json: &'a str, key: &str) -> &'a str {
    let pat = format!("\"{key}\":\"");
    let rest = json.split_once(&pat).unwrap().1;
    rest.split_once('"').unwrap().0
}

fn json_u32(json: &str, key: &str) -> u32 {
    let pat = format!("\"{key}\":");
    let rest = json.split_once(&pat).unwrap().1.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().unwrap()
}

fn nested_id<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":{{");
    let rest = json.split_once(&pat)?.1;
    let after = rest.split_once("\"id\":\"")?.1;
    Some(after.split_once('"')?.0)
}

fn result_id(json: &str, key: &str) -> String {
    if let Some(id) = nested_id(json, key) {
        return id.to_string();
    }
    json_field(json, key).to_string()
}

unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

fn pid_alive(pid: u32) -> bool {
    // kill(pid, 0) is portable; /proc exists only on Linux.
    unsafe { kill(pid as i32, 0) == 0 }
}

#[test]
fn p3_19_second_workspace_is_new_wn_and_second_live_master() {
    let xdg = temp_xdg();
    let mut server = start_server(&xdg);
    let sock = session_sock(&xdg);

    let snap = rpc(&sock, r#"{"op":"snapshot"}"#);
    assert!(snap.contains("\"live\":true"), "{snap}");
    let first_ws = json_field(&snap, "workspace").to_string();
    let first_pid = json_u32(&snap, "pid");
    assert!(first_pid > 0, "snapshot pid missing: {snap}");
    assert!(
        pid_alive(first_pid),
        "first master {first_pid} not in /proc"
    );

    let created = Command::new(dory_bin())
        .args(["workspace", "create"])
        .env("XDG_RUNTIME_DIR", &xdg)
        .env("DORY_SOCKET", &sock)
        .env("DORY_ENV", "1")
        .output()
        .expect("dory workspace create");
    assert!(
        created.status.success(),
        "workspace create: {}",
        String::from_utf8_lossy(&created.stderr)
    );
    let body = String::from_utf8_lossy(&created.stdout);
    assert!(body.contains("\"ok\":true"), "{body}");
    assert!(
        body.contains("\"occupant\":null"),
        "P3-19 occupant must be null on create: {body}"
    );

    let ws = result_id(&body, "workspace");
    let root = result_id(&body, "root_pane");
    assert!(ws.starts_with('w'), "{ws}");
    assert_ne!(ws, first_ws, "P3-19: second workspace must be a new wN");
    assert_ne!(
        ws, "w1",
        "P3-19: do not treat hardcoded w1 as the create result"
    );
    assert!(root.contains(":p"), "{root}");

    let got = rpc(&sock, &format!(r#"{{"op":"pane.get","pane":"{root}"}}"#));
    assert!(
        got.contains("\"ok\":true"),
        "P3-19: pane.get must expose the second master pid; reply={got}"
    );
    assert!(
        got.contains("\"occupant\":null"),
        "P3-19 occupant must stay null on pane.get: {got}"
    );
    let second_pid = json_u32(&got, "pid");
    assert!(second_pid > 0, "pane.get pid missing: {got}");
    assert_ne!(
        second_pid, first_pid,
        "P3-19: second master pid must differ from snapshot pid"
    );
    assert!(
        pid_alive(first_pid),
        "first master {first_pid} died after second create"
    );
    assert!(
        pid_alive(second_pid),
        "second master {second_pid} not in /proc"
    );

    let snap2 = rpc(&sock, r#"{"op":"snapshot"}"#);
    assert_eq!(json_u32(&snap2, "pid"), first_pid);

    let _ = stop_server(&xdg);
    let _ = server.wait();
    let _ = fs::remove_dir_all(&xdg);
}
