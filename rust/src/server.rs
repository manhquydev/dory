use crate::envelope;
use crate::ids::Ids;
use crate::pty::{self, HeldPty, Occupancy};

use crate::socket::{self, SessionPaths};
use std::env;
use std::fs;
use std::ffi::OsString;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_SESSION: &str = "default";

struct Occupant {
    name: String,
    seen: bool,
    ready: bool,
    word: OccupantWord,
    argv0: String,
    classified: bool,
    scan_from: usize,
    report: Option<OccupantWord>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OccupantWord {
    Unknown,
    Working,
    Blocked,
    Idle,
    Done,
}

impl OccupantWord {
    fn as_str(self) -> &'static str {
        match self {
            OccupantWord::Unknown => "unknown",
            OccupantWord::Working => "working",
            OccupantWord::Blocked => "blocked",
            OccupantWord::Idle => "idle",
            OccupantWord::Done => "done",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "unknown" => Some(Self::Unknown),
            "working" => Some(Self::Working),
            "blocked" => Some(Self::Blocked),
            "idle" => Some(Self::Idle),
            "done" => Some(Self::Done),
            _ => None,
        }
    }
}

struct Pane {
    id: String,
    occupant: Option<Occupant>,
    held: HeldPty,
}


struct Tab {
    id: String,
    root_pane: String,
    panes: Vec<Pane>,
}

struct Workspace {
    id: String,
    tabs: Vec<Tab>,
}

struct World {
    ids: Ids,
    workspaces: Vec<Workspace>,
    socket: PathBuf,
    bin: PathBuf,
    cwd: PathBuf,
    focused: String,
}

pub fn run_foreground() -> i32 {
    match serve_session(DEFAULT_SESSION) {
        Ok(()) => 0,
        Err(code) => code,
    }
}

pub fn stop() -> i32 {
    match send_op(DEFAULT_SESSION, "stop") {
        Ok(reply) => {
            print!("{reply}");
            if !reply.contains("\"live\":false") {
                return 1;
            }
            0
        }
        Err(code) => code,
    }
}

/// One-shot newline JSON against `DORY_SOCKET`, else the default session sock.
pub fn rpc_line(line: &str) -> Result<String, i32> {
    let mut stream = connect_control()?;
    writeln!(stream, "{line}").map_err(|err| {
        eprintln!("dory: {err}");
        1
    })?;
    let _ = stream.flush();
    read_reply(stream)
}

fn serve_session(session: &str) -> Result<(), i32> {
    let paths = session_paths_or_exit(session)?;
    let listener = match socket::prepare_bind(&paths) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("{err}");
            return Err(match err {
                socket::Error::MissingRuntimeDir | socket::Error::NestedServer => 2,
                socket::Error::Io(_) => 1,
            });
        }
    };

    let cwd = env::current_dir().map_err(|err| {
        eprintln!("dory: {err}");
        1
    })?;
    let bin = env::current_exe().map_err(|err| {
        eprintln!("dory: {err}");
        1
    })?;
    let mut world = World {
        ids: Ids::new(),
        workspaces: Vec::new(),
        socket: paths.sock.clone(),
        bin,
        cwd,
        focused: String::new(),
    };
    create_workspace(&mut world).map_err(|err| {
        eprintln!("{err}");
        1
    })?;

    serve_loop(listener, &mut world)
}

fn serve_loop(listener: UnixListener, world: &mut World) -> Result<(), i32> {
    loop {
        let (stream, _) = match listener.accept() {
            Ok(pair) => pair,
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => {
                eprintln!("dory: {err}");
                kill_all(world);
                return Err(1);
            }
        };
        match handle_client(stream, world) {
            ClientAction::Continue => {}
            ClientAction::Stop => return Ok(()),
        }
    }
}

enum ClientAction {
    Continue,
    Stop,
}

fn handle_client(stream: UnixStream, world: &mut World) -> ClientAction {
    match socket::peer_same_uid(&stream) {
        Ok(true) => {}
        Ok(false) | Err(_) => return ClientAction::Continue,
    }

    let writer = match stream.try_clone() {
        Ok(w) => w,
        Err(_) => return ClientAction::Continue,
    };
    let mut writer = writer;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => return ClientAction::Continue,
            Ok(_) => {}
            Err(_) => return ClientAction::Continue,
        }
        if line.trim().is_empty() {
            continue;
        }

        match dispatch_line(world, &line) {
            LineReply::Stop(image) => {
                let _ = writeln!(writer, "{image}");
                let _ = writer.flush();
                return ClientAction::Stop;
            }
            LineReply::Msg(reply) => {
                if writeln!(writer, "{reply}").is_err() {
                    return ClientAction::Continue;
                }
                let _ = writer.flush();
            }
        }
    }
}

enum LineReply {
    Msg(String),
    Stop(String),
}

fn dispatch_line(world: &mut World, line: &str) -> LineReply {
    match parse_op(line) {
        Some("ping") => LineReply::Msg("{\"ok\":true}".to_string()),
        Some("snapshot") => LineReply::Msg(live_snapshot(world)),
        Some("stop") => {
            let pid = first_pid(world);
            kill_all(world);
            LineReply::Stop(dead_snapshot(pid))
        }
        Some("workspace.create") => LineReply::Msg(match create_workspace(world) {
            Ok((ws, tab, pane)) => {
                let mut result = envelope::result_workspace(&ws, &tab, &pane);
                result.pop();
                result.push_str(",\"occupant\":null}");
                envelope::success(&result)
            }
            Err(err) => envelope::runtime_error(&err),
        }),
        Some("workspace.list") => LineReply::Msg(envelope::success(&list_workspaces(world))),
        Some("workspace.get") => LineReply::Msg(match json_str_field(line, "workspace") {
            Some(id) => get_workspace(world, id),
            None => envelope::runtime_error("workspace id required"),
        }),
        Some("tab.create") => LineReply::Msg(match json_str_field(line, "workspace") {
            Some(id) => match create_tab(world, id) {
                Ok((tab, pane)) => envelope::success(&result_tab(&tab, &pane)),
                Err(err) => envelope::runtime_error(&err),
            },
            None => envelope::runtime_error("workspace id required"),
        }),
        Some("tab.close") => LineReply::Msg(match json_str_field(line, "tab") {
            Some(id) => close_tab(world, id),
            None => envelope::runtime_error("tab id required"),
        }),
        Some("tab.list") => LineReply::Msg(match json_str_field(line, "workspace") {
            Some(id) => list_tabs(world, id),
            None => envelope::runtime_error("workspace id required"),
        }),
        Some("pane.split") => LineReply::Msg(match json_str_field(line, "pane") {
            Some(id) => split_pane(
                world,
                id,
                json_str_field(line, "direction"),
                json_bool_field(line, "no_focus").unwrap_or(true),
            ),
            None => envelope::runtime_error("pane id required"),
        }),
        Some("pane.resize") => LineReply::Msg(match json_str_field(line, "pane") {
            Some(id) => match (json_u16_field(line, "cols"), json_u16_field(line, "rows")) {
                (Some(cols), Some(rows)) => resize_pane(world, id, cols, rows),
                _ => envelope::runtime_error("cols and rows required"),
            },
            None => envelope::runtime_error("pane id required"),
        }),
        Some("pane.write") => LineReply::Msg(match json_str_field(line, "pane") {
            Some(id) => match json_decoded_str_field(line, "text") {
                Some(text) => write_pane(world, id, &text),
                None => envelope::runtime_error("text required"),
            },
            None => envelope::runtime_error("pane id required"),
        }),
        Some("pane.get") => LineReply::Msg(match json_str_field(line, "pane") {
            Some(id) => get_pane(world, id),
            None => envelope::runtime_error("pane id required"),
        }),
        Some("pane.list") => LineReply::Msg(match json_str_field(line, "workspace") {
            Some(id) => list_panes(world, id),
            None => envelope::runtime_error("workspace id required"),
        }),
        Some("pane.read") => LineReply::Msg(match json_str_field(line, "pane") {
            Some(id) => read_pane(
                world,
                id,
                json_str_field(line, "source").unwrap_or("recent"),
            ),
            None => envelope::runtime_error("pane id required"),
        }),
        Some("pane.wait") => LineReply::Msg(match json_str_field(line, "pane") {
            Some(id) => wait_pane(
                world,
                id,
                json_decoded_str_field(line, "match"),
                json_decoded_str_field(line, "regex"),
                json_u64_field(line, "timeout").unwrap_or(5000),
            ),
            None => envelope::runtime_error("pane id required"),
        }),
        Some("agent.start") => LineReply::Msg(agent_start(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
            json_str_array_field(line, "argv"),
            json_u64_field(line, "timeout"),
        )),
        Some("agent.prompt") => LineReply::Msg(agent_prompt(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
            json_decoded_str_field(line, "text"),
            json_bool_field(line, "wait").unwrap_or(false),
            json_u64_field(line, "timeout"),
        )),
        Some("agent.wait") => LineReply::Msg(agent_wait(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
            json_str_field(line, "until"),
            json_u64_field(line, "timeout"),
        )),
        Some("agent.get") => LineReply::Msg(agent_get(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
        )),
        Some("agent.read") => LineReply::Msg(agent_read(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
            json_str_field(line, "source").unwrap_or("recent"),
        )),
        Some("agent.focus") => LineReply::Msg(agent_focus(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
        )),
        Some("agent.send-keys") => LineReply::Msg(agent_send_keys(
            world,
            json_str_field(line, "name"),
            json_str_field(line, "pane"),
            json_str_field(line, "key"),
        )),
        Some("agent.report") => LineReply::Msg(agent_report(
            world,
            json_str_field(line, "pane"),
            json_str_field(line, "state"),
        )),
        _ => LineReply::Msg(envelope::runtime_error("unknown op")),

    }
}

fn create_workspace(world: &mut World) -> Result<(String, String, String), String> {
    let ws = world.ids.workspace().map_err(|e| e.to_string())?;
    let tab = world.ids.tab(&ws).map_err(|e| e.to_string())?;
    let pane = world.ids.pane(&ws).map_err(|e| e.to_string())?;
    let cwd = world.cwd.clone();
    let held = spawn_root(world, &ws, &tab, &pane, &cwd)?;
    world.workspaces.push(Workspace {
        id: ws.clone(),
        tabs: vec![Tab {
            id: tab.clone(),
            root_pane: pane.clone(),
            panes: vec![Pane {
                id: pane.clone(),
                occupant: None,
                held,
            }],
        }],
    });
    if world.focused.is_empty() {
        world.focused = pane.clone();
    }
    Ok((ws, tab, pane))
}

fn create_tab(world: &mut World, workspace_id: &str) -> Result<(String, String), String> {
    let idx = world
        .workspaces
        .iter()
        .position(|w| w.id == workspace_id)
        .ok_or_else(|| format!("unknown workspace {workspace_id}"))?;
    let tab = world
        .ids
        .tab(workspace_id)
        .map_err(|e| e.to_string())?;
    let pane = world
        .ids
        .pane(workspace_id)
        .map_err(|e| e.to_string())?;
    let cwd = world.cwd.clone();
    let held = spawn_root(world, workspace_id, &tab, &pane, &cwd)?;
    world.workspaces[idx].tabs.push(Tab {
        id: tab.clone(),
        root_pane: pane.clone(),
        panes: vec![Pane {
            id: pane.clone(),
            occupant: None,
            held,
        }],
    });
    Ok((tab, pane))
}

fn close_tab(world: &mut World, tab_id: &str) -> String {
    let mut found = None;
    for (wi, ws) in world.workspaces.iter().enumerate() {
        if let Some(ti) = ws.tabs.iter().position(|t| t.id == tab_id) {
            found = Some((wi, ti));
            break;
        }
    }
    let Some((wi, ti)) = found else {
        return envelope::runtime_error(&format!("unknown tab {tab_id}"));
    };
    let mut tab = world.workspaces[wi].tabs.remove(ti);
    for pane in &mut tab.panes {
        let _ = pane.held.kill_group();
        world.ids.retire(&pane.id);
    }
    world.ids.retire(&tab.id);
    retarget_focus(world);
    envelope::success(&format!(
        "{{\"tab\":{{\"id\":\"{}\"}},\"retired\":true}}",
        tab.id
    ))
}

fn get_workspace(world: &World, id: &str) -> String {
    match world.workspaces.iter().find(|w| w.id == id) {
        Some(ws) => envelope::success(&workspace_object(ws)),
        None => envelope::runtime_error(&format!("unknown workspace {id}")),
    }
}

fn list_workspaces(world: &World) -> String {
    let mut out = String::from("{\"workspaces\":[");
    for (i, ws) in world.workspaces.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&workspace_object(ws));
    }
    out.push_str("]}");
    out
}

fn list_tabs(world: &World, workspace_id: &str) -> String {
    match world.workspaces.iter().find(|w| w.id == workspace_id) {
        Some(ws) => {
            let mut out = String::from("{\"tabs\":[");
            for (i, tab) in ws.tabs.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&format!(
                    "{{\"id\":\"{}\",\"occupant\":{}}}",
                    tab.id,
                    tab.panes
                        .first()
                        .map(pane_occupant_json)
                        .unwrap_or_else(|| "null".to_string())
                ));
            }
            out.push_str("]}");
            envelope::success(&out)
        }
        None => envelope::runtime_error(&format!("unknown workspace {workspace_id}")),
    }
}

fn list_panes(world: &World, workspace_id: &str) -> String {
    match world.workspaces.iter().find(|w| w.id == workspace_id) {
        Some(ws) => {
            let mut out = String::from("{\"panes\":[");
            let mut first = true;
            for tab in &ws.tabs {
                for pane in &tab.panes {
                    if !first {
                        out.push(',');
                    }
                    first = false;
                    out.push_str(&format!(
                        "{{\"id\":\"{}\",\"occupant\":{}}}",
                        pane.id,
                        pane_occupant_json(pane)
                    ));
                }
            }
            out.push_str("]}");
            envelope::success(&out)
        }
        None => envelope::runtime_error(&format!("unknown workspace {workspace_id}")),
    }
}

fn workspace_object(ws: &Workspace) -> String {
    let mut out = format!("{{\"workspace\":{{\"id\":\"{}\"}},\"tabs\":[", ws.id);
    for (i, tab) in ws.tabs.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"id\":\"{}\",\"root_pane\":{{\"id\":\"{}\"}},\"occupant\":{}}}",
            tab.id,
            tab.root_pane,
            tab.panes
                .first()
                .map(pane_occupant_json)
                .unwrap_or_else(|| "null".to_string())
        ));

    }
    out.push_str("]}");
    out
}

fn result_tab(tab_id: &str, pane_id: &str) -> String {
    format!(
        "{{\"tab\":{{\"id\":\"{tab_id}\"}},\"root_pane\":{{\"id\":\"{pane_id}\"}},\"occupant\":null}}"
    )
}

fn pane_occupant_json(pane: &Pane) -> String {
    match pane.occupant.as_ref() {
        None => "null".to_string(),
        Some(occ) => {
            let (word, _, _) = classify_word(pane);
            format!(
                "{{\"name\":{},\"state\":{},\"seen\":{}}}",
                envelope::json_string(&occ.name),
                envelope::json_string(word.as_str()),
                if occ.seen { "true" } else { "false" }
            )
        }
    }
}


struct PaneLoc {
    wi: usize,
    ti: usize,
    pi: usize,
}

fn locate_pane(world: &World, pane_id: &str) -> Option<PaneLoc> {
    for (wi, ws) in world.workspaces.iter().enumerate() {
        for (ti, tab) in ws.tabs.iter().enumerate() {
            if let Some(pi) = tab.panes.iter().position(|p| p.id == pane_id) {
                return Some(PaneLoc { wi, ti, pi });
            }
        }
    }
    None
}

fn split_direction(cols: u16, rows: u16, requested: Option<&str>) -> Result<&'static str, String> {
    match requested {
        Some("right") => Ok("right"),
        Some("down") => Ok("down"),
        Some(other) => Err(format!("unknown direction {other}")),
        None if cols > rows => Ok("right"),
        None => Ok("down"),
    }
}

fn proc_cwd(pid: u32, fallback: &Path) -> PathBuf {
    fs::read_link(format!("/proc/{pid}/cwd")).unwrap_or_else(|_| fallback.to_path_buf())
}

fn retarget_focus(world: &mut World) {
    if locate_pane(world, &world.focused).is_some() {
        return;
    }
    world.focused = world
        .workspaces
        .iter()
        .flat_map(|w| w.tabs.iter())
        .flat_map(|t| t.panes.iter())
        .map(|p| p.id.clone())
        .next()
        .unwrap_or_default();
}

fn split_pane(
    world: &mut World,
    pane_id: &str,
    direction: Option<&str>,
    no_focus: bool,
) -> String {
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    let (size, caller_pid, workspace_id, tab_id) = {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        let size = match pane.held.size() {
            Ok(s) => s,
            Err(err) => return envelope::runtime_error(&err.to_string()),
        };
        (
            size,
            pane.held.child_pid(),
            world.workspaces[loc.wi].id.clone(),
            world.workspaces[loc.wi].tabs[loc.ti].id.clone(),
        )
    };
    let dir = match split_direction(size.0, size.1, direction) {
        Ok(dir) => dir,
        Err(err) => return envelope::runtime_error(&err),
    };
    let cwd = proc_cwd(caller_pid, &world.cwd);
    let new_id = match world.ids.pane(&workspace_id) {
        Ok(id) => id,
        Err(err) => return envelope::runtime_error(&err.to_string()),
    };
    let held = match spawn_root(world, &workspace_id, &tab_id, &new_id, &cwd) {
        Ok(held) => held,
        Err(err) => return envelope::runtime_error(&err),
    };
    world.workspaces[loc.wi].tabs[loc.ti].panes.push(Pane {
        id: new_id.clone(),
        occupant: None,
        held,
    });
    if !no_focus {
        world.focused = new_id.clone();
    }
    envelope::success(&format!(
        "{{\"pane\":{{\"id\":\"{new_id}\"}},\"direction\":\"{dir}\",\"occupant\":null}}"
    ))
}

fn resize_pane(world: &World, pane_id: &str, cols: u16, rows: u16) -> String {
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    match pane.held.resize(cols, rows) {
        Ok(()) => envelope::success(&format!(
            "{{\"pane\":{{\"id\":\"{pane_id}\"}},\"cols\":{cols},\"rows\":{rows}}}"
        )),
        Err(err) => envelope::runtime_error(&err.to_string()),
    }
}

fn write_pane(world: &World, pane_id: &str, text: &str) -> String {
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    let mut bytes = text.as_bytes().to_vec();
    if !bytes.ends_with(&[b'\n']) {
        bytes.push(b'\n');
    }

    match pane.held.write_all(&bytes) {
        Ok(()) => envelope::success(&format!("{{\"pane\":{{\"id\":\"{pane_id}\"}}}}")),
        Err(err) => envelope::runtime_error(&err.to_string()),
    }
}

fn get_pane(world: &World, pane_id: &str) -> String {
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    envelope::success(&format!(
        "{{\"pane\":{{\"id\":\"{}\"}},\"pid\":{},\"occupant\":{}}}",
        pane.id,
        pane.held.child_pid(),
        pane_occupant_json(pane)


    ))
}

const BLOCKED_NEEDLE: &str = "DORY_OCC_BLOCKED";
const READY_NEEDLE: &str = "DORY_OCC_READY";
const DEFAULT_AGENT_TIMEOUT_MS: u64 = 5000;
const PROMPT_STALL_MS: u64 = 5000;

fn argv0_comm(argv0: &str) -> &str {
    Path::new(argv0)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(argv0)
}

fn comm_allowlisted(comm: &str) -> bool {
    matches!(comm, "sleep" | "cat" | "sh" | "bash" | "true" | "false")
}

fn comm_noninteractive(comm: &str) -> bool {
    matches!(comm, "sleep" | "true" | "false")
}

fn shell_quote(arg: &str) -> String {
    if arg.is_empty() {
        return "''".to_string();
    }
    if arg
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'/' | b':' | b'='))
    {
        return arg.to_string();
    }
    let mut out = String::from("'");
    for c in arg.chars() {
        if c == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(c);
        }
    }
    out.push('\'');
    out
}

fn shell_join(argv: &[String]) -> String {
    argv.iter().map(|a| shell_quote(a)).collect::<Vec<_>>().join(" ")
}

fn classify_word(pane: &Pane) -> (OccupantWord, bool, bool) {
    let Some(occ) = pane.occupant.as_ref() else {
        return (OccupantWord::Unknown, false, false);
    };
    if let Some(reported) = occ.report {
        let word = if reported == OccupantWord::Idle && !occ.seen {
            OccupantWord::Done
        } else {
            reported
        };
        let ready = matches!(word, OccupantWord::Idle | OccupantWord::Done);
        return (word, ready, true);
    }
    let text = pane.held.recent_unwrapped();
    if text.contains(BLOCKED_NEEDLE) {
        return (OccupantWord::Blocked, false, occ.classified);
    }
    let comm = argv0_comm(&occ.argv0);
    let allow = comm_allowlisted(comm);
    let classified = allow
        && pty::descendant_comms(pane.held.child_pid())
            .iter()
            .any(|c| c == comm);
    if !classified {
        return (OccupantWord::Unknown, false, false);
    }
    if comm_noninteractive(comm) {
        return (OccupantWord::Working, false, true);
    }
    let slice = if occ.scan_from <= text.len() {
        &text[occ.scan_from..]
    } else {
        ""
    };
    let ready = slice.contains(READY_NEEDLE);
    if ready {
        let word = if occ.seen {
            OccupantWord::Idle
        } else {
            OccupantWord::Done
        };
        (word, true, true)
    } else {
        (OccupantWord::Working, false, true)
    }
}

fn refresh_occupant(pane: &mut Pane) {
    let (word, ready, classified) = classify_word(pane);
    if let Some(occ) = pane.occupant.as_mut() {
        occ.word = word;
        occ.ready = ready;
        occ.classified = classified;
    }
}



fn live_occupant_name(world: &World, name: &str) -> Option<PaneLoc> {
    for (wi, ws) in world.workspaces.iter().enumerate() {
        for (ti, tab) in ws.tabs.iter().enumerate() {
            for (pi, pane) in tab.panes.iter().enumerate() {
                if pane
                    .occupant
                    .as_ref()
                    .is_some_and(|o| o.name == name)
                {
                    return Some(PaneLoc { wi, ti, pi });
                }
            }
        }
    }
    None
}

fn locate_agent(
    world: &World,
    name: Option<&str>,
    pane: Option<&str>,
) -> Result<PaneLoc, String> {
    if let Some(n) = name {
        if let Some(loc) = live_occupant_name(world, n) {
            return Ok(loc);
        }
        if pane.is_none() {
            if let Some(loc) = locate_pane(world, n) {
                if world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]
                    .occupant
                    .is_some()
                {
                    return Ok(loc);
                }
            }
            return Err(format!("unknown agent {n}"));
        }
    }
    let Some(pane_id) = pane else {
        return Err("agent name or pane required".to_string());
    };
    let Some(loc) = locate_pane(world, pane_id) else {
        return Err(format!("unknown pane {pane_id}"));
    };
    if world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]
        .occupant
        .is_none()
    {
        return Err(format!("no occupant on {pane_id}"));
    }
    Ok(loc)
}

fn agent_snapshot(pane: &Pane) -> String {
    let occ = pane.occupant.as_ref().unwrap();
    let (word, _, _) = classify_word(pane);
    format!(
        "{{\"agent\":{{\"name\":{},\"pane\":{},\"state\":{},\"seen\":{}}}}}",
        envelope::json_string(&occ.name),
        envelope::json_string(&pane.id),
        envelope::json_string(word.as_str()),
        if occ.seen { "true" } else { "false" }
    )
}


fn agent_start(
    world: &mut World,
    name: Option<&str>,
    pane_id: Option<&str>,
    argv: Option<Vec<String>>,
    timeout_ms: Option<u64>,
) -> String {
    let Some(name) = name else {
        return envelope::runtime_error("name required");
    };
    if !crate::agent::valid_occupant_name(name) {
        return envelope::runtime_error("invalid occupant name");
    }
    let Some(pane_id) = pane_id else {
        return envelope::runtime_error("pane id required");
    };
    let Some(argv) = argv.filter(|a| !a.is_empty()) else {
        return envelope::runtime_error("argv required");
    };
    if let Err(err) = pty::refuse_spawn_argv(&argv) {
        return envelope::runtime_error(&err.to_string());
    }
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        if pane.occupant.is_some() {
            return envelope::runtime_error(&format!("pane busy {pane_id}"));
        }
    }
    if live_occupant_name(world, name).is_some() {
        return envelope::runtime_error(&format!("occupant name in use {name}"));
    }
    let argv0 = argv[0].clone();
    let line = shell_join(&argv);
    let mut bytes = line.into_bytes();
    bytes.push(b'\n');
    {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        if let Err(err) = pane.held.write_all(&bytes) {
            return envelope::runtime_error(&err.to_string());
        }
    }
    world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi].occupant = Some(Occupant {
        name: name.to_string(),
        seen: false,
        ready: false,
        word: OccupantWord::Unknown,
        argv0,
        classified: false,
        scan_from: 0,
        report: None,
    });
    let timeout = timeout_ms.unwrap_or(DEFAULT_AGENT_TIMEOUT_MS);
    let deadline = Instant::now() + Duration::from_millis(timeout);
    let comm = {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        argv0_comm(&pane.occupant.as_ref().unwrap().argv0).to_string()
    };
    if comm_allowlisted(&comm) {
        loop {
            let pane = &mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
            refresh_occupant(pane);
            if pane.occupant.as_ref().is_some_and(|o| o.classified) {
                break;
            }
            if Instant::now() >= deadline {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
    } else {
        refresh_occupant(&mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]);
    }
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    envelope::success(&agent_snapshot(pane))
}

fn live_bracketed_paste(drain: &str) -> bool {
    let on = drain.rfind("\x1b[?2004h");
    let off = drain.rfind("\x1b[?2004l");
    match (on, off) {
        (Some(h), Some(l)) => h > l,
        (Some(_), None) => true,
        _ => false,
    }
}

fn agent_prompt(
    world: &mut World,
    name: Option<&str>,
    pane_id: Option<&str>,
    text: Option<String>,
    wait: bool,
    timeout_ms: Option<u64>,
) -> String {
    let loc = match locate_agent(world, name, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    let Some(text) = text else {
        return envelope::runtime_error("text required");
    };
    refresh_occupant(&mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]);
    let before = {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        pane.occupant.as_ref().unwrap().word
    };
    if before == OccupantWord::Blocked {
        return envelope::runtime_error("agent_blocked");
    }
    let bytes = {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        if live_bracketed_paste(&pane.held.recent()) {
            let inner = text.trim_end_matches(['\n', '\r']);
            let mut out = Vec::with_capacity(inner.len() + 11);
            out.extend_from_slice(b"\x1b[200~");
            out.extend_from_slice(inner.as_bytes());
            out.extend_from_slice(b"\x1b[201~");
            // TUI compose treats LF after paste as a line, not submit.
            out.push(b'\r');
            out
        } else {
            let mut bytes = text.into_bytes();
            if !bytes.ends_with(&[b'\n']) {
                bytes.push(b'\n');
            }
            bytes
        }
    };
    {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        if let Err(err) = pane.held.write_all(&bytes) {
            return envelope::runtime_error(&err.to_string());
        }
    }
    {
        let pane = &mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        let mark = pane.held.recent_unwrapped().len();
        if let Some(occ) = pane.occupant.as_mut() {
            occ.scan_from = mark;
            occ.ready = false;
        }
        refresh_occupant(pane);
    }
    if matches!(before, OccupantWord::Idle | OccupantWord::Done) {
        let stall_at = Instant::now() + Duration::from_millis(PROMPT_STALL_MS);
        loop {
            refresh_occupant(&mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]);
            let now = world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]
                .occupant
                .as_ref()
                .unwrap()
                .word;
            if now != before {
                break;
            }
            if Instant::now() >= stall_at {
                return envelope::runtime_error("agent_prompt_stalled");
            }
            thread::sleep(Duration::from_millis(20));
        }
    }
    if wait {
        agent_wait_loop(world, loc, None, timeout_ms.unwrap_or(DEFAULT_AGENT_TIMEOUT_MS))
    } else {
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        envelope::success(&agent_snapshot(pane))
    }
}

fn wait_hit(word: OccupantWord, until: Option<OccupantWord>) -> bool {
    match until {
        Some(u) => word == u,
        None => matches!(
            word,
            OccupantWord::Idle | OccupantWord::Done | OccupantWord::Blocked
        ),
    }
}

fn agent_wait_loop(
    world: &mut World,
    loc: PaneLoc,
    until: Option<OccupantWord>,
    timeout_ms: u64,
) -> String {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        refresh_occupant(&mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]);
        let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        let word = pane.occupant.as_ref().unwrap().word;
        if wait_hit(word, until) {
            return envelope::success(&agent_snapshot(pane));
        }
        if Instant::now() >= deadline {
            return envelope::runtime_error("timeout");
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn agent_wait(
    world: &mut World,
    name: Option<&str>,
    pane_id: Option<&str>,
    until: Option<&str>,
    timeout_ms: Option<u64>,
) -> String {
    let loc = match locate_agent(world, name, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    let until = match until {
        None => None,
        Some(s) => match OccupantWord::parse(s) {
            Some(w) => Some(w),
            None => return envelope::runtime_error(&format!("unknown until {s}")),
        },
    };
    agent_wait_loop(
        world,
        loc,
        until,
        timeout_ms.unwrap_or(DEFAULT_AGENT_TIMEOUT_MS),
    )
}

fn agent_get(world: &mut World, name: Option<&str>, pane_id: Option<&str>) -> String {
    let loc = match locate_agent(world, name, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    refresh_occupant(&mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]);
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    envelope::success(&agent_snapshot(pane))
}

fn agent_read(
    world: &World,
    name: Option<&str>,
    pane_id: Option<&str>,
    source: &str,
) -> String {
    let loc = match locate_agent(world, name, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    let text = match source {
        "visible" => pane.held.visible(),
        "recent" => pane.held.recent(),
        "recent-unwrapped" => pane.held.recent_unwrapped(),
        other => return envelope::runtime_error(&format!("unknown source {other}")),
    };
    let mut result = agent_snapshot(pane);
    result.pop();
    result.push_str(&format!(
        ",\"source\":{},\"text\":{}}}",
        envelope::json_string(source),
        envelope::json_string(&text)
    ));
    envelope::success(&result)
}

fn agent_focus(world: &mut World, name: Option<&str>, pane_id: Option<&str>) -> String {
    let loc = match locate_agent(world, name, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    let pane_id = world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]
        .id
        .clone();
    world.focused = pane_id;
    {
        let pane = &mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        if let Some(occ) = pane.occupant.as_mut() {
            occ.seen = true;
        }
        refresh_occupant(pane);
    }
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    envelope::success(&agent_snapshot(pane))
}


fn agent_send_keys(
    world: &mut World,
    name: Option<&str>,
    pane_id: Option<&str>,
    key: Option<&str>,
) -> String {
    let loc = match locate_agent(world, name, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    let Some(key) = key else {
        return envelope::runtime_error("key required");
    };
    refresh_occupant(&mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]);
    if world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi]
        .occupant
        .as_ref()
        .is_some_and(|o| o.word == OccupantWord::Blocked)
    {
        return envelope::runtime_error("agent_blocked");
    }
    let bytes: &[u8] = match key {
        "enter" => b"\n",
        "esc" => b"\x1b",
        "ctrl+c" => b"\x03",
        other => return envelope::runtime_error(&format!("unknown key {other}")),
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    if let Err(err) = pane.held.write_all(bytes) {
        return envelope::runtime_error(&err.to_string());
    }
    envelope::success(&agent_snapshot(pane))
}

fn agent_report(world: &mut World, pane_id: Option<&str>, state: Option<&str>) -> String {
    let word = match state {
        Some("working") => OccupantWord::Working,
        Some("blocked") => OccupantWord::Blocked,
        Some("idle") => OccupantWord::Idle,
        Some(other) => return envelope::runtime_error(&format!("invalid report state {other}")),
        None => return envelope::runtime_error("state required"),
    };
    let loc = match locate_agent(world, None, pane_id) {
        Ok(loc) => loc,
        Err(err) => return envelope::runtime_error(&err),
    };
    {
        let pane = &mut world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
        if let Some(occ) = pane.occupant.as_mut() {
            occ.report = Some(word);
        }
        refresh_occupant(pane);
    }
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    envelope::success(&agent_snapshot(pane))
}


fn read_pane(world: &World, pane_id: &str, source: &str) -> String {
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    let text = match source {
        "visible" => pane.held.visible(),
        "recent" => pane.held.recent(),
        "recent-unwrapped" => pane.held.recent_unwrapped(),
        other => return envelope::runtime_error(&format!("unknown source {other}")),
    };
    envelope::success(&format!(
        "{{\"pane\":{{\"id\":{}}},\"source\":{},\"text\":{}}}",
        envelope::json_string(&pane.id),
        envelope::json_string(source),
        envelope::json_string(&text)
    ))
}

fn wait_pane(
    world: &World,
    pane_id: &str,
    needle: Option<String>,
    regex: Option<String>,
    timeout_ms: u64,
) -> String {
    let Some(loc) = locate_pane(world, pane_id) else {
        return envelope::runtime_error(&format!("unknown pane {pane_id}"));
    };
    if needle.is_none() && regex.is_none() {
        return envelope::runtime_error("match or regex required");
    }
    if needle.is_some() && regex.is_some() {
        return envelope::runtime_error("match and regex are exclusive");
    }
    let compiled = match regex.as_deref() {
        Some(pat) => match compile_regex(pat) {
            Ok(re) => Some(re),
            Err(err) => return envelope::runtime_error(&err),
        },
        None => None,
    };
    let pane = &world.workspaces[loc.wi].tabs[loc.ti].panes[loc.pi];
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        let text = pane.held.recent_unwrapped();
        let hit = if let Some(n) = needle.as_deref() {
            text.contains(n)
        } else if let Some(re) = compiled.as_ref() {
            regex_search(re, &text)
        } else {
            false
        };
        if hit {
            return envelope::success(&format!(
                "{{\"pane\":{{\"id\":{}}},\"matched\":true,\"text\":{}}}",
                envelope::json_string(pane_id),
                envelope::json_string(&text)
            ));
        }
        if Instant::now() >= deadline {
            return envelope::runtime_error("timeout");
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn spawn_root(
    world: &World,
    workspace: &str,
    tab: &str,
    pane: &str,
    cwd: &Path,
) -> Result<HeldPty, String> {
    let occ = Occupancy {
        socket: world.socket.clone(),
        bin: world.bin.clone(),
        workspace_id: workspace.to_string(),
        tab_id: tab.to_string(),
        pane_id: pane.to_string(),
    };
    HeldPty::spawn_occupied(cwd, &first_shell(), &occ).map_err(|e| e.to_string())
}

fn first_pid(world: &World) -> u32 {
    world
        .workspaces
        .first()
        .and_then(|w| w.tabs.first())
        .and_then(|t| t.panes.first())
        .map(|p| p.held.child_pid())
        .unwrap_or(0)
}

fn first_ids(world: &World) -> (&str, &str, &str) {
    let ws = world.workspaces.first();
    let tab = ws.and_then(|w| w.tabs.first());
    let pane = tab.and_then(|t| t.panes.first());
    (
        ws.map(|w| w.id.as_str()).unwrap_or(""),
        tab.map(|t| t.id.as_str()).unwrap_or(""),
        pane.map(|p| p.id.as_str()).unwrap_or(""),
    )
}

fn kill_all(world: &mut World) {
    for ws in &mut world.workspaces {
        for tab in &mut ws.tabs {
            for pane in &mut tab.panes {
                let _ = pane.held.kill_group();
            }
        }
    }
}

fn connect_control() -> Result<UnixStream, i32> {
    if let Some(path) = env::var_os("DORY_SOCKET").filter(|v| !v.is_empty()) {
        return UnixStream::connect(path).map_err(|err| {
            eprintln!("dory: {err}");
            1
        });
    }
    let paths = session_paths_or_exit(DEFAULT_SESSION)?;
    UnixStream::connect(&paths.sock).map_err(|err| {
        eprintln!("dory: {err}");
        1
    })
}

fn send_op(session: &str, op: &str) -> Result<String, i32> {
    let paths = session_paths_or_exit(session)?;
    let mut stream = UnixStream::connect(&paths.sock).map_err(|err| {
        eprintln!("dory: {err}");
        1
    })?;
    writeln!(stream, "{{\"op\":\"{op}\"}}").map_err(|err| {
        eprintln!("dory: {err}");
        1
    })?;
    let _ = stream.flush();
    read_reply(stream)
}

fn read_reply(stream: UnixStream) -> Result<String, i32> {
    let mut reply = String::new();
    BufReader::new(stream)
        .read_line(&mut reply)
        .map_err(|err| {
            eprintln!("dory: {err}");
            1
        })?;
    if reply.trim().is_empty() {
        eprintln!("dory: empty reply");
        return Err(1);
    }
    Ok(reply)
}

fn session_paths_or_exit(session: &str) -> Result<SessionPaths, i32> {
    match socket::session_paths(session) {
        Ok(paths) => Ok(paths),
        Err(err) => {
            eprintln!("{err}");
            Err(match err {
                socket::Error::MissingRuntimeDir => 2,
                socket::Error::NestedServer => 2,
                socket::Error::Io(_) => 1,
            })
        }
    }
}

fn first_shell() -> Vec<OsString> {
    if Path::new("/bin/bash").exists() {
        vec![
            OsString::from("/bin/bash"),
            OsString::from("--norc"),
            OsString::from("--noprofile"),
        ]
    } else {
        vec![OsString::from("/bin/sh")]
    }
}

fn json_str_field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{key}\"");
    let idx = line.find(&needle)?;
    let mut rest = line[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(&rest[..end])
}

fn json_bool_field(line: &str, key: &str) -> Option<bool> {
    let needle = format!("\"{key}\"");
    let idx = line.find(&needle)?;
    let mut rest = line[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

fn json_u16_field(line: &str, key: &str) -> Option<u16> {
    let needle = format!("\"{key}\"");
    let idx = line.find(&needle)?;
    let mut rest = line[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn json_u64_field(line: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\"");
    let idx = line.find(&needle)?;
    let mut rest = line[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn json_decoded_str_field(line: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let idx = line.find(&needle)?;
    let mut rest = line[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    rest = rest.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => match chars.next()? {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => {
                    let mut hex = String::new();
                    for _ in 0..4 {
                        hex.push(chars.next()?);
                    }
                    let n = u32::from_str_radix(&hex, 16).ok()?;
                    out.push(char::from_u32(n)?);
                }
                other => out.push(other),
            },
            c => out.push(c),
        }
    }
    None
}

fn json_str_array_field(line: &str, key: &str) -> Option<Vec<String>> {
    let needle = format!("\"{key}\"");
    let idx = line.find(&needle)?;
    let mut rest = line[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    rest = rest.strip_prefix('[')?;
    let mut out = Vec::new();
    loop {
        rest = rest.trim_start();
        if rest.starts_with(']') {
            return Some(out);
        }
        if rest.starts_with(',') {
            rest = rest[1..].trim_start();
            continue;
        }
        rest = rest.strip_prefix('"')?;
        let mut item = String::new();
        let mut chars = rest.chars();
        loop {
            let c = chars.next()?;
            match c {
                '"' => {
                    rest = chars.as_str();
                    out.push(item);
                    break;
                }
                '\\' => match chars.next()? {
                    '"' => item.push('"'),
                    '\\' => item.push('\\'),
                    '/' => item.push('/'),
                    'n' => item.push('\n'),
                    'r' => item.push('\r'),
                    't' => item.push('\t'),
                    'u' => {
                        let mut hex = String::new();
                        for _ in 0..4 {
                            hex.push(chars.next()?);
                        }
                        let n = u32::from_str_radix(&hex, 16).ok()?;
                        item.push(char::from_u32(n)?);
                    }
                    other => item.push(other),
                },
                c => item.push(c),
            }
        }
    }
}


#[derive(Clone)]
enum ReAtom {
    Any,
    Lit(char),
    Class { negated: bool, items: Vec<ReClass> },
}

#[derive(Clone)]
enum ReClass {
    One(char),
    Range(char, char),
}

#[derive(Clone)]
enum ReOp {
    Atom(ReAtom),
    Star(ReAtom),
    Plus(ReAtom),
    Ques(ReAtom),
}

struct CompiledRegex {
    start: bool,
    end: bool,
    ops: Vec<ReOp>,
}

fn compile_regex(pat: &str) -> Result<CompiledRegex, String> {
    let chars: Vec<char> = pat.chars().collect();
    let mut i = 0;
    let start = chars.first() == Some(&'^');
    if start {
        i = 1;
    }
    let end = chars.last() == Some(&'$') && chars.get(chars.len().saturating_sub(2)) != Some(&'\\');
    let last = if end { chars.len().saturating_sub(1) } else { chars.len() };
    let mut ops = Vec::new();
    while i < last {
        let (atom, next) = parse_re_atom(&chars, i, last)?;
        i = next;
        let op = match chars.get(i) {
            Some('*') => {
                i += 1;
                ReOp::Star(atom)
            }
            Some('+') => {
                i += 1;
                ReOp::Plus(atom)
            }
            Some('?') => {
                i += 1;
                ReOp::Ques(atom)
            }
            _ => ReOp::Atom(atom),
        };
        ops.push(op);
    }
    Ok(CompiledRegex { start, end, ops })
}

fn parse_re_atom(chars: &[char], i: usize, last: usize) -> Result<(ReAtom, usize), String> {
    let Some(c) = chars.get(i).copied() else {
        return Err("dangling quantifier".into());
    };
    match c {
        '*' | '+' | '?' => Err("dangling quantifier".into()),
        '|' | '(' | ')' => Err("unsupported regex syntax".into()),
        '.' => Ok((ReAtom::Any, i + 1)),
        '\\' => {
            let Some(n) = chars.get(i + 1).copied() else {
                return Err("dangling escape".into());
            };
            let lit = match n {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                other => other,
            };
            Ok((ReAtom::Lit(lit), i + 2))
        }
        '[' => parse_re_class(chars, i + 1, last),
        other => Ok((ReAtom::Lit(other), i + 1)),
    }
}

fn parse_re_class(chars: &[char], mut i: usize, last: usize) -> Result<(ReAtom, usize), String> {
    let negated = chars.get(i) == Some(&'^');
    if negated {
        i += 1;
    }
    let mut items = Vec::new();
    if chars.get(i) == Some(&']') {
        items.push(ReClass::One(']'));
        i += 1;
    }
    while i < last {
        let c = chars[i];
        if c == ']' {
            return Ok((ReAtom::Class { negated, items }, i + 1));
        }
        if c == '\\' {
            let Some(n) = chars.get(i + 1).copied() else {
                return Err("unclosed character class".into());
            };
            items.push(ReClass::One(n));
            i += 2;
            continue;
        }
        if chars.get(i + 1) == Some(&'-') && chars.get(i + 2).is_some_and(|n| *n != ']') {
            items.push(ReClass::Range(c, chars[i + 2]));
            i += 3;
            continue;
        }
        items.push(ReClass::One(c));
        i += 1;
    }
    Err("unclosed character class".into())
}

fn regex_search(re: &CompiledRegex, hay: &str) -> bool {
    let chars: Vec<char> = hay.chars().collect();
    if re.start {
        return regex_at(re, &chars, 0);
    }
    for pos in 0..=chars.len() {
        if regex_at(re, &chars, pos) {
            return true;
        }
    }
    false
}

fn regex_at(re: &CompiledRegex, hay: &[char], pos: usize) -> bool {
    fn walk(ops: &[ReOp], hay: &[char], pos: usize, must_end: bool) -> bool {
        if ops.is_empty() {
            return !must_end || pos == hay.len();
        }
        match &ops[0] {
            ReOp::Atom(atom) => {
                if pos < hay.len() && atom_hit(atom, hay[pos]) {
                    walk(&ops[1..], hay, pos + 1, must_end)
                } else {
                    false
                }
            }
            ReOp::Ques(atom) => {
                (pos < hay.len()
                    && atom_hit(atom, hay[pos])
                    && walk(&ops[1..], hay, pos + 1, must_end))
                    || walk(&ops[1..], hay, pos, must_end)
            }
            ReOp::Star(atom) => {
                let mut i = pos;
                loop {
                    if walk(&ops[1..], hay, i, must_end) {
                        return true;
                    }
                    if i >= hay.len() || !atom_hit(atom, hay[i]) {
                        return false;
                    }
                    i += 1;
                }
            }
            ReOp::Plus(atom) => {
                if pos >= hay.len() || !atom_hit(atom, hay[pos]) {
                    return false;
                }
                let mut i = pos + 1;
                loop {
                    if walk(&ops[1..], hay, i, must_end) {
                        return true;
                    }
                    if i >= hay.len() || !atom_hit(atom, hay[i]) {
                        return false;
                    }
                    i += 1;
                }
            }
        }
    }
    walk(&re.ops, hay, pos, re.end)
}

fn atom_hit(atom: &ReAtom, c: char) -> bool {
    match atom {
        ReAtom::Any => c != '\n',
        ReAtom::Lit(lit) => *lit == c,
        ReAtom::Class { negated, items } => {
            let hit = items.iter().any(|item| match item {
                ReClass::One(x) => *x == c,
                ReClass::Range(a, b) => *a <= c && c <= *b,
            });
            if *negated { !hit } else { hit }
        }
    }
}


fn parse_op(line: &str) -> Option<&str> {
    json_str_field(line, "op")
}

fn live_snapshot(world: &World) -> String {
    let (workspace, tab, pane) = first_ids(world);
    let pid = first_pid(world);
    format!(
        "{{\"live\":true,\"workspace\":\"{workspace}\",\"tab\":\"{tab}\",\"pane\":\"{pane}\",\"pid\":{pid},\"focused\":\"{}\"}}",
        world.focused
    )
}

fn dead_snapshot(pid: u32) -> String {
    format!(
        "{{\"live\":false,\"alive\":false,\"note\":\"server stop; this snapshot is not a live PTY\",\"pid\":{pid}}}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;
    use std::process::{Child, Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    fn dory_bin() -> PathBuf {
        let mut path = env::current_exe().expect("current_exe");
        path.pop();
        if path.ends_with("deps") {
            path.pop();
        }
        let bin = path.join(env!("CARGO_PKG_NAME"));
        assert!(
            bin.is_file(),
            "dory binary missing at {}",
            bin.display()
        );
        bin
    }

    fn temp_xdg() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let path = env::temp_dir().join(format!(
            "dory-srv-{}-{}",
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn session_sock(xdg: &Path) -> PathBuf {
        xdg.join("dory").join(DEFAULT_SESSION).join("dory.sock")
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

    fn rpc(sock: &Path, line: &str) -> String {
        let mut stream = UnixStream::connect(sock).unwrap();
        writeln!(stream, "{line}").unwrap();
        let _ = stream.flush();
        let mut reply = String::new();
        BufReader::new(stream).read_line(&mut reply).unwrap();
        reply
    }

    fn rpc_op(sock: &Path, op: &str) -> String {
        rpc(sock, &format!("{{\"op\":\"{op}\"}}"))
    }

    fn json_u32(json: &str, key: &str) -> u32 {
        let pat = format!("\"{key}\":");
        let rest = json.split_once(&pat).unwrap().1.trim_start();
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        digits.parse().unwrap()
    }

    fn result_id(json: &str, key: &str) -> String {
        if let Some(id) = nested_id(json, key) {
            return id.to_string();
        }
        json_field(json, key).to_string()
    }

    fn nested_id<'a>(json: &'a str, key: &str) -> Option<&'a str> {
        let pat = format!("\"{key}\":{{");
        let rest = json.split_once(&pat)?.1;
        let after = rest.split_once("\"id\":\"")?.1;
        Some(after.split_once('"')?.0)
    }

    fn pid_alive(pid: u32) -> bool {
        Path::new(&format!("/proc/{pid}")).exists()
    }

    fn wait_dead(pid: u32) {
        let start = Instant::now();
        while pid_alive(pid) {
            if start.elapsed() > Duration::from_secs(3) {
                panic!("pid {pid} still alive");
            }
            thread::sleep(Duration::from_millis(20));
        }
    }

    fn stop_server(xdg: &Path) -> String {
        let out = Command::new(dory_bin())
            .args(["server", "stop"])
            .env("XDG_RUNTIME_DIR", xdg)
            .output()
            .expect("dory server stop");
        assert!(
            out.status.success(),
            "stop failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn cli(xdg: &Path, sock: &Path, env_on: bool, args: &[&str]) -> std::process::Output {
        let mut cmd = Command::new(dory_bin());
        cmd.args(args)
            .env("XDG_RUNTIME_DIR", xdg)
            .env("DORY_SOCKET", sock);
        if env_on {
            cmd.env("DORY_ENV", "1");
        } else {
            cmd.env_remove("DORY_ENV");
        }
        cmd.output().expect("dory cli")
    }

    fn proc_environ(pid: u32) -> String {
        let raw = fs::read(format!("/proc/{pid}/environ")).unwrap_or_default();
        String::from_utf8_lossy(&raw).replace('\0', "\n")
    }

    /// `/proc/pid/environ` is empty until execve finishes (same race as exe).
    fn wait_environ(pid: u32, needle: &str, budget: Duration) -> String {
        let start = Instant::now();
        loop {
            let env = proc_environ(pid);
            if env.contains(needle) {
                return env;
            }
            if start.elapsed() >= budget {
                panic!(
                    "timed out waiting for {needle} in /proc/{pid}/environ (alive={}): {env}",
                    pid_alive(pid)
                );
            }
            thread::sleep(Duration::from_millis(20));
        }
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
            thread::sleep(Duration::from_millis(20));
        }
    }

    #[test]
    fn parse_op_reads_newline_json() {
        assert_eq!(parse_op("{\"op\":\"ping\"}\n"), Some("ping"));
        assert_eq!(parse_op(" { \"op\" : \"stop\" } "), Some("stop"));
        assert_eq!(parse_op("{\"op\":\"snapshot\"}"), Some("snapshot"));
        assert_eq!(parse_op("{\"op\":\"workspace.create\"}"), Some("workspace.create"));
        assert_eq!(
            json_str_field("{\"op\":\"tab.create\",\"workspace\":\"w2\"}", "workspace"),
            Some("w2")
        );
        assert_eq!(parse_op("not json"), None);
        assert_eq!(parse_op("{\"op\":\"pane.split\"}"), Some("pane.split"));
        assert_eq!(
            json_bool_field("{\"op\":\"pane.split\",\"no_focus\":true}", "no_focus"),
            Some(true)
        );
    }

    #[test]
    fn p2_8_unset_xdg_runtime_dir_exits_two() {
        let out = Command::new(dory_bin())
            .arg("server")
            .env_remove("XDG_RUNTIME_DIR")
            .output()
            .expect("spawn dory server");
        assert_eq!(out.status.code(), Some(2));
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(err.contains("XDG_RUNTIME_DIR"));
        assert!(!err.contains("/tmp"));
    }

    #[test]
    fn p2_3_client_disconnect_does_not_kill() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let snap = rpc_op(&sock, "snapshot");
        assert!(snap.contains("\"live\":true"));
        assert_eq!(json_field(&snap, "workspace"), "w1");
        assert_eq!(json_field(&snap, "tab"), "w1:t1");
        assert_eq!(json_field(&snap, "pane"), "w1:p1");
        let pid = json_u32(&snap, "pid");
        assert!(pid_alive(pid), "child {pid} should be live");

        {
            let _ = rpc_op(&sock, "ping");
        }
        assert!(pid_alive(pid), "detach must not kill pid {pid}");
        let again = rpc_op(&sock, "snapshot");
        assert!(again.contains("\"live\":true"));
        assert_eq!(json_u32(&again, "pid"), pid);
        assert!(pid_alive(pid));

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p2_4_stop_kills_group_and_next_server_is_new_pid() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let pid = json_u32(&rpc_op(&sock, "snapshot"), "pid");
        assert!(pid_alive(pid));

        let image = stop_server(&xdg);
        assert!(image.contains("\"live\":false"));
        assert!(image.contains("not a live PTY"));
        wait_dead(pid);
        let status = server.wait().unwrap();
        assert_eq!(status.code(), Some(0));

        let mut server2 = start_server(&xdg);
        let snap2 = rpc_op(&sock, "snapshot");
        assert!(snap2.contains("\"live\":true"));
        let pid2 = json_u32(&snap2, "pid");
        assert_ne!(pid2, pid, "reopen must not be the old pid");
        assert!(pid_alive(pid2));

        let _ = stop_server(&xdg);
        let _ = server2.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p2_10_second_server_same_session_refuses() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let pid = json_u32(&rpc_op(&sock, "snapshot"), "pid");

        let second = Command::new(dory_bin())
            .arg("server")
            .env("XDG_RUNTIME_DIR", &xdg)
            .output()
            .expect("second dory server");
        assert_eq!(second.status.code(), Some(2));
        let err = String::from_utf8_lossy(&second.stderr);
        assert!(
            err.contains("nested server refused"),
            "unexpected refuse text: {err}"
        );
        assert!(pid_alive(pid));
        assert!(rpc_op(&sock, "snapshot").contains("\"live\":true"));

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p3_1_first_pane_environ_has_dory_vars() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let snap = rpc_op(&sock, "snapshot");
        let pid = json_u32(&snap, "pid");
        let env = wait_environ(pid, "DORY_ENV=1", Duration::from_secs(3));
        assert!(env.contains("DORY_ENV=1"), "{env}");
        assert!(
            env.contains(&format!("DORY_SOCKET={}", sock.display())),
            "{env}"
        );
        assert!(env.contains("DORY_BIN="), "{env}");
        assert!(env.contains("DORY_WORKSPACE_ID=w1"), "{env}");
        assert!(env.contains("DORY_TAB_ID=w1:t1"), "{env}");
        assert!(env.contains("DORY_PANE_ID=w1:p1"), "{env}");

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p3_workspace_tab_create_close_and_env_gate() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);

        let denied = cli(&xdg, &sock, false, &["workspace", "create"]);
        assert_eq!(denied.status.code(), Some(1), "P3-15");
        let denied_err = String::from_utf8_lossy(&denied.stderr);
        assert!(denied_err.contains("\"ok\":false"), "{denied_err}");
        assert!(denied.stdout.is_empty());

        let created = cli(&xdg, &sock, true, &["workspace", "create"]);
        assert!(
            created.status.success(),
            "create: {}",
            String::from_utf8_lossy(&created.stderr)
        );
        let body = String::from_utf8_lossy(&created.stdout);
        assert!(body.contains("\"ok\":true"), "{body}");
        let ws = result_id(&body, "workspace");
        let tab = result_id(&body, "tab");
        let root = result_id(&body, "root_pane");
        assert!(ws.starts_with('w'), "{ws}");
        assert_ne!(ws, "w1", "P3-3: do not assume first create is w1 as next input");
        assert!(tab.contains(':'), "{tab}");
        assert!(root.contains(":p"), "{root}");
        assert!(body.contains("\"occupant\":null"), "{body}");

        let listed = cli(&xdg, &sock, true, &["workspace", "list"]);
        assert!(listed.status.success());
        let list_body = String::from_utf8_lossy(&listed.stdout);
        assert!(list_body.contains(&ws), "{list_body}");

        let got = cli(&xdg, &sock, true, &["workspace", "get", &ws]);
        assert!(got.status.success());
        let get_body = String::from_utf8_lossy(&got.stdout);
        assert_eq!(result_id(&get_body, "workspace"), ws);

        let tab_out = cli(
            &xdg,
            &sock,
            true,
            &["tab", "create", "--workspace", &ws],
        );
        assert!(
            tab_out.status.success(),
            "tab create: {}",
            String::from_utf8_lossy(&tab_out.stderr)
        );
        let tab_body = String::from_utf8_lossy(&tab_out.stdout);
        let new_tab = result_id(&tab_body, "tab");
        let new_pane = result_id(&tab_body, "root_pane");
        assert_ne!(new_tab, tab);
        assert!(tab_body.contains("\"occupant\":null"), "{tab_body}");
        assert!(new_pane.contains(":p"), "{new_pane}");

        let closed = cli(&xdg, &sock, true, &["tab", "close", &new_tab]);
        assert!(
            closed.status.success(),
            "tab close: {}",
            String::from_utf8_lossy(&closed.stderr)
        );
        let closed_body = String::from_utf8_lossy(&closed.stdout);
        assert!(closed_body.contains("\"retired\":true"), "{closed_body}");

        let again = cli(
            &xdg,
            &sock,
            true,
            &["tab", "create", "--workspace", &ws],
        );
        assert!(again.status.success());
        let again_tab = result_id(&String::from_utf8_lossy(&again.stdout), "tab");
        assert_ne!(again_tab, new_tab, "P3-6 retired tab id must not return");

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn split_direction_heuristic() {
        assert_eq!(split_direction(80, 24, None).unwrap(), "right");
        assert_eq!(split_direction(20, 40, None).unwrap(), "down");
        assert_eq!(split_direction(24, 24, None).unwrap(), "down");
        assert_eq!(split_direction(20, 40, Some("right")).unwrap(), "right");
        assert_eq!(split_direction(80, 24, Some("down")).unwrap(), "down");
    }

    #[test]
    fn p3_7_right_split_returns_new_pane_caller_stays_focused() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let snap = rpc_op(&sock, "snapshot");
        let caller = json_field(&snap, "pane").to_string();
        assert_eq!(json_field(&snap, "focused"), caller.as_str());

        let denied = cli(
            &xdg,
            &sock,
            false,
            &[
                "pane",
                "split",
                "--pane",
                &caller,
                "--direction",
                "right",
                "--no-focus",
            ],
        );
        assert_eq!(denied.status.code(), Some(1), "P3-15");
        assert!(denied.stdout.is_empty());

        let out = cli(
            &xdg,
            &sock,
            true,
            &[
                "pane",
                "split",
                "--pane",
                &caller,
                "--direction",
                "right",
                "--no-focus",
            ],
        );
        assert!(
            out.status.success(),
            "split: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let body = String::from_utf8_lossy(&out.stdout);
        assert!(body.contains("\"ok\":true"), "{body}");
        let new_pane = result_id(&body, "pane");
        assert_ne!(new_pane, caller);
        assert!(new_pane.contains(":p"), "{new_pane}");
        assert!(body.contains("\"direction\":\"right\""), "{body}");
        assert!(body.contains("\"occupant\":null"), "{body}");
        assert!(!body.contains("\"idle\""), "{body}");
        assert!(!body.contains("\"done\""), "{body}");
        assert!(!body.contains("\"blocked\""), "{body}");

        let snap2 = rpc_op(&sock, "snapshot");
        assert_eq!(json_field(&snap2, "focused"), caller.as_str());

        let got = rpc(&sock, &format!(r#"{{"op":"pane.get","pane":"{new_pane}"}}"#));
        assert!(got.contains("\"ok\":true"), "{got}");
        let pid = json_u32(&got, "pid");
        let env = wait_environ(pid, "DORY_ENV=1", Duration::from_secs(3));
        assert!(
            env.contains("DORY_ENV=1"),
            "pane pid {pid} environ missing DORY_ENV=1 (len={})",
            env.len()
        );
        assert!(
            env.contains(&format!("DORY_PANE_ID={new_pane}")),
            "pane pid {pid} environ missing DORY_PANE_ID"
        );
        assert!(env.contains("DORY_SOCKET="), "pane pid {pid} missing DORY_SOCKET");
        assert!(env.contains("DORY_BIN="), "pane pid {pid} missing DORY_BIN");
        assert!(
            env.contains("DORY_WORKSPACE_ID="),
            "pane pid {pid} missing DORY_WORKSPACE_ID"
        );
        assert!(env.contains("DORY_TAB_ID="), "pane pid {pid} missing DORY_TAB_ID");

        let ws2 = cli(&xdg, &sock, true, &["workspace", "create"]);
        assert!(
            ws2.status.success(),
            "P3-19: {}",
            String::from_utf8_lossy(&ws2.stderr)
        );
        let ws2_id = result_id(&String::from_utf8_lossy(&ws2.stdout), "workspace");
        assert_ne!(ws2_id, json_field(&snap, "workspace"), "P3-19");

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p3_8_direction_heuristic_from_ptysize() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let caller = json_field(&rpc_op(&sock, "snapshot"), "pane").to_string();

        let wide = cli(&xdg, &sock, true, &["pane", "split", "--pane", &caller]);
        assert!(
            wide.status.success(),
            "wide: {}",
            String::from_utf8_lossy(&wide.stderr)
        );
        let wide_body = String::from_utf8_lossy(&wide.stdout);
        assert!(wide_body.contains("\"direction\":\"right\""), "{wide_body}");

        let resized = rpc(
            &sock,
            &format!(r#"{{"op":"pane.resize","pane":"{caller}","cols":20,"rows":40}}"#),
        );
        assert!(resized.contains("\"ok\":true"), "{resized}");

        let tall = cli(&xdg, &sock, true, &["pane", "split", "--pane", &caller]);
        assert!(
            tall.status.success(),
            "tall: {}",
            String::from_utf8_lossy(&tall.stderr)
        );
        let tall_body = String::from_utf8_lossy(&tall.stdout);
        assert!(tall_body.contains("\"direction\":\"down\""), "{tall_body}");

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p3_9_both_panes_pwd_match() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let caller = json_field(&rpc_op(&sock, "snapshot"), "pane").to_string();
        let caller_pwd = xdg.join("caller.pwd");
        let other_pwd = xdg.join("other.pwd");

        let write1 = rpc(
            &sock,
            &format!(
                r#"{{"op":"pane.write","pane":"{caller}","text":"pwd > {}"}}"#,
                caller_pwd.display()
            ),
        );
        assert!(write1.contains("\"ok\":true"), "{write1}");
        let body1 = wait_file(&caller_pwd, Duration::from_secs(5));

        let out = cli(
            &xdg,
            &sock,
            true,
            &[
                "pane",
                "split",
                "--pane",
                &caller,
                "--direction",
                "right",
                "--no-focus",
            ],
        );
        assert!(
            out.status.success(),
            "split: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let new_pane = result_id(&String::from_utf8_lossy(&out.stdout), "pane");

        let write2 = rpc(
            &sock,
            &format!(
                r#"{{"op":"pane.write","pane":"{new_pane}","text":"pwd > {}"}}"#,
                other_pwd.display()
            ),
        );
        assert!(write2.contains("\"ok\":true"), "{write2}");
        let body2 = wait_file(&other_pwd, Duration::from_secs(5));
        assert_eq!(body1.trim(), body2.trim());

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn p3_10_current_vs_omit_target() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let snap = rpc_op(&sock, "snapshot");
        let caller = json_field(&snap, "pane").to_string();

        let omit = cli(
            &xdg,
            &sock,
            true,
            &["pane", "split", "--direction", "right"],
        );
        assert_eq!(omit.status.code(), Some(2), "omit target is usage");
        let after_omit = rpc_op(&sock, "snapshot");
        assert_eq!(json_field(&after_omit, "focused"), caller.as_str());
        assert_eq!(json_field(&after_omit, "pane"), caller.as_str());

        let out = Command::new(dory_bin())
            .args([
                "pane",
                "split",
                "--current",
                "--direction",
                "right",
                "--no-focus",
            ])
            .env("XDG_RUNTIME_DIR", &xdg)
            .env("DORY_SOCKET", &sock)
            .env("DORY_ENV", "1")
            .env("DORY_PANE_ID", &caller)
            .output()
            .expect("pane split --current");
        assert!(
            out.status.success(),
            "current: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let body = String::from_utf8_lossy(&out.stdout);
        let new_pane = result_id(&body, "pane");
        assert_ne!(new_pane, caller);
        assert_eq!(json_field(&rpc_op(&sock, "snapshot"), "focused"), caller.as_str());

        let missing = Command::new(dory_bin())
            .args(["pane", "split", "--current"])
            .env("XDG_RUNTIME_DIR", &xdg)
            .env("DORY_SOCKET", &sock)
            .env("DORY_ENV", "1")
            .env_remove("DORY_PANE_ID")
            .output()
            .expect("pane split --current without pane id");
        assert_eq!(missing.status.code(), Some(1));
        let missing_err = String::from_utf8_lossy(&missing.stderr);
        assert!(missing_err.contains("\"ok\":false"), "{missing_err}");

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    #[test]
    fn regex_search_covers_literal_and_dotstar() {
        let re = compile_regex("DORY.*MARK").unwrap();
        assert!(regex_search(&re, "pre DORY_P3_11_MARK post"));
        assert!(!regex_search(&re, "nope"));
        let lit = compile_regex("abc").unwrap();
        assert!(regex_search(&lit, "xxabc"));
        assert!(compile_regex("[").is_err());
    }

    #[test]
    fn p3_11_run_read_then_wait_existing() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);
        let caller = json_field(&rpc_op(&sock, "snapshot"), "pane").to_string();
        const MARK: &str = "DORY_P3_11_MARK";

        let run = cli(
            &xdg,
            &sock,
            true,
            &["pane", "run", "--pane", &caller, "echo", MARK],
        );
        assert!(
            run.status.success(),
            "run: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        let run_body = String::from_utf8_lossy(&run.stdout);
        assert!(run_body.contains("\"ok\":true"), "{run_body}");
        assert!(!run_body.contains("\"idle\""), "{run_body}");
        assert!(!run_body.contains("\"done\""), "{run_body}");
        assert!(!run_body.contains("\"blocked\""), "{run_body}");

        let waited = cli(
            &xdg,
            &sock,
            true,
            &[
                "pane",
                "wait-output",
                "--pane",
                &caller,
                "--match",
                MARK,
                "--timeout",
                "5000",
            ],
        );
        assert!(
            waited.status.success(),
            "wait: {}",
            String::from_utf8_lossy(&waited.stderr)
        );
        let wait_body = String::from_utf8_lossy(&waited.stdout);
        assert!(wait_body.contains(MARK), "{wait_body}");
        assert!(wait_body.contains("\"matched\":true"), "{wait_body}");
        assert!(!wait_body.contains("\"idle\""), "{wait_body}");
        assert!(!wait_body.contains("\"done\""), "{wait_body}");
        assert!(!wait_body.contains("\"blocked\""), "{wait_body}");

        let again = cli(
            &xdg,
            &sock,
            true,
            &[
                "pane",
                "wait-output",
                "--pane",
                &caller,
                "--match",
                MARK,
                "--timeout",
                "200",
            ],
        );
        assert!(
            again.status.success(),
            "existing: {}",
            String::from_utf8_lossy(&again.stderr)
        );

        let read1 = cli(&xdg, &sock, true, &["pane", "read", "--pane", &caller]);
        assert!(
            read1.status.success(),
            "read1: {}",
            String::from_utf8_lossy(&read1.stderr)
        );
        let body1 = String::from_utf8_lossy(&read1.stdout);
        assert!(body1.contains(MARK), "{body1}");
        assert!(!body1.contains("\"seen\""), "{body1}");
        assert!(!body1.contains("\"idle\""), "{body1}");
        assert!(!body1.contains("\"done\""), "{body1}");
        assert!(!body1.contains("\"blocked\""), "{body1}");

        let read2 = cli(
            &xdg,
            &sock,
            true,
            &[
                "pane",
                "read",
                "--pane",
                &caller,
                "--source",
                "recent-unwrapped",
            ],
        );
        assert!(
            read2.status.success(),
            "read2: {}",
            String::from_utf8_lossy(&read2.stderr)
        );
        let body2 = String::from_utf8_lossy(&read2.stdout);
        assert!(body2.contains(MARK), "{body2}");
        assert!(!body2.contains("\"seen\""), "{body2}");
        assert!(!body2.contains("\"idle\""), "{body2}");
        assert!(!body2.contains("\"done\""), "{body2}");
        assert!(!body2.contains("\"blocked\""), "{body2}");

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }


    #[test]
    fn tab_pane_list_get_are_inspect() {
        let xdg = temp_xdg();
        let mut server = start_server(&xdg);
        let sock = session_sock(&xdg);

        let created = cli(&xdg, &sock, true, &["workspace", "create"]);
        assert!(
            created.status.success(),
            "create: {}",
            String::from_utf8_lossy(&created.stderr)
        );
        let body = String::from_utf8_lossy(&created.stdout);
        let ws = result_id(&body, "workspace");
        let tab = result_id(&body, "tab");
        let root = result_id(&body, "root_pane");

        let tabs = cli(&xdg, &sock, false, &["tab", "list", "--workspace", &ws]);
        assert!(
            tabs.status.success(),
            "tab list without env: {}",
            String::from_utf8_lossy(&tabs.stderr)
        );
        let tabs_body = String::from_utf8_lossy(&tabs.stdout);
        assert!(tabs_body.contains("\"ok\":true"), "{tabs_body}");
        assert!(tabs_body.contains(&format!("\"id\":\"{tab}\"")), "{tabs_body}");
        assert!(tabs_body.contains("\"occupant\":null"), "{tabs_body}");
        assert!(!tabs_body.contains(":7380"), "{tabs_body}");

        let before = cli(&xdg, &sock, false, &["pane", "list", "--workspace", &ws]);
        assert!(
            before.status.success(),
            "pane list: {}",
            String::from_utf8_lossy(&before.stderr)
        );
        let before_body = String::from_utf8_lossy(&before.stdout);
        let before_count = before_body.matches("\"id\":\"").count();

        let split = cli(
            &xdg,
            &sock,
            true,
            &["pane", "split", "--pane", &root, "--direction", "right"],
        );
        assert!(
            split.status.success(),
            "split: {}",
            String::from_utf8_lossy(&split.stderr)
        );
        let new_pane = result_id(&String::from_utf8_lossy(&split.stdout), "pane");
        assert_ne!(new_pane, root);

        let after = cli(&xdg, &sock, false, &["pane", "list", "--workspace", &ws]);
        assert!(
            after.status.success(),
            "pane list after split: {}",
            String::from_utf8_lossy(&after.stderr)
        );
        let after_body = String::from_utf8_lossy(&after.stdout);
        assert!(after_body.contains(&format!("\"id\":\"{root}\"")), "{after_body}");
        assert!(
            after_body.contains(&format!("\"id\":\"{new_pane}\"")),
            "{after_body}"
        );
        assert_eq!(
            after_body.matches("\"id\":\"").count(),
            before_count + 1,
            "{after_body}"
        );

        let got = cli(&xdg, &sock, false, &["pane", "get", "--pane", &new_pane]);
        assert!(
            got.status.success(),
            "pane get: {}",
            String::from_utf8_lossy(&got.stderr)
        );
        let got_body = String::from_utf8_lossy(&got.stdout);
        assert_eq!(result_id(&got_body, "pane"), new_pane);
        assert!(got_body.contains("\"occupant\":null"), "{got_body}");
        assert!(!got_body.contains(":7380"), "{got_body}");

        let omitted = cli(&xdg, &sock, false, &["pane", "get"]);
        assert_eq!(omitted.status.code(), Some(2));

        let _ = stop_server(&xdg);
        let _ = server.wait();
        let _ = fs::remove_dir_all(&xdg);
    }

    fn json_field<'a>(json: &'a str, key: &str) -> &'a str {
        let pat = format!("\"{key}\":\"");
        let rest = json.split_once(&pat).unwrap().1;
        rest.split_once('"').unwrap().0
    }
}
