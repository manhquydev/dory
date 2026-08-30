//! P5 discovery verbs — `tab list` / `pane list` / `pane get`.
//!
//! Harness copied from `pane_io.rs`: `CARGO_BIN_EXE_dory`, temp
//! `XDG_RUNTIME_DIR`, `dory server`. IDs from snapshot / `.result`.
//! Never hardcode the next id as `w1`. No `:7380`.
//!
//! list/get match `workspace list`: inspect, no `DORY_ENV` required.
//! Omit `pane get` target is usage 2, not the focused pane.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
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

struct Created {
    ws: String,
    tab: String,
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
    let path =
        std::env::temp_dir().join(format!("dory-p5-discover-{}-{}", std::process::id(), nanos));
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

fn cli_no_env(h: &Harness, args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("XDG_RUNTIME_DIR", &h.xdg)
        .env("DORY_SOCKET", &h.sock)
        .env_remove("DORY_ENV")
        .env_remove("DORY_PANE_ID")
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

fn extract_kind_ids(json: &str, mid: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let bytes = json.as_bytes();
    let needle = b"\"id\":\"";
    let mut i = 0;
    while i + needle.len() < bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let start = i + needle.len();
            if let Some(rel) = json[start..].find('"') {
                let id = &json[start..start + rel];
                if id.contains(mid) && !ids.iter().any(|have| have == id) {
                    ids.push(id.to_string());
                }
                i = start + rel + 1;
                continue;
            }
        }
        i += 1;
    }
    ids
}

fn assert_no_http(json: &str) {
    // ":7380" also matches JSON `"pid":7380` (desk PTY pid). Lamp is a URL/header.
    let lamp = json.contains("127.0.0.1:7380")
        || json.contains("0.0.0.0:7380")
        || json.contains("[::1]:7380")
        || json.contains(":7380/")
        || json.contains("X-Dory-Inside");
    assert!(!lamp, "discover must not use lamp :7380 or X-Dory-Inside: {json}");
}

fn assert_ok(out: &Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} exit={:?} stdout={} stderr={}",
        out.status.code(),
        stdout(out),
        stderr(out)
    );
    let body = stdout(out);
    assert!(body.contains("\"ok\":true"), "{what}: {body}");
    assert!(
        body.contains("\"result\""),
        "{what} must wrap .result: {body}"
    );
    assert_no_http(&body);
    assert_no_http(&stderr(out));
    body
}

fn snapshot(h: &Harness) -> String {
    let mut stream = UnixStream::connect(&h.sock).expect("snapshot connect");
    writeln!(stream, r#"{{"op":"snapshot"}}"#).unwrap();
    let _ = stream.flush();
    let mut snap = String::new();
    BufReader::new(stream).read_line(&mut snap).unwrap();
    snap
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
    let body = assert_ok(&created, "workspace create");
    let ws = result_id(&body, "workspace");
    let tab = result_id(&body, "tab");
    let pane = result_id(&body, "root_pane");
    assert_ne!(ws, "w1", "next workspace must come from .result, not w1");
    assert_ne!(
        pane, snap_pane,
        "created pane must not be the snapshot pane"
    );
    Created { ws, tab, pane }
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
    let body = assert_ok(&out, "pane split");
    let pane = result_id(&body, "pane");
    assert_ne!(pane, caller, "split must return a new pane");
    pane
}

fn tab_list(h: &Harness, pane: &str, ws: &str) -> Output {
    cli(h, pane, &["tab", "list", "--workspace", ws])
}

fn pane_list(h: &Harness, pane: &str, ws: &str) -> Output {
    cli(h, pane, &["pane", "list", "--workspace", ws])
}

#[test]
fn tab_list_contains_created_tab() {
    let h = start();
    let created = created_layout(&h);
    let out = tab_list(&h, &created.pane, &created.ws);
    let body = assert_ok(&out, "tab list");
    let tabs = extract_kind_ids(&body, ":t");
    assert!(
        tabs.iter().any(|id| id == &created.tab),
        "tab list must contain created tab {}: {body}",
        created.tab
    );
    assert!(
        body.contains("\"occupant\""),
        "tab list items need occupant null-or-object: {body}"
    );
}

#[test]
fn pane_list_grows_after_split() {
    let h = start();
    let created = created_layout(&h);
    let before_out = pane_list(&h, &created.pane, &created.ws);
    let before_body = assert_ok(&before_out, "pane list before split");
    let before = extract_kind_ids(&before_body, ":p");
    assert!(
        before.iter().any(|id| id == &created.pane),
        "pane list must contain root pane {}: {before_body}",
        created.pane
    );

    let new_pane = split_pane(&h, &created.pane);
    let after_out = pane_list(&h, &created.pane, &created.ws);
    let after_body = assert_ok(&after_out, "pane list after split");
    let after = extract_kind_ids(&after_body, ":p");
    assert!(
        after.iter().any(|id| id == &created.pane),
        "pane list must keep original pane {}: {after_body}",
        created.pane
    );
    assert!(
        after.iter().any(|id| id == &new_pane),
        "pane list must contain split pane {new_pane}: {after_body}"
    );
    assert_eq!(
        after.len(),
        before.len() + 1,
        "pane list count must grow by 1; before={before:?} after={after:?}"
    );
    assert!(
        after_body.contains("\"occupant\""),
        "pane list items need occupant null-or-object: {after_body}"
    );
}

#[test]
fn pane_get_matches_split_id_occupant_null_until_start() {
    let h = start();
    let created = created_layout(&h);
    let new_pane = split_pane(&h, &created.pane);

    let got = cli(&h, &created.pane, &["pane", "get", "--pane", &new_pane]);
    let body = assert_ok(&got, "pane get --pane");
    assert_eq!(result_id(&body, "pane"), new_pane, "{body}");
    assert!(
        occupant_name(&body).is_none(),
        "occupant must be null until agent start: {body}"
    );
    assert!(
        body.contains("\"occupant\":null"),
        "occupant must be null until agent start: {body}"
    );

    let started = cli(
        &h,
        &created.pane,
        &[
            "agent",
            "start",
            "scout",
            "--pane",
            &new_pane,
            "--",
            "/bin/sleep",
            "30",
        ],
    );
    assert_ok(&started, "agent start");

    let after = cli(&h, &created.pane, &["pane", "get", "--pane", &new_pane]);
    let after_body = assert_ok(&after, "pane get after start");
    assert_eq!(result_id(&after_body, "pane"), new_pane, "{after_body}");
    assert_eq!(
        occupant_name(&after_body).as_deref(),
        Some("scout"),
        "occupant must be an object after agent start: {after_body}"
    );
}

#[test]
fn omit_pane_get_target_is_usage_two() {
    let h = start();
    let created = created_layout(&h);
    let out = cli(&h, &created.pane, &["pane", "get"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "omit pane get target must be usage 2, not focused pane; stdout={} stderr={}",
        stdout(&out),
        stderr(&out)
    );
    assert_no_http(&stdout(&out));
    assert_no_http(&stderr(&out));
}

#[test]
fn no_dory_env_still_lists_like_workspace_list() {
    let h = start();
    let created = created_layout(&h);

    let ws_list = cli_no_env(&h, &["workspace", "list"]);
    let ws_body = assert_ok(&ws_list, "workspace list without DORY_ENV");
    assert!(
        extract_kind_ids(&ws_body, "")
            .iter()
            .any(|id| id == &created.ws)
            || ws_body.contains(&created.ws),
        "workspace list without env must still see {}: {ws_body}",
        created.ws
    );

    let tabs = cli_no_env(&h, &["tab", "list", "--workspace", &created.ws]);
    let tab_body = assert_ok(&tabs, "tab list without DORY_ENV");
    assert!(
        extract_kind_ids(&tab_body, ":t")
            .iter()
            .any(|id| id == &created.tab),
        "tab list without env must contain {}: {tab_body}",
        created.tab
    );

    let panes = cli_no_env(&h, &["pane", "list", "--workspace", &created.ws]);
    let pane_body = assert_ok(&panes, "pane list without DORY_ENV");
    assert!(
        extract_kind_ids(&pane_body, ":p")
            .iter()
            .any(|id| id == &created.pane),
        "pane list without env must contain {}: {pane_body}",
        created.pane
    );

    let got = cli_no_env(&h, &["pane", "get", "--pane", &created.pane]);
    let got_body = assert_ok(&got, "pane get without DORY_ENV");
    assert_eq!(result_id(&got_body, "pane"), created.pane, "{got_body}");
}

#[test]
fn discover_outputs_have_no_http_port() {
    let h = start();
    let created = created_layout(&h);
    for args in [
        ["tab", "list", "--workspace", created.ws.as_str()].as_slice(),
        ["pane", "list", "--workspace", created.ws.as_str()].as_slice(),
        ["pane", "get", "--pane", created.pane.as_str()].as_slice(),
        ["workspace", "list"].as_slice(),
    ] {
        let out = cli(&h, &created.pane, args);
        let combined = format!("{}{}", stdout(&out), stderr(&out));
        assert_no_http(&combined);
        assert!(
            out.status.success(),
            "{args:?} exit={:?} {combined}",
            out.status.code()
        );
    }
}
