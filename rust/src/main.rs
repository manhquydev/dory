mod agent;
mod attach;
mod current;
mod desk;
mod envelope;
mod flow;
mod ids;
mod layout;
mod pty;
mod server;
mod socket;

use std::env;
use std::process;

const USAGE: &str = "\
dory — workplace skill mux

Usage:
  dory
  dory attach [--pane <id>] [--plain]
  dory --help
  dory server
  dory server stop
  dory workspace create
  dory workspace list
  dory workspace get <id>
  dory workspace close <id>
  dory tab create --workspace <id>
  dory tab list --workspace <id>
  dory tab close <id>
  dory pane list --workspace <id>
  dory pane close [--current | --pane <id>]
  dory pane get [--current | --pane <id>]
  dory pane split [--current | --pane <id>] [--direction right|down] [--no-focus]
  dory pane run [--current | --pane <id>] <text>
  dory pane read [--current | --pane <id>] [--source visible|recent|recent-unwrapped]
  dory pane wait-output [--current | --pane <id>] [--match LIT | --regex RE] [--timeout MS]
  dory pane resize [--current | --pane <id>] --cols N --rows N
  dory pane focus [--current | --pane <id>]
  dory agent start <name> --pane <id> [--timeout MS] -- <argv>
  dory agent prompt <name> [--wait] [--timeout MS] [--] <text>
  dory agent wait <name> [--until idle|done|blocked|working|unknown] [--timeout MS]
  dory agent get <name>
  dory agent read <name> [--source visible|recent|recent-unwrapped]
  dory agent focus <name>
  dory agent send-keys <name> <key>
  dory agent report [--current | --pane <id>] --state working|blocked|idle
  dory flow -- <args>

Mutating workspace/tab/pane/agent/flow verbs require DORY_ENV=1.
Bare `dory` opens the desk (sidebar + tiled live panes).
Sit-down shells use $SHELL with rc (`DORY_SIT_SHELL=1`). `dory server` in tests stays
`bash --norc --noprofile`. New tabs follow the focused pane cwd; desk new workspace
uses the directory where you typed `dory`. `dory server stop` then `dory` if an old
daemon still has a bare bash-5.2 pane.
Desk prefix is Ctrl-b: q/d detach; w workspace picker; Shift-n new workspace;
n/p and 1-9 tabs in this workspace; hjkl panes; x close pane; Shift-x close tab;
Shift-d close workspace; z zoom (streams stay); b sidebar; ? help; drag>=2 copy.
`dory attach --plain` n/p still walk panes, not tabs. Occupants use CLI, not the desk.
`dory attach --plain` is the raw PTY client.
The Node `dory serve` lamp is not this binary.
";

fn dispatch(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{USAGE}");
        return 0;
    }
    if args.is_empty() {
        return desk::run();
    }
    match args[0].as_str() {
        "attach" => attach::run(args),
        "serve" => {
            eprintln!("dory: 'serve' is the Node journal lamp, not this binary");
            2
        }
        "server" => match args.get(1).map(String::as_str) {
            None => server::run_foreground(),
            Some("stop") => server::stop(),
            Some(other) => {
                eprintln!("dory: unknown server subcommand '{other}'");
                2
            }
        },
        "workspace" => workspace_cmd(args),
        "tab" => tab_cmd(args),
        "pane" => pane_cmd(args),
        "agent" => agent::cmd(args),

        "flow" => flow::cmd(args),
        other => {
            eprintln!("dory: unknown command '{other}'");
            eprintln!("{USAGE}");
            2
        }
    }
}

fn workspace_cmd(args: &[String]) -> i32 {
    match args.get(1).map(String::as_str) {
        None | Some("--help") | Some("-h") => {
            print!("{USAGE}");
            2
        }
        Some("create") => {
            if args.len() != 2 {
                eprintln!("dory: usage: dory workspace create");
                return 2;
            }
            if let Err(code) = require_skill_env() {
                return code;
            }
            print_rpc(r#"{"op":"workspace.create"}"#)
        }
        Some("list") => {
            if args.len() != 2 {
                eprintln!("dory: usage: dory workspace list");
                return 2;
            }
            print_rpc(r#"{"op":"workspace.list"}"#)
        }
        Some("get") => {
            let Some(id) = args.get(2).map(String::as_str).and_then(json_safe_id) else {
                eprintln!("dory: usage: dory workspace get <id>");
                return 2;
            };
            if args.len() != 3 {
                eprintln!("dory: usage: dory workspace get <id>");
                return 2;
            }
            print_rpc(&format!(r#"{{"op":"workspace.get","workspace":"{id}"}}"#))
        }
        Some("close") => {
            let Some(id) = args.get(2).map(String::as_str).and_then(json_safe_id) else {
                eprintln!("dory: usage: dory workspace close <id>");
                return 2;
            };
            if args.len() != 3 {
                eprintln!("dory: usage: dory workspace close <id>");
                return 2;
            }
            if let Err(code) = require_skill_env() {
                return code;
            }
            print_rpc(&format!(r#"{{"op":"workspace.close","workspace":"{id}"}}"#))
        }
        Some(other) => {
            eprintln!("dory: unknown workspace subcommand '{other}'");
            2
        }
    }
}

fn tab_cmd(args: &[String]) -> i32 {
    match args.get(1).map(String::as_str) {
        None | Some("--help") | Some("-h") => {
            print!("{USAGE}");
            2
        }
        Some("create") => {
            let Some(id) = flag_value(args, "--workspace").and_then(json_safe_id) else {
                eprintln!("dory: usage: dory tab create --workspace <id>");
                return 2;
            };
            if let Err(code) = require_skill_env() {
                return code;
            }
            print_rpc(&format!(r#"{{"op":"tab.create","workspace":"{id}"}}"#))
        }
        Some("list") => {
            let Some(id) = flag_value(args, "--workspace").and_then(json_safe_id) else {
                eprintln!("dory: usage: dory tab list --workspace <id>");
                return 2;
            };
            print_rpc(&format!(r#"{{"op":"tab.list","workspace":"{id}"}}"#))
        }
        Some("close") => {
            let Some(id) = args.get(2).map(String::as_str).and_then(json_safe_id) else {
                eprintln!("dory: usage: dory tab close <id>");
                return 2;
            };
            if args.len() != 3 {
                eprintln!("dory: usage: dory tab close <id>");
                return 2;
            }
            if let Err(code) = require_skill_env() {
                return code;
            }
            print_rpc(&format!(r#"{{"op":"tab.close","tab":"{id}"}}"#))
        }
        Some(other) => {
            eprintln!("dory: unknown tab subcommand '{other}'");
            2
        }
    }
}

fn pane_cmd(args: &[String]) -> i32 {
    match args.get(1).map(String::as_str) {
        None | Some("--help") | Some("-h") => {
            print!("{USAGE}");
            2
        }
        Some("list") => pane_list_cmd(args),
        Some("get") => pane_get_cmd(args),
        Some("close") => pane_close_cmd(args),
        Some("split") => pane_split_cmd(args),
        Some("run") => pane_run_cmd(args),
        Some("read") => pane_read_cmd(args),
        Some("wait-output") => pane_wait_output_cmd(args),
        Some("resize") => pane_resize_cmd(args),
        Some("focus") => pane_focus_cmd(args),
        Some(other) => {
            eprintln!("dory: unknown pane subcommand '{other}'");
            2
        }
    }
}

fn pane_list_cmd(args: &[String]) -> i32 {
    let Some(id) = flag_value(args, "--workspace").and_then(json_safe_id) else {
        eprintln!("dory: usage: dory pane list --workspace <id>");
        return 2;
    };
    print_rpc(&format!(r#"{{"op":"pane.list","workspace":"{id}"}}"#))
}

fn pane_close_cmd(args: &[String]) -> i32 {
    const USAGE_CLOSE: &str = "dory: usage: dory pane close [--current | --pane <id>]";
    let target = match pane_target(args, USAGE_CLOSE) {
        Ok(id) => id,
        Err(code) => return code,
    };
    if let Err(code) = require_skill_env() {
        return code;
    }
    print_rpc(&format!(r#"{{"op":"pane.close","pane":"{target}"}}"#))
}

fn pane_get_cmd(args: &[String]) -> i32 {
    const USAGE_GET: &str = "dory: usage: dory pane get [--current | --pane <id>]";
    let target = match pane_target(args, USAGE_GET) {
        Ok(id) => id,
        Err(code) => return code,
    };
    print_rpc(&format!(r#"{{"op":"pane.get","pane":"{target}"}}"#))
}

fn pane_split_cmd(args: &[String]) -> i32 {
    const USAGE_SPLIT: &str = "dory: usage: dory pane split [--current | --pane <id>] [--direction right|down] [--no-focus]";
    let mut direction: Option<&str> = None;
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--current" || a == "--no-focus" {
            i += 1;
            continue;
        }
        if a == "--direction" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_SPLIT}");
                return 2;
            };
            direction = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--direction=") {
            direction = Some(v);
            i += 1;
            continue;
        }
        if a == "--pane" {
            if args.get(i + 1).is_none() {
                eprintln!("{USAGE_SPLIT}");
                return 2;
            }
            i += 2;
            continue;
        }
        if a.starts_with("--pane=") {
            i += 1;
            continue;
        }
        eprintln!("dory: unknown pane split flag '{a}'");
        return 2;
    }
    if let Some(d) = direction {
        if d != "right" && d != "down" {
            eprintln!("{USAGE_SPLIT}");
            return 2;
        }
    }

    let target = match pane_target(args, USAGE_SPLIT) {
        Ok(id) => id,
        Err(code) => return code,
    };
    if let Err(code) = require_skill_env() {
        return code;
    }

    let mut line = format!(r#"{{"op":"pane.split","pane":"{target}","no_focus":true"#);
    if let Some(d) = direction {
        line.push_str(&format!(r#","direction":"{d}""#));
    }
    line.push('}');
    print_rpc(&line)
}

fn pane_run_cmd(args: &[String]) -> i32 {
    const USAGE_RUN: &str = "dory: usage: dory pane run [--current | --pane <id>] <text>";
    let mut text_parts: Vec<&str> = Vec::new();
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--current" {
            i += 1;
            continue;
        }
        if a == "--pane" {
            if args.get(i + 1).is_none() {
                eprintln!("{USAGE_RUN}");
                return 2;
            }
            i += 2;
            continue;
        }
        if a.starts_with("--pane=") {
            i += 1;
            continue;
        }
        if a.starts_with("--") {
            eprintln!("dory: unknown pane run flag '{a}'");
            return 2;
        }
        text_parts.push(a);
        i += 1;
    }
    if text_parts.is_empty() {
        eprintln!("{USAGE_RUN}");
        return 2;
    }
    let target = match pane_target(args, USAGE_RUN) {
        Ok(id) => id,
        Err(code) => return code,
    };
    if let Err(code) = require_skill_env() {
        return code;
    }
    let text = text_parts.join(" ");
    print_rpc(&format!(
        r#"{{"op":"pane.write","pane":"{target}","text":{}}}"#,
        envelope::json_string(&text)
    ))
}

fn pane_read_cmd(args: &[String]) -> i32 {
    const USAGE_READ: &str = "dory: usage: dory pane read [--current | --pane <id>] [--source visible|recent|recent-unwrapped]";
    let mut source = "recent";
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--current" {
            i += 1;
            continue;
        }
        if a == "--pane" {
            if args.get(i + 1).is_none() {
                eprintln!("{USAGE_READ}");
                return 2;
            }
            i += 2;
            continue;
        }
        if a.starts_with("--pane=") {
            i += 1;
            continue;
        }
        if a == "--source" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_READ}");
                return 2;
            };
            source = v;
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--source=") {
            source = v;
            i += 1;
            continue;
        }
        eprintln!("dory: unknown pane read flag '{a}'");
        return 2;
    }
    if source != "visible" && source != "recent" && source != "recent-unwrapped" {
        eprintln!("{USAGE_READ}");
        return 2;
    }
    let target = match pane_target(args, USAGE_READ) {
        Ok(id) => id,
        Err(code) => return code,
    };
    print_rpc(&format!(
        r#"{{"op":"pane.read","pane":"{target}","source":"{source}"}}"#
    ))
}

fn pane_wait_output_cmd(args: &[String]) -> i32 {
    const USAGE_WAIT: &str = "dory: usage: dory pane wait-output [--current | --pane <id>] [--match LIT | --regex RE] [--timeout MS]";
    let mut lit: Option<&str> = None;
    let mut regex: Option<&str> = None;
    let mut timeout: Option<u64> = None;
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--current" {
            i += 1;
            continue;
        }
        if a == "--pane" {
            if args.get(i + 1).is_none() {
                eprintln!("{USAGE_WAIT}");
                return 2;
            }
            i += 2;
            continue;
        }
        if a.starts_with("--pane=") {
            i += 1;
            continue;
        }
        if a == "--match" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_WAIT}");
                return 2;
            };
            lit = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--match=") {
            lit = Some(v);
            i += 1;
            continue;
        }
        if a == "--regex" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_WAIT}");
                return 2;
            };
            regex = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--regex=") {
            regex = Some(v);
            i += 1;
            continue;
        }
        if a == "--timeout" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_WAIT}");
                return 2;
            };
            timeout = match v.parse() {
                Ok(ms) => Some(ms),
                Err(_) => {
                    eprintln!("{USAGE_WAIT}");
                    return 2;
                }
            };
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--timeout=") {
            timeout = match v.parse() {
                Ok(ms) => Some(ms),
                Err(_) => {
                    eprintln!("{USAGE_WAIT}");
                    return 2;
                }
            };
            i += 1;
            continue;
        }
        eprintln!("dory: unknown pane wait-output flag '{a}'");
        return 2;
    }
    let lit = lit.filter(|s| !s.is_empty());
    let regex = regex.filter(|s| !s.is_empty());
    match (lit, regex) {
        (Some(_), Some(_)) | (None, None) => {
            eprintln!("{USAGE_WAIT}");
            return 2;
        }
        _ => {}
    }
    let target = match pane_target(args, USAGE_WAIT) {
        Ok(id) => id,
        Err(code) => return code,
    };
    if let Err(code) = require_skill_env() {
        return code;
    }
    let timeout = timeout.unwrap_or(5000);
    let mut line = format!(r#"{{"op":"pane.wait","pane":"{target}","timeout":{timeout}"#);
    if let Some(m) = lit {
        line.push_str(&format!(r#","match":{}"#, envelope::json_string(m)));
    }
    if let Some(r) = regex {
        line.push_str(&format!(r#","regex":{}"#, envelope::json_string(r)));
    }
    line.push('}');
    print_rpc(&line)
}

fn pane_resize_cmd(args: &[String]) -> i32 {
    const USAGE_RESIZE: &str =
        "dory: usage: dory pane resize [--current | --pane <id>] --cols N --rows N";
    let mut cols: Option<u16> = None;
    let mut rows: Option<u16> = None;
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--current" {
            i += 1;
            continue;
        }
        if a == "--pane" {
            if args.get(i + 1).is_none() {
                eprintln!("{USAGE_RESIZE}");
                return 2;
            }
            i += 2;
            continue;
        }
        if a.starts_with("--pane=") {
            i += 1;
            continue;
        }
        if a == "--cols" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_RESIZE}");
                return 2;
            };
            cols = match v.parse() {
                Ok(n) => Some(n),
                Err(_) => {
                    eprintln!("{USAGE_RESIZE}");
                    return 2;
                }
            };
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--cols=") {
            cols = match v.parse() {
                Ok(n) => Some(n),
                Err(_) => {
                    eprintln!("{USAGE_RESIZE}");
                    return 2;
                }
            };
            i += 1;
            continue;
        }
        if a == "--rows" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_RESIZE}");
                return 2;
            };
            rows = match v.parse() {
                Ok(n) => Some(n),
                Err(_) => {
                    eprintln!("{USAGE_RESIZE}");
                    return 2;
                }
            };
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--rows=") {
            rows = match v.parse() {
                Ok(n) => Some(n),
                Err(_) => {
                    eprintln!("{USAGE_RESIZE}");
                    return 2;
                }
            };
            i += 1;
            continue;
        }
        eprintln!("dory: unknown pane resize flag '{a}'");
        return 2;
    }
    let (Some(cols), Some(rows)) = (cols, rows) else {
        eprintln!("{USAGE_RESIZE}");
        return 2;
    };
    let target = match pane_target(args, USAGE_RESIZE) {
        Ok(id) => id,
        Err(code) => return code,
    };
    if let Err(code) = require_skill_env() {
        return code;
    }
    print_rpc(&format!(
        r#"{{"op":"pane.resize","pane":"{target}","cols":{cols},"rows":{rows}}}"#
    ))
}

fn pane_focus_cmd(args: &[String]) -> i32 {
    const USAGE_FOCUS: &str = "dory: usage: dory pane focus [--current | --pane <id>]";
    let target = match pane_target(args, USAGE_FOCUS) {
        Ok(id) => id,
        Err(code) => return code,
    };
    if let Err(code) = require_skill_env() {
        return code;
    }
    print_rpc(&format!(r#"{{"op":"pane.focus","pane":"{target}"}}"#))
}

fn pane_target(args: &[String], usage: &str) -> Result<String, i32> {
    let id = match current::pane_from_current_flag(args) {
        Ok(id) => id,
        Err(err) => {
            match err {
                current::TargetError::OutsideEnv => {
                    eprintln!(
                        "{}",
                        envelope::runtime_error("I am not running inside a Dory-managed pane")
                    );
                }
                current::TargetError::OmitTarget => eprintln!("{usage}"),
            }
            return Err(current::exit_code(err));
        }
    };
    match json_safe_id(&id) {
        Some(_) => Ok(id),
        None => {
            eprintln!("{}", envelope::runtime_error("invalid pane id"));
            Err(1)
        }
    }
}

pub(crate) fn require_skill_env() -> Result<(), i32> {
    if env::var_os("DORY_ENV").as_deref() == Some(std::ffi::OsStr::new("1")) {
        return Ok(());
    }
    eprintln!(
        "{}",
        envelope::runtime_error("I am not running inside a Dory-managed pane")
    );
    Err(1)
}

pub(crate) fn print_rpc(line: &str) -> i32 {
    match server::rpc_line(line) {
        Ok(reply) => {
            let ok = reply.contains("\"ok\":true");
            if ok {
                print!("{reply}");
                if !reply.ends_with('\n') {
                    println!();
                }
                0
            } else {
                eprint!("{reply}");
                if !reply.ends_with('\n') {
                    eprintln!();
                }
                1
            }
        }
        Err(code) => code,
    }
}

pub(crate) fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    let mut i = 2;
    while i < args.len() {
        if args[i] == flag {
            return args.get(i + 1).map(String::as_str);
        }
        if let Some(rest) = args[i].strip_prefix(&format!("{flag}=")) {
            return Some(rest);
        }
        i += 1;
    }
    None
}

pub(crate) fn json_safe_id(id: &str) -> Option<&str> {
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

fn main() {
    process::exit(dispatch(&env::args().skip(1).collect::<Vec<_>>()));
}

#[cfg(test)]
mod tests {
    use super::dispatch;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn help_exits_zero() {
        assert_eq!(dispatch(&args(&["--help"])), 0);
    }

    #[test]
    fn group_stubs_exit_two() {
        for g in ["workspace", "tab", "pane", "agent", "flow"] {
            assert_eq!(dispatch(&args(&[g])), 2, "{g}");
        }
    }

    #[test]
    fn serve_is_not_this_binary() {
        assert_eq!(dispatch(&args(&["serve"])), 2);
    }

    #[test]
    fn workspace_create_without_env_is_runtime_error() {
        assert_eq!(dispatch(&args(&["workspace", "create"])), 1);
        assert_eq!(dispatch(&args(&["workspace", "close", "w1"])), 1);
        assert_eq!(dispatch(&args(&["pane", "close", "--pane", "w1:p1"])), 1);
        assert_eq!(dispatch(&args(&["tab", "create", "--workspace", "w2"])), 1);
        assert_eq!(dispatch(&args(&["pane", "split", "--current"])), 1);
        assert_eq!(dispatch(&args(&["pane", "split", "--pane", "w1:p1"])), 1);
        assert_eq!(
            dispatch(&args(&["pane", "run", "--pane", "w1:p1", "echo hi"])),
            1
        );
        assert_eq!(
            dispatch(&args(&[
                "pane",
                "wait-output",
                "--pane",
                "w1:p1",
                "--match",
                "x"
            ])),
            1
        );
        assert_eq!(dispatch(&args(&["flow", "--"])), 1);
        assert_eq!(dispatch(&args(&["flow", "--", "status"])), 1);
        assert_eq!(dispatch(&args(&["pane", "get", "--current"])), 1);
    }

    #[test]
    fn pane_split_omit_target_is_usage() {
        assert_eq!(dispatch(&args(&["pane", "split"])), 2);
        assert_eq!(
            dispatch(&args(&["pane", "split", "--direction", "right"])),
            2
        );
        assert_eq!(dispatch(&args(&["pane", "run", "echo hi"])), 2);
        assert_eq!(dispatch(&args(&["pane", "read"])), 2);
        assert_eq!(dispatch(&args(&["pane", "wait-output", "--match", "x"])), 2);
        assert_eq!(dispatch(&args(&["pane", "get"])), 2);
        assert_eq!(dispatch(&args(&["pane", "close"])), 2);
        assert_eq!(dispatch(&args(&["workspace", "close"])), 2);
        assert_eq!(dispatch(&args(&["tab", "list"])), 2);
        assert_eq!(dispatch(&args(&["pane", "list"])), 2);
        assert_eq!(
            dispatch(&args(&["pane", "resize", "--cols", "80", "--rows", "24"])),
            2
        );
        assert_eq!(dispatch(&args(&["pane", "focus"])), 2);
    }

    #[test]
    fn usage_names_groups_not_http_workplace() {
        assert!(super::USAGE.contains("dory attach"));
        assert!(super::USAGE.contains("Bare `dory` opens the desk"));
        assert!(super::USAGE.contains("tiled live panes"));
        assert!(super::USAGE.contains("--plain"));
        assert!(super::USAGE.contains("dory workspace"));
        assert!(super::USAGE.contains("dory tab list --workspace"));
        assert!(super::USAGE.contains("dory pane list --workspace"));
        assert!(super::USAGE.contains("dory pane get [--current | --pane <id>]"));
        assert!(super::USAGE.contains("dory pane run"));
        assert!(super::USAGE.contains("dory pane read"));
        assert!(super::USAGE.contains("dory pane wait-output"));
        assert!(super::USAGE.contains(
            "dory pane resize [--current | --pane <id>] --cols N --rows N"
        ));
        assert!(super::USAGE.contains("dory pane focus [--current | --pane <id>]"));
        assert!(super::USAGE.contains("dory pane"));
        assert!(super::USAGE.contains("dory pane close"));
        assert!(super::USAGE.contains("dory workspace close"));
        assert!(super::USAGE.contains("workspace picker"));
        assert!(super::USAGE.contains("Ctrl-b"));
        assert!(super::USAGE.contains("n/p still walk panes"));
        assert!(super::USAGE.contains("dory flow --"));
        assert!(super::USAGE.contains("dory agent start"));
        assert!(
            !super::USAGE.contains("Group agent is a stub"),
            "agent is occupant wait, not a stub"
        );
        assert!(
            !super::USAGE.contains("Groups agent/flow are stubs"),
            "flow is a taxi, not a stub"
        );
        assert!(!super::USAGE.contains("/workplace"));
        assert!(!super::USAGE.contains("Starts the server if needed"));
        assert!(!super::USAGE.contains("X-Dory-Inside"));
        assert!(!super::USAGE.contains("--kind"));
        assert!(!super::USAGE.contains(":7380"));
    }
}
