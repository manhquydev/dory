//! CARGO_BIN_EXE_dory tests for `dory flow --`.
//!
//! Contract: DORY_ENV gate, FLOW_BIN taxi, session journal receipts,
//! refuse-list, workspace cwd. No next/card/check.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

struct Scratch {
    path: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn scratch(tag: &str) -> Scratch {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!(
        "dory-flow-taxi-{tag}-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&path).unwrap();
    Scratch { path }
}

fn write_exec(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
    let mut perm = fs::metadata(path).unwrap().permissions();
    perm.set_mode(0o755);
    fs::set_permissions(path, perm).unwrap();
}

fn run(
    cwd: &Path,
    dory_env: Option<&str>,
    flow_bin: Option<&str>,
    extra_env: &[(&str, &str)],
    args: &[&str],
) -> Output {
    let mut cmd = Command::new(bin());
    cmd.args(args)
        .current_dir(cwd)
        .env_remove("FLOW_BIN")
        .env_remove("DORY_ENV")
        .env_remove("DORY_WORKSPACE_DIR");
    if let Some(v) = dory_env {
        cmd.env("DORY_ENV", v);
    }
    if let Some(v) = flow_bin {
        cmd.env("FLOW_BIN", v);
    }
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    cmd.output().expect("dory")
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn session_journal(ws: &Path) -> PathBuf {
    ws.join(".dory").join("sessions").join("s1.jsonl")
}

fn journal_body(ws: &Path) -> String {
    fs::read_to_string(session_journal(ws)).unwrap_or_default()
}

fn journal_types(ws: &Path) -> Vec<String> {
    journal_body(ws)
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let rest = line.split_once("\"type\":\"")?.1;
            Some(rest.split_once('"')?.0.to_string())
        })
        .collect()
}

fn assert_json_runtime_err(out: &Output) {
    let err = stderr(out);
    let trimmed = err.trim();
    assert!(
        trimmed.starts_with('{') && trimmed.contains("\"ok\":false"),
        "expected JSON runtime error on stderr, got: {err}"
    );
}

fn marker_script(dir: &Path, marker: &Path) -> PathBuf {
    let script = dir.join("mark-flow");
    write_exec(
        &script,
        &format!(
            "#!/bin/sh\nprintf ran > {}\n",
            shell_single_quote(&marker.to_string_lossy())
        ),
    );
    script
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[test]
fn without_dory_env_exits_one_json_stderr_no_exec() {
    let tmp = scratch("no-env");
    let marker = tmp.path.join("executed");
    let script = marker_script(&tmp.path, &marker);

    let out = run(
        &tmp.path,
        None,
        Some(script.to_str().unwrap()),
        &[],
        &["flow", "--"],
    );
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(&out));
    assert_json_runtime_err(&out);
    assert!(
        stderr(&out).contains("I am not running inside a Dory-managed pane"),
        "stderr={}",
        stderr(&out)
    );
    assert!(
        !marker.exists(),
        "FLOW_BIN must not exec without DORY_ENV=1"
    );
    assert!(
        !session_journal(&tmp.path).exists(),
        "gate must not write a journal"
    );
}

#[test]
fn flow_bin_true_empty_dash_exits_zero_and_journals() {
    let tmp = scratch("true-empty");
    let out = run(
        &tmp.path,
        Some("1"),
        Some("/bin/true"),
        &[],
        &["flow", "--"],
    );
    assert_eq!(out.status.code(), Some(0), "stderr={}", stderr(&out));
    let types = journal_types(&tmp.path);
    assert!(
        types.iter().any(|t| t == "flow/invoke"),
        "missing flow/invoke in {}",
        journal_body(&tmp.path)
    );
    assert!(
        types.iter().any(|t| t == "flow/result"),
        "missing flow/result in {}",
        journal_body(&tmp.path)
    );
    let body = journal_body(&tmp.path);
    assert!(
        body.contains("\"args\":[\"status\"]"),
        "empty `--` defaults to status: {body}"
    );
}

#[test]
fn flow_bin_true_hello_exits_zero_and_journals() {
    let tmp = scratch("true-hello");
    let out = run(
        &tmp.path,
        Some("1"),
        Some("/bin/true"),
        &[],
        &["flow", "--", "hello"],
    );
    assert_eq!(out.status.code(), Some(0), "stderr={}", stderr(&out));
    let types = journal_types(&tmp.path);
    assert!(
        types.iter().any(|t| t == "flow/invoke"),
        "missing flow/invoke in {}",
        journal_body(&tmp.path)
    );
    assert!(
        types.iter().any(|t| t == "flow/result"),
        "missing flow/result in {}",
        journal_body(&tmp.path)
    );
    let body = journal_body(&tmp.path);
    assert!(
        body.contains("\"args\":[\"hello\"]"),
        "hello must be passed through: {body}"
    );
}

#[test]
fn flow_bin_script_exit_seven_is_preserved() {
    let tmp = scratch("exit-7");
    let script = tmp.path.join("exiter");
    write_exec(&script, "#!/bin/sh\nexit 7\n");

    let out = run(
        &tmp.path,
        Some("1"),
        Some(script.to_str().unwrap()),
        &[],
        &["flow", "--"],
    );
    assert_eq!(out.status.code(), Some(7), "stderr={}", stderr(&out));
}

#[test]
fn refuse_herdr_name_no_exec() {
    refuse_named_bin("herdr");
}

#[test]
fn refuse_dsh_name_no_exec() {
    refuse_named_bin("dsh");
}

fn refuse_named_bin(name: &str) {
    let tmp = scratch(name);
    let marker = tmp.path.join("executed");
    let bindir = tmp.path.join("bin");
    write_exec(
        &bindir.join(name),
        &format!(
            "#!/bin/sh\nprintf ran > {}\n",
            shell_single_quote(&marker.to_string_lossy())
        ),
    );
    let path = format!("{}:/usr/bin:/bin", bindir.display());

    let out = run(
        &tmp.path,
        Some("1"),
        Some(name),
        &[("PATH", &path)],
        &["flow", "--"],
    );
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(&out));
    assert_json_runtime_err(&out);
    assert!(
        stderr(&out).contains("refusing to exec"),
        "stderr={}",
        stderr(&out)
    );
    assert!(!marker.exists(), "{name} on PATH must not exec");
}

#[test]
fn refuse_path_ending_herdr_no_exec() {
    refuse_path_ending("herdr");
}

#[test]
fn refuse_path_ending_dsh_no_exec() {
    refuse_path_ending("dsh");
}

fn refuse_path_ending(name: &str) {
    let tmp = scratch(&format!("path-{name}"));
    let marker = tmp.path.join("executed");
    let bin_path = tmp.path.join("wrapped").join(name);
    write_exec(
        &bin_path,
        &format!(
            "#!/bin/sh\nprintf ran > {}\n",
            shell_single_quote(&marker.to_string_lossy())
        ),
    );

    let out = run(
        &tmp.path,
        Some("1"),
        Some(bin_path.to_str().unwrap()),
        &[],
        &["flow", "--"],
    );
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(&out));
    assert_json_runtime_err(&out);
    assert!(
        stderr(&out).contains("refusing to exec"),
        "stderr={}",
        stderr(&out)
    );
    assert!(!marker.exists(), "path ending in {name} must not exec");
}

#[test]
fn refuse_deepseek_dsh_package_no_exec() {
    let tmp = scratch("deepseek");
    let marker = tmp.path.join("executed");
    let packaged = tmp
        .path
        .join("node_modules")
        .join("@deepseek-ai")
        .join("dsh")
        .join("cli");
    write_exec(
        &packaged,
        &format!(
            "#!/bin/sh\nprintf ran > {}\n",
            shell_single_quote(&marker.to_string_lossy())
        ),
    );

    for flow_bin in [packaged.to_str().unwrap(), "@deepseek-ai/dsh"] {
        let out = run(&tmp.path, Some("1"), Some(flow_bin), &[], &["flow", "--"]);
        assert_eq!(
            out.status.code(),
            Some(1),
            "bin={flow_bin} stderr={}",
            stderr(&out)
        );
        assert_json_runtime_err(&out);
        assert!(
            stderr(&out).contains("refusing to exec"),
            "bin={flow_bin} stderr={}",
            stderr(&out)
        );
        assert!(
            !marker.exists(),
            "@deepseek-ai/dsh must not exec (FLOW_BIN={flow_bin})"
        );
    }
}

#[test]
fn child_cwd_is_intended_workspace_dir() {
    let tmp = scratch("cwd");
    let workspace = tmp.path.join("workspace");
    let elsewhere = tmp.path.join("elsewhere");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(&elsewhere).unwrap();

    let pwd_out = tmp.path.join("pwd.out");
    let script = tmp.path.join("pwd-flow");
    write_exec(
        &script,
        &format!(
            "#!/bin/sh\npwd > {}\n",
            shell_single_quote(&pwd_out.to_string_lossy())
        ),
    );

    let out = run(
        &elsewhere,
        Some("1"),
        Some(script.to_str().unwrap()),
        &[("DORY_WORKSPACE_DIR", workspace.to_str().unwrap())],
        &["flow", "--"],
    );
    assert_eq!(out.status.code(), Some(0), "stderr={}", stderr(&out));
    let got = fs::read_to_string(&pwd_out).unwrap_or_default();
    assert_eq!(
        got.trim(),
        workspace.to_str().unwrap(),
        "child pwd must be the workspace dir, not the CLI cwd"
    );
}
