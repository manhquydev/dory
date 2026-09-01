//! `dory agent` occupant lifecycle. Layout stays a pane concern.
//!
//! Thin JSON client of the Unix socket. No `--kind`, no HTTP.

use crate::envelope;
use crate::pty;
use crate::{current, json_safe_id, print_rpc, require_skill_env};

const USAGE_START: &str =
    "dory: usage: dory agent start <name> [--pane <id> | --current] [--timeout MS] -- <argv>";
const USAGE_PROMPT: &str =
    "dory: usage: dory agent prompt [<name> | --current | --pane <id>] [--wait] [--timeout MS] [--] <text>";
const USAGE_WAIT: &str = "dory: usage: dory agent wait <name> [--until idle|done|blocked|working|unknown] [--timeout MS]";
const USAGE_GET: &str = "dory: usage: dory agent get <name>";
const USAGE_READ: &str =
    "dory: usage: dory agent read <name> [--source visible|recent|recent-unwrapped]";
const USAGE_FOCUS: &str = "dory: usage: dory agent focus <name>";
const USAGE_KEYS: &str = "dory: usage: dory agent send-keys <name> <key>";
const USAGE_REPORT: &str =
    "dory: usage: dory agent report [--current | --pane <id>] --state working|blocked|idle";

pub fn cmd(args: &[String]) -> i32 {
    match args.get(1).map(String::as_str) {
        None | Some("--help") | Some("-h") => {
            eprintln!(
                "dory: usage: dory agent <start|prompt|wait|get|read|focus|send-keys|report> ..."
            );
            2
        }
        Some("start") => start_cmd(args),
        Some("prompt") => prompt_cmd(args),
        Some("wait") => wait_cmd(args),
        Some("get") => get_cmd(args),
        Some("read") => read_cmd(args),
        Some("focus") => focus_cmd(args),
        Some("send-keys") => send_keys_cmd(args),
        Some("report") => report_cmd(args),
        Some(other) => {
            eprintln!("dory: unknown agent subcommand '{other}'");
            2
        }
    }
}

pub(crate) fn valid_occupant_name(name: &str) -> bool {
    let b = name.as_bytes();
    if b.is_empty() || b.len() > 32 {
        return false;
    }
    if !b[0].is_ascii_lowercase() {
        return false;
    }
    b.iter()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_' || *c == b'-')
}

fn start_cmd(args: &[String]) -> i32 {
    if args
        .iter()
        .any(|a| a == "--kind" || a.starts_with("--kind="))
    {
        eprintln!("{USAGE_START}");
        return 2;
    }

    let mut name: Option<&str> = None;
    let mut pane: Option<&str> = None;
    let mut current = false;
    let mut timeout: Option<u64> = None;
    let mut i = 2;
    let mut dash = None;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--" {
            dash = Some(i);
            break;
        }
        if a == "--pane" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_START}");
                return 2;
            };
            pane = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--pane=") {
            pane = Some(v);
            i += 1;
            continue;
        }
        if a == "--timeout" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_START}");
                return 2;
            };
            timeout = match v.parse() {
                Ok(ms) => Some(ms),
                Err(_) => {
                    eprintln!("{USAGE_START}");
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
                    eprintln!("{USAGE_START}");
                    return 2;
                }
            };
            i += 1;
            continue;
        }
        if a == "--current" {
            if current {
                eprintln!("{USAGE_START}");
                return 2;
            }
            current = true;
            i += 1;
            continue;
        }
        if a.starts_with("--") {
            eprintln!("dory: unknown agent start flag '{a}'");
            return 2;
        }
        if name.is_some() {
            eprintln!("{USAGE_START}");
            return 2;
        }
        name = Some(a);
        i += 1;
    }

    let Some(dash) = dash else {
        eprintln!("{USAGE_START}");
        return 2;
    };
    let argv = &args[dash + 1..];
    if argv.is_empty() {
        eprintln!("{USAGE_START}");
        return 2;
    }
    let Some(name) = name.filter(|n| valid_occupant_name(n)) else {
        eprintln!("{USAGE_START}");
        return 2;
    };
    let pane = match (pane, current) {
        (Some(id), false) => match json_safe_id(id) {
            Some(id) => id.to_string(),
            None => {
                eprintln!("{USAGE_START}");
                return 2;
            }
        },
        (None, true) => match current::pane_from_current_flag(args) {
            Ok(id) => match json_safe_id(&id) {
                Some(_) => id,
                None => {
                    eprintln!("{}", envelope::runtime_error("invalid pane id"));
                    return 1;
                }
            },
            Err(err) => {
                match err {
                    current::TargetError::OutsideEnv => {
                        eprintln!(
                            "{}",
                            envelope::runtime_error("I am not running inside a Dory-managed pane")
                        );
                    }
                    current::TargetError::OmitTarget => eprintln!("{USAGE_START}"),
                }
                return current::exit_code(err);
            }
        },
        _ => {
            eprintln!("{USAGE_START}");
            return 2;
        }
    };
    if let Err(err) = pty::refuse_spawn_argv(argv) {
        eprintln!("{}", envelope::runtime_error(&err.to_string()));
        return 1;
    }
    if let Err(code) = require_skill_env() {
        return code;
    }

    let mut line = format!(
        r#"{{"op":"agent.start","name":{name},"pane":"{pane}","argv":{argv}"#,
        name = envelope::json_string(name),
        argv = json_string_array(argv),
    );
    if let Some(ms) = timeout {
        line.push_str(&format!(r#","timeout":{ms}"#));
    }
    line.push('}');
    print_rpc(&line)
}

fn prompt_cmd(args: &[String]) -> i32 {
    if args
        .iter()
        .any(|a| a == "--kind" || a.starts_with("--kind="))
    {
        eprintln!("{USAGE_PROMPT}");
        return 2;
    }

    let mut name: Option<&str> = None;
    let mut pane: Option<&str> = None;
    let mut current = false;
    let mut wait = false;
    let mut timeout: Option<u64> = None;
    let mut text_parts: Vec<&str> = Vec::new();
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--" {
            text_parts.extend(args[i + 1..].iter().map(String::as_str));
            break;
        }
        if a == "--wait" {
            wait = true;
            i += 1;
            continue;
        }
        if a == "--timeout" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_PROMPT}");
                return 2;
            };
            timeout = match v.parse() {
                Ok(ms) => Some(ms),
                Err(_) => {
                    eprintln!("{USAGE_PROMPT}");
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
                    eprintln!("{USAGE_PROMPT}");
                    return 2;
                }
            };
            i += 1;
            continue;
        }
        if a == "--pane" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_PROMPT}");
                return 2;
            };
            pane = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--pane=") {
            pane = Some(v);
            i += 1;
            continue;
        }
        if a == "--current" {
            if current {
                eprintln!("{USAGE_PROMPT}");
                return 2;
            }
            current = true;
            i += 1;
            continue;
        }
        if a.starts_with("--") {
            eprintln!("dory: unknown agent prompt flag '{a}'");
            return 2;
        }
        if name.is_none() {
            name = Some(a);
        } else {
            text_parts.push(a);
        }
        i += 1;
    }
    if text_parts.is_empty() {
        eprintln!("{USAGE_PROMPT}");
        return 2;
    }
    let text = text_parts.join(" ");
    let wait_json = if wait { "true" } else { "false" };

    match (name, pane, current) {
        (Some(n), None, false) => {
            if !json_safe_token(n) {
                eprintln!("{USAGE_PROMPT}");
                return 2;
            }
            if let Err(code) = require_skill_env() {
                return code;
            }
            let mut line = format!(
                r#"{{"op":"agent.prompt","name":{},"text":{},"wait":{}"#,
                envelope::json_string(n),
                envelope::json_string(&text),
                wait_json,
            );
            if let Some(ms) = timeout {
                line.push_str(&format!(r#","timeout":{ms}"#));
            }
            line.push('}');
            print_rpc(&line)
        }
        (None, Some(id), false) => {
            let Some(id) = json_safe_id(id) else {
                eprintln!("{}", envelope::runtime_error("invalid pane id"));
                return 1;
            };
            if let Err(code) = require_skill_env() {
                return code;
            }
            let mut line = format!(
                r#"{{"op":"agent.prompt","pane":"{id}","text":{},"wait":{}"#,
                envelope::json_string(&text),
                wait_json,
            );
            if let Some(ms) = timeout {
                line.push_str(&format!(r#","timeout":{ms}"#));
            }
            line.push('}');
            print_rpc(&line)
        }
        (None, None, true) => {
            let pane = match current::pane_from_current_flag(args) {
                Ok(id) => match json_safe_id(&id) {
                    Some(_) => id,
                    None => {
                        eprintln!("{}", envelope::runtime_error("invalid pane id"));
                        return 1;
                    }
                },
                Err(err) => {
                    match err {
                        current::TargetError::OutsideEnv => {
                            eprintln!(
                                "{}",
                                envelope::runtime_error(
                                    "I am not running inside a Dory-managed pane"
                                )
                            );
                        }
                        current::TargetError::OmitTarget => eprintln!("{USAGE_PROMPT}"),
                    }
                    return current::exit_code(err);
                }
            };
            if let Err(code) = require_skill_env() {
                return code;
            }
            let mut line = format!(
                r#"{{"op":"agent.prompt","pane":"{pane}","text":{},"wait":{}"#,
                envelope::json_string(&text),
                wait_json,
            );
            if let Some(ms) = timeout {
                line.push_str(&format!(r#","timeout":{ms}"#));
            }
            line.push('}');
            print_rpc(&line)
        }
        _ => {
            eprintln!("{USAGE_PROMPT}");
            2
        }
    }
}

fn wait_cmd(args: &[String]) -> i32 {
    let mut name: Option<&str> = None;
    let mut until: Option<&str> = None;
    let mut timeout: Option<u64> = None;
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--until" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_WAIT}");
                return 2;
            };
            until = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--until=") {
            until = Some(v);
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
        if a.starts_with("--") {
            eprintln!("dory: unknown agent wait flag '{a}'");
            return 2;
        }
        if name.is_some() {
            eprintln!("{USAGE_WAIT}");
            return 2;
        }
        name = Some(a);
        i += 1;
    }
    let Some(name) = name.filter(|n| json_safe_token(n)) else {
        eprintln!("{USAGE_WAIT}");
        return 2;
    };
    if let Some(u) = until {
        if !matches!(u, "idle" | "done" | "blocked" | "working" | "unknown") {
            eprintln!("{USAGE_WAIT}");
            return 2;
        }
    }
    if let Err(code) = require_skill_env() {
        return code;
    }
    let mut line = format!(
        r#"{{"op":"agent.wait","name":{}"#,
        envelope::json_string(name)
    );
    if let Some(u) = until {
        line.push_str(&format!(r#","until":"{u}""#));
    }
    if let Some(ms) = timeout {
        line.push_str(&format!(r#","timeout":{ms}"#));
    }
    line.push('}');
    print_rpc(&line)
}

fn get_cmd(args: &[String]) -> i32 {
    let Some(name) = args
        .get(2)
        .map(String::as_str)
        .filter(|n| json_safe_token(n))
    else {
        eprintln!("{USAGE_GET}");
        return 2;
    };
    if args.len() != 3 {
        eprintln!("{USAGE_GET}");
        return 2;
    }
    print_rpc(&format!(
        r#"{{"op":"agent.get","name":{}}}"#,
        envelope::json_string(name)
    ))
}

fn read_cmd(args: &[String]) -> i32 {
    let mut name: Option<&str> = None;
    let mut source = "recent";
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
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
        if a.starts_with("--") {
            eprintln!("dory: unknown agent read flag '{a}'");
            return 2;
        }
        if name.is_some() {
            eprintln!("{USAGE_READ}");
            return 2;
        }
        name = Some(a);
        i += 1;
    }
    if source != "visible" && source != "recent" && source != "recent-unwrapped" {
        eprintln!("{USAGE_READ}");
        return 2;
    }
    let Some(name) = name.filter(|n| json_safe_token(n)) else {
        eprintln!("{USAGE_READ}");
        return 2;
    };
    print_rpc(&format!(
        r#"{{"op":"agent.read","name":{},"source":"{source}"}}"#,
        envelope::json_string(name)
    ))
}

fn focus_cmd(args: &[String]) -> i32 {
    let Some(name) = args
        .get(2)
        .map(String::as_str)
        .filter(|n| json_safe_token(n))
    else {
        eprintln!("{USAGE_FOCUS}");
        return 2;
    };
    if args.len() != 3 {
        eprintln!("{USAGE_FOCUS}");
        return 2;
    }
    if let Err(code) = require_skill_env() {
        return code;
    }
    print_rpc(&format!(
        r#"{{"op":"agent.focus","name":{}}}"#,
        envelope::json_string(name)
    ))
}

fn send_keys_cmd(args: &[String]) -> i32 {
    let Some(name) = args
        .get(2)
        .map(String::as_str)
        .filter(|n| json_safe_token(n))
    else {
        eprintln!("{USAGE_KEYS}");
        return 2;
    };
    let Some(key) = args.get(3).map(String::as_str) else {
        eprintln!("{USAGE_KEYS}");
        return 2;
    };
    if args.len() != 4 {
        eprintln!("{USAGE_KEYS}");
        return 2;
    }
    if !matches!(key, "enter" | "esc" | "ctrl+c") {
        eprintln!("{USAGE_KEYS}");
        return 2;
    }
    if let Err(code) = require_skill_env() {
        return code;
    }
    print_rpc(&format!(
        r#"{{"op":"agent.send-keys","name":{},"key":{}}}"#,
        envelope::json_string(name),
        envelope::json_string(key)
    ))
}

fn report_cmd(args: &[String]) -> i32 {
    if args
        .iter()
        .any(|a| a == "--kind" || a.starts_with("--kind="))
    {
        eprintln!("{USAGE_REPORT}");
        return 2;
    }

    let mut state: Option<&str> = None;
    let mut i = 2;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--current" {
            i += 1;
            continue;
        }
        if a == "--pane" {
            if args.get(i + 1).is_none() {
                eprintln!("{USAGE_REPORT}");
                return 2;
            }
            i += 2;
            continue;
        }
        if a.starts_with("--pane=") {
            i += 1;
            continue;
        }
        if a == "--state" {
            let Some(v) = args.get(i + 1).map(String::as_str) else {
                eprintln!("{USAGE_REPORT}");
                return 2;
            };
            state = Some(v);
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--state=") {
            state = Some(v);
            i += 1;
            continue;
        }
        if a.starts_with("--") {
            eprintln!("dory: unknown agent report flag '{a}'");
            return 2;
        }
        eprintln!("{USAGE_REPORT}");
        return 2;
    }
    let Some(state) = state.filter(|s| matches!(*s, "working" | "blocked" | "idle")) else {
        eprintln!("{USAGE_REPORT}");
        return 2;
    };

    let pane = match current::pane_from_current_flag(args) {
        Ok(id) => id,
        Err(err) => {
            match err {
                current::TargetError::OutsideEnv => {
                    eprintln!(
                        "{}",
                        envelope::runtime_error("I am not running inside a Dory-managed pane")
                    );
                }
                current::TargetError::OmitTarget => eprintln!("{USAGE_REPORT}"),
            }
            return current::exit_code(err);
        }
    };
    let Some(pane) = json_safe_id(&pane) else {
        eprintln!("{}", envelope::runtime_error("invalid pane id"));
        return 1;
    };
    if let Err(code) = require_skill_env() {
        return code;
    }
    print_rpc(&format!(
        r#"{{"op":"agent.report","pane":"{pane}","state":"{state}"}}"#
    ))
}

fn json_safe_token(s: &str) -> bool {
    !s.is_empty() && !s.bytes().any(|b| matches!(b, b'"' | b'\\' | b'\n' | b'\r'))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupant_name_charset() {
        assert!(valid_occupant_name("a"));
        assert!(valid_occupant_name("alice"));
        assert!(valid_occupant_name("a_b-1"));
        assert!(!valid_occupant_name(""));
        assert!(!valid_occupant_name("Alice"));
        assert!(!valid_occupant_name("1bad"));
        assert!(!valid_occupant_name("a@b"));
        assert!(!valid_occupant_name(&"a".repeat(33)));
        assert!(valid_occupant_name(&"a".repeat(32)));
    }

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn start_usage_names_current() {
        assert!(USAGE_START.contains("[--pane <id> | --current]"));
        assert!(!USAGE_START.contains("--kind"));
    }

    #[test]
    fn start_neither_both_kind_is_usage() {
        assert_eq!(cmd(&args(&["agent", "start", "alice", "--", "echo"])), 2);
        assert_eq!(
            cmd(&args(&[
                "agent",
                "start",
                "alice",
                "--pane",
                "w1:p1",
                "--current",
                "--",
                "echo"
            ])),
            2
        );
        assert_eq!(
            cmd(&args(&["agent", "start", "alice", "--kind", "--", "echo"])),
            2
        );
        assert_eq!(
            cmd(&args(&[
                "agent",
                "start",
                "alice",
                "--current",
                "--kind",
                "--",
                "echo"
            ])),
            2
        );
    }

    #[test]
    fn start_current_without_env_is_runtime_error() {
        assert_eq!(
            cmd(&args(&[
                "agent",
                "start",
                "alice",
                "--current",
                "--",
                "echo"
            ])),
            1
        );
    }

    #[test]
    fn prompt_usage_names_current() {
        assert!(USAGE_PROMPT.contains("[<name> | --current | --pane <id>]"));
        assert!(USAGE_PROMPT.contains("--current"));
        assert!(USAGE_PROMPT.contains("--pane"));
        assert!(!USAGE_PROMPT.contains("--kind"));
    }

    #[test]
    fn prompt_neither_both_kind_omit_text_is_usage() {
        assert_eq!(cmd(&args(&["agent", "prompt", "--", "hello"])), 2);
        assert_eq!(
            cmd(&args(&["agent", "prompt", "alice", "--current", "hello"])),
            2
        );
        assert_eq!(
            cmd(&args(&[
                "agent",
                "prompt",
                "--pane",
                "w1:p1",
                "--current",
                "hello"
            ])),
            2
        );
        assert_eq!(
            cmd(&args(&["agent", "prompt", "alice", "--kind", "hello"])),
            2
        );
        assert_eq!(cmd(&args(&["agent", "prompt", "alice"])), 2);
    }

    #[test]
    fn prompt_current_or_pane_without_env_is_runtime_error() {
        assert_eq!(
            cmd(&args(&["agent", "prompt", "--current", "--", "hello"])),
            1
        );
        assert_eq!(
            cmd(&args(&["agent", "prompt", "--pane", "w1:p1", "--", "hello"])),
            1
        );
    }

    #[test]
    fn prompt_named_wait_still_parses() {
        assert_eq!(
            cmd(&args(&[
                "agent",
                "prompt",
                "alice",
                "--wait",
                "--",
                "hello"
            ])),
            1
        );
    }
}
