use std::fs;
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn dory_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_dory"))
}

fn temp_xdg() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!("dory-p3-17-{}-{}", std::process::id(), nanos));
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

/// P3-17: nested `dory server` with pane env still refuses.
///
/// Sets `DORY_ENV=1` and `DORY_SOCKET` on the second process. Refuse is the
/// existing same-session lock (env-blind). Exit 2 alone is not enough:
/// unset `XDG_RUNTIME_DIR` is also exit 2.
#[test]
fn p3_17_nested_server_from_pane_env_exits_two() {
    let xdg = temp_xdg();
    let mut server = start_server(&xdg);
    let sock = session_sock(&xdg);

    let second = Command::new(dory_bin())
        .arg("server")
        .env("XDG_RUNTIME_DIR", &xdg)
        .env("DORY_ENV", "1")
        .env("DORY_SOCKET", &sock)
        .output()
        .expect("second dory server");

    let err = String::from_utf8_lossy(&second.stderr);
    assert_eq!(
        second.status.code(),
        Some(2),
        "nested server exit: stdout={} stderr={err}",
        String::from_utf8_lossy(&second.stdout)
    );
    assert!(
        err.contains("nested server refused"),
        "must be lock refuse, not missing XDG: {err}"
    );
    assert!(
        UnixStream::connect(&sock).is_ok(),
        "first server must stay bound"
    );

    let _ = Command::new(dory_bin())
        .args(["server", "stop"])
        .env("XDG_RUNTIME_DIR", &xdg)
        .output();
    let _ = server.wait();
    let _ = fs::remove_dir_all(&xdg);
}
