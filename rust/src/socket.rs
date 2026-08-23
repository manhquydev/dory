use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

const SOCK_NAME: &str = "dory.sock";
const LOCK_NAME: &str = "dory.lock";

const LOCK_EX: i32 = 2;
const LOCK_NB: i32 = 4;

#[cfg(target_os = "linux")]
const SOL_SOCKET: i32 = 1;
#[cfg(target_os = "linux")]
const SO_PEERCRED: i32 = 17;

unsafe extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
    fn getuid() -> u32;

    #[cfg(target_os = "linux")]
    fn getsockopt(
        sockfd: i32,
        level: i32,
        optname: i32,
        optval: *mut std::ffi::c_void,
        optlen: *mut u32,
    ) -> i32;

    #[cfg(not(target_os = "linux"))]
    fn getpeereid(s: i32, euid: *mut u32, egid: *mut u32) -> i32;
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct Ucred {
    pid: i32,
    uid: u32,
    gid: u32,
}

/// `$XDG_RUNTIME_DIR/dory/<session>/{dory.sock,dory.lock}`.
#[derive(Debug)]
pub struct SessionPaths {
    pub dir: PathBuf,
    pub sock: PathBuf,
    pub lock: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    MissingRuntimeDir,
    NestedServer,
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingRuntimeDir => {
                write!(f, "dory: XDG_RUNTIME_DIR is unset")
            }
            Error::NestedServer => write!(f, "dory: nested server refused"),
            Error::Io(err) => write!(f, "dory: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::MissingRuntimeDir | Error::NestedServer => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

/// Resolve session paths. Does not invent `/tmp` and does not pick a default
/// session token — the caller supplies `<session>`.
pub fn session_paths(session: &str) -> Result<SessionPaths, Error> {
    let runtime = env::var_os("XDG_RUNTIME_DIR").filter(|v| !v.is_empty());
    let runtime = runtime.ok_or(Error::MissingRuntimeDir)?;
    let dir = PathBuf::from(runtime).join("dory").join(session);
    Ok(SessionPaths {
        sock: dir.join(SOCK_NAME),
        lock: dir.join(LOCK_NAME),
        dir,
    })
}

/// Bind `dory.sock` for this session. Live peer → nested refuse. Stale
/// sock/lock are unlinked first. Exclusive `flock` on `dory.lock` is held
/// until process exit.
pub fn prepare_bind(paths: &SessionPaths) -> Result<UnixListener, Error> {
    fs::create_dir_all(&paths.dir)?;
    if let Some(parent) = paths.dir.parent() {
        let _ = set_mode(parent, 0o700);
    }
    set_mode(&paths.dir, 0o700)?;

    if sock_connectable(&paths.sock) {
        return Err(Error::NestedServer);
    }
    // Leftover lock with no connectable sock is stale. Do not treat it as live.
    let _ = fs::remove_file(&paths.sock);
    let _ = fs::remove_file(&paths.lock);

    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .open(&paths.lock)?;
    flock_exclusive_nb(lock.as_raw_fd())?;

    let listener = match UnixListener::bind(&paths.sock) {
        Ok(listener) => listener,
        Err(err) if err.kind() == ErrorKind::AddrInUse => {
            return Err(Error::NestedServer);
        }
        Err(err) => return Err(Error::Io(err)),
    };
    set_mode(&paths.sock, 0o600)?;

    // Flock dies with the process; the helper returns only the listener.
    std::mem::forget(lock);
    Ok(listener)
}

/// Linux `SO_PEERCRED` (macOS `getpeereid`). Same-uid only — not occupancy.
pub fn peer_same_uid(stream: &UnixStream) -> io::Result<bool> {
    Ok(peer_uid(stream)? == current_uid())
}

fn set_mode(path: &Path, mode: u32) -> io::Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(mode);
    fs::set_permissions(path, perms)
}

fn sock_connectable(path: &Path) -> bool {
    UnixStream::connect(path).is_ok()
}

fn flock_exclusive_nb(fd: i32) -> Result<(), Error> {
    let rc = unsafe { flock(fd, LOCK_EX | LOCK_NB) };
    if rc == 0 {
        return Ok(());
    }
    let err = io::Error::last_os_error();
    if err.kind() == ErrorKind::WouldBlock {
        Err(Error::NestedServer)
    } else {
        Err(Error::Io(err))
    }
}

fn current_uid() -> u32 {
    unsafe { getuid() }
}

#[cfg(target_os = "linux")]
fn peer_uid(stream: &UnixStream) -> io::Result<u32> {
    let mut cred = Ucred {
        pid: 0,
        uid: 0,
        gid: 0,
    };
    let mut len = size_of::<Ucred>() as u32;
    let rc = unsafe {
        getsockopt(
            stream.as_raw_fd(),
            SOL_SOCKET,
            SO_PEERCRED,
            (&raw mut cred).cast(),
            &raw mut len,
        )
    };
    if rc == 0 {
        Ok(cred.uid)
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(not(target_os = "linux"))]
fn peer_uid(stream: &UnixStream) -> io::Result<u32> {
    let mut euid = 0u32;
    let mut egid = 0u32;
    let rc = unsafe { getpeereid(stream.as_raw_fd(), &raw mut euid, &raw mut egid) };
    if rc == 0 {
        Ok(euid)
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());
    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn env_guard() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    struct RuntimeDir {
        path: PathBuf,
        prev: Option<std::ffi::OsString>,
        _guard: MutexGuard<'static, ()>,
    }

    impl RuntimeDir {
        fn set(value: Option<&Path>) -> Self {
            let guard = env_guard();
            let prev = env::var_os("XDG_RUNTIME_DIR");
            match value {
                Some(path) => unsafe { env::set_var("XDG_RUNTIME_DIR", path) },
                None => unsafe { env::remove_var("XDG_RUNTIME_DIR") },
            }
            Self {
                path: value.map(Path::to_path_buf).unwrap_or_default(),
                prev,
                _guard: guard,
            }
        }

        fn fresh() -> Self {
            let path = env::temp_dir().join(format!(
                "dory-socket-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self::set(Some(&path))
        }
    }

    impl Drop for RuntimeDir {
        fn drop(&mut self) {
            if !self.path.as_os_str().is_empty() {
                let _ = fs::remove_dir_all(&self.path);
            }
            match &self.prev {
                Some(v) => unsafe { env::set_var("XDG_RUNTIME_DIR", v) },
                None => unsafe { env::remove_var("XDG_RUNTIME_DIR") },
            }
        }
    }

    #[test]
    fn p2_6_paths_under_xdg_runtime_dir_not_tmp() {
        let xdg = RuntimeDir::fresh();
        let paths = session_paths("default").unwrap();
        assert_eq!(paths.dir, xdg.path.join("dory").join("default"));
        assert_eq!(paths.sock, paths.dir.join("dory.sock"));
        assert_eq!(paths.lock, paths.dir.join("dory.lock"));
        assert_ne!(paths.dir, PathBuf::from("/tmp"));
        assert_ne!(paths.dir, PathBuf::from("/tmp/dory/default"));
        assert!(!paths.sock.starts_with("/tmp/dory/default"));

        let _listener = prepare_bind(&paths).unwrap();
        let dir_mode = fs::metadata(&paths.dir).unwrap().permissions().mode() & 0o777;
        let sock_mode = fs::metadata(&paths.sock).unwrap().permissions().mode() & 0o777;
        assert_eq!(dir_mode, 0o700);
        assert_eq!(sock_mode, 0o600);
    }

    #[test]
    fn p2_8_unset_or_empty_xdg_runtime_dir_refuses() {
        let _xdg = RuntimeDir::set(None);
        let err = session_paths("default").unwrap_err();
        assert!(matches!(err, Error::MissingRuntimeDir));
        assert!(!err.to_string().contains("/tmp"));

        drop(_xdg);
        let _xdg = RuntimeDir::set(Some(Path::new("")));
        let err = session_paths("default").unwrap_err();
        assert!(matches!(err, Error::MissingRuntimeDir));
    }

    #[test]
    fn p2_10_live_peer_nested_refuse() {
        let _xdg = RuntimeDir::fresh();
        let paths = session_paths("default").unwrap();
        let _listener = prepare_bind(&paths).unwrap();
        let err = prepare_bind(&paths).unwrap_err();
        assert!(matches!(err, Error::NestedServer));
        assert!(UnixStream::connect(&paths.sock).is_ok());
    }

    #[test]
    fn p2_10_stale_sock_and_lock_reclaimed() {
        let _xdg = RuntimeDir::fresh();
        let paths = session_paths("default").unwrap();
        fs::create_dir_all(&paths.dir).unwrap();
        fs::write(&paths.sock, b"dead").unwrap();
        fs::write(&paths.lock, b"stale").unwrap();

        let _listener = prepare_bind(&paths).unwrap();
        assert!(UnixStream::connect(&paths.sock).is_ok());
        let sock_mode = fs::metadata(&paths.sock).unwrap().permissions().mode() & 0o777;
        assert_eq!(sock_mode, 0o600);
    }

    #[test]
    fn p2_7_same_uid_peercred_allows() {
        let _xdg = RuntimeDir::fresh();
        let paths = session_paths("default").unwrap();
        let listener = prepare_bind(&paths).unwrap();
        let client = UnixStream::connect(&paths.sock).unwrap();
        let (server, _) = listener.accept().unwrap();
        assert!(peer_same_uid(&server).unwrap());
        assert!(peer_same_uid(&client).unwrap());
    }
}
