//! Phase 2: parked waits must not hold accept.
//! Temp XDG only. Never the factory default sock.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const FACTORY_SOCK: &str = "/run/user/1000/dory/default/dory.sock";

struct Harness {
    xdg: PathBuf,
    sock: PathBuf,
    server: Child,
}

impl Drop for Harness {
    fn drop(&mut self) {
        if let Ok(mut stream) = UnixStream::connect(&self.sock) {
            let _ = writeln!(stream, r#"{{"op":"stop"}}"#);
            let _ = stream.flush();
        }
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
        "dory-accept-wait-{}-{}",
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
        .env_remove("DORY_SOCKET")
        .current_dir(&xdg)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn dory server");
    let sock = session_sock(&xdg);
    assert_ne!(
        sock, PathBuf::from(FACTORY_SOCK),
        "test must not use factory default sock"
    );
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

fn rpc(sock: &Path, line: &str) -> String {
    let mut stream = UnixStream::connect(sock).expect("rpc connect");
    writeln!(stream, "{line}").unwrap();
    let _ = stream.flush();
    let mut reply = String::new();
    BufReader::new(stream).read_line(&mut reply).unwrap();
    reply
}

fn rpc_timed(sock: &Path, line: &str) -> (String, Duration) {
    let t = Instant::now();
    let reply = rpc(sock, line);
    (reply, t.elapsed())
}

fn json_field<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let rest = json.split_once(&pat)?.1.trim_start();
    if rest.starts_with('"') {
        let inner = rest.get(1..)?;
        return Some(inner.split_once('"')?.0);
    }
    let end = rest
        .find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace())
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

fn first_pane(h: &Harness) -> String {
    let snap = rpc(&h.sock, r#"{"op":"snapshot"}"#);
    json_field(&snap, "pane")
        .expect("snapshot pane")
        .to_string()
}

fn send_line(sock: &Path, line: &str) -> UnixStream {
    let mut stream = UnixStream::connect(sock).expect("connect");
    writeln!(stream, "{line}").unwrap();
    let _ = stream.flush();
    stream
}

fn read_line(stream: UnixStream) -> String {
    let mut reply = String::new();
    BufReader::new(stream).read_line(&mut reply).unwrap();
    reply
}

unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

fn wait_dead(pid: u32) {
    let start = Instant::now();
    loop {
        let rc = unsafe { kill(pid as i32, 0) };
        if rc != 0 && std::io::Error::last_os_error().raw_os_error() == Some(3) {
            return;
        }
        if start.elapsed() > Duration::from_secs(3) {
            panic!("pid {pid} still alive");
        }
        thread::sleep(Duration::from_millis(20));
    }
}


#[test]
fn ping_during_pane_wait_under_200ms() {
    let h = start();
    let pane = first_pane(&h);
    let sock = h.sock.clone();
    let wait_line = format!(
        r#"{{"op":"pane.wait","pane":"{pane}","match":"NEVER_MATCH_P2_ACCEPT","timeout":8000}}"#
    );
    let waiter = thread::spawn({
        let sock = sock.clone();
        move || rpc(&sock, &wait_line)
    });
    thread::sleep(Duration::from_millis(100));
    let (ping, dt) = rpc_timed(&h.sock, r#"{"op":"ping"}"#);
    assert!(ping.contains("\"ok\":true"), "ping={ping}");
    assert!(
        dt < Duration::from_millis(200),
        "ping during pane.wait took {dt:?}"
    );
    let waited = waiter.join().expect("waiter");
    assert!(waited.contains("\"ok\":false"), "wait={waited}");
    assert!(waited.contains("timeout"), "wait={waited}");
}

#[test]
fn desk_tree_during_prompt_wait_under_200ms() {
    let h = start();
    let pane = first_pane(&h);
    let started = rpc(
        &h.sock,
        &format!(
            r#"{{"op":"agent.start","name":"waiter","pane":"{pane}","argv":["/bin/sleep","120"],"timeout":8000}}"#
        ),
    );
    assert!(started.contains("\"ok\":true"), "start={started}");
    let sock = h.sock.clone();
    let prompt_line = format!(
        r#"{{"op":"agent.prompt","name":"waiter","pane":"{pane}","text":"hi","wait":true,"timeout":8000}}"#
    );
    let waiter = thread::spawn({
        let sock = sock.clone();
        move || rpc(&sock, &prompt_line)
    });
    thread::sleep(Duration::from_millis(100));
    let (tree, dt) = rpc_timed(&h.sock, r#"{"op":"desk.tree"}"#);
    assert!(tree.contains("\"ok\":true"), "tree={tree}");
    assert!(
        dt < Duration::from_millis(200),
        "desk.tree during prompt --wait took {dt:?}"
    );
    let waited = waiter.join().expect("prompt wait");
    assert!(waited.contains("\"ok\":false"), "prompt wait={waited}");
    assert!(waited.contains("timeout"), "prompt wait={waited}");
}

#[test]
fn unread_waiter_does_not_block_ping() {
    let h = start();
    let pane = first_pane(&h);
    let line = format!(
        r#"{{"op":"pane.wait","pane":"{pane}","match":"NEVER_MATCH_P2_UNREAD","timeout":8000}}"#
    );
    let _hold = send_line(&h.sock, &line);
    thread::sleep(Duration::from_millis(100));
    let (ping, dt) = rpc_timed(&h.sock, r#"{"op":"ping"}"#);
    assert!(ping.contains("\"ok\":true"), "ping={ping}");
    assert!(
        dt < Duration::from_millis(200),
        "ping with unread waiter took {dt:?}"
    );
}

#[test]
fn close_parked_wait_json_error_no_text() {
    let h = start();
    let pane = first_pane(&h);
    let split = rpc(
        &h.sock,
        &format!(r#"{{"op":"pane.split","pane":"{pane}","direction":"right"}}"#),
    );
    assert!(split.contains("\"ok\":true"), "split={split}");
    let extra = json_field(&split, "id")
        .filter(|id| *id != pane)
        .map(str::to_string)
        .or_else(|| {
            let list = rpc(
                &h.sock,
                &format!(
                    r#"{{"op":"pane.list","workspace":"{}"}}"#,
                    json_field(&rpc(&h.sock, r#"{"op":"snapshot"}"#), "workspace").unwrap()
                ),
            );
            json_field(&list, "id").map(str::to_string)
        });
    let target = extra.expect("split pane id");
    let sock = h.sock.clone();
    let wait_line = format!(
        r#"{{"op":"pane.wait","pane":"{target}","match":"NEVER_MATCH_P2_CLOSE","timeout":8000}}"#
    );
    let waiter = thread::spawn({
        let sock = sock.clone();
        move || rpc(&sock, &wait_line)
    });
    thread::sleep(Duration::from_millis(100));
    let closed = rpc(
        &h.sock,
        &format!(r#"{{"op":"pane.close","pane":"{target}"}}"#),
    );
    assert!(closed.contains("\"ok\":true"), "close={closed}");
    let waited = waiter.join().expect("waiter");
    assert!(waited.contains("\"ok\":false"), "wait={waited}");
    assert!(
        !waited.contains("\"text\":"),
        "closed wait must not include sibling text: {waited}"
    );
}

#[test]
fn overlapping_agent_start_pane_busy() {
    let h = start();
    let pane = first_pane(&h);
    let start_a = format!(
        r#"{{"op":"agent.start","name":"alice","pane":"{pane}","argv":["/bin/sleep","120"],"timeout":8000}}"#
    );
    let _hold = send_line(&h.sock, &start_a);
    thread::sleep(Duration::from_millis(80));
    let second = rpc(
        &h.sock,
        &format!(
            r#"{{"op":"agent.start","name":"bob","pane":"{pane}","argv":["/bin/sleep","5"],"timeout":8000}}"#
        ),
    );
    assert!(second.contains("\"ok\":false"), "second={second}");
    assert!(second.contains("pane busy"), "second={second}");
}

#[test]
fn thirty_third_wait_fails_closed() {
    let h = start();
    let pane = first_pane(&h);
    let line = format!(
        r#"{{"op":"pane.wait","pane":"{pane}","match":"NEVER_MATCH_P2_CAP","timeout":8000}}"#
    );
    let mut holds = Vec::new();
    for _ in 0..32 {
        holds.push(send_line(&h.sock, &line));
    }
    thread::sleep(Duration::from_millis(200));
    let (reply, dt) = rpc_timed(&h.sock, &line);
    assert!(
        dt < Duration::from_millis(200),
        "33rd wait took {dt:?}"
    );
    assert!(reply.contains("\"ok\":false"), "33rd={reply}");
    assert!(
        reply.contains("too many waits"),
        "33rd={reply}"
    );
    drop(holds);
}

#[test]
fn stop_writes_json_to_parked_then_exits() {
    let mut h = start();
    let pane = first_pane(&h);
    let snap = rpc(&h.sock, r#"{"op":"snapshot"}"#);
    let pid: u32 = json_field(&snap, "pid")
        .expect("pid")
        .parse()
        .expect("pid u32");
    let wait_line = format!(
        r#"{{"op":"pane.wait","pane":"{pane}","match":"NEVER_MATCH_P2_STOP","timeout":8000}}"#
    );
    let parked = send_line(&h.sock, &wait_line);
    thread::sleep(Duration::from_millis(100));
    let stopped = rpc(&h.sock, r#"{"op":"stop"}"#);
    assert!(stopped.contains("\"live\":false"), "stop={stopped}");
    let parked_reply = read_line(parked);
    assert!(
        parked_reply.contains("\"ok\":false"),
        "parked stop json={parked_reply}"
    );
    let status = h.server.wait().expect("server wait");
    assert!(status.success(), "server exit={status}");
    wait_dead(pid);
}

#[test]
fn prompt_stall_does_not_block_ping() {
    let h = start();
    let pane = first_pane(&h);
    let started = rpc(
        &h.sock,
        &format!(
            r#"{{"op":"agent.start","name":"idlebot","pane":"{pane}","argv":["/bin/sleep","120"],"timeout":8000}}"#
        ),
    );
    assert!(started.contains("\"ok\":true"), "start={started}");
    let reported = rpc(
        &h.sock,
        &format!(r#"{{"op":"agent.report","pane":"{pane}","state":"idle"}}"#),
    );
    assert!(reported.contains("\"ok\":true"), "report={reported}");
    let sock = h.sock.clone();
    let prompt_line = format!(
        r#"{{"op":"agent.prompt","name":"idlebot","pane":"{pane}","text":"ping-stall","wait":false}}"#
    );
    let waiter = thread::spawn({
        let sock = sock.clone();
        move || rpc(&sock, &prompt_line)
    });
    thread::sleep(Duration::from_millis(100));
    let (ping, dt) = rpc_timed(&h.sock, r#"{"op":"ping"}"#);
    assert!(ping.contains("\"ok\":true"), "ping={ping}");
    assert!(
        dt < Duration::from_millis(200),
        "ping during prompt stall took {dt:?}"
    );
    let prompted = waiter.join().expect("prompt");
    assert!(
        prompted.contains("agent_prompt_stalled") || prompted.contains("\"ok\":true"),
        "prompt={prompted}"
    );
}

#[test]
fn agent_wait_timeout_envelope() {
    let h = start();
    let pane = first_pane(&h);
    let started = rpc(
        &h.sock,
        &format!(
            r#"{{"op":"agent.start","name":"workbot","pane":"{pane}","argv":["/bin/sleep","120"],"timeout":8000}}"#
        ),
    );
    assert!(started.contains("\"ok\":true"), "start={started}");
    let waited = rpc(
        &h.sock,
        &format!(
            r#"{{"op":"agent.wait","name":"workbot","pane":"{pane}","timeout":400}}"#
        ),
    );
    assert!(waited.contains("\"ok\":false"), "wait={waited}");
    assert!(waited.contains("timeout"), "wait={waited}");
    assert!(!waited.contains("\"text\":"), "wait={waited}");
}
