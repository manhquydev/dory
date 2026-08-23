//! P5 real-repo inside-pane loop — occupant *inside* the first created pane
//! drives split / start / prompt / wait / `flow -- status` via `$DORY_BIN`
//! against a **copied** real Flow tree, not an empty /tmp.
//!
//! Harness shape copied from `p5_inside.rs`. IDs parsed from snapshot /
//! `.result` / the slave's recent JSON. Never hardcode `w1`.
//!
//! The test process only: starts the server, `workspace create`s, then
//! `pane run`s commands into that first pane and `pane wait-output` /
//! `pane read`s JSON. It does **not** call `dory pane split`, `dory agent start`,
//! or `dory flow` as the primary driver.
//!
//! Slave sequence:
//! 1. `printenv DORY_ENV` (must print `1`)
//! 2. `"$DORY_BIN" pane split --current --direction right --no-focus`
//! 3. parse `.result.pane.id` from that pane's recent JSON
//! 4. `"$DORY_BIN" agent start <name> --pane <new> -- /bin/sh -c '<ready fixture>'`
//! 5. `"$DORY_BIN" agent prompt <name> --wait -- <text>` then `"$DORY_BIN" agent wait <name>`
//! 6. `"$DORY_BIN" flow -- status` with foreign `FLOW_BIN` and temp git
//!    `FLOW_PROJECT_ROOT` / cwd that holds a copy of `flow-skill/flow/`
//!
//! Judge is the foreign flow-skill `flow.sh`, not `eval/phase5-project` or
//! `/bin/true`. Do not cwd / `FLOW_PROJECT_ROOT` into `flow-skill` itself.
//! Journal `{project}/.dory/sessions/s1.jsonl` must contain `flow/invoke` +
//! `flow/result`. Preserve judge exit. No `:7380`. No `X-Dory-Inside`.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const READY_FIXTURE: &str = concat!(
    r#"printf '%s\n' DORY_OCC_READY; "#,
    r#"while IFS= read -r line; do "#,
    r#"printf 'got:%s\n' \"\$line\"; "#,
    r#"printf '%s\n' DORY_OCC_READY; "#,
    r#"done"#,
);
const AGENT: &str = "realrepo";
const REAL_PROMPT: &str = "P5_REAL_REPO_PROMPT_260822";
const START_TIMEOUT: &str = "8000";
const SETTLE_TIMEOUT: &str = "8000";
const FLOW_BIN: &str = "/home/manhquy/Downloads/flow/flow-skill/skills/flow/runner/flow.sh";
const FLOW_SIG: &str = "/home/manhquy/Downloads/flow/flow-skill/flow";
const FLOW_SKILL_ROOT: &str = "/home/manhquy/Downloads/flow/flow-skill";

const ENV_END: &str = "DORY_REAL_ENV_END";
const SPLIT_END: &str = "DORY_REAL_SPLIT_END";
const START_END: &str = "DORY_REAL_START_END";
const PROMPT_END: &str = "DORY_REAL_PROMPT_END";
const WAIT_END: &str = "DORY_REAL_WAIT_END";
const FLOW_END: &str = "DORY_REAL_FLOW_END";
const FLOW_EXIT_MARK: &str = "DORY_REAL_FLOW_EXIT:";

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

struct Scratch {
    path: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

struct Created {
    pane: String,
}

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_dory")
}

fn temp_dir(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!(
        "dory-p5-real-{tag}-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

fn scratch(tag: &str) -> Scratch {
    Scratch {
        path: temp_dir(tag),
    }
}

fn session_sock(xdg: &Path) -> PathBuf {
    xdg.join("dory").join("default").join("dory.sock")
}

fn start() -> Harness {
    let xdg = temp_dir("xdg");
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

/// Test-process CLI. Used for harness setup (`workspace create`) and for
/// `pane run` / `pane wait-output` / `pane read` only.
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

fn json_string_value(json: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":\"");
    let rest = json.split_once(&pat)?.1;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                other => out.push(other),
            }
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    None
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

fn rpc(h: &Harness, line: &str) -> String {
    let mut stream = UnixStream::connect(&h.sock).expect("rpc connect");
    writeln!(stream, "{line}").unwrap();
    let _ = stream.flush();
    let mut reply = String::new();
    BufReader::new(stream).read_line(&mut reply).unwrap();
    reply
}

fn snapshot(h: &Harness) -> String {
    rpc(h, r#"{"op":"snapshot"}"#)
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
    assert!(
        created.status.success(),
        "workspace create: {}",
        stderr(&created)
    );
    let body = stdout(&created);
    assert!(body.contains("\"ok\":true"), "{body}");
    let ws = result_id(&body, "workspace");
    let pane = result_id(&body, "root_pane");
    assert_ne!(ws, "w1", "next workspace must come from .result, not w1");
    assert_ne!(
        pane, snap_pane,
        "created pane must not be the snapshot pane"
    );
    assert!(
        occupant_name(&body).is_none(),
        "create must not start an occupant: {body}"
    );
    Created { pane }
}

fn assert_ok(out: &Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} exit={:?} stdout={} stderr={}",
        out.status.code(),
        stdout(out),
        stderr(out)
    );
    let body = stdout(out);
    assert!(body.contains("\"ok\":true"), "{what}: {body}");
}

fn pane_run(h: &Harness, pane: &str, text: &str) {
    let out = cli(h, pane, &["pane", "run", "--pane", pane, text]);
    assert_ok(&out, &format!("pane run {text}"));
}

fn pane_read_text(h: &Harness, pane: &str) -> String {
    let out = cli(
        h,
        pane,
        &["pane", "read", "--pane", pane, "--source", "recent"],
    );
    assert_ok(&out, "pane read");
    let body = stdout(&out);
    json_string_value(&body, "text").unwrap_or_else(|| panic!("missing text in {body}"))
}

/// Poll `pane read` until `needle` appears. Do not use blocking `wait-output`
/// while the slave still needs the single-threaded server for `$DORY_BIN`.
fn wait_slave_text(h: &Harness, pane: &str, needle: &str) -> String {
    let start = Instant::now();
    loop {
        let text = pane_read_text(h, pane);
        if text.contains(needle) {
            let already = cli(
                h,
                pane,
                &[
                    "pane",
                    "wait-output",
                    "--pane",
                    pane,
                    "--match",
                    needle,
                    "--timeout",
                    "2000",
                ],
            );
            assert!(
                already.status.success(),
                "wait-output already-present {needle}: stdout={} stderr={}",
                stdout(&already),
                stderr(&already)
            );
            return text;
        }
        if start.elapsed() >= Duration::from_secs(40) {
            panic!("timed out waiting for {needle} in slave recent: {text}");
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn assert_settled_in(text: &str, what: &str) {
    let state = json_field(text, "state").unwrap_or("");
    assert!(
        matches!(state, "idle" | "done" | "blocked"),
        "{what}: expected idle|done|blocked, got {state} in {text}"
    );
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
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

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap_or_else(|e| {
        panic!("mkdir {}: {e}", dst.display());
    });
    for entry in fs::read_dir(src).unwrap_or_else(|e| {
        panic!("read_dir {}: {e}", src.display());
    }) {
        let entry = entry.unwrap();
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let ft = entry.file_type().unwrap();
        if ft.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).unwrap_or_else(|e| {
                panic!("copy {} -> {}: {e}", from.display(), to.display());
            });
        }
    }
}

fn git_init(dir: &Path) {
    let out = Command::new("git")
        .args(["init"])
        .current_dir(dir)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .expect("git init");
    assert!(
        out.status.success(),
        "git init {}: stdout={} stderr={}",
        dir.display(),
        stdout(&out),
        stderr(&out)
    );
    assert!(
        dir.join(".git").exists(),
        "git init left no .git in {}",
        dir.display()
    );
}

fn real_flow_repo() -> Scratch {
    let src = Path::new(FLOW_SIG);
    assert!(
        src.join("00-idea.md").is_file(),
        "missing real Flow signature {}",
        src.join("00-idea.md").display()
    );
    let dest = scratch("project");
    let flow_dest = dest.path.join("flow");
    copy_tree(src, &flow_dest);
    git_init(&dest.path);
    dest
}

fn assert_real_signature(project: &Path) {
    let idea = project.join("flow").join("00-idea.md");
    assert!(
        idea.is_file(),
        "copied tree must contain real 00-idea.md at {}",
        idea.display()
    );
    let src = Path::new(FLOW_SIG).join("00-idea.md");
    let got = fs::read(&idea).expect("read copied 00-idea.md");
    let want = fs::read(&src).expect("read source 00-idea.md");
    assert_eq!(
        got, want,
        "copied 00-idea.md must match the real flow-skill signature"
    );
    assert!(
        project.join(".git").exists(),
        "project must be a git repo: {}",
        project.display()
    );
}

fn assert_foreign_project(project: &Path) {
    let canon_project = canon(project);
    let canon_crate = canon(&crate_dir());
    let canon_skill = canon(Path::new(FLOW_SKILL_ROOT));
    assert!(
        !canon_project.starts_with(&canon_crate),
        "FLOW_PROJECT_ROOT must not be the dory crate: {} under {}",
        canon_project.display(),
        canon_crate.display()
    );
    assert_ne!(
        canon_project, canon_skill,
        "must not cwd / FLOW_PROJECT_ROOT into flow-skill itself"
    );
    assert!(
        !canon_project.starts_with(&canon_skill),
        "FLOW_PROJECT_ROOT must not live under flow-skill: {}",
        canon_project.display()
    );
    let shown = canon_project.to_string_lossy();
    assert!(
        !shown.contains("eval/phase5-project"),
        "FLOW_PROJECT_ROOT must not be eval/phase5-project: {shown}"
    );
}

fn assert_flow_skill_untouched() {
    let skill = Path::new(FLOW_SKILL_ROOT);
    assert!(
        !skill.join(".dory").exists(),
        "flow-skill must not gain a .dory journal house"
    );
}

fn assert_foreign_judge() {
    assert_ne!(FLOW_BIN, "/bin/true", "judge must not be /bin/true");
    assert!(
        !FLOW_BIN.contains("eval/phase5-project"),
        "judge must not be eval/phase5-project: {FLOW_BIN}"
    );
    assert!(
        Path::new(FLOW_BIN).is_file(),
        "missing foreign FLOW_BIN {FLOW_BIN}"
    );
}

fn result_code(journal: &str) -> Option<i32> {
    let line = journal
        .lines()
        .find(|l| l.contains("\"type\":\"flow/result\""))?;
    let rest = line.split_once("\"code\":")?.1.trim_start();
    if rest.starts_with("null") {
        return None;
    }
    let tok: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    tok.parse().ok()
}

fn parse_flow_exit(text: &str) -> i32 {
    let rest = text
        .split(FLOW_EXIT_MARK)
        .nth(1)
        .unwrap_or_else(|| panic!("missing {FLOW_EXIT_MARK} in {text}"));
    let tok: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    tok.parse()
        .unwrap_or_else(|_| panic!("bad flow exit after {FLOW_EXIT_MARK} in {text}"))
}

/// Print `full` as two halves so `pane wait-output --match full` cannot hit the typed line.
fn printf_mark(full: &str) -> String {
    let mid = full.len() / 2;
    let (head, tail) = full.split_at(mid);
    format!("printf '%s%s\\n' {head} {tail}")
}

fn slave_env_cmd() -> String {
    format!(
        "printf 'DORY_ENV='; printenv DORY_ENV; {}",
        printf_mark(ENV_END)
    )
}

fn slave_split_cmd() -> String {
    format!(
        r#""$DORY_BIN" pane split --current --direction right --no-focus; {}"#,
        printf_mark(SPLIT_END)
    )
}

fn slave_start_cmd(new_pane: &str) -> String {
    format!(
        r#""$DORY_BIN" agent start {AGENT} --pane {new_pane} --timeout {START_TIMEOUT} -- /bin/sh -c "{READY_FIXTURE}"; {}"#,
        printf_mark(START_END)
    )
}

fn slave_prompt_cmd() -> String {
    format!(
        r#""$DORY_BIN" agent prompt {AGENT} --wait --timeout {SETTLE_TIMEOUT} -- {REAL_PROMPT}; {}"#,
        printf_mark(PROMPT_END)
    )
}

fn slave_wait_cmd() -> String {
    format!(
        r#""$DORY_BIN" agent wait {AGENT} --timeout {SETTLE_TIMEOUT}; {}"#,
        printf_mark(WAIT_END)
    )
}

fn slave_flow_cmd(project: &Path) -> String {
    let project_q = shell_single_quote(&project.to_string_lossy());
    let bin_q = shell_single_quote(FLOW_BIN);
    let (exit_head, exit_tail) = FLOW_EXIT_MARK.split_at(FLOW_EXIT_MARK.len() / 2);
    format!(
        "cd {project_q} && FLOW_BIN={bin_q} FLOW_PROJECT_ROOT={project_q} \
         \"$DORY_BIN\" flow -- status; \
         printf '%s%s%s\\n' {exit_head} {exit_tail} \"$?\"; \
         {}",
        printf_mark(FLOW_END)
    )
}

fn assert_printed_dory_env(text: &str) {
    assert!(
        text.contains("printenv DORY_ENV"),
        "slave must type printenv DORY_ENV: {text}"
    );
    assert!(
        text.contains("DORY_ENV=1"),
        "slave must print DORY_ENV=1: {text}"
    );
}

#[test]
fn p5_real_repo_slave_drives_flow_on_copied_git_tree() {
    assert_foreign_judge();
    assert_flow_skill_untouched();

    let project = real_flow_repo();
    assert_foreign_project(&project.path);
    assert_real_signature(&project.path);

    let h = start();
    let created = created_layout(&h);
    let first = created.pane.as_str();

    pane_run(&h, first, &slave_env_cmd());
    let env_text = wait_slave_text(&h, first, ENV_END);
    assert_printed_dory_env(&env_text);

    pane_run(&h, first, &slave_split_cmd());
    let split_text = wait_slave_text(&h, first, SPLIT_END);

    assert!(
        split_text.contains(r#""$DORY_BIN" pane split --current --direction right --no-focus"#),
        "slave must type the split verb: {split_text}"
    );
    let new_pane = nested_id(&split_text, "pane")
        .or_else(|| json_field(&split_text, "pane"))
        .unwrap_or_else(|| panic!("missing .result.pane.id in slave recent JSON: {split_text}"))
        .to_string();
    assert_ne!(new_pane, first, "split must return a new pane");
    assert_ne!(new_pane, "w1", "split pane must come from .result, not w1");
    assert!(
        occupant_name(&split_text).is_none(),
        "split must not start an occupant: {split_text}"
    );

    pane_run(&h, first, &slave_start_cmd(&new_pane));
    let start_text = wait_slave_text(&h, first, START_END);
    assert!(
        start_text.contains(&format!(
            r#""$DORY_BIN" agent start {AGENT} --pane {new_pane}"#
        )),
        "slave must type agent start: {start_text}"
    );
    let start_json = start_text.split(START_END).next().unwrap_or(&start_text);
    assert!(
        start_json.contains(&format!("\"name\":\"{AGENT}\"")),
        "start JSON missing occupant name: {start_text}"
    );

    pane_run(&h, first, &slave_prompt_cmd());
    let prompt_text = wait_slave_text(&h, first, PROMPT_END);

    assert!(
        prompt_text.contains(&format!(r#""$DORY_BIN" agent prompt {AGENT} --wait"#)),
        "slave must type agent prompt --wait: {prompt_text}"
    );
    let prompt_json = prompt_text
        .split(PROMPT_END)
        .next()
        .and_then(|s| s.rsplit_once(START_END).map(|(_, rest)| rest))
        .unwrap_or(&prompt_text);
    assert_settled_in(prompt_json, "prompt --wait");

    pane_run(&h, first, &slave_wait_cmd());
    let wait_text = wait_slave_text(&h, first, WAIT_END);
    assert!(
        wait_text.contains(&format!(r#""$DORY_BIN" agent wait {AGENT}"#)),
        "slave must type agent wait: {wait_text}"
    );
    let wait_json = wait_text
        .split(WAIT_END)
        .next()
        .and_then(|s| s.rsplit_once(PROMPT_END).map(|(_, rest)| rest))
        .unwrap_or(&wait_text);
    assert_settled_in(wait_json, "agent wait");

    pane_run(&h, first, &slave_flow_cmd(&project.path));
    let flow_text = wait_slave_text(&h, first, FLOW_END);
    assert!(
        flow_text.contains(r#""$DORY_BIN" flow -- status"#),
        "slave must type flow -- status: {flow_text}"
    );
    assert!(
        !flow_text.contains(":7380") && !flow_text.contains("X-Dory-Inside"),
        "inside loop must not use :7380 or X-Dory-Inside: {flow_text}"
    );
    let shown_project = project.path.to_string_lossy();
    assert!(
        !flow_text.contains(FLOW_SKILL_ROOT) || flow_text.contains(&*shown_project),
        "slave must cd the temp git copy, not flow-skill itself: {flow_text}"
    );
    let taxi_exit = parse_flow_exit(&flow_text);

    let types = journal_types(&project.path);
    let body = journal_body(&project.path);
    let preserved = result_code(&body).unwrap_or(1);
    assert_eq!(
        taxi_exit, preserved,
        "slave dory flow must preserve judge exit={preserved} journal={body} pane={flow_text}"
    );

    assert!(
        types.iter().any(|t| t == "flow/invoke"),
        "missing flow/invoke in {body}"
    );
    assert!(
        types.iter().any(|t| t == "flow/result"),
        "missing flow/result in {body}"
    );
    assert!(
        body.contains(&format!("\"bin\":\"{FLOW_BIN}\"")),
        "journal must record foreign FLOW_BIN: {body}"
    );
    assert!(
        body.contains("\"args\":[\"status\"]"),
        "journal must record status argv: {body}"
    );
    assert!(
        !body.contains("phase5-project") && !body.contains("/bin/true"),
        "journal must not record a fake judge: {body}"
    );
    assert!(
        !body.contains(":7380") && !body.contains("X-Dory-Inside"),
        "journal must not mention :7380 or X-Dory-Inside: {body}"
    );
    assert_flow_skill_untouched();
    assert!(
        !Path::new(FLOW_SKILL_ROOT).join(".dory").exists(),
        "journal must land in the temp copy, not flow-skill"
    );
    assert!(
        session_journal(&project.path).is_file(),
        "journal must be {{temp}}/.dory/sessions/s1.jsonl"
    );
}
