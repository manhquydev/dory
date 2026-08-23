//! `dory flow --` taxi. Port of `src/flow.js` contract, not the HTTP door.
//!
//! Exec `FLOW_BIN` or `flow.sh`. No gate logic. No `next`/`card`/`check`.

use crate::envelope;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_TIMEOUT: Duration = Duration::from_millis(15_000);
const KILL_GRACE: Duration = Duration::from_secs(1);
const SESSION_ID: &str = "s1";

const SIGTERM: i32 = 15;
const SIGKILL: i32 = 9;

unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

pub fn cmd(args: &[String]) -> i32 {
    let Some(dash) = args.iter().position(|a| a == "--") else {
        eprintln!("dory: usage: dory flow -- <args>");
        return 2;
    };
    if let Err(code) = crate::require_skill_env() {
        return code;
    }

    let mut argv: Vec<String> = args[dash + 1..].to_vec();
    if argv.is_empty() {
        argv.push("status".into());
    }

    let bin = match resolve_flow_bin(env::var("FLOW_BIN").ok().as_deref()) {
        Ok(bin) => bin,
        Err(msg) => {
            eprintln!("{}", envelope::runtime_error(&msg));
            return 1;
        }
    };
    if let Some(name) = forbidden_name(&bin) {
        eprintln!(
            "{}",
            envelope::runtime_error(&format!("dory: refusing to exec {name}"))
        );
        return 1;
    }
    if let Some(name) = argv.iter().find_map(|a| forbidden_name(a)) {
        eprintln!(
            "{}",
            envelope::runtime_error(&format!("dory: refusing to exec {name}"))
        );
        return 1;
    }

    let cwd = match workspace_dir() {
        Ok(dir) => dir,
        Err(code) => return code,
    };

    let journal = session_journal_path(&cwd);
    let invoke = record_json(
        "flow/invoke",
        &bin,
        &argv,
        &cwd,
        None,
        None,
        "",
        "",
        None,
        false,
    );
    if let Err(err) = append_jsonl(&journal, &invoke) {
        eprintln!(
            "{}",
            envelope::runtime_error(&format!("dory: journal: {err}"))
        );
        return 1;
    }

    let result = invoke_flow(&bin, &argv, &cwd, DEFAULT_TIMEOUT);
    let event = record_json(
        "flow/result",
        &result.bin,
        &result.args,
        Path::new(&result.cwd),
        result.code,
        result.signal.as_deref(),
        &result.stdout,
        &result.stderr,
        result.error.as_deref(),
        true,
    );
    let _ = append_jsonl(&journal, &event);
    println!("{}", envelope::success(&event));
    result.code.unwrap_or(1)
}

fn workspace_dir() -> Result<PathBuf, i32> {
    if let Some(dir) = env::var_os("DORY_WORKSPACE_DIR").filter(|v| !v.is_empty()) {
        return Ok(PathBuf::from(dir));
    }
    env::current_dir().map_err(|err| {
        eprintln!("{}", envelope::runtime_error(&format!("dory: cwd: {err}")));
        1
    })
}

fn session_journal_path(cwd: &Path) -> PathBuf {
    cwd.join(".dory")
        .join("sessions")
        .join(format!("{SESSION_ID}.jsonl"))
}

pub(crate) fn resolve_flow_bin(flow_bin: Option<&str>) -> Result<String, String> {
    let bin = match flow_bin {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => "flow.sh".into(),
    };
    if let Some(name) = forbidden_name(&bin) {
        return Err(format!("dory: refusing to exec {name}"));
    }
    Ok(bin)
}

/// Basename `herdr`/`dsh` (optional `.exe`) or any token containing `@deepseek-ai/dsh`.
pub(crate) fn forbidden_name(token: &str) -> Option<String> {
    if token.contains("@deepseek-ai/dsh") {
        return Some("@deepseek-ai/dsh".into());
    }
    let base = Path::new(token)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(token);
    let lower = base.to_ascii_lowercase();
    match lower.as_str() {
        "herdr" | "herdr.exe" => Some(base.to_string()),
        "dsh" | "dsh.exe" => Some(base.to_string()),
        _ => None,
    }
}

struct FlowResult {
    bin: String,
    args: Vec<String>,
    cwd: String,
    code: Option<i32>,
    signal: Option<String>,
    stdout: String,
    stderr: String,
    error: Option<String>,
}

fn invoke_flow(bin: &str, argv: &[String], cwd: &Path, timeout: Duration) -> FlowResult {
    let cwd_s = cwd.to_string_lossy().into_owned();
    let mut cmd = Command::new(bin);
    cmd.args(argv)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            return FlowResult {
                bin: bin.to_string(),
                args: argv.to_vec(),
                cwd: cwd_s,
                code: None,
                signal: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some(err.to_string()),
            };
        }
    };

    let mut stdout_pipe = child.stdout.take().expect("stdout pipe");
    let mut stderr_pipe = child.stderr.take().expect("stderr pipe");
    let t_out = thread::spawn(move || {
        let mut buf = String::new();
        let _ = stdout_pipe.read_to_string(&mut buf);
        buf
    });
    let t_err = thread::spawn(move || {
        let mut buf = String::new();
        let _ = stderr_pipe.read_to_string(&mut buf);
        buf
    });

    let waited = wait_timeout(&mut child, timeout);
    let (code, signal, error) = match waited {
        Ok(Some(status)) => decode_status(status),
        Ok(None) => {
            send_signal(child.id(), SIGTERM);
            thread::sleep(KILL_GRACE);
            if child.try_wait().ok().flatten().is_none() {
                send_signal(child.id(), SIGKILL);
                let _ = child.wait();
            }
            (
                None,
                Some("SIGTERM".into()),
                Some(format!("timed out after {}ms", timeout.as_millis())),
            )
        }
        Err(err) => (None, None, Some(err.to_string())),
    };

    let stdout = t_out.join().unwrap_or_default();
    let stderr = t_err.join().unwrap_or_default();
    FlowResult {
        bin: bin.to_string(),
        args: argv.to_vec(),
        cwd: cwd_s,
        code,
        signal,
        stdout,
        stderr,
        error,
    }
}

fn wait_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> io::Result<Option<std::process::ExitStatus>> {
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        if start.elapsed() >= timeout {
            return Ok(None);
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn decode_status(
    status: std::process::ExitStatus,
) -> (Option<i32>, Option<String>, Option<String>) {
    use std::os::unix::process::ExitStatusExt;
    if let Some(code) = status.code() {
        return (Some(code), None, None);
    }
    if let Some(sig) = status.signal() {
        return (None, Some(signal_name(sig)), None);
    }
    (None, None, None)
}

fn signal_name(sig: i32) -> String {
    match sig {
        1 => "SIGHUP".into(),
        2 => "SIGINT".into(),
        9 => "SIGKILL".into(),
        15 => "SIGTERM".into(),
        n => format!("{n}"),
    }
}

fn send_signal(pid: u32, sig: i32) {
    unsafe {
        kill(pid as i32, sig);
    }
}

fn append_jsonl(path: &Path, line: &str) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

fn record_json(
    typ: &str,
    bin: &str,
    args: &[String],
    cwd: &Path,
    code: Option<i32>,
    signal: Option<&str>,
    stdout: &str,
    stderr: &str,
    error: Option<&str>,
    with_result_fields: bool,
) -> String {
    let mut out = String::from("{");
    push_field(&mut out, "ts", &envelope::json_string(&iso8601_now()), true);
    push_field(&mut out, "type", &envelope::json_string(typ), false);
    push_field(&mut out, "bin", &envelope::json_string(bin), false);
    push_field(&mut out, "args", &json_string_array(args), false);
    push_field(
        &mut out,
        "cwd",
        &envelope::json_string(&cwd.to_string_lossy()),
        false,
    );
    if with_result_fields {
        push_field(&mut out, "code", &json_i32_or_null(code), false);
        push_field(&mut out, "signal", &json_str_or_null(signal), false);
        push_field(&mut out, "stdout", &envelope::json_string(stdout), false);
        push_field(&mut out, "stderr", &envelope::json_string(stderr), false);
        push_field(&mut out, "error", &json_str_or_null(error), false);
    }
    out.push('}');
    out
}

fn push_field(out: &mut String, key: &str, value: &str, first: bool) {
    if !first {
        out.push(',');
    }
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
    out.push_str(value);
}

fn json_string_array(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&envelope::json_string(item));
    }
    out.push(']');
    out
}

fn json_i32_or_null(v: Option<i32>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => "null".into(),
    }
}

fn json_str_or_null(v: Option<&str>) -> String {
    match v {
        Some(s) => envelope::json_string(s),
        None => "null".into(),
    }
}

fn iso8601_now() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();
    let days = (secs / 86_400) as i64;
    let tod = secs % 86_400;
    let hh = tod / 3600;
    let mm = (tod % 3600) / 60;
    let ss = tod % 60;
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}.{millis:03}Z")
}

/// Howard Hinnant civil-from-days. `z` is days since 1970-01-01 UTC.
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn default_bin_is_flow_sh() {
        assert_eq!(resolve_flow_bin(None).unwrap(), "flow.sh");
        assert_eq!(resolve_flow_bin(Some("")).unwrap(), "flow.sh");
        assert_eq!(
            resolve_flow_bin(Some("/opt/flow.sh")).unwrap(),
            "/opt/flow.sh"
        );
    }

    #[test]
    fn refuse_herdr_dsh_and_deepseek_package() {
        for bin in [
            "herdr",
            "/usr/bin/herdr",
            "./herdr",
            "HERDR",
            "herdr.exe",
            "dsh",
            "/opt/bin/dsh",
            "DSH.EXE",
            "@deepseek-ai/dsh",
            "/tmp/node_modules/@deepseek-ai/dsh/cli",
        ] {
            assert!(resolve_flow_bin(Some(bin)).is_err(), "must refuse {bin}");
            assert!(forbidden_name(bin).is_some(), "forbidden_name {bin}");
        }
        assert!(forbidden_name("npx").is_none());
        assert!(forbidden_name("flow.sh").is_none());
        assert!(forbidden_name("@deepseek-ai/dsh").is_some());
    }

    #[test]
    fn invoke_missing_bin_is_error_not_hang() {
        let dir = std::env::temp_dir().join(format!(
            "dory-flow-enoent-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&dir).unwrap();
        let r = invoke_flow(
            dir.join("no-such-flow").to_str().unwrap(),
            &["status".into()],
            &dir,
            Duration::from_secs(2),
        );
        assert!(r.code.is_none());
        assert!(r.error.is_some());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn invoke_preserves_exit_and_cwd() {
        let dir = std::env::temp_dir().join(format!(
            "dory-flow-ok-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&dir).unwrap();
        let bin = dir.join("fake-flow");
        fs::write(
            &bin,
            "#!/bin/sh\nprintf 'cwd=%s\\n' \"$(pwd)\"\nprintf 'arg=%s\\n' \"$1\"\nexit 7\n",
        )
        .unwrap();
        let mut perm = fs::metadata(&bin).unwrap().permissions();
        perm.set_mode(0o755);
        fs::set_permissions(&bin, perm).unwrap();

        let r = invoke_flow(
            bin.to_str().unwrap(),
            &["status".into()],
            &dir,
            Duration::from_secs(2),
        );
        assert_eq!(r.code, Some(7));
        assert!(r.stdout.contains(&format!("cwd={}", dir.display())));
        assert!(r.stdout.contains("arg=status"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn iso8601_looks_like_node_specimen() {
        let ts = iso8601_now();
        assert!(ts.ends_with('Z'), "{ts}");
        assert_eq!(ts.len(), 24, "{ts}");
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[10..11], "T");
    }
}
