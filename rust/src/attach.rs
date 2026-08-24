//! Human sit-down client. Not the skill. Not a Ratatui clone.
//!
//! Bare `dory` / `dory attach` starts the daemon if needed and sits at the
//! desk (tiled live panes). `--plain` is one raw PTY. Detach ≠ kill.

use crate::server;
use std::env;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const CTRL_B: u8 = 0x02;
const CTRL_BACKSLASH: u8 = 0x1c;

unsafe extern "C" {
    fn setsid() -> i32;
    fn isatty(fd: i32) -> i32;
    fn tcgetattr(fd: i32, t: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, action: i32, t: *const Termios) -> i32;
    fn ioctl(fd: i32, req: u64, arg: *mut WinSize) -> i32;
    fn poll(fds: *mut PollFd, nfds: u64, timeout: i32) -> i32;
}

const TCSANOW: i32 = 0;
const POLLIN: i16 = 0x0001;
#[cfg(target_os = "linux")]
const TIOCGWINSZ: u64 = 0x5413;

#[repr(C)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WinSize {
    row: u16,
    col: u16,
    xpixel: u16,
    ypixel: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

pub fn run(args: &[String]) -> i32 {
    let mut pane = None;
    let mut plain = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_attach_help();
                return 0;
            }
            "--plain" => {
                plain = true;
                i += 1;
            }
            "--pane" => {
                let Some(id) = args.get(i + 1).and_then(|s| safe_id(s)) else {
                    eprintln!("dory: usage: dory attach [--pane <id>] [--plain]");
                    return 2;
                };
                pane = Some(id.to_string());
                i += 2;
            }
            other => {
                if let Some(id) = other.strip_prefix("--pane=") {
                    let Some(id) = safe_id(id) else {
                        eprintln!("dory: usage: dory attach [--pane <id>] [--plain]");
                        return 2;
                    };
                    pane = Some(id.to_string());
                    i += 1;
                } else {
                    eprintln!("dory: usage: dory attach [--pane <id>] [--plain]");
                    return 2;
                }
            }
        }
    }
    if plain {
        sit(pane.as_deref())
    } else {
        crate::desk::run_with_pane(pane.as_deref())
    }
}

fn print_attach_help() {
    print!(
        "\
dory attach — sit on the workplace desk

Usage:
  dory
  dory attach
  dory attach --pane <id>
  dory attach --plain [--pane <id>]

Starts `dory server` if needed. Default is the desk (sidebar + tiled live panes).
`--plain` is the raw PTY client.

Desk prefix Ctrl-b:
  q / d   detach (PTY stays)
  c       new tab
  v       split right (focus new)
  -       split down (focus new)
  h/j/k/l spatial pane
  n / p   next / prev pane (list)
  z       zoom focused tile
  w       new workspace
Click a tile or the sidebar to focus. Drag the divider to resize.
Detach does not kill the PTY.
"
    );
}

fn sit(pane: Option<&str>) -> i32 {
    if let Err(code) = ensure_server() {
        return code;
    }
    if !stdin_is_tty() || !stdout_is_tty() {
        eprintln!("dory: needs a tty (server is up; try `dory` in a terminal)");
        return 1;
    }
    let mut target = pane.map(str::to_string);
    loop {
        match sit_one(target.as_deref()) {
            SitEnd::Detach => return 0,
            SitEnd::Switch(id) => target = Some(id),
            SitEnd::Fail(code) => return code,
        }
    }
}

enum SitEnd {
    Detach,
    Switch(String),
    Fail(i32),
}

fn sit_one(pane: Option<&str>) -> SitEnd {
    let (cols, rows) = tty_size();
    let mut op = String::from(r#"{"op":"pane.attach""#);
    if let Some(id) = pane {
        op.push_str(&format!(r#","pane":"{id}""#));
    }
    if let (Some(c), Some(r)) = (cols, rows) {
        op.push_str(&format!(r#","cols":{c},"rows":{r}"#));
    }
    op.push('}');

    let stream = match server::connect_for_attach() {
        Ok(s) => s,
        Err(code) => return SitEnd::Fail(code),
    };
    let mut stream = stream;
    if writeln!(stream, "{op}").is_err() {
        eprintln!("dory: attach write failed");
        return SitEnd::Fail(1);
    }
    let _ = stream.flush();
    let mut reader = BufReader::new(stream);
    let mut ack = String::new();
    if reader.read_line(&mut ack).is_err() || ack.trim().is_empty() {
        eprintln!("dory: attach handshake failed");
        return SitEnd::Fail(1);
    }
    if !ack.contains("\"ok\":true") {
        eprint!("{ack}");
        return SitEnd::Fail(1);
    }
    let stream = match reader.into_inner() {
        s => s,
    };

    if let Ok(snap) = server::rpc_line(r#"{"op":"desk.snapshot"}"#) {
        if let Some(text) = json_string_field(&snap, "text") {
            let _ = writeln!(io::stderr(), "{text}");
            let _ = writeln!(
                io::stderr(),
                "Ctrl-b n/p switch  Ctrl-b s structure  Ctrl-b d detach"
            );
        }
    }

    let raw = match RawTty::enter() {
        Ok(raw) => raw,
        Err(err) => {
            eprintln!("dory: {err}");
            return SitEnd::Fail(1);
        }
    };
    let end = proxy_tty(stream);
    drop(raw);
    end
}

fn proxy_tty(mut stream: UnixStream) -> SitEnd {
    let _ = stream.set_nonblocking(true);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let in_fd = stdin.as_raw_fd();
    let sock_fd = stream.as_raw_fd();
    let mut in_buf = [0u8; 4096];
    let mut out_buf = [0u8; 4096];
    let mut prefix = false;
    loop {
        let (in_ready, sock_ready) = poll_pair(in_fd, sock_fd);
        if sock_ready {
            match stream.read(&mut out_buf) {
                Ok(0) => return SitEnd::Detach,
                Ok(n) => {
                    if stdout.write_all(&out_buf[..n]).is_err() {
                        return SitEnd::Detach;
                    }
                    let _ = stdout.flush();
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
                Err(err) if err.kind() == io::ErrorKind::Interrupted => {}
                Err(_) => return SitEnd::Detach,
            }
        }
        if in_ready {
            let n = {
                let mut stdin_lock = stdin.lock();
                match stdin_lock.read(&mut in_buf) {
                    Ok(n) => n,
                    Err(err)
                        if err.kind() == io::ErrorKind::WouldBlock
                            || err.kind() == io::ErrorKind::Interrupted =>
                    {
                        0
                    }
                    Err(_) => return SitEnd::Detach,
                }
            };
            if n == 0 {
                return SitEnd::Detach;
            }
            let mut i = 0;
            while i < n {
                let b = in_buf[i];
                i += 1;
                if prefix {
                    prefix = false;
                    match b {
                        b'd' | b'q' | b'D' | b'Q' => return SitEnd::Detach,
                        b'n' | b'N' => return switch("next"),
                        b'p' | b'P' => return switch("prev"),
                        b's' | b'S' => show_structure(),
                        b'b' | CTRL_B => {
                            if stream.write_all(&[CTRL_B]).is_err() {
                                return SitEnd::Detach;
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
                if b == CTRL_BACKSLASH {
                    return SitEnd::Detach;
                }
                if b == CTRL_B {
                    prefix = true;
                    continue;
                }
                if stream.write_all(&[b]).is_err() {
                    return SitEnd::Detach;
                }
            }
            let _ = stream.flush();
        }
    }
}

fn poll_pair(a: i32, b: i32) -> (bool, bool) {
    let mut fds = [
        PollFd {
            fd: a,
            events: POLLIN,
            revents: 0,
        },
        PollFd {
            fd: b,
            events: POLLIN,
            revents: 0,
        },
    ];
    let n = unsafe { poll(fds.as_mut_ptr(), 2, 200) };
    if n <= 0 {
        return (false, false);
    }
    (fds[0].revents & POLLIN != 0, fds[1].revents & POLLIN != 0)
}

fn switch(step: &str) -> SitEnd {
    match server::rpc_line(&format!(r#"{{"op":"desk.neighbor","step":"{step}"}}"#)) {
        Ok(body) => json_string_field(&body, "id")
            .map(SitEnd::Switch)
            .unwrap_or(SitEnd::Detach),
        Err(_) => SitEnd::Detach,
    }
}

fn show_structure() {
    if let Ok(snap) = server::rpc_line(r#"{"op":"desk.snapshot"}"#) {
        if let Some(text) = json_string_field(&snap, "text") {
            let _ = write!(io::stderr(), "\r\n{text}\r\n");
            let _ = io::stderr().flush();
        }
    }
}

pub fn ensure_server() -> Result<(), i32> {
    if ping() {
        return Ok(());
    }
    let exe = env::current_exe().map_err(|err| {
        eprintln!("dory: {err}");
        1
    })?;
    let mut cmd = Command::new(exe);
    cmd.arg("server")
        .env("DORY_SIT_SHELL", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    unsafe {
        cmd.pre_exec(|| {
            if setsid() < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
    cmd.spawn().map_err(|err| {
        eprintln!("dory: start server: {err}");
        1
    })?;
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if ping() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    eprintln!("dory: server did not come up");
    Err(1)
}

fn ping() -> bool {
    server::rpc_line_quiet(r#"{"op":"ping"}"#)
        .ok()
        .is_some_and(|s| s.contains("\"ok\":true"))
}

fn stdin_is_tty() -> bool {
    unsafe { isatty(0) == 1 }
}

fn stdout_is_tty() -> bool {
    unsafe { isatty(1) == 1 }
}

fn tty_size() -> (Option<u16>, Option<u16>) {
    let mut ws = WinSize {
        row: 0,
        col: 0,
        xpixel: 0,
        ypixel: 0,
    };
    let fd = io::stdout().as_raw_fd();
    let rc = unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) };
    if rc == 0 && ws.col > 0 && ws.row > 0 {
        (Some(ws.col), Some(ws.row))
    } else {
        (None, None)
    }
}

fn safe_id(id: &str) -> Option<&str> {
    if id.is_empty()
        || id
            .bytes()
            .any(|b| matches!(b, b'"' | b'\\' | b'\n' | b'\r'))
    {
        None
    } else {
        Some(id)
    }
}

pub(crate) fn json_string_field(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let idx = body.find(&needle)?;
    let mut rest = body[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    rest = rest.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                other => out.push(other),
            },
            c => out.push(c),
        }
    }
    None
}

struct RawTty {
    fd: i32,
    orig: Termios,
}

impl RawTty {
    fn enter() -> io::Result<Self> {
        let fd = io::stdin().as_raw_fd();
        let mut orig = Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        if unsafe { tcgetattr(fd, &mut orig) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let mut raw = orig;
        raw.c_lflag &= !0x000B;
        raw.c_oflag &= !0x0001;
        raw.c_iflag &= !0x1600;
        raw.c_cc[6] = 1;
        raw.c_cc[5] = 0;
        if unsafe { tcsetattr(fd, TCSANOW, &raw) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self { fd, orig })
    }
}

impl Drop for RawTty {
    fn drop(&mut self) {
        unsafe {
            let _ = tcsetattr(self.fd, TCSANOW, &self.orig);
        }
    }
}
