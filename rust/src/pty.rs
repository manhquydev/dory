use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::collections::{HashSet, VecDeque};

use std::ffi::OsString;
use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const DEFAULT_COLS: u16 = 80;
const DEFAULT_ROWS: u16 = 24;
const TERM: &str = "xterm-256color";
const DRAIN_CAP: usize = 256 * 1024;
const DRAIN_POLL_MS: i32 = 20;
const POLLIN: i16 = 0x0001;

const SIGTERM: i32 = 15;
const SIGKILL: i32 = 9;

#[repr(C)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}

unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
    fn dup(fd: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn read(fd: i32, buf: *mut u8, n: usize) -> isize;
    fn poll(fds: *mut PollFd, nfds: u64, timeout: i32) -> i32;
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::new(format!("dory: pty: {err}"))
    }
}

fn pty_err(err: impl fmt::Display) -> Error {
    Error::new(format!("dory: pty: {err}"))
}

/// CHARTER: shipped spawn must not run herdr / dsh / @deepseek-ai/dsh.
pub fn refuse_spawn_argv(argv: &[impl AsRef<str>]) -> Result<(), Error> {
    for arg in argv {
        if let Some(name) = refused_spawn_name(arg.as_ref()) {
            return Err(Error::new(format!("dory: refuse spawn of {name}")));
        }
    }
    Ok(())
}

fn refused_spawn_name(token: &str) -> Option<&'static str> {
    let token = token.trim().trim_end_matches('/');
    if token.is_empty() {
        return None;
    }
    if token.contains("@deepseek-ai/dsh") {
        return Some("@deepseek-ai/dsh");
    }
    let base = Path::new(token)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(token);
    match base.to_ascii_lowercase().as_str() {
        "herdr" | "herdr.exe" => Some("herdr"),
        "dsh" | "dsh.exe" => Some("dsh"),
        _ => None,
    }
}

/// Walk children of `root_pid` and collect `comm` names.
/// Linux reads `/proc`. Darwin uses libproc (no extra child — do not
/// spawn `ps` from the server; SIGCHLD would reap pane slaves).
pub fn descendant_comms(root_pid: u32) -> Vec<String> {
    #[cfg(target_os = "linux")]
    {
        descendant_comms_proc(root_pid)
    }
    #[cfg(target_os = "macos")]
    {
        descendant_comms_libproc(root_pid)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = root_pid;
        Vec::new()
    }
}

#[cfg(target_os = "linux")]
fn descendant_comms_proc(root_pid: u32) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![root_pid];
    while let Some(pid) = stack.pop() {
        if !seen.insert(pid) {
            continue;
        }
        if pid != root_pid {
            if let Ok(comm) = std::fs::read_to_string(format!("/proc/{pid}/comm")) {
                let name = comm.trim();
                if !name.is_empty() {
                    out.push(name.to_string());
                }
            }
        }
        if let Ok(kids) = std::fs::read_to_string(format!("/proc/{pid}/task/{pid}/children")) {
            for tok in kids.split_whitespace() {
                if let Ok(child) = tok.parse::<u32>() {
                    stack.push(child);
                }
            }
        }
    }
    out
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn proc_listchildpids(pid: i32, buffer: *mut u8, buffersize: i32) -> i32;
    fn proc_name(pid: i32, buffer: *mut u8, buffersize: u32) -> i32;
}

#[cfg(target_os = "macos")]
fn proc_comm(pid: u32) -> Option<String> {
    let mut buf = [0u8; 32];
    let n = unsafe { proc_name(pid as i32, buf.as_mut_ptr(), buf.len() as u32) };
    if n <= 0 {
        return None;
    }
    let end = buf.iter().position(|&b| b == 0).unwrap_or(n as usize);
    let name = std::str::from_utf8(&buf[..end]).ok()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

#[cfg(target_os = "macos")]
fn proc_children(pid: u32) -> Vec<u32> {
    unsafe {
        let hint = proc_listchildpids(pid as i32, std::ptr::null_mut(), 0);
        let cap = if hint > 0 {
            (hint as usize / 4) + 8
        } else {
            32
        };
        let mut buf = vec![0i32; cap];
        let n = proc_listchildpids(
            pid as i32,
            buf.as_mut_ptr() as *mut u8,
            (buf.len() * 4) as i32,
        );
        if n <= 0 {
            return Vec::new();
        }
        buf[..(n as usize / 4)]
            .iter()
            .copied()
            .filter(|p| *p > 1)
            .map(|p| p as u32)
            .collect()
    }
}

#[cfg(target_os = "macos")]
fn push_comm(out: &mut Vec<String>, name: &str) {
    if !out.iter().any(|c| c == name) {
        out.push(name.to_string());
    }
    let stripped = name.trim_start_matches('-');
    if stripped != name && !out.iter().any(|c| c == stripped) {
        out.push(stripped.to_string());
    }
    // /bin/sh is bash (or dash) on Darwin; argv0_comm stays "sh".
    if matches!(stripped, "bash" | "dash") && !out.iter().any(|c| c == "sh") {
        out.push("sh".to_string());
    }
}

#[cfg(target_os = "macos")]
fn descendant_comms_libproc(root_pid: u32) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![root_pid];
    while let Some(pid) = stack.pop() {
        if !seen.insert(pid) {
            continue;
        }
        if let Some(name) = proc_comm(pid) {
            push_comm(&mut out, &name);
        }
        stack.extend(proc_children(pid));
    }
    out
}

/// Occupancy proof injected into a managed pane at spawn. Not an HTTP header.
pub struct Occupancy {
    pub socket: PathBuf,
    pub bin: PathBuf,
    pub workspace_id: String,
    pub tab_id: String,
    pub pane_id: String,
}

/// Server-held PTY master plus the slave's session-leader child.
/// Dropping a cloned reader is not `kill_group`. Detach ≠ kill.
pub struct HeldPty {
    master: Box<dyn MasterPty + Send>,
    /// portable-pty `take_writer` (no `try_clone_writer`). Drop sends EOF.
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    drain: Arc<OutputDrain>,
    stop: Arc<AtomicBool>,
    drain_thread: Option<JoinHandle<()>>,
    child: Box<dyn Child + Send + Sync>,
    pid: u32,
}

struct OutputDrain {
    bytes: Mutex<VecDeque<u8>>,
    total: AtomicU64,
    dead: AtomicBool,
    cv: Condvar,
}

/// Thread-safe handles so an attach client can sit on a live PTY
/// without holding `&mut World`. Detach drops these; it does not kill.
pub struct AttachIO {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    drain: Arc<OutputDrain>,
}

impl HeldPty {
    pub fn spawn(cwd: &Path, argv: &[OsString]) -> Result<Self, Error> {
        Self::spawn_inner(cwd, argv, None)
    }

    pub fn spawn_occupied(cwd: &Path, argv: &[OsString], occ: &Occupancy) -> Result<Self, Error> {
        Self::spawn_inner(cwd, argv, Some(occ))
    }

    fn spawn_inner(cwd: &Path, argv: &[OsString], occ: Option<&Occupancy>) -> Result<Self, Error> {
        if argv.is_empty() {
            return Err(Error::new("dory: empty spawn argv"));
        }
        let tokens: Vec<String> = argv
            .iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        refuse_spawn_argv(&tokens)?;

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: DEFAULT_ROWS,
                cols: DEFAULT_COLS,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(pty_err)?;

        let mut cmd = CommandBuilder::from_argv(argv.to_vec());
        cmd.cwd(cwd);
        cmd.env("TERM", TERM);
        if let Some(occ) = occ {
            cmd.env("DORY_ENV", "1");
            cmd.env("DORY_SOCKET", &occ.socket);
            cmd.env("DORY_BIN", &occ.bin);
            cmd.env("DORY_WORKSPACE_ID", &occ.workspace_id);
            cmd.env("DORY_TAB_ID", &occ.tab_id);
            cmd.env("DORY_PANE_ID", &occ.pane_id);
        }

        let writer = pair.master.take_writer().map_err(pty_err)?;
        let drain = OutputDrain::shared();
        let stop = Arc::new(AtomicBool::new(false));
        let drain_thread = spawn_drain(pair.master.as_ref(), &drain, &stop)?;

        let child = match pair.slave.spawn_command(cmd) {
            Ok(child) => child,
            Err(err) => {
                stop_drain(&stop, drain_thread);
                return Err(pty_err(err));
            }
        };
        let pid = match child.process_id() {
            Some(pid) => pid,
            None => {
                stop_drain(&stop, drain_thread);
                return Err(Error::new("dory: pty child has no pid"));
            }
        };

        Ok(Self {
            master: pair.master,
            writer: Arc::new(Mutex::new(writer)),
            drain,
            stop,
            drain_thread: Some(drain_thread),
            child,
            pid,
        })
    }

    /// TIOCSWINSZ on **our** master fd. Not `stty` bytes into the slave.
    pub fn resize(&self, cols: u16, rows: u16) -> Result<(), Error> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(pty_err)
    }

    pub fn size(&self) -> Result<(u16, u16), Error> {
        let sz = self.master.get_size().map_err(pty_err)?;
        Ok((sz.cols, sz.rows))
    }

    pub fn child_pid(&self) -> u32 {
        self.pid
    }

    /// Write to the slave via the held master writer. Caller adds Enter.
    pub fn write_all(&self, bytes: &[u8]) -> Result<(), Error> {
        let mut writer = self.writer.lock().unwrap_or_else(|e| e.into_inner());
        writer.write_all(bytes)?;
        writer.flush()?;
        Ok(())
    }

    pub fn attach_io(&self) -> AttachIO {
        AttachIO {
            writer: Arc::clone(&self.writer),
            drain: Arc::clone(&self.drain),
        }
    }

    /// Recent drained master bytes. Soft wraps stay as the slave emitted them.
    pub fn recent(&self) -> String {
        String::from_utf8_lossy(&self.drain.snapshot()).into_owned()
    }

    /// Same as [`recent`]: no wrap-join without a terminal emulator.
    pub fn recent_unwrapped(&self) -> String {
        self.recent()
    }

    /// Tail of [`recent`]. Not a real viewport.
    pub fn visible(&self) -> String {
        let rows = self
            .master
            .get_size()
            .map(|s| s.rows)
            .unwrap_or(DEFAULT_ROWS) as usize;
        tail_lines(&self.recent(), rows.max(1))
    }

    /// Kill the process **group**, not a lone pid.
    pub fn kill_group(&mut self) -> Result<(), Error> {
        let pgid = self
            .master
            .process_group_leader()
            .filter(|p| *p > 0)
            .unwrap_or(self.pid as i32);
        if pgid <= 0 {
            return Err(Error::new("dory: no process group to kill"));
        }
        signal_group(pgid, SIGTERM)?;
        if !wait_child_exit(&mut *self.child, Duration::from_millis(250)) {
            signal_group(pgid, SIGKILL)?;
            let _ = self.child.try_wait();
        }
        self.drain.mark_dead();
        Ok(())
    }
}

impl Drop for HeldPty {
    fn drop(&mut self) {
        self.drain.mark_dead();
        if let Some(handle) = self.drain_thread.take() {
            stop_drain(&self.stop, handle);
        }
    }
}

impl AttachIO {
    pub fn write_all(&self, bytes: &[u8]) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap_or_else(|e| e.into_inner());
        writer.write_all(bytes)?;
        writer.flush()
    }

    pub fn snapshot(&self) -> Vec<u8> {
        self.drain.snapshot()
    }

    pub fn cursor(&self) -> u64 {
        self.drain.total()
    }

    pub fn since(&self, seen: u64) -> (u64, Vec<u8>) {
        self.drain.since(seen)
    }

    pub fn wait_since(&self, seen: u64, timeout: Duration) -> (u64, Vec<u8>) {
        self.drain.wait_since(seen, timeout)
    }

    pub fn is_dead(&self) -> bool {
        self.drain.is_dead()
    }
}

impl OutputDrain {
    fn shared() -> Arc<Self> {
        Arc::new(Self {
            bytes: Mutex::new(VecDeque::new()),
            total: AtomicU64::new(0),
            dead: AtomicBool::new(false),
            cv: Condvar::new(),
        })
    }

    fn mark_dead(&self) {
        self.dead.store(true, Ordering::SeqCst);
        self.cv.notify_all();
    }

    fn is_dead(&self) -> bool {
        self.dead.load(Ordering::SeqCst)
    }

    fn total(&self) -> u64 {
        self.total.load(Ordering::SeqCst)
    }

    fn push(&self, chunk: &[u8]) {
        if chunk.is_empty() {
            return;
        }
        let mut buf = self.bytes.lock().unwrap_or_else(|e| e.into_inner());
        buf.extend(chunk.iter().copied());
        let extra = buf.len().saturating_sub(DRAIN_CAP);
        if extra > 0 {
            buf.drain(..extra);
        }
        self.total.fetch_add(chunk.len() as u64, Ordering::SeqCst);
        self.cv.notify_all();
    }

    fn snapshot(&self) -> Vec<u8> {
        let buf = self.bytes.lock().unwrap_or_else(|e| e.into_inner());
        buf.iter().copied().collect()
    }

    fn since(&self, seen: u64) -> (u64, Vec<u8>) {
        let buf = self.bytes.lock().unwrap_or_else(|e| e.into_inner());
        let total = self.total.load(Ordering::SeqCst);
        if seen >= total {
            return (seen, Vec::new());
        }
        let start = total.saturating_sub(buf.len() as u64);
        let skip = seen.saturating_sub(start) as usize;
        let out = buf.iter().skip(skip).copied().collect();
        (total, out)
    }

    fn wait_since(&self, seen: u64, timeout: Duration) -> (u64, Vec<u8>) {
        let deadline = Instant::now() + timeout;
        let mut buf = self.bytes.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            let total = self.total.load(Ordering::SeqCst);
            if seen < total || self.is_dead() {
                drop(buf);
                return self.since(seen);
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return (seen, Vec::new());
            }
            let (g, result) = self
                .cv
                .wait_timeout(buf, left)
                .unwrap_or_else(|e| e.into_inner());
            buf = g;
            if result.timed_out() {
                return (seen, Vec::new());
            }
        }
    }
}

fn spawn_drain(
    master: &dyn MasterPty,
    drain: &Arc<OutputDrain>,
    stop: &Arc<AtomicBool>,
) -> Result<JoinHandle<()>, Error> {
    let fd = master
        .as_raw_fd()
        .ok_or_else(|| Error::new("dory: pty master has no fd"))?;
    let fd = unsafe { dup(fd) };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let drain = Arc::clone(drain);
    let stop = Arc::clone(stop);
    thread::Builder::new()
        .name("dory-pty-drain".into())
        .spawn(move || drain_loop(fd, drain, stop))
        .map_err(Error::from)
}

fn stop_drain(stop: &AtomicBool, handle: JoinHandle<()>) {
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
}

fn drain_loop(fd: i32, drain: Arc<OutputDrain>, stop: Arc<AtomicBool>) {
    let mut tmp = [0u8; 4096];
    while !stop.load(Ordering::Relaxed) {
        let mut pfd = PollFd {
            fd,
            events: POLLIN,
            revents: 0,
        };
        let n = unsafe { poll(&mut pfd, 1, DRAIN_POLL_MS) };
        if n < 0 {
            if io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
                continue;
            }
            break;
        }
        if n == 0 {
            continue;
        }
        let got = unsafe { read(fd, tmp.as_mut_ptr(), tmp.len()) };
        if got < 0 {
            if io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
                continue;
            }
            break;
        }
        if got == 0 {
            break;
        }
        drain.push(&tmp[..got as usize]);
    }
    unsafe {
        close(fd);
    }
}

fn tail_lines(text: &str, rows: usize) -> String {
    let mut parts: Vec<&str> = text.split_inclusive('\n').collect();
    if parts.len() <= rows {
        return text.to_string();
    }
    parts.drain(..parts.len() - rows);
    parts.concat()
}

fn signal_group(pgid: i32, sig: i32) -> Result<(), Error> {
    let rc = unsafe { kill(-pgid, sig) };
    if rc == 0 {
        return Ok(());
    }
    let err = io::Error::last_os_error();
    if err.raw_os_error() == Some(libc_esrch()) {
        return Ok(());
    }
    Err(err.into())
}

fn libc_esrch() -> i32 {
    3
}

fn wait_child_exit(child: &mut dyn Child, budget: Duration) -> bool {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return true,
            Ok(None) if start.elapsed() >= budget => return false,
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => return false,
        }
    }
}

fn pid_exists(pid: u32) -> bool {
    unsafe { kill(pid as i32, 0) == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn temp_path(tag: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("dory-pty-{}-{}-{}", tag, std::process::id(), nanos))
    }

    fn os(args: &[&str]) -> Vec<OsString> {
        args.iter().map(OsString::from).collect()
    }

    fn wait_file(path: &Path, budget: Duration) -> String {
        let start = Instant::now();
        loop {
            if let Ok(body) = fs::read_to_string(path) {
                if !body.is_empty() {
                    return body;
                }
            }
            if start.elapsed() >= budget {
                panic!("timed out waiting for {}", path.display());
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn wait_exe_basename(pid: u32, want: &str, budget: Duration) -> PathBuf {
        let start = Instant::now();
        let path = format!("/proc/{pid}/exe");
        loop {
            if let Ok(exe) = fs::read_link(&path) {
                if exe.file_name().and_then(|n| n.to_str()) == Some(want) {
                    return exe;
                }
            }
            if start.elapsed() >= budget {
                let got = fs::read_link(&path).ok();
                panic!("timed out waiting for {path} basename {want}, last {got:?}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn wait_recent(pty: &HeldPty, needle: &str, budget: Duration) -> String {
        let start = Instant::now();
        loop {
            let got = pty.recent();
            if got.contains(needle) {
                return got;
            }
            if start.elapsed() >= budget {
                panic!("timed out waiting for {needle:?} in recent, got {got:?}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    /// P2-1: child stdout is a real slave tty. Marker is written by the child.
    #[test]
    fn p2_1_child_isatty() {
        let mark = temp_path("isatty");
        let _ = fs::remove_file(&mark);
        let script = r#"if [ -t 1 ]; then printf 1 > "$1"; else printf 0 > "$1"; fi"#;
        let argv = os(&["/bin/sh", "-c", script, "isatty", mark.to_str().unwrap()]);
        let mut pty = HeldPty::spawn(std::env::temp_dir().as_path(), &argv).unwrap();
        let body = wait_file(&mark, Duration::from_secs(5));
        let _ = pty.child.try_wait();
        let _ = fs::remove_file(&mark);
        assert_eq!(body.trim(), "1", "child stdout must be a tty");
    }

    /// P2-2: TIOCSWINSZ on our master; child TIOCGWINSZ / `stty size` is rows cols.
    #[test]
    fn p2_2_resize_is_master_tiocswinsz() {
        let flag = temp_path("resize-go");
        let out = temp_path("resize-out");
        let _ = fs::remove_file(&flag);
        let _ = fs::remove_file(&out);
        let script = r#"
while [ ! -f "$1" ]; do sleep 0.05; done
stty size < /dev/tty > "$2"
"#;
        let argv = os(&[
            "/bin/sh",
            "-c",
            script,
            "resize",
            flag.to_str().unwrap(),
            out.to_str().unwrap(),
        ]);
        let mut pty = HeldPty::spawn(std::env::temp_dir().as_path(), &argv).unwrap();
        pty.resize(100, 30).unwrap();
        fs::write(&flag, b"go").unwrap();
        let body = wait_file(&out, Duration::from_secs(5));
        let _ = pty.kill_group();
        let _ = fs::remove_file(&flag);
        let _ = fs::remove_file(&out);
        assert_eq!(
            body.split_whitespace().collect::<Vec<_>>(),
            ["30", "100"],
            "stty size is rows cols after master TIOCSWINSZ"
        );
    }

    /// P2-9: refuse herdr/dsh/@deepseek-ai/dsh; slave Command is not `script`.
    #[test]
    fn p2_9_refuse_herdr_dsh_and_spawn_is_not_script() {
        for argv in [
            vec!["herdr"],
            vec!["/usr/bin/herdr"],
            vec!["./herdr"],
            vec!["dsh"],
            vec!["/opt/bin/dsh"],
            vec!["@deepseek-ai/dsh"],
            vec!["npx", "@deepseek-ai/dsh"],
            vec!["HERDR"],
            vec!["/usr/bin/herdr.exe"],
            vec!["DSH.EXE"],
        ] {
            let err = refuse_spawn_argv(&argv).unwrap_err();
            let msg = err.to_string();
            assert!(
                msg.contains("refuse spawn"),
                "expected refuse for {argv:?}, got {msg}"
            );
            assert!(
                HeldPty::spawn(
                    std::env::temp_dir().as_path(),
                    &argv.iter().map(OsString::from).collect::<Vec<_>>()
                )
                .is_err(),
                "spawn must refuse {argv:?}"
            );
        }
        assert!(refuse_spawn_argv(&["/bin/sh"]).is_ok());

        let argv = os(&["/bin/sleep", "8"]);
        // Keep a waiter that is unambiguously not `script`.
        let mut pty = HeldPty::spawn(std::env::temp_dir().as_path(), &argv).unwrap();
        let pid = pty.child_pid();
        assert!(pid_exists(pid), "sleep slave must stay live pid {pid}");
        // /proc/pid/exe + cmdline are Linux-only; refuse list already
        // covers "not script". Darwin has no /proc.
        #[cfg(target_os = "linux")]
        {
            let exe = wait_exe_basename(pid, "sleep", Duration::from_secs(3));
            let cmdline = fs::read(format!("/proc/{pid}/cmdline")).unwrap();
            let cmdline = String::from_utf8_lossy(&cmdline);
            assert!(
                exe.ends_with("sleep"),
                "child exe must be sleep, got {exe:?}"
            );
            assert!(
                !cmdline
                    .split('\0')
                    .next()
                    .unwrap_or("")
                    .ends_with("/script"),
                "spawn argv must not be script: {cmdline:?}"
            );
        }
        pty.kill_group().unwrap();
    }

    /// P2-3 law at this layer: dropping a dummy reader ≠ kill_group.
    #[test]
    fn drop_reader_does_not_kill_child() {
        let argv = os(&["/bin/sleep", "12"]);
        let mut pty = HeldPty::spawn(std::env::temp_dir().as_path(), &argv).unwrap();
        let pid = pty.child_pid();
        {
            let reader = pty.master.try_clone_reader().expect("clone reader");
            drop(reader);
        }
        assert!(pid_exists(pid), "dropping a reader must not kill pid {pid}");
        assert!(
            pty.child.try_wait().unwrap().is_none(),
            "child must still be running after reader drop"
        );
        assert!(
            pty.master.get_size().is_ok(),
            "master must still be alive after reader drop"
        );
        pty.kill_group().unwrap();
        assert!(
            !pid_exists(pid) || pty.child.try_wait().ok().flatten().is_some(),
            "kill_group must be what reaps the child"
        );
    }

    /// P3-1: occupancy env is injected at spawn. Not a skill gate (that is P3-15).
    #[test]
    fn p3_1_spawn_occupied_injects_dory_env() {
        let mark = temp_path("occupancy");
        let _ = fs::remove_file(&mark);
        let socket = temp_path("dory.sock");
        let bin = temp_path("dory-bin");
        let occ = Occupancy {
            socket: socket.clone(),
            bin: bin.clone(),
            workspace_id: "w2".to_string(),
            tab_id: "w2:t3".to_string(),
            pane_id: "w2:p4".to_string(),
        };
        // BSD printenv (macOS) takes one name; GNU printenv takes many.
        let script = r#"printf '%s\n' "$DORY_ENV" "$DORY_SOCKET" "$DORY_BIN" "$DORY_WORKSPACE_ID" "$DORY_TAB_ID" "$DORY_PANE_ID" > "$1""#;
        let argv = os(&["/bin/sh", "-c", script, "occupancy", mark.to_str().unwrap()]);
        let mut pty = HeldPty::spawn_occupied(std::env::temp_dir().as_path(), &argv, &occ).unwrap();
        let body = wait_file(&mark, Duration::from_secs(5));
        let _ = pty.child.try_wait();
        let _ = fs::remove_file(&mark);
        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(
            lines,
            [
                "1",
                socket.to_str().unwrap(),
                bin.to_str().unwrap(),
                "w2",
                "w2:t3",
                "w2:p4",
            ],
            "slave must see DORY_ENV=1 and the five occupancy values"
        );
    }

    /// write_all reaches the slave; drain keeps those bytes for later read.
    #[test]
    fn write_all_reaches_slave_and_recent() {
        let mark = temp_path("write-in");
        let _ = fs::remove_file(&mark);
        let script = r#"read x; printf "%s" "$x" > "$1""#;
        let argv = os(&["/bin/sh", "-c", script, "write", mark.to_str().unwrap()]);
        let mut pty = HeldPty::spawn(std::env::temp_dir().as_path(), &argv).unwrap();
        pty.write_all(b"hi\n").unwrap();
        let body = wait_file(&mark, Duration::from_secs(5));
        let recent = wait_recent(&pty, "hi", Duration::from_secs(5));
        let _ = pty.child.try_wait();
        let _ = fs::remove_file(&mark);
        assert_eq!(body, "hi", "slave must consume write_all bytes");
        assert!(recent.contains("hi"), "drain must retain {recent:?}");
        assert_eq!(
            pty.recent(),
            pty.recent_unwrapped(),
            "no wrap-join without a terminal emulator"
        );
        let visible = pty.visible();
        assert!(
            pty.recent().ends_with(&visible),
            "visible is the tail of recent"
        );
    }

    #[test]
    fn write_all_to_cat_appears_in_recent() {
        let argv = os(&["/bin/cat"]);
        let mut pty = HeldPty::spawn(std::env::temp_dir().as_path(), &argv).unwrap();
        pty.write_all(b"hi\n").unwrap();
        let recent = wait_recent(&pty, "hi", Duration::from_secs(5));
        pty.kill_group().unwrap();
        assert!(recent.contains("hi"), "cat must echo write_all into drain");
    }
}
