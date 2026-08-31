//! Human desk: sidebar + tiled live panes. Client of the socket.
//! Not Ratatui. Not a Herdr clone. Detach ≠ kill.

use crate::attach;
use crate::server;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{
    BeginSynchronizedUpdate, DisableLineWrap, EnableLineWrap, EndSynchronizedUpdate,
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, queue};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

const SIDEBAR: u16 = 26;
const SIDEBAR_DOTS: u16 = 4;
const AGENT_REGION: u16 = 6;
const AGENT_REGION_DOTS: u16 = 4;
const TAB_ROW: u16 = 1;
const TAB_CHIP_MAX: usize = 16;
const CTRL_B: u8 = 0x02;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Terminal,
    Prefix,
    Picker,
    Confirm,
    Help,
    Menu,
    Onboard,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ConfirmKind {
    Pane,
    Tab,
    Workspace,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MenuKind {
    Pane,
    Tab,
    Workspace,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MenuVerb {
    SplitRight,
    SplitDown,
    Zoom,
    ClosePane,
    NewTab,
    CloseTab,
    Picker,
    NewWs,
    CloseWs,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MenuPick {
    Cancel,
    Run(MenuVerb),
    Ignore,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ConfirmPick {
    Yes,
    No,
    Ignore,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OnboardKey {
    Persist,
    Dismiss,
    Prefix,
    Eat,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OnboardMouse {
    Release,
    Eat,
}

#[derive(Clone, Debug)]
struct MenuTarget {
    kind: MenuKind,
    focus: String,
    tab: String,
    workspace: String,
}

#[derive(Clone, Debug)]
struct Confirm {
    kind: ConfirmKind,
    pane: String,
    tab: String,
    workspace: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SideKind {
    Chrome,
    Workspace,
    Agent,
    Rule,
}
const TITLE_BG: Color = Color::Rgb {
    r: 16,
    g: 22,
    b: 32,
};
const SIDE_BG: Color = Color::Rgb {
    r: 10,
    g: 14,
    b: 20,
};
const PANE_BG: Color = Color::Rgb { r: 8, g: 10, b: 14 };
const ACCENT: Color = Color::Rgb {
    r: 72,
    g: 180,
    b: 196,
};
const MUTED: Color = Color::Rgb {
    r: 108,
    g: 120,
    b: 136,
};
const TEXT: Color = Color::Rgb {
    r: 220,
    g: 226,
    b: 232,
};
const FOCUSED_FG: Color = Color::Rgb {
    r: 255,
    g: 214,
    b: 102,
};
const BLOCKED_FG: Color = Color::Rgb {
    r: 232,
    g: 96,
    b: 88,
};

pub fn run() -> i32 {
    run_with_pane(None)
}

pub fn run_with_pane(pane: Option<&str>) -> i32 {
    if let Err(code) = attach::ensure_server() {
        return code;
    }
    if !stdin_is_tty() || !stdout_is_tty() {
        eprintln!("dory: needs a tty (server is up; try `dory` in a terminal)");
        return 1;
    }
    match run_ui(pane) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("dory: {err}");
            1
        }
    }
}

fn run_ui(start_pane: Option<&str>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        DisableLineWrap,
        EnableMouseCapture,
        EnableBracketedPaste,
        Hide
    )?;
    let _guard = TermGuard;
    let mut desk = Desk::open(start_pane)?;
    desk.loop_ui(&mut stdout)
}

struct TermGuard;

impl Drop for TermGuard {
    fn drop(&mut self) {
        let mut out = io::stdout();
        let _ = execute!(
            out,
            DisableMouseCapture,
            DisableBracketedPaste,
            EnableLineWrap,
            LeaveAlternateScreen,
            Show,
            ResetColor
        );
        let _ = disable_raw_mode();
    }
}

#[derive(Clone)]
struct Row {
    kind: char,
    id: String,
    #[allow(dead_code)]
    focus_pane: String,
    occ: String,
    st: String,
    cwd: String,
}

struct Tile {
    id: String,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    stream: Option<UnixStream>,
    parser: vt100::Parser,
}

struct Drag {
    a: String,
    b: String,
    dir: crate::layout::SplitDir,
    _start: u16,
    span: u16,
    last_ratio: f32,
}

struct Desk {
    tiles: Vec<Tile>,
    cells: Vec<crate::layout::Cell>,
    rows: Vec<Row>,
    focused: String,
    workspace: String,
    tab: String,
    status: String,
    mode: Mode,
    prefix_at: Instant,
    last_tree: Instant,
    cols: u16,
    rows_n: u16,
    pty_cols: u16,
    pty_rows: u16,
    sidebar_cols: u16,
    top_rows: u16,
    chrome_dirty: bool,
    tiles_dirty: bool,
    drag: Option<Drag>,
    zoomed: bool,
    picker_idx: usize,
    confirm: Option<Confirm>,
    menu: Option<MenuKind>,
    menu_target: Option<MenuTarget>,
    menu_anchor: Option<(u16, u16)>,
    sel_from: Option<(u16, u16)>,
    sel_to: Option<(u16, u16)>,
    retired_ws: Vec<String>,
    retired_tab: Vec<String>,
    retired_pane: Vec<String>,
    flow_glance: Option<String>,
    flow_mtime: Option<SystemTime>,
    flow_path: Option<PathBuf>,
    footer_dirty: bool,
}

impl Desk {
    fn open(start_pane: Option<&str>) -> io::Result<Self> {
        let (cols, rows_n) = term_size();
        let (pty_cols, pty_rows) = pane_size(cols, rows_n, SIDEBAR, 2); // title + tab bar
        let mut desk = Self {
            tiles: Vec::new(),
            cells: Vec::new(),
            rows: Vec::new(),
            focused: String::new(),
            workspace: String::new(),
            tab: String::new(),
            status: String::new(),
            mode: Mode::Terminal,
            prefix_at: Instant::now(),
            last_tree: Instant::now() - Duration::from_secs(2),
            cols,
            rows_n,
            pty_cols,
            pty_rows,
            sidebar_cols: SIDEBAR,
            top_rows: 2,
            chrome_dirty: true,
            tiles_dirty: true,
            drag: None,
            zoomed: false,
            picker_idx: 0,
            confirm: None,
            menu: None,
            menu_target: None,
            menu_anchor: None,
            sel_from: None,
            sel_to: None,
            retired_ws: Vec::new(),
            retired_tab: Vec::new(),
            retired_pane: Vec::new(),
            flow_glance: None,
            flow_mtime: None,
            flow_path: None,
            footer_dirty: false,
        };
        desk.refresh_tree();
        desk.mode = initial_mode(
            onboard_state_path().as_deref(),
            skip_onboard_from_env(),
        );
        if desk.mode == Mode::Onboard {
            desk.chrome_dirty = true;
            desk.tiles_dirty = true;
        }
        if let Some(id) = start_pane {
            desk.focused = id.to_string();
            let _ = server::rpc_line_quiet(&format!(r#"{{"op":"pane.focus","pane":"{id}"}}"#));
        }
        desk.reconcile_tiles();
        Ok(desk)
    }

    fn loop_ui(&mut self, out: &mut io::Stdout) -> io::Result<()> {
        self.draw(out)?;
        loop {
            if self.pump_pty() {
                self.tiles_dirty = true;
            }
            if self.last_tree.elapsed() >= Duration::from_millis(400) {
                let before = self.tree_sig();
                self.refresh_tree();
                let focused_painted = self.tiles.iter().any(|t| t.id == self.focused);
                if self.tree_sig() != before || !focused_painted {
                    self.reconcile_tiles();
                    self.chrome_dirty = true;
                    self.tiles_dirty = true;
                }
            }
            if self.mode == Mode::Prefix && self.prefix_at.elapsed() > Duration::from_secs(2) {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.chrome_dirty = true;
                self.tiles_dirty = true;
            }
            if event::poll(Duration::from_millis(30))? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Release {
                            continue;
                        }
                        if self.handle_key(key) {
                            return Ok(());
                        }
                    }
                    Event::Mouse(mouse) => {
                        if self.handle_mouse(mouse) {
                            return Ok(());
                        }
                    }
                    Event::Resize(cols, rows) => {
                        self.resize(cols, rows);
                        self.chrome_dirty = true;
                        self.tiles_dirty = true;
                    }
                    Event::Paste(text) => {
                        if self.mode == Mode::Terminal {
                            self.write_pty(text.as_bytes());
                            self.tiles_dirty = true;
                        }
                    }
                    Event::FocusGained | Event::FocusLost => {}
                }
            }
            if self.chrome_dirty || self.tiles_dirty || self.footer_dirty {
                self.draw(out)?;
                self.chrome_dirty = false;
                self.tiles_dirty = false;
                self.footer_dirty = false;
            }
        }
    }

    fn tree_sig(&self) -> String {
        tree_rows_sig(&self.focused, &self.rows)
    }

    fn refresh_tree(&mut self) {
        let Ok(body) = server::rpc_line_quiet(r#"{"op":"desk.tree"}"#) else {
            return;
        };
        if !body.contains("\"ok\":true") {
            return;
        }
        self.last_tree = Instant::now();
        let before = self.tree_sig();
        let (rows, focused, workspace, tab) = parse_tree(&body);
        sweep_retired(
            &rows,
            &mut self.retired_ws,
            &mut self.retired_tab,
            &mut self.retired_pane,
        );
        self.rows = rows;
        apply_retired(
            &mut self.rows,
            &self.retired_ws,
            &self.retired_tab,
            &self.retired_pane,
        );
        if self.focused.is_empty() {
            self.focused = focused;
        } else if !self
            .rows
            .iter()
            .any(|r| r.kind == 'p' && r.id == self.focused)
        {
            self.focused = focused;
        }
        self.workspace = workspace;
        self.tab = tab;
        self.repair_location();
        if self.tree_sig() != before {
            self.chrome_dirty = true;
            self.tiles_dirty = true;
        }
        self.refresh_flow_glance();
    }

    fn refresh_flow_glance(&mut self) {
        let path = workspace_cwd(&self.rows, &self.workspace).map(flow_journal_path);
        if poll_flow_glance(
            path.as_deref(),
            &mut self.flow_glance,
            &mut self.flow_mtime,
            &mut self.flow_path,
        ) {
            self.footer_dirty = true;
        }
    }

    fn repair_location(&mut self) {
        if self.focused.is_empty()
            || !self
                .rows
                .iter()
                .any(|r| r.kind == 'p' && r.id == self.focused)
        {
            self.focused = first_live_pane(&self.rows).unwrap_or_default();
        }
        if let Some(w) = workspace_of(&self.rows, &self.focused) {
            self.workspace = w;
        } else if !self
            .rows
            .iter()
            .any(|r| r.kind == 'w' && r.id == self.workspace)
        {
            self.workspace.clear();
        }
        if let Some(t) = tab_of(&self.rows, &self.focused) {
            self.tab = t;
        } else if !self.rows.iter().any(|r| r.kind == 't' && r.id == self.tab) {
            self.tab.clear();
        }
    }

    fn pane_painted(&self, id: &str) -> bool {
        self.tiles.iter().any(|t| t.id == id)
    }

    fn focus_tile(&mut self, id: &str) {
        if id.is_empty() {
            return;
        }
        let accepted = server::rpc_line_quiet(&format!(r#"{{"op":"pane.focus","pane":"{id}"}}"#))
            .map(|body| body.contains("\"ok\":true"))
            .unwrap_or(false);
        if accepted {
            self.focused = id.to_string();
        }
        self.refresh_tree();
        if !accepted && self.focused == id {
            self.repair_location();
        }
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn reconcile_tiles(&mut self) {
        let body = match server::rpc_line_quiet(&format!(
            r#"{{"op":"desk.layout","cols":{},"rows":{}}}"#,
            self.pty_cols, self.pty_rows
        )) {
            Ok(b) if b.contains("\"ok\":true") => b,
            _ => return,
        };
        self.cells = parse_layout_cells(&body);
        if self.focused.is_empty() {
            if let Some(id) = attach::json_string_field(&body, "focused") {
                self.focused = id;
            }
        }
        let wanted = wanted_rects(
            &self.cells,
            &self.focused,
            self.zoomed,
            self.pty_cols,
            self.pty_rows,
        );
        self.tiles
            .retain(|t| wanted.iter().any(|(id, ..)| id == &t.id));
        for (id, x, y, w, h) in &wanted {
            if let Some(tile) = self.tiles.iter_mut().find(|t| t.id == *id) {
                let skip_resize = self.zoomed && *id != self.focused;
                if !skip_resize && (tile.w != *w || tile.h != *h) {
                    tile.parser.set_size(*h, *w);
                    let _ = server::rpc_line_quiet(&format!(
                        r#"{{"op":"pane.resize","pane":"{id}","cols":{w},"rows":{h}}}"#
                    ));
                    tile.w = *w;
                    tile.h = *h;
                }
                if !skip_resize {
                    tile.x = *x;
                    tile.y = *y;
                }
            } else {
                let no_focus = *id != self.focused;
                match open_attach(id, *w, *h, no_focus) {
                    Ok((stream, leftover)) => {
                        let _ = stream.set_nonblocking(true);
                        let mut parser = vt100::Parser::new(*h, *w, 2000);
                        if !leftover.is_empty() {
                            parser.process(&leftover);
                        }
                        self.tiles.push(Tile {
                            id: id.clone(),
                            x: *x,
                            y: *y,
                            w: *w,
                            h: *h,
                            stream: Some(stream),
                            parser,
                        });
                    }
                    Err(msg) => self.status = msg,
                }
            }
        }
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows_n = rows;
        let (pty_cols, pty_rows) = pane_size(cols, rows, self.sidebar_cols, self.top_rows);
        if pty_cols == self.pty_cols && pty_rows == self.pty_rows {
            return;
        }
        self.pty_cols = pty_cols;
        self.pty_rows = pty_rows;
        self.reconcile_tiles();
    }

    fn pump_pty(&mut self) -> bool {
        let mut any = false;
        let mut buf = [0u8; 8192];
        for tile in &mut self.tiles {
            let Some(stream) = tile.stream.as_mut() else {
                continue;
            };
            loop {
                match stream.read(&mut buf) {
                    Ok(0) => {
                        tile.stream = None;
                        any = true;
                        break;
                    }
                    Ok(n) => {
                        tile.parser.process(&buf[..n]);
                        any = true;
                    }
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                    Err(err) if err.kind() == io::ErrorKind::Interrupted => break,
                    Err(_) => {
                        tile.stream = None;
                        any = true;
                        break;
                    }
                }
            }
        }
        any
    }

    fn write_pty(&mut self, bytes: &[u8]) {
        if let Some(tile) = self.tiles.iter_mut().find(|t| t.id == self.focused) {
            if let Some(stream) = tile.stream.as_mut() {
                let _ = stream.write_all(bytes);
                let _ = stream.flush();
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            Mode::Onboard => {
                match onboard_key(key) {
                    OnboardKey::Prefix => {
                        self.dismiss_onboard();
                        self.mode = Mode::Prefix;
                        self.prefix_at = Instant::now();
                        self.status = PREFIX_FOOTER.to_string();
                        self.chrome_dirty = true;
                        self.tiles_dirty = true;
                        false
                    }
                    OnboardKey::Persist => {
                        self.complete_onboard();
                        false
                    }
                    OnboardKey::Dismiss => {
                        self.dismiss_onboard();
                        false
                    }
                    OnboardKey::Eat => {
                        self.dismiss_onboard();
                        self.handle_key(key)
                    }
                }
            }
            Mode::Help => {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('q' | 'Q' | '?')) {
                    self.mode = Mode::Terminal;
                    self.status.clear();
                    self.chrome_dirty = true;
                }
                false
            }
            Mode::Confirm => self.handle_confirm(key),
            Mode::Picker => {
                self.handle_picker(key);
                false
            }
            Mode::Menu => {
                self.handle_menu_key(key);
                false
            }
            Mode::Prefix => {
                self.mode = Mode::Terminal;
                self.prefix_cmd(key)
            }
            Mode::Terminal => {
                if is_ctrl_b(&key) {
                    self.mode = Mode::Prefix;
                    self.prefix_at = Instant::now();
                    self.status = PREFIX_FOOTER.to_string();
                    self.chrome_dirty = true;
                    return false;
                }
                if let Some(bytes) = encode_key(key) {
                    self.write_pty(&bytes);
                    self.tiles_dirty = true;
                }
                false
            }
        }
    }

    fn handle_confirm(&mut self, key: KeyEvent) -> bool {
        match confirm_key(key.code) {
            ConfirmPick::Yes => self.confirm_yes(),
            ConfirmPick::No => self.confirm_no(),
            ConfirmPick::Ignore => {}
        }
        false
    }

    fn confirm_yes(&mut self) {
        let confirm = self.confirm.take();
        self.mode = Mode::Terminal;
        if let Some(confirm) = confirm {
            self.run_close(confirm);
        }
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn confirm_no(&mut self) {
        self.confirm = None;
        self.mode = Mode::Terminal;
        self.status.clear();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn complete_onboard(&mut self) {
        match onboard_state_path() {
            Some(path) => match mark_onboarded(&path) {
                Ok(()) => self.status.clear(),
                Err(_) => self.status = "could not remember".to_string(),
            },
            None => self.status.clear(),
        }
        self.mode = Mode::Terminal;
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn dismiss_onboard(&mut self) {
        self.mode = Mode::Terminal;
        self.status.clear();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn handle_picker(&mut self, key: KeyEvent) {
        let spaces = workspaces_of(&self.rows);
        match key.code {
            KeyCode::Char('j' | 'J') | KeyCode::Down => {
                if !spaces.is_empty() {
                    self.picker_idx = (self.picker_idx + 1) % spaces.len();
                }
                self.chrome_dirty = true;
            }
            KeyCode::Char('k' | 'K') | KeyCode::Up => {
                if !spaces.is_empty() {
                    self.picker_idx = self.picker_idx.checked_sub(1).unwrap_or(spaces.len() - 1);
                }
                self.chrome_dirty = true;
            }
            KeyCode::Enter => self.picker_enter(),
            KeyCode::Esc => {
                self.close_picker();
            }
            _ => {}
        }
    }

    fn picker_enter(&mut self) {
        let spaces = workspaces_of(&self.rows);
        if let Some(ws) = spaces.get(self.picker_idx) {
            if let Some(pane) = first_pane_of(&self.rows, ws) {
                self.zoomed = false;
                self.focus_tile(&pane);
                self.reconcile_tiles();
            }
        }
        self.close_picker();
    }

    fn prefix_cmd(&mut self, key: KeyEvent) -> bool {
        self.status.clear();
        self.chrome_dirty = true;
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);
        match key.code {
            KeyCode::Char('q' | 'Q') => return true,
            KeyCode::Char('d' | 'D') if shift => {
                self.ask_confirm(ConfirmKind::Workspace);
            }
            KeyCode::Char('d' | 'D') => return true,
            KeyCode::Char('c' | 'C') => self.new_tab(),
            KeyCode::Char('v' | 'V') => self.split("right"),
            KeyCode::Char('-' | '_') => self.split("down"),
            KeyCode::Char('n' | 'N') if shift => self.new_workspace(),
            KeyCode::Char('n' | 'N') => self.next_tab(1),
            KeyCode::Char('p' | 'P') => self.next_tab(-1),
            KeyCode::Char('h' | 'H') => self.neighbor("left"),
            KeyCode::Char('j' | 'J') => self.neighbor("down"),
            KeyCode::Char('k' | 'K') => self.neighbor("up"),
            KeyCode::Char('l' | 'L') => self.neighbor("right"),
            KeyCode::Char('z' | 'Z') => {
                self.zoomed = !self.zoomed;
                self.drag = None;
                self.reconcile_tiles();
            }
            KeyCode::Char('w' | 'W') => self.open_picker(),
            KeyCode::Char('x' | 'X') if shift => self.ask_confirm(ConfirmKind::Tab),
            KeyCode::Char('x' | 'X') => self.close_pane_or_confirm(),
            KeyCode::Char('b') if !is_ctrl_b(&key) => self.toggle_sidebar(),
            KeyCode::Char('?') => {
                self.mode = Mode::Help;
                self.status.clear();
                self.chrome_dirty = true;
            }
            KeyCode::Char(c @ '1'..='9') => {
                let n = (c as u8 - b'1') as usize;
                self.focus_tab_index(n);
            }
            KeyCode::Esc => {}
            _ => {
                if is_ctrl_b(&key) {
                    self.write_pty(&[CTRL_B]);
                    self.tiles_dirty = true;
                }
            }
        }
        false
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
        if self.mode == Mode::Onboard {
            match onboard_mouse(mouse.kind) {
                OnboardMouse::Release => self.dismiss_onboard(),
                OnboardMouse::Eat => return false,
            }
        }
        if self.mode == Mode::Help {
            return false;
        }
        if self.mode == Mode::Picker {
            return self.handle_picker_mouse(mouse);
        }
        if self.mode == Mode::Confirm {
            return self.handle_confirm_mouse(mouse);
        }
        if self.mode == Mode::Menu {
            return self.handle_menu_mouse(mouse);
        }
        let side = self.sidebar_cols;
        let top = self.top_rows;
        let (origin_x, origin_y) = content_origin(side, top);
        let content_x = mouse.column.saturating_sub(origin_x);
        let content_y = mouse.row.saturating_sub(origin_y);
        match mouse.kind {
            MouseEventKind::Moved if self.drag.is_none() && self.sel_from.is_none() => return false,
            MouseEventKind::Down(MouseButton::Left) => {
                self.sel_from = None;
                self.sel_to = None;
                if mouse.row == 0 || mouse.row + 1 >= self.rows_n {
                    return false;
                }
                if mouse.row == TAB_ROW {
                    if let Some(pane) = tab_chip_at(&self.rows, &self.workspace, mouse.column, side)
                    {
                        self.zoomed = false;
                        self.focus_tile(&pane);
                        self.reconcile_tiles();
                    }
                    return false;
                }
                if side > 0 && mouse.column < side {
                    self.drag = None;
                    if let Some(id) = sidebar_focus_at(
                        &self.rows,
                        mouse.row,
                        self.rows_n,
                        &self.workspace,
                        side,
                    ) {
                        if id != self.focused || !self.pane_painted(&id) {
                            self.zoomed = false;
                            self.focus_tile(&id);
                            self.reconcile_tiles();
                        }
                    }
                    self.chrome_dirty = true;
                    self.tiles_dirty = true;
                    return false;
                }
                if self.zoomed {
                    self.sel_from = Some((content_x, content_y));
                    self.sel_to = Some((content_x, content_y));
                    return false;
                }
                if let Some((a, b, dir)) =
                    crate::layout::divider_at(&self.cells, content_x, content_y)
                {
                    let span = match dir {
                        crate::layout::SplitDir::Right => self.pty_cols,
                        crate::layout::SplitDir::Down => self.pty_rows,
                    };
                    self.drag = Some(Drag {
                        a,
                        b,
                        dir,
                        _start: match dir {
                            crate::layout::SplitDir::Right => content_x,
                            crate::layout::SplitDir::Down => content_y,
                        },
                        span: span.max(2),
                        last_ratio: 0.5,
                    });
                    self.chrome_dirty = true;
                    self.tiles_dirty = true;
                    return false;
                }
                if let Some(cell) = crate::layout::cell_at(&self.cells, content_x, content_y) {
                    let id = cell.id.clone();
                    if id != self.focused {
                        self.focus_tile(&id);
                    }
                    self.sel_from = Some((content_x, content_y));
                    self.sel_to = Some((content_x, content_y));
                    self.chrome_dirty = true;
                    self.tiles_dirty = true;
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.sel_from.is_some() && self.drag.is_none() {
                    self.sel_to = Some((content_x, content_y));
                    self.tiles_dirty = true;
                    return false;
                }
                if let Some(drag) = self.drag.as_ref() {
                    let pos = match drag.dir {
                        crate::layout::SplitDir::Right => content_x,
                        crate::layout::SplitDir::Down => content_y,
                    };
                    let ratio = (pos as f32 / drag.span as f32).clamp(0.05, 0.95);
                    if (ratio - drag.last_ratio).abs() >= 0.02 {
                        let a = drag.a.clone();
                        let b = drag.b.clone();
                        let _ = server::rpc_line_quiet(&format!(
                            r#"{{"op":"desk.divider","a":"{a}","b":"{b}","ratio":{ratio}}}"#
                        ));
                        if let Some(d) = self.drag.as_mut() {
                            d.last_ratio = ratio;
                        }
                        self.reconcile_tiles();
                        self.chrome_dirty = true;
                    self.tiles_dirty = true;
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if let (Some(a), Some(b)) = (self.sel_from.take(), self.sel_to.take()) {
                    if cell_drag_span(a, b) >= 2 {
                        if let Some(text) = selection_text(&self.tiles, &self.focused, a, b) {
                            if emit_osc52(&text).is_ok() {
                                self.status = "copied".to_string();
                                self.chrome_dirty = true;
                            }
                        }
                    }
                }
                if let Some(drag) = self.drag.take() {
                    let pos = match drag.dir {
                        crate::layout::SplitDir::Right => content_x,
                        crate::layout::SplitDir::Down => content_y,
                    };
                    let ratio = (pos as f32 / drag.span as f32).clamp(0.05, 0.95);
                    let _ = server::rpc_line_quiet(&format!(
                        r#"{{"op":"desk.divider","a":"{}","b":"{}","ratio":{ratio}}}"#,
                        drag.a, drag.b
                    ));
                    self.reconcile_tiles();
                    self.chrome_dirty = true;
                    self.tiles_dirty = true;
                }
            }
            MouseEventKind::ScrollUp if mouse.column >= side && self.mode == Mode::Terminal => {
                self.write_pty(b"\x1b[A");
            }
            MouseEventKind::ScrollDown if mouse.column >= side && self.mode == Mode::Terminal => {
                self.write_pty(b"\x1b[B");
            }
            MouseEventKind::Down(MouseButton::Right) => {
                self.sel_from = None;
                self.sel_to = None;
                self.drag = None;
                if self.mode == Mode::Prefix {
                    self.mode = Mode::Terminal;
                }
                if let Some(target) = menu_hit(
                    &self.rows,
                    &self.cells,
                    mouse.column,
                    mouse.row,
                    self.rows_n,
                    side,
                    top,
                    self.zoomed,
                    &self.focused,
                    &self.workspace,
                ) {
                    self.focus_then_open_menu(target, Some((mouse.column, mouse.row)));
                }
            }
            MouseEventKind::Drag(MouseButton::Right) | MouseEventKind::Up(MouseButton::Right) => {}
            _ => {}
        }
        false
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        let Some(kind) = self.menu else {
            self.close_menu();
            return;
        };
        match menu_pick(kind, key.code) {
            MenuPick::Cancel => self.close_menu(),
            MenuPick::Run(verb) => self.run_menu_verb(verb),
            MenuPick::Ignore => {}
        }
    }

    fn handle_confirm_mouse(&mut self, mouse: MouseEvent) -> bool {
        match mouse.kind {
            MouseEventKind::Down(btn) => {
                let side = self.sidebar_cols;
                let top = self.top_rows;
                let lines = self
                    .confirm
                    .as_ref()
                    .map(|c| confirm_lines(c.kind))
                    .unwrap_or_default();
                let max_line_w = lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
                let card = overlay_box(
                    Mode::Confirm,
                    lines.len(),
                    max_line_w,
                    self.cols,
                    self.rows_n,
                    side,
                    top,
                    None,
                );
                if overlay_contains(card, mouse.column, mouse.row, self.rows_n) {
                    match confirm_overlay_pick(mouse.row.saturating_sub(card.y)) {
                        ConfirmPick::Yes => self.confirm_yes(),
                        ConfirmPick::No => self.confirm_no(),
                        ConfirmPick::Ignore => {}
                    }
                    return false;
                }
                self.confirm_no();
                if btn == MouseButton::Right {
                    if let Some(target) = menu_hit(
                        &self.rows,
                        &self.cells,
                        mouse.column,
                        mouse.row,
                        self.rows_n,
                        side,
                        top,
                        self.zoomed,
                        &self.focused,
                        &self.workspace,
                    ) {
                        self.focus_then_open_menu(target, Some((mouse.column, mouse.row)));
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn close_picker(&mut self) {
        self.mode = Mode::Terminal;
        self.status.clear();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn handle_picker_mouse(&mut self, mouse: MouseEvent) -> bool {
        match mouse.kind {
            MouseEventKind::Down(btn) => {
                let side = self.sidebar_cols;
                let top = self.top_rows;
                let lines = picker_lines(&self.rows, self.picker_idx);
                let max_line_w = lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
                let card = overlay_box(
                    Mode::Picker,
                    lines.len(),
                    max_line_w,
                    self.cols,
                    self.rows_n,
                    side,
                    top,
                    None,
                );
                if overlay_contains(card, mouse.column, mouse.row, self.rows_n) {
                    let overlay_row = mouse.row.saturating_sub(card.y);
                    if let Some(idx) =
                        picker_mouse_pick(overlay_row, workspaces_of(&self.rows).len())
                    {
                        self.picker_idx = idx;
                        self.picker_enter();
                    }
                    return false;
                }
                self.close_picker();
                if btn == MouseButton::Right {
                    if let Some(target) = menu_hit(
                        &self.rows,
                        &self.cells,
                        mouse.column,
                        mouse.row,
                        self.rows_n,
                        side,
                        top,
                        self.zoomed,
                        &self.focused,
                        &self.workspace,
                    ) {
                        self.focus_then_open_menu(target, Some((mouse.column, mouse.row)));
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn handle_menu_mouse(&mut self, mouse: MouseEvent) -> bool {
        match mouse.kind {
            MouseEventKind::Down(btn) => {
                let side = self.sidebar_cols;
                let top = self.top_rows;
                let lines = self.menu.map(menu_lines).unwrap_or_default();
                let max_line_w = lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
                let card = overlay_box(
                    Mode::Menu,
                    lines.len(),
                    max_line_w,
                    self.cols,
                    self.rows_n,
                    side,
                    top,
                    self.menu_anchor,
                );
                if overlay_contains(card, mouse.column, mouse.row, self.rows_n) {
                    let overlay_row = mouse.row.saturating_sub(card.y);
                    let items = self.menu.map(menu_items).unwrap_or(&[]);
                    if overlay_row >= 1 {
                        let idx = (overlay_row - 1) as usize;
                        if let Some((_, verb)) = items.get(idx) {
                            self.run_menu_verb(*verb);
                            return false;
                        }
                    }
                    self.close_menu();
                    return false;
                }
                self.close_menu();
                if btn == MouseButton::Right {
                    if let Some(target) = menu_hit(
                        &self.rows,
                        &self.cells,
                        mouse.column,
                        mouse.row,
                        self.rows_n,
                        side,
                        top,
                        self.zoomed,
                        &self.focused,
                        &self.workspace,
                    ) {
                        self.focus_then_open_menu(target, Some((mouse.column, mouse.row)));
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn focus_then_open_menu(&mut self, target: MenuTarget, at: Option<(u16, u16)>) {
        if !target.workspace.is_empty() {
            self.workspace = target.workspace.clone();
        }
        if !target.tab.is_empty() {
            self.tab = target.tab.clone();
        }
        if !target.focus.is_empty() && target.focus != self.focused {
            self.zoomed = false;
            self.focus_tile(&target.focus);
            self.reconcile_tiles();
        }
        self.menu_target = Some(target.clone());
        self.menu_anchor = at;
        self.open_menu(target.kind);
    }

    fn open_menu(&mut self, kind: MenuKind) {
        self.menu = Some(kind);
        self.mode = Mode::Menu;
        self.status.clear();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn close_menu(&mut self) {
        self.menu = None;
        self.menu_target = None;
        self.menu_anchor = None;
        self.mode = Mode::Terminal;
        self.status.clear();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn run_menu_verb(&mut self, verb: MenuVerb) {
        self.menu = None;
        self.menu_anchor = None;
        let target = self.menu_target.take();
        match verb {
            MenuVerb::SplitRight => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.split("right");
            }
            MenuVerb::SplitDown => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.split("down");
            }
            MenuVerb::Zoom => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.zoomed = !self.zoomed;
                self.drag = None;
                self.reconcile_tiles();
            }
            MenuVerb::ClosePane => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.close_pane_or_confirm();
            }
            MenuVerb::NewTab => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.new_tab();
            }
            MenuVerb::CloseTab => self.ask_confirm_target(self.confirm_for(ConfirmKind::Tab, target.as_ref())),
            MenuVerb::Picker => self.open_picker(),
            MenuVerb::NewWs => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.new_workspace();
            }
            MenuVerb::CloseWs => {
                self.ask_confirm_target(self.confirm_for(ConfirmKind::Workspace, target.as_ref()))
            }
        }
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn new_tab(&mut self) {
        if self.workspace.is_empty() {
            self.refresh_tree();
        }
        if self.workspace.is_empty() {
            self.status = "no window".to_string();
            return;
        }
        let body = match server::rpc_line_quiet(&format!(
            r#"{{"op":"tab.create","workspace":"{}"}}"#,
            self.workspace
        )) {
            Ok(b) => b,
            Err(_) => {
                self.status = "tab failed".to_string();
                return;
            }
        };
        self.zoomed = false;
        self.drag = None;
        if let Some(id) = pane_id_from(body.as_str()) {
            self.focus_tile(&id);
            self.tiles.clear();
            self.reconcile_tiles();
        } else {
            self.status = "tab: no pane".to_string();
            self.refresh_tree();
            self.reconcile_tiles();
        }
    }

    fn new_workspace(&mut self) {
        let line = match env::current_dir()
            .ok()
            .and_then(|p| p.into_os_string().into_string().ok())
            .filter(|s| !s.is_empty() && !s.contains(['"', '\\', '\n']))
        {
            Some(cwd) => format!(r#"{{"op":"workspace.create","cwd":"{cwd}"}}"#),
            None => r#"{"op":"workspace.create"}"#.to_string(),
        };
        let body = match server::rpc_line_quiet(&line) {
            Ok(b) => b,
            Err(_) => {
                self.status = "window failed".to_string();
                return;
            }
        };
        self.zoomed = false;
        self.drag = None;
        if let Some(id) = pane_id_from(body.as_str()) {
            self.focus_tile(&id);
            self.tiles.clear();
            self.reconcile_tiles();
        } else {
            self.status = "window: no pane".to_string();
            self.refresh_tree();
            self.reconcile_tiles();
        }
    }

    fn split(&mut self, direction: &str) {
        if self.focused.is_empty() {
            self.status = "no pane".to_string();
            return;
        }
        let body = match server::rpc_line_quiet(&format!(
            r#"{{"op":"pane.split","pane":"{}","direction":"{direction}","no_focus":false}}"#,
            self.focused
        )) {
            Ok(b) => b,
            Err(_) => {
                self.status = "split failed".to_string();
                return;
            }
        };
        self.zoomed = false;
        self.drag = None;
        if let Some(id) = pane_id_from(body.as_str()) {
            self.focus_tile(&id);
            self.reconcile_tiles();
        } else {
            self.status = "split: no pane".to_string();
            self.refresh_tree();
            self.reconcile_tiles();
        }
    }

    fn open_picker(&mut self) {
        let spaces = workspaces_of(&self.rows);
        self.picker_idx = spaces
            .iter()
            .position(|w| w == &self.workspace)
            .unwrap_or(0);
        self.mode = Mode::Picker;
        self.status.clear();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn toggle_sidebar(&mut self) {
        self.sidebar_cols = match self.sidebar_cols {
            SIDEBAR => SIDEBAR_DOTS,
            SIDEBAR_DOTS => 0,
            _ => SIDEBAR,
        };
        let (w, h) = pane_size(self.cols, self.rows_n, self.sidebar_cols, self.top_rows);
        self.pty_cols = w;
        self.pty_rows = h;
        self.reconcile_tiles();
        self.chrome_dirty = true;
    }

    fn next_tab(&mut self, dir: i32) {
        let tabs = tabs_of(&self.rows, &self.workspace);
        if tabs.is_empty() {
            return;
        }
        let cur = tabs.iter().position(|(id, _)| id == &self.tab).unwrap_or(0);
        let next = if dir < 0 {
            cur.checked_sub(1).unwrap_or(tabs.len() - 1)
        } else {
            (cur + 1) % tabs.len()
        };
        if let Some((_, pane)) = tabs.get(next) {
            self.zoomed = false;
            self.focus_tile(pane);
            self.reconcile_tiles();
        }
    }

    fn focus_tab_index(&mut self, idx: usize) {
        let tabs = tabs_of(&self.rows, &self.workspace);
        if let Some((_, pane)) = tabs.get(idx) {
            self.zoomed = false;
            self.focus_tile(pane);
            self.reconcile_tiles();
        }
    }

    fn confirm_here(&self, kind: ConfirmKind) -> Confirm {
        Confirm {
            kind,
            pane: self.focused.clone(),
            tab: self.tab.clone(),
            workspace: self.workspace.clone(),
        }
    }

    fn confirm_for(&self, kind: ConfirmKind, target: Option<&MenuTarget>) -> Confirm {
        confirm_from_target(self.confirm_here(kind), target)
    }

    fn ask_confirm(&mut self, kind: ConfirmKind) {
        self.ask_confirm_target(self.confirm_here(kind));
    }

    fn ask_confirm_target(&mut self, confirm: Confirm) {
        self.status.clear();
        self.confirm = Some(confirm);
        self.mode = Mode::Confirm;
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn close_pane_or_confirm(&mut self) {
        let tabs = tabs_of(&self.rows, &self.workspace);
        let panes_here = panes_of_tab(&self.rows, &self.tab);
        if panes_here <= 1 {
            if tabs.len() <= 1 {
                self.ask_confirm(ConfirmKind::Workspace);
            } else {
                self.ask_confirm(ConfirmKind::Tab);
            }
            return;
        }
        self.run_close(self.confirm_here(ConfirmKind::Pane));
    }

    fn run_close(&mut self, confirm: Confirm) {
        let Confirm {
            kind,
            pane,
            tab,
            workspace,
        } = confirm;
        let empty = match kind {
            ConfirmKind::Pane => pane.is_empty(),
            ConfirmKind::Tab => tab.is_empty(),
            ConfirmKind::Workspace => workspace.is_empty(),
        };
        if empty {
            self.status = "not closed".to_string();
            self.chrome_dirty = true;
            self.tiles_dirty = true;
            return;
        }
        let line = close_rpc_line(kind, &pane, &tab, &workspace);
        match server::rpc_line_quiet(&line) {
            Ok(body) if body.contains("\"ok\":true") => {
                self.retire_ids(kind, &pane, &tab, &workspace);
                self.forget_closed(kind, &pane, &tab, &workspace);
                self.status.clear();
            }
            Ok(body)
                if body.contains("unknown workspace")
                    || body.contains("unknown tab")
                    || body.contains("unknown pane") =>
            {
                self.retire_ids(kind, &pane, &tab, &workspace);
                self.forget_closed(kind, &pane, &tab, &workspace);
                self.status.clear();
            }
            Ok(body) => {
                self.status = if body.contains("last live pane") {
                    last_room_copy(kind).to_string()
                } else {
                    "not closed".to_string()
                };
            }
            Err(_) => self.status = "close failed".to_string(),
        }
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn retire_ids(&mut self, kind: ConfirmKind, pane: &str, tab: &str, workspace: &str) {
        match kind {
            ConfirmKind::Workspace => retire_id(&mut self.retired_ws, workspace),
            ConfirmKind::Tab => retire_id(&mut self.retired_tab, tab),
            ConfirmKind::Pane => retire_id(&mut self.retired_pane, pane),
        }
    }

    fn forget_closed(&mut self, kind: ConfirmKind, pane: &str, tab: &str, workspace: &str) {
        self.zoomed = false;
        self.drag = None;
        self.focused.clear();
        self.refresh_tree();
        drop_closed_from_rows(&mut self.rows, kind, pane, tab, workspace);
        self.repair_location();
        self.reconcile_tiles();
        self.chrome_dirty = true;
        self.tiles_dirty = true;
    }

    fn neighbor(&mut self, step: &str) {
        let from = self.focused.clone();
        let spatial = matches!(step, "left" | "right" | "up" | "down");
        let line = if spatial {
            format!(
                r#"{{"op":"desk.neighbor","from":"{from}","step":"{step}","cols":{},"rows":{}}}"#,
                self.pty_cols, self.pty_rows
            )
        } else {
            format!(r#"{{"op":"desk.neighbor","from":"{from}","step":"{step}"}}"#)
        };
        let body = match server::rpc_line_quiet(&line) {
            Ok(b) => b,
            Err(_) => return,
        };
        if let Some(id) = pane_id_from(body.as_str()) {
            if id != from {
                self.focus_tile(&id);
                // List-walk n/p may leave the tab. Always retile so keys follow
                // the focused stream; same-tab is cheap (ids match, resize skipped).
                self.reconcile_tiles();
            }
        }
    }

    fn draw(&mut self, out: &mut io::Stdout) -> io::Result<()> {
        let cols = self.cols.max(1);
        let rows = self.rows_n.max(3);
        queue!(out, Hide, ResetColor, BeginSynchronizedUpdate)?;
        if self.chrome_dirty {
            self.draw_title(out, cols)?;
            self.draw_tab_bar(out, cols)?;
            self.draw_sidebar(out, rows)?;
            self.draw_footer(out, cols, rows)?;
        } else if self.footer_dirty {
            self.draw_footer(out, cols, rows)?;
        }
        if self.tiles_dirty {
            self.draw_tiles(out, cols, rows)?;
        }
        if overlay_paints(self.mode) {
            self.draw_overlay(out, cols, rows)?;
        }
        self.place_cursor(out)?;
        queue!(out, EndSynchronizedUpdate)?;
        out.flush()
    }

    fn draw_title(&self, out: &mut io::Stdout, cols: u16) -> io::Result<()> {
        let loc = title_loc(&self.rows, &self.workspace, &self.tab, &self.focused);
        let chips = title_chips(self.mode, self.zoomed);
        let left = " dory";
        let line = bar_line(&format!("{left}{loc}{chips}"), cols);
        queue!(
            out,
            MoveTo(0, 0),
            SetBackgroundColor(TITLE_BG),
            SetForegroundColor(ACCENT),
            SetAttribute(Attribute::Bold),
            Print(&line[..left.len().min(line.len())]),
            SetAttribute(Attribute::Reset),
            SetBackgroundColor(TITLE_BG),
            SetForegroundColor(TEXT),
            Print(&line[left.len().min(line.len())..]),
            ResetColor
        )
    }

    fn draw_tab_bar(&self, out: &mut io::Stdout, cols: u16) -> io::Result<()> {
        let side = self.sidebar_cols;
        let mut line = String::new();
        if side > 0 {
            line.push_str(&" ".repeat(side as usize));
            line.push('│');
        }
        for (id, _) in tabs_of(&self.rows, &self.workspace) {
            line.push_str(&tab_chip_text(&self.rows, &self.workspace, &id, &self.tab));
        }
        let painted = bar_line(&line, cols);
        queue!(
            out,
            MoveTo(0, TAB_ROW),
            SetBackgroundColor(TITLE_BG),
            SetForegroundColor(TEXT),
            Print(painted),
            ResetColor
        )
    }

    fn draw_sidebar(&self, out: &mut io::Stdout, rows: u16) -> io::Result<()> {
        let side = self.sidebar_cols;
        if side == 0 {
            return Ok(());
        }
        let height = rows.saturating_sub(3);
        let hits = sidebar_model(&self.rows, &self.workspace, side, height);
        let ah = agent_region_rows(side, height, agents_from(&self.rows).len());
        for y in 0..height {
            let screen_y = y + self.top_rows;
            queue!(
                out,
                MoveTo(0, screen_y),
                SetBackgroundColor(sidebar_row_bg(y, height, ah))
            )?;
            match hits.get(y as usize) {
                Some(h) if matches!(h.kind, SideKind::Workspace | SideKind::Agent) => {
                    let (lead, lead_fg, mid, mid_fg, rest, rest_fg) =
                        sidebar_row_spans(h, side as usize, &self.workspace);
                    queue!(out, SetForegroundColor(lead_fg), Print(lead))?;
                    if !mid.is_empty() {
                        queue!(out, SetForegroundColor(mid_fg), Print(mid))?;
                    }
                    queue!(out, SetForegroundColor(rest_fg), Print(rest))?;
                }
                Some(h) => {
                    let fg = match h.kind {
                        SideKind::Chrome | SideKind::Rule => MUTED,
                        _ => TEXT,
                    };
                    queue!(
                        out,
                        SetForegroundColor(fg),
                        Print(sidebar_paint_text(h.kind, &h.text, side as usize))
                    )?;
                }
                None => {
                    queue!(
                        out,
                        SetForegroundColor(TEXT),
                        Print(pad_cols(String::new(), side as usize))
                    )?;
                }
            }
            queue!(
                out,
                SetBackgroundColor(TITLE_BG),
                SetForegroundColor(MUTED),
                Print("│"),
                ResetColor
            )?;
        }
        Ok(())
    }

    fn draw_tiles(&self, out: &mut io::Stdout, cols: u16, rows: u16) -> io::Result<()> {
        let (origin_x, origin_y) = content_origin(self.sidebar_cols, self.top_rows);
        let width = cols.saturating_sub(origin_x);
        let height = rows.saturating_sub(3);
        if width == 0 || height == 0 {
            return Ok(());
        }
        // Leave the last terminal column empty so a wrap-on-last-cell host
        // cannot scroll the alternate screen (one keystroke used to jump).
        // Wipe only on chrome/layout. A working occupant's PTY dirties tiles
        // every pump; blanking first flashes the sit, then the cell loop
        // reprints. Idle has no bytes → no draw → the sit looks still.
        let fill = width
            .min(cols.saturating_sub(origin_x).saturating_sub(1))
            .max(1);
        if pane_wipe_on_tile_draw(self.chrome_dirty) {
            for y in 0..height {
                queue!(
                    out,
                    MoveTo(origin_x, y + origin_y),
                    SetBackgroundColor(PANE_BG),
                    SetForegroundColor(PANE_BG),
                    Print(" ".repeat(fill as usize))
                )?;
            }
        }
        if self.tiles.is_empty() {
            let msg = if self.status.is_empty() {
                " empty pane  Ctrl-b c tab · v/- split"
            } else {
                self.status.as_str()
            };
            queue!(
                out,
                MoveTo(origin_x, origin_y),
                SetBackgroundColor(PANE_BG),
                SetForegroundColor(MUTED),
                Print(format!(" {msg}"))
            )?;
            queue!(out, ResetColor)?;
            return Ok(());
        }
        for tile in &self.tiles {
            if self.zoomed && tile.id != self.focused {
                continue;
            }
            let screen = tile.parser.screen();
            for y in 0..tile.h {
                if y >= height {
                    break;
                }
                queue!(out, MoveTo(origin_x + tile.x, y + origin_y + tile.y))?;
                for x in 0..tile.w {
                    if tile.x + x >= width || origin_x + tile.x + x + 1 >= cols {
                        break;
                    }
                    match screen.cell(y, x) {
                        Some(cell) => {
                            let (fg, bg) = cell_colors(cell);
                            let mut contents = cell.contents();
                            if contents.is_empty() {
                                contents = " ".to_string();
                            }
                            if cell.bold() {
                                queue!(out, SetAttribute(Attribute::Bold))?;
                            }
                            if cell.underline() {
                                queue!(out, SetAttribute(Attribute::Underlined))?;
                            }
                            queue!(
                                out,
                                SetForegroundColor(fg),
                                SetBackgroundColor(bg),
                                Print(contents)
                            )?;
                            if cell.bold() || cell.underline() {
                                queue!(out, SetAttribute(Attribute::Reset))?;
                            }
                        }
                        None => {
                            queue!(out, SetBackgroundColor(PANE_BG), Print(" "))?;
                        }
                    }
                }
            }
        }
        if !self.zoomed {
            for cell in &self.cells {
                if let Some((x, y, w, h)) = crate::layout::inset(&self.cells, &cell.id) {
                    if w < cell.w && x + w < width {
                        for row in y..y.saturating_add(h).min(height) {
                            if origin_x + x + w + 1 >= cols {
                                break;
                            }
                            let fg = if divider_touches_focus(
                                &self.cells,
                                x + w,
                                row,
                                &self.focused,
                            ) {
                                ACCENT
                            } else {
                                MUTED
                            };
                            queue!(
                                out,
                                MoveTo(origin_x + x + w, origin_y + row),
                                SetBackgroundColor(TITLE_BG),
                                SetForegroundColor(fg),
                                Print("│")
                            )?;
                        }
                    }
                    if h < cell.h && y + h < height && origin_x + x + 1 < cols {
                        let bar_w = w.min(width.saturating_sub(x));
                        let bar_w = if origin_x + x + bar_w >= cols {
                            cols.saturating_sub(origin_x + x + 1)
                        } else {
                            bar_w
                        };
                        if bar_w == 0 {
                            continue;
                        }
                        let fg = if divider_touches_focus(
                            &self.cells,
                            x,
                            y + h,
                            &self.focused,
                        ) {
                            ACCENT
                        } else {
                            MUTED
                        };
                        queue!(
                            out,
                            MoveTo(origin_x + x, origin_y + y + h),
                            SetBackgroundColor(TITLE_BG),
                            SetForegroundColor(fg),
                            Print("─".repeat(bar_w as usize))
                        )?;
                    }
                }
            }
        }
        queue!(out, ResetColor)?;
        Ok(())
    }

    fn draw_footer(&self, out: &mut io::Stdout, cols: u16, rows: u16) -> io::Result<()> {
        let hint = footer_line(self.mode, &self.status, self.flow_glance.as_deref());
        let line = bar_line(&format!(" {hint}"), cols);
        queue!(
            out,
            MoveTo(0, rows.saturating_sub(1)),
            SetBackgroundColor(TITLE_BG),
            SetForegroundColor(TEXT),
            Print(line),
            ResetColor
        )
    }

    fn draw_overlay(&self, out: &mut io::Stdout, cols: u16, rows: u16) -> io::Result<()> {
        let lines: Vec<String> = match self.mode {
            Mode::Help => help_text().lines().map(|s| s.to_string()).collect(),
            Mode::Picker => picker_lines(&self.rows, self.picker_idx),
            Mode::Menu => self.menu.map(menu_lines).unwrap_or_default(),
            Mode::Confirm => self
                .confirm
                .as_ref()
                .map(|c| confirm_lines(c.kind))
                .unwrap_or_default(),
            Mode::Onboard => onboard_lines(),
            Mode::Terminal | Mode::Prefix => return Ok(()),
        };
        let max_line_w = lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
        let card = overlay_box(
            self.mode,
            lines.len(),
            max_line_w,
            cols,
            rows,
            self.sidebar_cols,
            self.top_rows,
            if matches!(self.mode, Mode::Menu) {
                self.menu_anchor
            } else {
                None
            },
        );
        if card.w == 0 || card.h == 0 {
            return Ok(());
        }
        for y in 0..card.h {
            let mut text = lines
                .get(y as usize)
                .cloned()
                .unwrap_or_else(|| String::new());
            clip_to(&mut text, card.w as usize);
            while display_width(&text) < card.w as usize {
                text.push(' ');
            }
            queue!(
                out,
                MoveTo(card.x, card.y + y),
                SetBackgroundColor(SIDE_BG),
                SetForegroundColor(if y == 0 { FOCUSED_FG } else { TEXT }),
                Print(text),
                ResetColor
            )?;
        }
        Ok(())
    }

    fn place_cursor(&self, out: &mut io::Stdout) -> io::Result<()> {
        let Some(tile) = self.tiles.iter().find(|t| t.id == self.focused) else {
            queue!(out, Hide)?;
            return Ok(());
        };
        if tile.stream.is_none() || self.mode != Mode::Terminal {
            queue!(out, Hide)?;
            return Ok(());
        }
        let (origin_x, origin_y) = content_origin(self.sidebar_cols, self.top_rows);
        let (cy, cx) = tile.parser.screen().cursor_position();
        let x = origin_x + tile.x + cx;
        let y = origin_y + tile.y + cy;
        if x < self.cols && y + 1 < self.rows_n {
            queue!(out, MoveTo(x, y), Show)?;
        } else {
            queue!(out, Hide)?;
        }
        Ok(())
    }
}

fn cell_colors(cell: &vt100::Cell) -> (Color, Color) {
    let mut fg = map_color(cell.fgcolor(), TEXT);
    let mut bg = map_color(cell.bgcolor(), PANE_BG);
    if cell.inverse() {
        std::mem::swap(&mut fg, &mut bg);
    }
    (fg, bg)
}

fn map_color(c: vt100::Color, fallback: Color) -> Color {
    match c {
        vt100::Color::Default => fallback,
        vt100::Color::Idx(i) => Color::AnsiValue(i),
        vt100::Color::Rgb(r, g, b) => Color::Rgb { r, g, b },
    }
}

fn open_attach(
    pane: &str,
    cols: u16,
    rows: u16,
    no_focus: bool,
) -> Result<(UnixStream, Vec<u8>), String> {
    let focus = if no_focus { ",\"no_focus\":true" } else { "" };
    let op =
        format!(r#"{{"op":"pane.attach","pane":"{pane}","cols":{cols},"rows":{rows}{focus}}}"#);
    let stream = server::connect_for_attach().map_err(|_| "attach connect failed".to_string())?;
    let mut stream = stream;
    writeln!(stream, "{op}").map_err(|_| "attach write failed".to_string())?;
    let _ = stream.flush();
    let mut reader = BufReader::new(stream);
    let mut ack = String::new();
    reader
        .read_line(&mut ack)
        .map_err(|_| "attach handshake failed".to_string())?;
    if ack.trim().is_empty() || !ack.contains("\"ok\":true") {
        return Err(attach::json_string_field(&ack, "error")
            .unwrap_or_else(|| "attach refused".to_string()));
    }
    let leftover = reader.buffer().to_vec();
    Ok((reader.into_inner(), leftover))
}

fn parse_layout_cells(body: &str) -> Vec<crate::layout::Cell> {
    let Some(idx) = body.find("\"cells\"") else {
        return Vec::new();
    };
    let rest = body[idx + 7..].trim_start();
    let rest = rest.strip_prefix(':').unwrap_or(rest).trim_start();
    let rest = rest.strip_prefix('[').unwrap_or(rest);
    let mut out = Vec::new();
    let mut i = 0;
    let bytes = rest.as_bytes();
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b']' {
            break;
        }
        let Some(obj) = slice_object(&rest[i..]) else {
            break;
        };
        if let (Some(id), Some(x), Some(y), Some(w), Some(h)) = (
            attach::json_string_field(obj, "id"),
            json_u16_in(obj, "x"),
            json_u16_in(obj, "y"),
            json_u16_in(obj, "w"),
            json_u16_in(obj, "h"),
        ) {
            out.push(crate::layout::Cell { id, x, y, w, h });
        }
        i += obj.len();
    }
    out
}

fn json_u16_in(body: &str, key: &str) -> Option<u16> {
    let needle = format!("\"{key}\"");
    let idx = body.find(&needle)?;
    let mut rest = body[idx + needle.len()..].trim_start();
    rest = rest.strip_prefix(':')?.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn parse_tree(body: &str) -> (Vec<Row>, String, String, String) {
    let focused = attach::json_string_field(body, "focused").unwrap_or_default();
    let items = parse_items(body);
    let mut rows = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let focus_pane = if item.k == 'p' {
            item.id.clone()
        } else {
            items[i + 1..]
                .iter()
                .find(|n| n.k == 'p')
                .map(|n| n.id.clone())
                .unwrap_or_default()
        };
        rows.push(Row {
            kind: item.k,
            id: item.id.clone(),
            focus_pane,
            occ: item.occ.clone(),
            st: item.st.clone(),
            cwd: item.cwd.clone(),
        });
    }
    let workspace = workspace_of(&rows, &focused).unwrap_or_default();
    let tab = tab_of(&rows, &focused).unwrap_or_default();
    (rows, focused, workspace, tab)
}

struct Item {
    k: char,
    id: String,
    occ: String,
    st: String,
    cwd: String,
}

fn parse_items(body: &str) -> Vec<Item> {
    let Some(idx) = body.find("\"items\"") else {
        return Vec::new();
    };
    let rest = body[idx + 7..].trim_start();
    let rest = rest.strip_prefix(':').unwrap_or(rest).trim_start();
    let rest = rest.strip_prefix('[').unwrap_or(rest);
    let mut out = Vec::new();
    let mut i = 0;
    let bytes = rest.as_bytes();
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b']' {
            break;
        }
        let Some(obj) = slice_object(&rest[i..]) else {
            break;
        };
        if let Some(item) = parse_item(obj) {
            out.push(item);
        }
        i += obj.len();
    }
    out
}

fn slice_object(s: &str) -> Option<&str> {
    if !s.starts_with('{') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    for (i, c) in s.char_indices() {
        if in_str {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[..=i]);
                }
            }
            _ => {}
        }
    }
    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FlowGlance {
    code: Option<i32>,
    arg0: String,
    error: Option<String>,
}

struct Jc<'a> {
    inner: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Jc<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            inner: s.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.inner.peek().copied()
    }

    fn bump(&mut self) -> Option<char> {
        self.inner.next()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.bump();
        }
    }

    fn eat(&mut self, want: char) -> Option<()> {
        if self.peek() == Some(want) {
            self.bump();
            Some(())
        } else {
            None
        }
    }

    fn eat_lit(&mut self, lit: &str) -> Option<()> {
        for c in lit.chars() {
            self.eat(c)?;
        }
        Some(())
    }

    fn hex4(&mut self) -> Option<u32> {
        let mut n = 0u32;
        for _ in 0..4 {
            let d = self.bump()?;
            n = (n << 4)
                + match d {
                    '0'..='9' => u32::from(d as u8 - b'0'),
                    'a'..='f' => u32::from(d as u8 - b'a' + 10),
                    'A'..='F' => u32::from(d as u8 - b'A' + 10),
                    _ => return None,
                };
        }
        Some(n)
    }

    fn string(&mut self) -> Option<String> {
        self.eat('"')?;
        let mut out = String::new();
        loop {
            let c = self.bump()?;
            match c {
                '"' => return Some(out),
                '\\' => match self.bump()? {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    'u' => {
                        let n = self.hex4()?;
                        out.push(char::from_u32(n)?);
                    }
                    _ => return None,
                },
                c if (c as u32) >= 0x20 => out.push(c),
                _ => return None,
            }
        }
    }

    fn skip_number(&mut self) -> Option<()> {
        let _ = self.eat('-');
        let mut any = false;
        while matches!(self.peek(), Some('0'..='9')) {
            any = true;
            self.bump();
        }
        if self.peek() == Some('.') {
            self.bump();
            while matches!(self.peek(), Some('0'..='9')) {
                self.bump();
            }
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            self.bump();
            let _ = self.eat('+');
            let _ = self.eat('-');
            while matches!(self.peek(), Some('0'..='9')) {
                self.bump();
            }
        }
        if any { Some(()) } else { None }
    }

    fn i32_or_null(&mut self) -> Option<Option<i32>> {
        self.skip_ws();
        if self.peek() == Some('n') {
            self.eat_lit("null")?;
            return Some(None);
        }
        let neg = self.eat('-').is_some();
        let mut raw = String::new();
        while matches!(self.peek(), Some('0'..='9')) {
            raw.push(self.bump()?);
        }
        if raw.is_empty() {
            return None;
        }
        let mut n: i32 = raw.parse().ok()?;
        if neg {
            n = n.checked_neg()?;
        }
        Some(Some(n))
    }

    fn skip_value(&mut self) -> Option<()> {
        self.skip_ws();
        match self.peek()? {
            '"' => {
                self.string()?;
                Some(())
            }
            '{' => self.skip_object(),
            '[' => self.skip_array(),
            't' => self.eat_lit("true"),
            'f' => self.eat_lit("false"),
            'n' => self.eat_lit("null"),
            '-' | '0'..='9' => self.skip_number(),
            _ => None,
        }
    }

    fn skip_object(&mut self) -> Option<()> {
        self.eat('{')?;
        loop {
            self.skip_ws();
            if self.eat('}').is_some() {
                return Some(());
            }
            self.string()?;
            self.skip_ws();
            self.eat(':')?;
            self.skip_value()?;
            self.skip_ws();
            if self.eat(',').is_some() {
                continue;
            }
            self.eat('}')?;
            return Some(());
        }
    }

    fn skip_array(&mut self) -> Option<()> {
        self.eat('[')?;
        loop {
            self.skip_ws();
            if self.eat(']').is_some() {
                return Some(());
            }
            self.skip_value()?;
            self.skip_ws();
            if self.eat(',').is_some() {
                continue;
            }
            self.eat(']')?;
            return Some(());
        }
    }

    fn first_array_string(&mut self) -> Option<String> {
        self.skip_ws();
        self.eat('[')?;
        self.skip_ws();
        if self.eat(']').is_some() {
            return Some(String::new());
        }
        let first = if self.peek() == Some('"') {
            self.string()?
        } else {
            self.skip_value()?;
            String::new()
        };
        loop {
            self.skip_ws();
            if self.eat(']').is_some() {
                return Some(first);
            }
            if self.eat(',').is_some() {
                self.skip_value()?;
                continue;
            }
            return None;
        }
    }
}

fn with_top_value<T>(obj: &str, key: &str, f: impl FnOnce(&mut Jc<'_>) -> Option<T>) -> Option<T> {
    let mut jc = Jc::new(obj);
    jc.skip_ws();
    jc.eat('{')?;
    loop {
        jc.skip_ws();
        if jc.eat('}').is_some() {
            return None;
        }
        let k = jc.string()?;
        jc.skip_ws();
        jc.eat(':')?;
        jc.skip_ws();
        if k == key {
            return f(&mut jc);
        }
        jc.skip_value()?;
        jc.skip_ws();
        if jc.eat(',').is_some() {
            continue;
        }
        jc.eat('}')?;
        return None;
    }
}

fn top_json_string(obj: &str, key: &str) -> Option<String> {
    with_top_value(obj, key, |jc| match jc.peek()? {
        '"' => jc.string(),
        'n' => {
            jc.eat_lit("null")?;
            None
        }
        _ => None,
    })
}

fn top_json_i32(obj: &str, key: &str) -> Option<i32> {
    with_top_value(obj, key, |jc| jc.i32_or_null())?
}

fn top_json_first_arg(obj: &str) -> Option<String> {
    with_top_value(obj, "args", |jc| jc.first_array_string())
}

fn flow_glance_from_obj(obj: &str) -> Option<FlowGlance> {
    let mut jc = Jc::new(obj);
    jc.skip_ws();
    jc.eat('{')?;
    let mut typ = None;
    loop {
        jc.skip_ws();
        if jc.eat('}').is_some() {
            break;
        }
        let key = jc.string()?;
        jc.skip_ws();
        jc.eat(':')?;
        jc.skip_ws();
        if key == "type" {
            typ = Some(jc.string()?);
        } else {
            jc.skip_value()?;
        }
        jc.skip_ws();
        if jc.eat(',').is_some() {
            continue;
        }
        if jc.eat('}').is_some() {
            break;
        }
        return None;
    }
    if typ.as_deref() != Some("flow/result") {
        return None;
    }
    Some(FlowGlance {
        code: top_json_i32(obj, "code"),
        arg0: top_json_first_arg(obj).unwrap_or_default(),
        error: top_json_string(obj, "error").filter(|s| !s.is_empty()),
    })
}

fn last_flow_result_bytes(buf: &[u8]) -> Option<FlowGlance> {
    for raw in buf.rsplit(|&b| b == b'\n') {
        if raw.is_empty() {
            continue;
        }
        let Ok(text) = std::str::from_utf8(raw) else {
            continue;
        };
        let Some(obj) = slice_object(text.trim_start()) else {
            continue;
        };
        if let Some(g) = flow_glance_from_obj(obj) {
            return Some(g);
        }
    }
    None
}

fn parse_item(obj: &str) -> Option<Item> {
    let k = attach::json_string_field(obj, "k")?;
    let id = attach::json_string_field(obj, "id")?;
    Some(Item {
        k: k.chars().next().unwrap_or('p'),
        id,
        occ: attach::json_string_field(obj, "occ").unwrap_or_default(),
        st: attach::json_string_field(obj, "st").unwrap_or_default(),
        cwd: attach::json_string_field(obj, "cwd").unwrap_or_default(),
    })
}

fn workspace_of(rows: &[Row], focused: &str) -> Option<String> {
    let mut ws = None;
    for row in rows {
        if row.kind == 'w' {
            ws = Some(row.id.clone());
        }
        if row.kind == 'p' && row.id == focused {
            return ws;
        }
    }
    None
}

fn tab_of(rows: &[Row], focused: &str) -> Option<String> {
    let mut tab = None;
    for row in rows {
        if row.kind == 't' {
            tab = Some(row.id.clone());
        }
        if row.kind == 'p' && row.id == focused {
            return tab;
        }
    }
    None
}

fn first_live_pane(rows: &[Row]) -> Option<String> {
    rows.iter().find(|r| r.kind == 'p').map(|r| r.id.clone())
}

#[cfg(test)]
fn closed_still_listed(
    rows: &[Row],
    kind: ConfirmKind,
    pane: &str,
    tab: &str,
    workspace: &str,
) -> bool {
    match kind {
        ConfirmKind::Pane => rows.iter().any(|r| r.kind == 'p' && r.id == pane),
        ConfirmKind::Tab => rows.iter().any(|r| r.kind == 't' && r.id == tab),
        ConfirmKind::Workspace => rows.iter().any(|r| r.kind == 'w' && r.id == workspace),
    }
}

fn drop_closed_from_rows(
    rows: &mut Vec<Row>,
    kind: ConfirmKind,
    pane: &str,
    tab: &str,
    workspace: &str,
) {
    match kind {
        ConfirmKind::Pane => rows.retain(|r| !(r.kind == 'p' && r.id == pane)),
        ConfirmKind::Tab => {
            let mut in_tab = false;
            rows.retain(|r| {
                if r.kind == 'w' {
                    in_tab = false;
                    return true;
                }
                if r.kind == 't' {
                    in_tab = r.id == tab;
                    return !in_tab;
                }
                !in_tab
            });
        }
        ConfirmKind::Workspace => {
            let mut in_ws = false;
            rows.retain(|r| {
                if r.kind == 'w' {
                    in_ws = r.id == workspace;
                    return !in_ws;
                }
                !in_ws
            });
        }
    }
    prune_empty_rooms(rows);
}

fn prune_empty_rooms(rows: &mut Vec<Row>) {
    let mut live_ws: Vec<String> = Vec::new();
    let mut live_tab: Vec<String> = Vec::new();
    let mut ws = String::new();
    let mut tab = String::new();
    for row in rows.iter() {
        match row.kind {
            'w' => ws = row.id.clone(),
            't' => tab = row.id.clone(),
            'p' => {
                if !ws.is_empty() && !live_ws.iter().any(|id| id == &ws) {
                    live_ws.push(ws.clone());
                }
                if !tab.is_empty() && !live_tab.iter().any(|id| id == &tab) {
                    live_tab.push(tab.clone());
                }
            }
            _ => {}
        }
    }
    rows.retain(|row| match row.kind {
        'w' => live_ws.iter().any(|id| id == &row.id),
        't' => live_tab.iter().any(|id| id == &row.id),
        _ => true,
    });
}

fn retire_id(list: &mut Vec<String>, id: &str) {
    if id.is_empty() || list.iter().any(|have| have == id) {
        return;
    }
    list.push(id.to_string());
}

fn sweep_retired(
    rows: &[Row],
    retired_ws: &mut Vec<String>,
    retired_tab: &mut Vec<String>,
    retired_pane: &mut Vec<String>,
) {
    retired_ws.retain(|id| rows.iter().any(|r| r.kind == 'w' && r.id == *id));
    retired_tab.retain(|id| rows.iter().any(|r| r.kind == 't' && r.id == *id));
    retired_pane.retain(|id| rows.iter().any(|r| r.kind == 'p' && r.id == *id));
}

fn apply_retired(
    rows: &mut Vec<Row>,
    retired_ws: &[String],
    retired_tab: &[String],
    retired_pane: &[String],
) {
    for ws in retired_ws {
        drop_closed_from_rows(rows, ConfirmKind::Workspace, "", "", ws);
    }
    for tab in retired_tab {
        drop_closed_from_rows(rows, ConfirmKind::Tab, "", tab, "");
    }
    for pane in retired_pane {
        drop_closed_from_rows(rows, ConfirmKind::Pane, pane, "", "");
    }
}

fn pane_id_from(body: &str) -> Option<String> {
    if !body.contains("\"ok\":true") {
        return None;
    }
    for key in ["\"pane\"", "\"root_pane\""] {
        if let Some(idx) = body.find(key) {
            if let Some(id) = attach::json_string_field(&body[idx..], "id") {
                if id.contains(":p") || id.contains('p') && id.contains(':') {
                    return Some(id);
                }
                if id.contains(':') && !id.contains(":t") {
                    return Some(id);
                }
            }
        }
    }
    attach::json_string_field(body, "id").filter(|id| id.contains(":p"))
}

fn content_origin(sidebar: u16, top: u16) -> (u16, u16) {
    let x = if sidebar == 0 { 0 } else { sidebar + 1 };
    (x, top)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OverlayBox {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

fn overlay_box(
    mode: Mode,
    line_count: usize,
    max_line_w: usize,
    cols: u16,
    rows: u16,
    sidebar: u16,
    top: u16,
    menu_anchor: Option<(u16, u16)>,
) -> OverlayBox {
    let height = rows.saturating_sub(top + 1);
    let h = overlay_paint_rows(mode, line_count, height);
    if matches!(mode, Mode::Menu) {
        if let Some((ax, ay)) = menu_anchor {
            let max_w = cols.saturating_sub(1);
            let w = (max_line_w as u16).max(8).min(max_w);
            let max_x = cols.saturating_sub(w.saturating_add(1));
            let max_y = rows.saturating_sub(h.saturating_add(1));
            return OverlayBox {
                x: ax.min(max_x),
                y: ay.min(max_y),
                w,
                h,
            };
        }
    }
    let (x, y) = content_origin(sidebar, top);
    let cap = cols.saturating_sub(x).saturating_sub(1);
    let w = if matches!(mode, Mode::Confirm | Mode::Picker) {
        (max_line_w as u16).max(8).min(cap)
    } else {
        cap
    };
    OverlayBox { x, y, w, h }
}

fn overlay_contains(card: OverlayBox, column: u16, row: u16, rows_n: u16) -> bool {
    if row + 1 >= rows_n {
        return false;
    }
    column >= card.x
        && column < card.x.saturating_add(card.w)
        && row >= card.y
        && row < card.y.saturating_add(card.h)
}

fn pane_wipe_on_tile_draw(chrome_dirty: bool) -> bool {
    chrome_dirty
}

fn pane_size(cols: u16, rows: u16, sidebar: u16, top: u16) -> (u16, u16) {
    let (origin_x, _) = content_origin(sidebar, top);
    let w = cols.saturating_sub(origin_x + 1).max(8);
    let h = rows.saturating_sub(top + 1).max(3);
    (w, h)
}

fn wanted_rects(
    cells: &[crate::layout::Cell],
    focused: &str,
    zoomed: bool,
    full_w: u16,
    full_h: u16,
) -> Vec<(String, u16, u16, u16, u16)> {
    cells
        .iter()
        .map(|c| {
            if zoomed && c.id == focused {
                (c.id.clone(), 0, 0, full_w.max(1), full_h.max(1))
            } else {
                let (x, y, w, h) =
                    crate::layout::inset(cells, &c.id).unwrap_or((c.x, c.y, c.w, c.h));
                (c.id.clone(), x, y, w.max(1), h.max(1))
            }
        })
        .collect()
}

fn rollup_rank(st: &str) -> u8 {
    match st {
        "blocked" => 5,
        "working" => 4,
        "done" => 3,
        "idle" => 2,
        _ => 1,
    }
}

fn normalize_st(st: &str) -> &'static str {
    match st {
        "blocked" => "blocked",
        "working" => "working",
        "done" => "done",
        "idle" => "idle",
        _ => "unknown",
    }
}

fn tree_rows_sig(focused: &str, rows: &[Row]) -> String {
    let mut s = focused.to_string();
    for row in rows {
        s.push('|');
        s.push_str(&row.id);
        s.push('/');
        s.push_str(&row.st);
        s.push('/');
        s.push_str(&row.occ);
        s.push('/');
        s.push_str(&row.cwd);
    }
    s
}

fn rollup_of(rows: &[Row], ws: &str) -> &'static str {
    let mut best = "";
    let mut best_r = 0u8;
    let mut in_ws = false;
    for row in rows {
        if row.kind == 'w' {
            in_ws = row.id == ws;
            continue;
        }
        if in_ws && row.kind == 'p' && !row.occ.is_empty() {
            let st = normalize_st(&row.st);
            let rank = rollup_rank(st);
            if rank > best_r {
                best_r = rank;
                best = st;
            }
        }
    }
    best
}

fn workspace_cwd_name(cwd: &str) -> Option<String> {
    let name = Path::new(cwd).file_name()?.to_string_lossy();
    if name.is_empty() {
        None
    } else {
        Some(name.into_owned())
    }
}

fn short_id(id: &str) -> &str {
    id.rsplit(':').next().unwrap_or(id)
}

fn row_cwd<'a>(rows: &'a [Row], kind: char, id: &str) -> &'a str {
    rows.iter()
        .find(|r| r.kind == kind && r.id == id)
        .map(|r| r.cwd.as_str())
        .unwrap_or("")
}

fn workspace_cwd<'a>(rows: &'a [Row], workspace: &str) -> Option<&'a str> {
    rows.iter()
        .find(|r| r.kind == 'w' && r.id == workspace && !r.cwd.is_empty())
        .map(|r| r.cwd.as_str())
}

fn flow_journal_path(cwd: &str) -> PathBuf {
    Path::new(cwd)
        .join(".dory")
        .join("sessions")
        .join("s1.jsonl")
}

fn folder_label(cwd: &str, id: &str) -> String {
    workspace_cwd_name(cwd).unwrap_or_else(|| short_id(id).to_string())
}

fn unique_folder_label(id: &str, labels: &[(String, String)]) -> String {
    let name = labels
        .iter()
        .find(|(i, _)| i == id)
        .map(|(_, n)| n.as_str())
        .unwrap_or_else(|| short_id(id));
    if labels.iter().any(|(i, n)| i != id && n == name) {
        format!("{name} {}", short_id(id))
    } else {
        name.to_string()
    }
}

fn workspace_labels(rows: &[Row]) -> Vec<(String, String)> {
    workspaces_of(rows)
        .into_iter()
        .map(|id| {
            let name = folder_label(row_cwd(rows, 'w', &id), &id);
            (id, name)
        })
        .collect()
}

fn tab_labels(rows: &[Row], workspace: &str) -> Vec<(String, String)> {
    tabs_of(rows, workspace)
        .into_iter()
        .map(|(id, _)| {
            let name = folder_label(row_cwd(rows, 't', &id), &id);
            (id, name)
        })
        .collect()
}

fn workspace_label(rows: &[Row], id: &str) -> String {
    unique_folder_label(id, &workspace_labels(rows))
}

fn tab_label(rows: &[Row], workspace: &str, id: &str) -> String {
    let mut name = unique_folder_label(id, &tab_labels(rows, workspace));
    clip_to(&mut name, TAB_CHIP_MAX);
    name
}

fn tab_chip_text(rows: &[Row], workspace: &str, id: &str, focused: &str) -> String {
    let label = tab_label(rows, workspace, id);
    if id == focused {
        format!("[{label}]")
    } else {
        format!(" {label} ")
    }
}

fn title_loc(rows: &[Row], workspace: &str, tab: &str, focused: &str) -> String {
    format!(
        "  {} · {} · {}",
        empty_dash(&workspace_label(rows, workspace)),
        empty_dash(&tab_label(rows, workspace, tab)),
        empty_dash(short_id(focused))
    )
}

fn title_chips(mode: Mode, zoomed: bool) -> String {
    let mut s = String::new();
    if mode == Mode::Prefix {
        s.push_str("  Ctrl-b");
    }
    if zoomed {
        s.push_str("  z");
    }
    s
}

fn divider_touches_focus(cells: &[crate::layout::Cell], x: u16, y: u16, focused: &str) -> bool {
    match crate::layout::divider_at(cells, x, y) {
        Some((a, b, _)) => a == focused || b == focused,
        None => false,
    }
}

fn workspaces_of(rows: &[Row]) -> Vec<String> {
    rows.iter()
        .filter(|r| r.kind == 'w')
        .map(|r| r.id.clone())
        .collect()
}

fn first_pane_of(rows: &[Row], ws: &str) -> Option<String> {
    let mut in_ws = false;
    for row in rows {
        if row.kind == 'w' {
            in_ws = row.id == ws;
        } else if in_ws && row.kind == 'p' {
            return Some(row.id.clone());
        }
    }
    None
}

fn tabs_of(rows: &[Row], ws: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut in_ws = false;
    let mut pending: Option<String> = None;
    for row in rows {
        if row.kind == 'w' {
            in_ws = row.id == ws;
            pending = None;
            continue;
        }
        if !in_ws {
            continue;
        }
        if row.kind == 't' {
            pending = Some(row.id.clone());
        } else if row.kind == 'p' {
            if let Some(tab) = pending.take() {
                out.push((tab, row.id.clone()));
            }
        }
    }
    out
}

fn panes_of_tab(rows: &[Row], tab: &str) -> usize {
    let mut count = 0;
    let mut in_tab = false;
    for row in rows {
        if row.kind == 'w' && in_tab {
            break;
        }
        if row.kind == 't' {
            in_tab = row.id == tab;
            continue;
        }
        if in_tab && row.kind == 'p' {
            count += 1;
        }
    }
    count
}

fn agents_from(rows: &[Row]) -> Vec<&Row> {
    let mut agents: Vec<&Row> = rows
        .iter()
        .filter(|r| r.kind == 'p' && !r.occ.is_empty())
        .collect();
    agents.sort_by_key(|r| std::cmp::Reverse(rollup_rank(normalize_st(&r.st))));
    agents
}

fn pad_cols(mut text: String, width: usize) -> String {
    clip_to(&mut text, width);
    while display_width(&text) < width {
        text.push(' ');
    }
    text
}

fn sidebar_rule(width: usize) -> String {
    "─".chars().cycle().take(width).collect()
}

fn sidebar_paint_text(kind: SideKind, text: &str, width: usize) -> String {
    if kind == SideKind::Rule {
        sidebar_rule(width)
    } else {
        pad_cols(text.to_string(), width)
    }
}

fn sidebar_row_bg(y: u16, height: u16, agent_rows: u16) -> Color {
    if agent_rows > 0 && y >= height.saturating_sub(agent_rows) {
        TITLE_BG
    } else {
        SIDE_BG
    }
}

#[derive(Clone)]
struct SideHit {
    text: String,
    pane: Option<String>,
    kind: SideKind,
    tab: String,
    workspace: String,
    st: &'static str,
    lead: String,
    tail: String,
}

fn show_status_word(st: &str) -> bool {
    !st.is_empty() && st != "idle"
}

fn status_fg(st: &str) -> Color {
    match st {
        "blocked" => BLOCKED_FG,
        "working" => ACCENT,
        _ => MUTED,
    }
}

fn compact_space_ch(st: &str) -> char {
    match st {
        "blocked" => 'B',
        "working" => 'W',
        "done" => 'D',
        "idle" => 'I',
        "" => '·',
        _ => 'U',
    }
}

fn sidebar_workspace_focused(kind: SideKind, hit_ws: &str, desk_ws: &str) -> bool {
    kind == SideKind::Workspace && !desk_ws.is_empty() && hit_ws == desk_ws
}

fn pad_display(s: &mut String, width: usize) {
    while display_width(s) < width {
        s.push(' ');
    }
    clip_to(s, width);
}

fn clip_lead(lead: &str, st: &str, tail: &str, width: usize) -> (String, String) {
    let compact = width <= SIDEBAR_DOTS as usize;
    let mid = if !compact && show_status_word(st) {
        format!(" {st}")
    } else {
        String::new()
    };
    let reserve = display_width(&mid) + display_width(tail);
    let budget = width.saturating_sub(reserve);
    let mut lead_c = lead.to_string();
    clip_to(&mut lead_c, budget);
    (lead_c, mid)
}

fn chrome_side_hit(kind: SideKind, raw: &str, width: usize) -> SideHit {
    SideHit {
        text: sidebar_paint_text(kind, raw, width),
        pane: None,
        kind,
        tab: String::new(),
        workspace: String::new(),
        st: "",
        lead: String::new(),
        tail: String::new(),
    }
}

fn occup_side_hit(
    kind: SideKind,
    pane: Option<String>,
    tab: &str,
    ws: &str,
    st: &'static str,
    lead: String,
    tail: String,
    width: usize,
) -> SideHit {
    let (lead_c, mid) = clip_lead(&lead, st, &tail, width);
    let mut text = format!("{lead_c}{mid}{tail}");
    pad_display(&mut text, width);
    SideHit {
        text,
        pane,
        kind,
        tab: tab.to_string(),
        workspace: ws.to_string(),
        st,
        lead: lead_c,
        tail,
    }
}

fn sidebar_row_spans(
    hit: &SideHit,
    side: usize,
    desk_ws: &str,
) -> (String, Color, String, Color, String, Color) {
    let compact = side <= SIDEBAR_DOTS as usize;
    let mid = if !compact && show_status_word(hit.st) {
        format!(" {}", hit.st)
    } else {
        String::new()
    };
    let focused = sidebar_workspace_focused(hit.kind, &hit.workspace, desk_ws);
    let lead_fg = if compact {
        status_fg(hit.st)
    } else if focused {
        FOCUSED_FG
    } else {
        TEXT
    };
    let mut rest = hit.tail.clone();
    let used = display_width(&hit.lead) + display_width(&mid) + display_width(&rest);
    let pad = side.saturating_sub(used);
    rest.push_str(&" ".repeat(pad));
    (
        hit.lead.clone(),
        lead_fg,
        mid,
        status_fg(hit.st),
        rest,
        TEXT,
    )
}

fn sidebar_paint_height(rows_n: u16) -> u16 {
    rows_n.saturating_sub(3)
}

fn agent_region_rows(width: u16, height: u16, agent_n: usize) -> u16 {
    if agent_n == 0 || height == 0 {
        return 0;
    }
    let chrome: u16 = if width <= SIDEBAR_DOTS { 1 } else { 2 };
    let cap = if width <= SIDEBAR_DOTS {
        AGENT_REGION_DOTS
    } else {
        AGENT_REGION
    };
    let want = chrome.saturating_add(agent_n as u16).min(cap);
    let need = chrome.saturating_add(1).min(height);
    let leave = if height > need {
        2u16.min(height - need)
    } else {
        0
    };
    want.min(height.saturating_sub(leave)).min(height)
}

fn sidebar_blank(width: usize) -> SideHit {
    chrome_side_hit(SideKind::Chrome, "", width)
}

fn sidebar_sections(rows: &[Row], workspace: &str, side: u16) -> (Vec<SideHit>, Vec<SideHit>) {
    let width = side as usize;
    let mut spaces = Vec::new();
    let mut agents_hits = Vec::new();
    if width <= SIDEBAR_DOTS as usize {
        for ws in workspaces_of(rows) {
            let st = rollup_of(rows, &ws);
            spaces.push(occup_side_hit(
                SideKind::Workspace,
                first_pane_of(rows, &ws),
                &first_tab_of(rows, &ws),
                &ws,
                st,
                format!(" {}", compact_space_ch(st)),
                String::new(),
                width,
            ));
        }
        let agents = agents_from(rows);
        if !agents.is_empty() {
            agents_hits.push(chrome_side_hit(SideKind::Rule, "", width));
            for pane in agents {
                let st = normalize_st(&pane.st);
                let ch = pane.occ.chars().next().unwrap_or('·');
                agents_hits.push(occup_side_hit(
                    SideKind::Agent,
                    Some(pane.id.clone()),
                    "",
                    "",
                    st,
                    format!(" {ch}"),
                    String::new(),
                    width,
                ));
            }
        }
        return (spaces, agents_hits);
    }
    spaces.push(chrome_side_hit(SideKind::Chrome, " Spaces", width));
    for ws in workspaces_of(rows) {
        let st = rollup_of(rows, &ws);
        let mark = if ws == workspace { "●" } else { "○" };
        let label = workspace_label(rows, &ws);
        spaces.push(occup_side_hit(
            SideKind::Workspace,
            first_pane_of(rows, &ws),
            &first_tab_of(rows, &ws),
            &ws,
            st,
            format!(" {mark} {label}"),
            String::new(),
            width,
        ));
    }
    let agents = agents_from(rows);
    if !agents.is_empty() {
        agents_hits.push(chrome_side_hit(SideKind::Rule, "", width));
        agents_hits.push(chrome_side_hit(SideKind::Chrome, " Agents", width));
        for pane in agents {
            let st = normalize_st(&pane.st);
            let short = pane.id.rsplit(':').next().unwrap_or(&pane.id);
            agents_hits.push(occup_side_hit(
                SideKind::Agent,
                Some(pane.id.clone()),
                "",
                "",
                st,
                format!(" {}", pane.occ),
                format!(" {short}"),
                width,
            ));
        }
    }
    (spaces, agents_hits)
}

fn sidebar_model(rows: &[Row], workspace: &str, side: u16, height: u16) -> Vec<SideHit> {
    let width = side as usize;
    if width == 0 {
        return Vec::new();
    }
    let (spaces, agents) = sidebar_sections(rows, workspace, side);
    if height == 0 {
        let mut out = spaces;
        out.extend(agents);
        return out;
    }
    let ah = agent_region_rows(side, height, agents_from(rows).len());
    let sh = height.saturating_sub(ah);
    let mut out = Vec::with_capacity(height as usize);
    for i in 0..sh as usize {
        out.push(spaces.get(i).cloned().unwrap_or_else(|| sidebar_blank(width)));
    }
    for i in 0..ah as usize {
        out.push(agents.get(i).cloned().unwrap_or_else(|| sidebar_blank(width)));
    }
    out
}

fn sidebar_focus_at(
    rows: &[Row],
    mouse_row: u16,
    rows_n: u16,
    workspace: &str,
    side: u16,
) -> Option<String> {
    if side == 0 || mouse_row < 2 || mouse_row + 1 >= rows_n {
        return None;
    }
    let idx = (mouse_row - 2) as usize;
    sidebar_model(rows, workspace, side, sidebar_paint_height(rows_n))
        .get(idx)
        .and_then(|h| h.pane.clone())
}

fn first_tab_of(rows: &[Row], ws: &str) -> String {
    tabs_of(rows, ws)
        .into_iter()
        .next()
        .map(|(id, _)| id)
        .unwrap_or_default()
}

fn tab_chip_at(rows: &[Row], workspace: &str, column: u16, side: u16) -> Option<String> {
    tab_chip_hit(rows, workspace, column, side).map(|(_, pane)| pane)
}

fn tab_chip_hit(rows: &[Row], workspace: &str, column: u16, side: u16) -> Option<(String, String)> {
    let mut x = if side == 0 { 0 } else { side + 1 };
    for (id, pane) in tabs_of(rows, workspace) {
        let width = display_width(&tab_chip_text(rows, workspace, &id, &id)) as u16;
        if column >= x && column < x + width {
            return Some((id, pane));
        }
        x = x.saturating_add(width);
    }
    None
}

fn menu_items(kind: MenuKind) -> &'static [(&'static str, MenuVerb)] {
    match kind {
        MenuKind::Pane => &[
            ("Split right", MenuVerb::SplitRight),
            ("Split down", MenuVerb::SplitDown),
            ("Zoom", MenuVerb::Zoom),
            ("Close pane", MenuVerb::ClosePane),
        ],
        MenuKind::Tab => &[
            ("New tab", MenuVerb::NewTab),
            ("Close tab", MenuVerb::CloseTab),
        ],
        MenuKind::Workspace => &[
            ("Pick window", MenuVerb::Picker),
            ("New window", MenuVerb::NewWs),
            ("Close window", MenuVerb::CloseWs),
        ],
    }
}

fn menu_pick(kind: MenuKind, code: KeyCode) -> MenuPick {
    match code {
        KeyCode::Esc => MenuPick::Cancel,
        KeyCode::Char(c @ '1'..='9') => {
            let i = (c as u8 - b'1') as usize;
            match menu_items(kind).get(i) {
                Some((_, verb)) => MenuPick::Run(*verb),
                None => MenuPick::Ignore,
            }
        }
        _ => MenuPick::Ignore,
    }
}

fn overlay_paint_rows(mode: Mode, line_count: usize, height: u16) -> u16 {
    if matches!(mode, Mode::Help) {
        height
    } else {
        (line_count as u16).min(height)
    }
}

fn menu_lines(kind: MenuKind) -> Vec<String> {
    let mut lines = vec![" pick  1..  esc".to_string()];
    for (i, (label, _)) in menu_items(kind).iter().enumerate() {
        lines.push(format!("{} {label}", i + 1));
    }
    lines
}

fn confirm_ask(kind: ConfirmKind) -> &'static str {
    match kind {
        ConfirmKind::Pane => "close pane?",
        ConfirmKind::Tab => "close tab?",
        ConfirmKind::Workspace => "close window?",
    }
}

fn confirm_lines(kind: ConfirmKind) -> Vec<String> {
    vec![
        format!(" {}", confirm_ask(kind)),
        " 1 yes".to_string(),
        " 2 no".to_string(),
    ]
}

fn confirm_overlay_pick(overlay_row: u16) -> ConfirmPick {
    match overlay_row {
        1 => ConfirmPick::Yes,
        2 => ConfirmPick::No,
        _ => ConfirmPick::Ignore,
    }
}

fn confirm_key(code: KeyCode) -> ConfirmPick {
    match code {
        KeyCode::Char('y' | 'Y' | '1') | KeyCode::Enter => ConfirmPick::Yes,
        KeyCode::Char('n' | 'N' | '2') | KeyCode::Esc => ConfirmPick::No,
        _ => ConfirmPick::Ignore,
    }
}

fn menu_hit(
    rows: &[Row],
    cells: &[crate::layout::Cell],
    column: u16,
    row: u16,
    rows_n: u16,
    side: u16,
    top: u16,
    zoomed: bool,
    focused: &str,
    workspace: &str,
) -> Option<MenuTarget> {
    if row == 0 || row + 1 >= rows_n {
        return None;
    }
    if row == TAB_ROW {
        return tab_chip_hit(rows, workspace, column, side).map(|(tab, focus)| MenuTarget {
            kind: MenuKind::Tab,
            focus,
            tab,
            workspace: workspace.to_string(),
        });
    }
    if side > 0 && column < side {
        if row < 2 || row + 1 >= rows_n {
            return None;
        }
        let idx = (row - 2) as usize;
        return sidebar_model(rows, workspace, side, sidebar_paint_height(rows_n)).get(idx).and_then(|h| {
            if h.kind != SideKind::Workspace {
                return None;
            }
            if h.workspace.is_empty() && h.pane.is_none() {
                return None;
            }
            Some(MenuTarget {
                kind: MenuKind::Workspace,
                focus: h.pane.clone().unwrap_or_default(),
                tab: h.tab.clone(),
                workspace: h.workspace.clone(),
            })
        });
    }
    let (origin_x, origin_y) = content_origin(side, top);
    if column < origin_x || row < origin_y {
        return None;
    }
    let content_x = column.saturating_sub(origin_x);
    let content_y = row.saturating_sub(origin_y);
    if zoomed {
        if focused.is_empty() {
            return None;
        }
        return Some(target_from_pane(MenuKind::Pane, focused, rows, workspace));
    }
    if crate::layout::divider_at(cells, content_x, content_y).is_some() {
        return None;
    }
    crate::layout::cell_at(cells, content_x, content_y)
        .map(|cell| target_from_pane(MenuKind::Pane, &cell.id, rows, workspace))
}

fn target_from_pane(kind: MenuKind, focus: &str, rows: &[Row], fallback_ws: &str) -> MenuTarget {
    MenuTarget {
        kind,
        focus: focus.to_string(),
        tab: tab_of(rows, focus).unwrap_or_default(),
        workspace: workspace_of(rows, focus).unwrap_or_else(|| fallback_ws.to_string()),
    }
}

fn confirm_from_target(mut confirm: Confirm, target: Option<&MenuTarget>) -> Confirm {
    if let Some(target) = target {
        if !target.focus.is_empty() {
            confirm.pane = target.focus.clone();
        }
        if !target.tab.is_empty() {
            confirm.tab = target.tab.clone();
        }
        if !target.workspace.is_empty() {
            confirm.workspace = target.workspace.clone();
        }
    }
    confirm
}

fn close_rpc_line(kind: ConfirmKind, pane: &str, tab: &str, workspace: &str) -> String {
    match kind {
        ConfirmKind::Pane => format!(r#"{{"op":"pane.close","pane":"{pane}"}}"#),
        ConfirmKind::Tab => format!(r#"{{"op":"tab.close","tab":"{tab}"}}"#),
        ConfirmKind::Workspace => {
            format!(r#"{{"op":"workspace.close","workspace":"{workspace}"}}"#)
        }
    }
}

fn last_room_copy(kind: ConfirmKind) -> &'static str {
    match kind {
        ConfirmKind::Workspace => "last window kept",
        _ => "last pane kept",
    }
}

const PREFIX_FOOTER: &str =
    " Ctrl-b. q leave  c tab  n/p tab  w pick  Shift-n window  x close  ? keys";

fn footer_hint(mode: Mode, status: &str) -> &str {
    match mode {
        Mode::Prefix => PREFIX_FOOTER,
        Mode::Onboard => " enter remember  esc dismiss  Ctrl-b q leave",
        Mode::Help => " esc close keys",
        Mode::Menu => " 1.. run  esc cancel",
        Mode::Picker => " j/k pick  enter  esc",
        Mode::Confirm => " 1 yes  2 no  esc",
        _ if !status.is_empty() => status,
        _ => " right-click menu  drag≥2 copy  Ctrl-b prefix",
    }
}

fn footer_line<'a>(mode: Mode, status: &'a str, glance: Option<&'a str>) -> &'a str {
    if mode == Mode::Terminal && status.is_empty() {
        if let Some(g) = glance {
            if !g.is_empty() {
                return g;
            }
        }
    }
    footer_hint(mode, status)
}

fn clip_glance(s: &str) -> String {
    s.chars()
        .map(|c| {
            let u = c as u32;
            if u < 0x20
                || u == 0x7F
                || (0x80..=0x9F).contains(&u)
                || (0x202A..=0x202E).contains(&u)
                || (0x2066..=0x2069).contains(&u)
                || u == 0x200E
                || u == 0x200F
                || u == 0x061C
                || u == 0x2028
                || u == 0x2029
            {
                ' '
            } else {
                c
            }
        })
        .collect()
}

fn flow_glance_line(g: &FlowGlance) -> String {
    let payload = g.arg0.as_str();
    let raw = if let Some(err) = g.error.as_deref().filter(|e| !e.is_empty()) {
        format!("Flow error. {err}")
    } else if let Some(n) = g.code {
        if payload.is_empty() {
            format!("Flow {n}.")
        } else {
            format!("Flow {n}. {payload}")
        }
    } else if payload.is_empty() {
        "Flow.".to_string()
    } else {
        format!("Flow. {payload}")
    };
    clip_glance(&raw)
}

fn overlay_paints(mode: Mode) -> bool {
    matches!(
        mode,
        Mode::Help | Mode::Picker | Mode::Menu | Mode::Confirm | Mode::Onboard
    )
}

fn skip_onboard_from_env() -> bool {
    env::var("DORY_SKIP_ONBOARD")
        .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true"))
        .unwrap_or(false)
}

fn onboard_state_path() -> Option<PathBuf> {
    if let Ok(dir) = env::var("XDG_STATE_HOME") {
        if !dir.is_empty() {
            return Some(PathBuf::from(dir).join("dory").join("onboarded"));
        }
    }
    let home = env::var("HOME").ok().filter(|h| !h.is_empty())?;
    Some(
        PathBuf::from(home)
            .join(".local")
            .join("state")
            .join("dory")
            .join("onboarded"),
    )
}

fn onboard_meta(path: &Path) -> io::Result<fs::Metadata> {
    fs::symlink_metadata(path)
}

fn is_regular_file(meta: &fs::Metadata) -> bool {
    meta.file_type().is_file()
}

fn last_flow_result(path: &Path) -> Option<FlowGlance> {
    let meta = fs::symlink_metadata(path).ok()?;
    if !is_regular_file(&meta) {
        return None;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(0o400000 | 0o4000)
        .open(path)
        .ok()?;
    let opened = file.metadata().ok()?;
    if !is_regular_file(&opened) {
        return None;
    }
    if opened.dev() != meta.dev() || opened.ino() != meta.ino() {
        return None;
    }
    let len = opened.len();
    file.seek(SeekFrom::Start(len.saturating_sub(65536))).ok()?;
    let mut buf = Vec::new();
    file.take(65536).read_to_end(&mut buf).ok()?;
    last_flow_result_bytes(&buf)
}

fn poll_flow_glance(
    path: Option<&Path>,
    prev: &mut Option<String>,
    mtime: &mut Option<SystemTime>,
    cached_path: &mut Option<PathBuf>,
) -> bool {
    let Some(path) = path else {
        let dirty = prev.is_some();
        *prev = None;
        *mtime = None;
        *cached_path = None;
        return dirty;
    };
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => {
            let dirty = prev.is_some();
            *prev = None;
            *mtime = None;
            *cached_path = None;
            return dirty;
        }
    };
    let new_mtime = meta.modified().ok();
    if cached_path.as_deref() == Some(path) && new_mtime.is_some() && new_mtime == *mtime {
        return false;
    }
    let next = last_flow_result(path)
        .map(|g| flow_glance_line(&g))
        .filter(|s| !s.is_empty());
    let dirty = next.as_deref() != prev.as_deref();
    *prev = next;
    *mtime = new_mtime;
    *cached_path = Some(path.to_path_buf());
    dirty
}

fn onboarded_file_done(path: &Path) -> bool {
    match onboard_meta(path) {
        Ok(meta) if is_regular_file(&meta) => fs::read(path).map(|b| !b.is_empty()).unwrap_or(false),
        _ => false,
    }
}

fn should_show_onboard(path: Option<&Path>, skip: bool) -> bool {
    if skip {
        return false;
    }
    let Some(path) = path else {
        return false;
    };
    match onboard_meta(path) {
        Ok(meta) if !is_regular_file(&meta) => false,
        Ok(meta) if is_regular_file(&meta) => !onboarded_file_done(path),
        _ => true,
    }
}

fn initial_mode(path: Option<&Path>, skip: bool) -> Mode {
    if should_show_onboard(path, skip) {
        Mode::Onboard
    } else {
        Mode::Terminal
    }
}

fn mark_onboarded(path: &Path) -> io::Result<()> {
    if let Ok(meta) = onboard_meta(path) {
        if !is_regular_file(&meta) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "onboard path is not a file",
            ));
        }
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
            let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o700));
        }
    }
    let tmp = path.with_file_name(format!(
        "{}.tmp",
        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("onboarded")
    ));
    let write = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&tmp)?;
        file.write_all(b"1\n")?;
        file.sync_all()?;
        fs::rename(&tmp, path)
    })();
    if write.is_err() {
        let _ = fs::remove_file(&tmp);
        if path.is_file() {
            if let Ok(bytes) = fs::read(path) {
                if bytes.is_empty() {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
    write
}

fn onboard_lines() -> Vec<String> {
    vec![
        "dory".to_string(),
        "sit. mouse first, prefix after".to_string(),
        String::new(),
        "right-click pane / tab chip / window card = menu".to_string(),
        "drag ≥2 panes = copy".to_string(),
        "Ctrl-b = prefix (q leave, ? keys)".to_string(),
        String::new(),
        "[enter] remember    mouse / Ctrl-b use, do not remember    esc dismiss".to_string(),
    ]
}

fn onboard_key(key: KeyEvent) -> OnboardKey {
    if is_ctrl_b(&key) {
        return OnboardKey::Prefix;
    }
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') => OnboardKey::Persist,
        KeyCode::Esc => OnboardKey::Dismiss,
        _ => OnboardKey::Eat,
    }
}

fn onboard_mouse(kind: MouseEventKind) -> OnboardMouse {
    match kind {
        MouseEventKind::Down(_)
        | MouseEventKind::ScrollUp
        | MouseEventKind::ScrollDown => OnboardMouse::Release,
        _ => OnboardMouse::Eat,
    }
}

fn help_text() -> &'static str {
    "Ctrl-b prefix\n\
     q / d       leave (PTY stays)\n\
     Shift-d     close window\n\
     c           new tab\n\
     v / -       split right / down\n\
     n / p       next / prev tab (this window)\n\
     1-9         tab\n\
     hjkl        pane\n\
     w           pick window (do not create)\n\
     Shift-n     new window\n\
     x           close pane\n\
     Shift-x     close tab\n\
     z           zoom (sibling streams stay)\n\
     b           hide sidebar\n\
     Ctrl-b      send C-b into pane\n\
     ?           this keys\n\
     drag ≥2     copy (OSC 52)\n\
     right-click menu at pointer (esc)\n\
     close confirm 1/2 (right-click cancel)\n\
     first sit   banner; mouse/Ctrl-b still split pane · tab · window; enter remember\n\
     Esc / q / ? close keys"
}

fn picker_lines(rows: &[Row], idx: usize) -> Vec<String> {
    let mut lines = vec![" pick window  j/k  enter  esc".to_string()];
    for (i, ws) in workspaces_of(rows).into_iter().enumerate() {
        let mark = if i == idx { '>' } else { ' ' };
        lines.push(format!("{mark} {}", workspace_label(rows, &ws)));
    }
    lines
}

fn picker_mouse_pick(overlay_row: u16, n: usize) -> Option<usize> {
    if overlay_row == 0 || n == 0 {
        None
    } else {
        Some((overlay_row as usize - 1).min(n - 1))
    }
}

fn cell_drag_span(a: (u16, u16), b: (u16, u16)) -> u32 {
    let dx = u32::from(a.0.abs_diff(b.0)) + 1;
    let dy = u32::from(a.1.abs_diff(b.1)) + 1;
    dx.saturating_mul(dy)
}

fn selection_from_parser(
    parser: &vt100::Parser,
    tx: u16,
    ty: u16,
    tw: u16,
    th: u16,
    a: (u16, u16),
    b: (u16, u16),
) -> Option<String> {
    let x0 = a.0.min(b.0);
    let x1 = a.0.max(b.0);
    let y0 = a.1.min(b.1);
    let y1 = a.1.max(b.1);
    if x1 < tx || y1 < ty || x0 >= tx.saturating_add(tw) || y0 >= ty.saturating_add(th) {
        return None;
    }
    let lx0 = x0.saturating_sub(tx).min(tw.saturating_sub(1));
    let lx1 = x1.saturating_sub(tx).min(tw.saturating_sub(1));
    let ly0 = y0.saturating_sub(ty).min(th.saturating_sub(1));
    let ly1 = y1.saturating_sub(ty).min(th.saturating_sub(1));
    let screen = parser.screen();
    let mut lines = Vec::new();
    for y in ly0..=ly1 {
        let mut line = String::new();
        for x in lx0..=lx1 {
            match screen.cell(y, x) {
                Some(cell) => {
                    let contents = cell.contents();
                    if contents.is_empty() {
                        line.push(' ');
                    } else {
                        line.push_str(&contents);
                    }
                }
                None => line.push(' '),
            }
        }
        lines.push(line.trim_end().to_string());
    }
    let text = lines.join("\n");
    if text.is_empty() { None } else { Some(text) }
}

fn selection_text(
    tiles: &[Tile],
    focused: &str,
    a: (u16, u16),
    b: (u16, u16),
) -> Option<String> {
    let tile = tiles.iter().find(|t| t.id == focused)?;
    selection_from_parser(&tile.parser, tile.x, tile.y, tile.w, tile.h, a, b)
}

const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn b64_encode(data: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };
        let n = (u32::from(b0) << 16) | (u32::from(b1) << 8) | u32::from(b2);
        out.push(B64[((n >> 18) & 63) as usize] as char);
        out.push(B64[((n >> 12) & 63) as usize] as char);
        if i + 1 < data.len() {
            out.push(B64[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(B64[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

fn osc52_payload(text: &str) -> String {
    format!("\x1b]52;c;{}\x07", b64_encode(text.as_bytes()))
}

fn emit_osc52(text: &str) -> io::Result<()> {
    let mut out = io::stdout();
    write!(out, "{}", osc52_payload(text))?;
    out.flush()
}

fn term_size() -> (u16, u16) {
    crossterm::terminal::size().unwrap_or((80, 24))
}

fn stdin_is_tty() -> bool {
    unsafe { libc_isatty(0) }
}

fn stdout_is_tty() -> bool {
    unsafe { libc_isatty(1) }
}

unsafe fn libc_isatty(fd: i32) -> bool {
    unsafe extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(fd) == 1 }
}

fn is_ctrl_b(key: &KeyEvent) -> bool {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char(c) = key.code {
            return c.eq_ignore_ascii_case(&'b') || c == '\u{2}';
        }
    }
    matches!(key.code, KeyCode::Char('\u{2}'))
}

fn encode_key(key: KeyEvent) -> Option<Vec<u8>> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);
    if ctrl {
        match key.code {
            KeyCode::Char(c) => {
                let b = c.to_ascii_lowercase() as u8 & 0x1f;
                if b == 0 {
                    return None;
                }
                return Some(vec![b]);
            }
            KeyCode::Up => return Some(b"\x1b[1;5A".to_vec()),
            KeyCode::Down => return Some(b"\x1b[1;5B".to_vec()),
            KeyCode::Right => return Some(b"\x1b[1;5C".to_vec()),
            KeyCode::Left => return Some(b"\x1b[1;5D".to_vec()),
            _ => {}
        }
    }
    let bytes = match key.code {
        KeyCode::Char(c) => c.to_string().into_bytes(),
        KeyCode::Enter => vec![b'\r'],
        KeyCode::Backspace => vec![0x7f],
        KeyCode::Tab => vec![b'\t'],
        KeyCode::Esc => vec![0x1b],
        KeyCode::Up => b"\x1b[A".to_vec(),
        KeyCode::Down => b"\x1b[B".to_vec(),
        KeyCode::Right => b"\x1b[C".to_vec(),
        KeyCode::Left => b"\x1b[D".to_vec(),
        KeyCode::Home => b"\x1b[H".to_vec(),
        KeyCode::End => b"\x1b[F".to_vec(),
        KeyCode::Delete => b"\x1b[3~".to_vec(),
        KeyCode::PageUp => b"\x1b[5~".to_vec(),
        KeyCode::PageDown => b"\x1b[6~".to_vec(),
        KeyCode::Insert => b"\x1b[2~".to_vec(),
        _ => return None,
    };
    if alt {
        let mut out = vec![0x1b];
        out.extend(bytes);
        Some(out)
    } else {
        Some(bytes)
    }
}

fn empty_dash(s: &str) -> &str {
    if s.is_empty() { "—" } else { s }
}

fn display_width(s: &str) -> usize {
    s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
}

/// Full-width chrome must not paint the last column: writing that cell with
/// host wrap on scrolls the whole screen on every redraw.
fn bar_line(text: &str, cols: u16) -> String {
    let max = cols.saturating_sub(1).max(1) as usize;
    let mut line = text.to_string();
    clip_to(&mut line, max);
    while display_width(&line) < max {
        line.push(' ');
    }
    line
}

fn clip_to(s: &mut String, width: usize) {
    if display_width(s) <= width {
        return;
    }
    let mut out = String::new();
    let mut w = 0;
    for c in s.chars() {
        let cw = if c.is_ascii() { 1 } else { 2 };
        if w + cw > width {
            break;
        }
        out.push(c);
        w += cw;
    }
    *s = out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tree_walks_split_siblings() {
        let body = r#"{"ok":true,"result":{"focused":"w1:p2","items":[{"k":"w","id":"w1"},{"k":"t","id":"w1:t1"},{"k":"p","id":"w1:p1","occ":"","st":""},{"k":"p","id":"w1:p2","occ":"coder","st":"working"}]}}"#;
        let (rows, focused, ws, tab) = parse_tree(body);
        assert_eq!(focused, "w1:p2");
        assert_eq!(ws, "w1");
        assert_eq!(tab, "w1:t1");
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[2].id, "w1:p1");
        assert_eq!(rows[3].occ, "coder");
        assert_eq!(rows[0].focus_pane, "w1:p1");
        assert_eq!(
            pane_id_from(r#"{"ok":true,"result":{"pane":{"id":"w1:p3"}}}"#).as_deref(),
            Some("w1:p3")
        );
        assert_eq!(
            pane_id_from(
                r#"{"ok":true,"result":{"tab":{"id":"w1:t2"},"root_pane":{"id":"w1:p4"}}}"#
            )
            .as_deref(),
            Some("w1:p4")
        );
    }

    #[test]
    fn bar_line_leaves_last_column() {
        let line = bar_line(" dory  w1 · t1 · p1", 20);
        assert_eq!(display_width(&line), 19);
        assert!(!line.chars().any(|c| c == '\n'));
    }

    #[test]
    fn pane_size_leaves_host_gutter() {
        let (w, h) = pane_size(80, 24, SIDEBAR, 2);
        assert_eq!(w + SIDEBAR + 2, 80);
        assert_eq!(h, 21);
        let (w0, _) = pane_size(80, 24, 0, 2);
        assert_eq!(w0 + 1, 80);
    }

    #[test]
    fn pty_char_does_not_take_prefix_chrome() {
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert!(!is_ctrl_b(&key));
        assert!(encode_key(key).is_some());
    }

    fn glance_rows() -> Vec<Row> {
        vec![
            Row {
                kind: 'w',
                id: "w1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 't',
                id: "w1:t1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 'p',
                id: "w1:p1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: "working".into(),
                cwd: String::new(),
            },
            Row {
                kind: 'p',
                id: "w1:p2".into(),
                focus_pane: "w1:p2".into(),
                occ: "coder".into(),
                st: "blocked".into(),
                cwd: String::new(),
            },
            Row {
                kind: 'w',
                id: "w2".into(),
                focus_pane: "w2:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 't',
                id: "w2:t1".into(),
                focus_pane: "w2:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 'p',
                id: "w2:p1".into(),
                focus_pane: "w2:p1".into(),
                occ: "reviewer".into(),
                st: "unknown".into(),
                cwd: String::new(),
            },
        ]
    }

    #[test]
    fn rollup_blocked_beats_working_and_keeps_unknown() {
        let rows = glance_rows();
        assert_eq!(rollup_of(&rows, "w1"), "blocked");
        assert_eq!(rollup_of(&rows, "w2"), "unknown");
        assert_ne!(normalize_st("unknown"), "idle");
        let agents = agents_from(&rows);
        assert_eq!(agents[0].occ, "coder");
        assert_eq!(agents[0].st, "blocked");
    }

    fn empty_shell_rows(cwd: &str) -> Vec<Row> {
        vec![
            Row {
                kind: 'w',
                id: "w1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: cwd.into(),
            },
            Row {
                kind: 't',
                id: "w1:t1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: cwd.into(),
            },
            Row {
                kind: 'p',
                id: "w1:p1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
        ]
    }

    #[test]
    fn empty_shell_space_card_is_folder_not_unknown() {
        let rows = empty_shell_rows("/home/manhquy/Downloads/flow");
        assert_eq!(rollup_of(&rows, "w1"), "");
        let model = sidebar_model(&rows, "w1", SIDEBAR, 0);
        let card = model
            .iter()
            .find(|h| matches!(h.kind, SideKind::Workspace))
            .expect("space card");
        assert!(card.text.contains("flow"), "{}", card.text);
        assert!(!card.text.contains("unknown"), "{}", card.text);
        assert!(!card.text.contains("w1"), "{}", card.text);
        let compact = sidebar_model(&rows, "w1", SIDEBAR_DOTS, 0);
        assert!(compact[0].text.contains('·'), "{}", compact[0].text);
        assert!(!compact[0].text.contains('U'), "{}", compact[0].text);
    }

    #[test]
    fn tree_sig_changes_when_cwd_changes() {
        let home = empty_shell_rows("/home/manhquy");
        let flow = empty_shell_rows("/home/manhquy/Downloads/flow");
        assert_ne!(
            tree_rows_sig("w1:p1", &home),
            tree_rows_sig("w1:p1", &flow)
        );
    }

    #[test]
    fn working_pty_frame_does_not_wipe_pane() {
        assert!(
            !pane_wipe_on_tile_draw(false),
            "PTY-only dirty (agent working) must paint over cells, not blank first"
        );
        assert!(
            pane_wipe_on_tile_draw(true),
            "chrome/layout dirty still wipes so leftover tiles do not ghost"
        );
    }

    #[test]
    fn sidebar_wide_rule_is_full_width_when_agents_exist() {
        let rows = glance_rows();
        let wide = sidebar_model(&rows, "w1", SIDEBAR, 0);
        let rule = wide
            .iter()
            .find(|h| matches!(h.kind, SideKind::Rule))
            .expect("rule");
        assert_eq!(rule.text.chars().count(), SIDEBAR as usize);
        assert!(rule.text.chars().all(|c| c == '─'), "{}", rule.text);
        let compact = sidebar_model(&rows, "w1", SIDEBAR_DOTS, 0);
        let compact_rule = compact
            .iter()
            .find(|h| matches!(h.kind, SideKind::Rule))
            .expect("compact rule");
        assert_eq!(compact_rule.text.chars().count(), SIDEBAR_DOTS as usize);
        let empty = sidebar_model(
            &empty_shell_rows("/home/manhquy/Downloads/flow"),
            "w1",
            SIDEBAR,
            0,
        );
        assert!(!empty.iter().any(|h| matches!(h.kind, SideKind::Rule)));
        assert!(!empty.iter().any(|h| h.text.contains("Agents")));
        assert_eq!(sidebar_row_bg(0, 8, 3), SIDE_BG);
        assert_eq!(sidebar_row_bg(5, 8, 3), TITLE_BG);
        assert_eq!(sidebar_row_bg(7, 8, 0), SIDE_BG);
        let w = SIDEBAR as usize;
        assert_eq!(
            pad_cols(sidebar_rule(w), w).chars().count(),
            13,
            "pad_cols lies: ─ is 2 in display_width; paint must use sidebar_paint_text"
        );
        assert_eq!(sidebar_paint_text(SideKind::Rule, "", w).chars().count(), w);
        let fitted = sidebar_model(&rows, "w1", SIDEBAR, 12);
        let fitted_rule = fitted
            .iter()
            .find(|h| matches!(h.kind, SideKind::Rule))
            .expect("fitted rule");
        assert_eq!(
            sidebar_paint_text(fitted_rule.kind, &fitted_rule.text, w)
                .chars()
                .count(),
            w
        );
    }

    fn working_only_rows() -> Vec<Row> {
        vec![
            Row {
                kind: 'w',
                id: "w1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: "/home/u/proj".into(),
            },
            Row {
                kind: 't',
                id: "w1:t1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 'p',
                id: "w1:p1".into(),
                focus_pane: "w1:p1".into(),
                occ: "coder".into(),
                st: "working".into(),
                cwd: "/home/u/proj".into(),
            },
        ]
    }

    fn long_working_rows() -> Vec<Row> {
        vec![
            Row {
                kind: 'w',
                id: "w1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: "/home/u/very-long-workspace-名称-that-must-clip".into(),
            },
            Row {
                kind: 't',
                id: "w1:t1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 'p',
                id: "w1:p1".into(),
                focus_pane: "w1:p1".into(),
                occ: "coder".into(),
                st: "working".into(),
                cwd: "/home/u/very-long-workspace-名称-that-must-clip".into(),
            },
        ]
    }

    #[test]
    fn status_fg_table() {
        assert_eq!(status_fg("blocked"), BLOCKED_FG);
        assert_eq!(status_fg("working"), ACCENT);
        assert_eq!(status_fg("done"), MUTED);
        assert_eq!(status_fg("idle"), MUTED);
        assert_eq!(status_fg(""), MUTED);
        assert_eq!(status_fg("unknown"), MUTED);
    }

    #[test]
    fn sidebar_status_color_focused_wide_working_keeps_gold_dot() {
        let rows = working_only_rows();
        let (spaces, _) = sidebar_sections(&rows, "w1", SIDEBAR);
        let hit = spaces
            .iter()
            .find(|h| h.kind == SideKind::Workspace)
            .expect("space");
        assert!(hit.lead.contains('●'));
        assert_eq!(hit.st, "working");
        let (lead, lead_fg, mid, mid_fg, rest, _) = sidebar_row_spans(hit, SIDEBAR as usize, "w1");
        assert_eq!(lead_fg, FOCUSED_FG);
        assert_eq!(mid, " working");
        assert_eq!(mid_fg, ACCENT);
        assert_eq!(
            display_width(&lead) + display_width(&mid) + display_width(&rest),
            SIDEBAR as usize
        );
    }

    #[test]
    fn sidebar_status_color_compact_working_is_accent_w() {
        let rows = working_only_rows();
        let (spaces, _) = sidebar_sections(&rows, "w1", SIDEBAR_DOTS);
        let hit = spaces
            .iter()
            .find(|h| h.kind == SideKind::Workspace)
            .expect("space");
        assert_eq!(hit.lead, " W");
        let (_, lead_fg, mid, _, _, _) = sidebar_row_spans(hit, SIDEBAR_DOTS as usize, "w1");
        assert_eq!(lead_fg, ACCENT);
        assert!(mid.is_empty());
    }

    #[test]
    fn sidebar_status_color_agent_blocked_colors_word_only() {
        let rows = glance_rows();
        let (_, agents) = sidebar_sections(&rows, "w1", SIDEBAR);
        let hit = agents
            .iter()
            .find(|h| h.kind == SideKind::Agent && h.st == "blocked")
            .expect("blocked agent");
        assert!(hit.lead.contains("coder"));
        assert!(hit.tail.contains("p2"));
        let (lead, lead_fg, mid, mid_fg, rest, _) = sidebar_row_spans(hit, SIDEBAR as usize, "w1");
        assert_eq!(lead_fg, TEXT);
        assert_eq!(mid, " blocked");
        assert_eq!(mid_fg, BLOCKED_FG);
        assert_eq!(
            display_width(&lead) + display_width(&mid) + display_width(&rest),
            SIDEBAR as usize
        );
    }

    #[test]
    fn sidebar_status_color_clips_lead_keeps_status_word() {
        let rows = long_working_rows();
        let (spaces, _) = sidebar_sections(&rows, "w1", SIDEBAR);
        let hit = spaces
            .iter()
            .find(|h| h.kind == SideKind::Workspace)
            .expect("space");
        assert_eq!(hit.st, "working");
        assert!(hit.lead.contains('●'));
        assert!(!hit.lead.contains("must-clip"));
        assert!(!hit.lead.contains('称'));
        let (lead, _, mid, mid_fg, rest, _) = sidebar_row_spans(hit, SIDEBAR as usize, "w1");
        assert_eq!(mid, " working");
        assert_eq!(mid_fg, ACCENT);
        assert!(hit.text.contains("working"));
        assert_eq!(
            display_width(&lead) + display_width(&mid) + display_width(&rest),
            SIDEBAR as usize
        );
    }

    #[test]
    fn sidebar_status_color_compact_agent_keeps_occ_initial() {
        let rows = glance_rows();
        let (_, agents) = sidebar_sections(&rows, "w1", SIDEBAR_DOTS);
        let hit = agents
            .iter()
            .find(|h| h.kind == SideKind::Agent && h.st == "blocked")
            .expect("blocked agent");
        assert_eq!(hit.lead, " c");
        assert!(!hit.lead.contains('B'));
        let (_, lead_fg, mid, _, _, _) = sidebar_row_spans(hit, SIDEBAR_DOTS as usize, "w1");
        assert_eq!(lead_fg, BLOCKED_FG);
        assert!(mid.is_empty());
    }

    #[test]
    fn sidebar_status_color_empty_shell_is_dot_not_unknown() {
        let rows = empty_shell_rows("/home/manhquy/Downloads/flow");
        let (spaces, _) = sidebar_sections(&rows, "w1", SIDEBAR_DOTS);
        let hit = spaces
            .iter()
            .find(|h| h.kind == SideKind::Workspace)
            .expect("space");
        assert_eq!(hit.st, "");
        assert_eq!(hit.lead, " ·");
        let (_, lead_fg, mid, _, _, _) = sidebar_row_spans(hit, SIDEBAR_DOTS as usize, "w1");
        assert_eq!(lead_fg, MUTED);
        assert!(mid.is_empty());
    }

    #[test]
    fn sidebar_hit_is_not_tree_row_index() {
        let rows = glance_rows();
        assert_eq!(sidebar_focus_at(&rows, 2, 24, "w1", SIDEBAR), None);
        assert_eq!(
            sidebar_focus_at(&rows, 4, 24, "w1", SIDEBAR).as_deref(),
            Some("w2:p1")
        );
        assert_ne!(rows[4 - 1].id, "w2:p1");
        let tabs = tabs_of(&rows, "w1");
        assert_eq!(tabs.len(), 1);
        assert_eq!(tab_chip_at(&rows, "w1", SIDEBAR + 1, SIDEBAR).as_deref(), Some("w1:p1"));
    }

    #[test]
    fn sidebar_hides_agents_when_empty() {
        let rows = vec![
            Row {
                kind: 'w',
                id: "w1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 't',
                id: "w1:t1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            },
            Row {
                kind: 'p',
                id: "w1:p1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: "idle".into(),
                cwd: String::new(),
            },
        ];
        let model = sidebar_model(&rows, "w1", SIDEBAR, 0);
        assert!(!model.iter().any(|h| h.text.contains("Agents")));
        assert!(!model.iter().any(|h| h.text.contains("idle")));
        assert!(!model.iter().any(|h| matches!(h.kind, SideKind::Rule)));
        let compact = sidebar_model(&rows, "w1", SIDEBAR_DOTS, 0);
        assert_eq!(compact.len(), 1);
        assert!(matches!(compact[0].kind, SideKind::Workspace));
        let glance = glance_rows();
        assert!(!agents_from(&glance).is_empty());
        assert_eq!(
            sidebar_focus_at(&glance, 4, 24, "w1", SIDEBAR).as_deref(),
            Some("w2:p1")
        );
        assert_eq!(
            menu_hit(
                &glance,
                &[],
                1,
                4,
                24,
                SIDEBAR,
                2,
                false,
                "w1:p1",
                "w1",
            )
            .map(|h| h.workspace),
            Some("w2".into())
        );
    }

    #[test]
    fn agents_stay_at_sidebar_bottom_when_spaces_overflow() {
        let mut rows = Vec::new();
        for i in 1..=8 {
            let w = format!("w{i}");
            let p = format!("{w}:p1");
            rows.push(Row {
                kind: 'w',
                id: w.clone(),
                focus_pane: p.clone(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            });
            rows.push(Row {
                kind: 't',
                id: format!("{w}:t1"),
                focus_pane: p.clone(),
                occ: String::new(),
                st: String::new(),
                cwd: String::new(),
            });
            rows.push(Row {
                kind: 'p',
                id: p.clone(),
                focus_pane: p,
                occ: if i == 1 {
                    String::from("coder")
                } else {
                    String::new()
                },
                st: String::from("idle"),
                cwd: String::new(),
            });
        }
        let height = 8;
        let model = sidebar_model(&rows, "w1", SIDEBAR, height);
        assert_eq!(model.len(), 8);
        assert!(model[0].text.contains("Spaces"));
        assert!(model[6].text.contains("Agents"));
        assert!(matches!(model[7].kind, SideKind::Agent));
        assert!(model[7].text.contains("coder"));
        assert!(!model.iter().any(|h| h.text.contains("w8")));
        let rows_n = height + 3;
        assert_eq!(
            sidebar_focus_at(&rows, 2 + 7, rows_n, "w1", SIDEBAR).as_deref(),
            Some("w1:p1")
        );
        let compact = sidebar_model(&rows, "w1", SIDEBAR_DOTS, 6);
        assert_eq!(compact.len(), 6);
        assert!(matches!(compact[5].kind, SideKind::Agent));
        assert_eq!(agent_region_rows(SIDEBAR, 8, 1), 3);
        assert_eq!(agent_region_rows(SIDEBAR, 4, 1), 3);
        assert_eq!(agent_region_rows(SIDEBAR, 8, 0), 0);
    }

    #[test]
    fn apply_retired_hides_closed_workspace_card() {
        let mut rows = glance_rows();
        apply_retired(&mut rows, &[String::from("w2")], &[], &[]);
        assert_eq!(workspaces_of(&rows), vec!["w1".to_string()]);
        let model = sidebar_model(&rows, "w1", SIDEBAR, 0);
        assert!(!model.iter().any(|h| h.text.contains("w2")));
        assert_eq!(sidebar_focus_at(&rows, 4, 24, "w1", SIDEBAR), None);
    }

    #[test]
    fn close_drops_retired_workspace_from_sidebar() {
        let mut rows = glance_rows();
        assert!(workspaces_of(&rows).iter().any(|w| w == "w2"));
        assert!(closed_still_listed(
            &rows,
            ConfirmKind::Workspace,
            "w2:p1",
            "w2:t1",
            "w2"
        ));
        drop_closed_from_rows(
            &mut rows,
            ConfirmKind::Workspace,
            "w2:p1",
            "w2:t1",
            "w2",
        );
        assert_eq!(workspaces_of(&rows), vec!["w1".to_string()]);
        assert!(workspace_of(&rows, "w2:p1").is_none());
        assert_eq!(workspace_of(&rows, "w1:p1").as_deref(), Some("w1"));
        assert_eq!(first_live_pane(&rows).as_deref(), Some("w1:p1"));
        let model = sidebar_model(&rows, "w1", SIDEBAR, 0);
        assert!(model.iter().any(|h| h.text.contains("w1")));
        assert!(!model.iter().any(|h| h.text.contains("w2")));
        assert_eq!(sidebar_focus_at(&rows, 4, 24, "w1", SIDEBAR), None);
    }

    #[test]
    fn close_last_pane_prunes_empty_workspace_card() {
        let mut rows = glance_rows();
        drop_closed_from_rows(
            &mut rows,
            ConfirmKind::Pane,
            "w2:p1",
            "w2:t1",
            "w2",
        );
        assert!(!workspaces_of(&rows).iter().any(|w| w == "w2"));
        assert!(!closed_still_listed(
            &rows,
            ConfirmKind::Workspace,
            "w2:p1",
            "w2:t1",
            "w2"
        ));
    }

    #[test]
    fn parse_tree_missing_focus_does_not_stick_last_workspace() {
        let body = r#"{"ok":true,"result":{"focused":"w2:p1","items":[{"k":"w","id":"w1"},{"k":"t","id":"w1:t1"},{"k":"p","id":"w1:p1"}]}}"#;
        let (rows, focused, ws, tab) = parse_tree(body);
        assert_eq!(focused, "w2:p1");
        assert!(ws.is_empty());
        assert!(tab.is_empty());
        assert_eq!(workspaces_of(&rows), vec!["w1".to_string()]);
        assert!(workspace_of(&rows, "w2:p1").is_none());
    }

    #[test]
    fn zoom_wanted_keeps_every_cell() {
        let cells = vec![
            crate::layout::Cell {
                id: "w1:p1".into(),
                x: 0,
                y: 0,
                w: 40,
                h: 21,
            },
            crate::layout::Cell {
                id: "w1:p2".into(),
                x: 40,
                y: 0,
                w: 40,
                h: 21,
            },
        ];
        let wanted = wanted_rects(&cells, "w1:p1", true, 80, 21);
        assert_eq!(wanted.len(), 2);
        assert_eq!(wanted[0], ("w1:p1".into(), 0, 0, 80, 21));
        assert_eq!(wanted[1].0, "w1:p2");
        assert_eq!(wanted[1].2, 0);
    }

    #[test]
    fn selection_and_osc52_encode() {
        let mut parser = vt100::Parser::new(4, 20, 0);
        parser.process(b"hello world");
        let text = selection_from_parser(&parser, 0, 0, 20, 4, (0, 0), (4, 0)).unwrap();
        assert_eq!(text, "hello");
        assert_eq!(cell_drag_span((0, 0), (0, 0)), 1);
        assert_eq!(cell_drag_span((0, 0), (1, 0)), 2);
        assert_eq!(b64_encode(b"hello"), "aGVsbG8=");
        let payload = osc52_payload("hello");
        assert!(payload.starts_with("\x1b]52;c;"));
        assert!(payload.contains("aGVsbG8="));
        assert!(help_text().contains("pick window"));
        assert!(help_text().contains("right-click"));
        assert!(help_text().contains("first sit"));
        assert!(help_text().contains("1/2"));
        assert!(!help_text().contains("y/n"));
        assert!(!help_text().contains("workspace picker"));
        assert!(!help_text().contains("detach"));
    }

    #[test]
    fn menu_hit_closed_table() {
        let rows = glance_rows();
        let body = r#"{"ok":true,"result":{"focused":"w1:p2","cells":[{"id":"w1:p1","x":0,"y":0,"w":40,"h":22},{"id":"w1:p2","x":40,"y":0,"w":40,"h":22}]}}"#;
        let cells = parse_layout_cells(body);
        let top = 2;
        let hit = |col, row, zoomed, focused: &str| {
            menu_hit(
                &rows,
                &cells,
                col,
                row,
                24,
                SIDEBAR,
                top,
                zoomed,
                focused,
                "w1",
            )
        };
        let tab = hit(SIDEBAR + 1, TAB_ROW, false, "w1:p1").expect("tab chip");
        assert_eq!(tab.kind, MenuKind::Tab);
        assert_eq!(tab.focus, "w1:p1");
        let w1 = hit(1, 3, false, "w1:p1").expect("w1 card");
        assert_eq!(w1.kind, MenuKind::Workspace);
        assert_eq!(w1.focus, "w1:p1");
        let w2 = hit(1, 4, false, "w1:p1").expect("w2 card");
        assert_eq!(w2.kind, MenuKind::Workspace);
        assert_eq!(w2.focus, "w2:p1");
        assert!(hit(1, 2, false, "w1:p1").is_none());
        assert!(hit(1, 7, false, "w1:p1").is_none());
        assert!(hit(1, 0, false, "w1:p1").is_none());
        assert!(hit(1, 23, false, "w1:p1").is_none());
        let pane = hit(SIDEBAR + 1 + 10, 3, false, "w1:p1").expect("tile");
        assert_eq!(pane.kind, MenuKind::Pane);
        assert_eq!(pane.focus, "w1:p1");
        assert!(hit(SIDEBAR + 1 + 39, 3, false, "w1:p1").is_none());
        let zoomed = hit(SIDEBAR + 1 + 40, 3, true, "w1:p1").expect("zoomed ignores sibling");
        assert_eq!(zoomed.kind, MenuKind::Pane);
        assert_eq!(zoomed.focus, "w1:p1");
    }

    #[test]
    fn workspace_close_uses_hit_card_not_focused_space() {
        let rows = glance_rows();
        let body = r#"{"ok":true,"result":{"focused":"w1:p1","cells":[{"id":"w1:p1","x":0,"y":0,"w":80,"h":22}]}}"#;
        let cells = parse_layout_cells(body);
        let hit = menu_hit(
            &rows,
            &cells,
            1,
            4,
            24,
            SIDEBAR,
            2,
            false,
            "w1:p1",
            "w1",
        )
        .expect("w2 card");
        assert_eq!(hit.kind, MenuKind::Workspace);
        assert_eq!(hit.workspace, "w2");
        assert_eq!(hit.focus, "w2:p1");
        assert_eq!(hit.tab, "w2:t1");
        let current = Confirm {
            kind: ConfirmKind::Workspace,
            pane: "w1:p1".into(),
            tab: "w1:t1".into(),
            workspace: "w1".into(),
        };
        let locked = confirm_from_target(current, Some(&hit));
        assert_eq!(locked.workspace, "w2");
        assert_eq!(locked.pane, "w2:p1");
        assert_eq!(
            close_rpc_line(locked.kind, &locked.pane, &locked.tab, &locked.workspace),
            r#"{"op":"workspace.close","workspace":"w2"}"#
        );
    }

    #[test]
    fn menu_pick_and_items_lock() {
        let pane = menu_items(MenuKind::Pane);
        assert_eq!(pane.len(), 4);
        assert!(pane.iter().any(|(l, _)| *l == "Split right"));
        assert!(pane.iter().any(|(l, _)| *l == "Close pane"));
        assert_eq!(
            menu_pick(MenuKind::Pane, KeyCode::Char('1')),
            MenuPick::Run(MenuVerb::SplitRight)
        );
        assert_eq!(
            menu_pick(MenuKind::Pane, KeyCode::Char('4')),
            MenuPick::Run(MenuVerb::ClosePane)
        );
        assert_eq!(menu_pick(MenuKind::Pane, KeyCode::Esc), MenuPick::Cancel);
        assert_eq!(menu_pick(MenuKind::Pane, KeyCode::Char('9')), MenuPick::Ignore);
        assert_eq!(
            menu_pick(MenuKind::Tab, KeyCode::Char('2')),
            MenuPick::Run(MenuVerb::CloseTab)
        );
        assert_eq!(
            menu_pick(MenuKind::Workspace, KeyCode::Char('1')),
            MenuPick::Run(MenuVerb::Picker)
        );
        let lines = menu_lines(MenuKind::Pane);
        assert!(lines.iter().any(|l| l.contains("1 Split right")));
        assert!(lines[0].contains("pick  1.."));
        let confirm = confirm_lines(ConfirmKind::Workspace);
        assert!(confirm[0].contains("close window"));
        assert!(!confirm[0].contains("y/n"));
        assert!(!confirm.iter().any(|l| l.contains("y/n")));
        assert_eq!(confirm_overlay_pick(1), ConfirmPick::Yes);
        assert_eq!(confirm_overlay_pick(2), ConfirmPick::No);
        assert_eq!(confirm_overlay_pick(0), ConfirmPick::Ignore);
        assert!(menu_items(MenuKind::Workspace)
            .iter()
            .any(|(label, _)| *label == "Pick window"));
        assert_eq!(overlay_paint_rows(Mode::Menu, 5, 40), 5);
        assert_eq!(overlay_paint_rows(Mode::Picker, 3, 40), 3);
        assert_eq!(overlay_paint_rows(Mode::Confirm, 3, 40), 3);
        assert_eq!(overlay_paint_rows(Mode::Onboard, 11, 40), 11);
        assert_eq!(overlay_paint_rows(Mode::Help, 5, 40), 40);
    }

    #[test]
    fn parse_layout_and_click_map() {
        let body = r#"{"ok":true,"result":{"focused":"w1:p2","cells":[{"id":"w1:p1","x":0,"y":0,"w":40,"h":22},{"id":"w1:p2","x":40,"y":0,"w":40,"h":22}]}}"#;
        let cells = parse_layout_cells(body);
        assert_eq!(cells.len(), 2);
        assert_eq!(crate::layout::cell_at(&cells, 10, 1).unwrap().id, "w1:p1");
        assert_eq!(crate::layout::cell_at(&cells, 40, 1).unwrap().id, "w1:p2");
        let leftover = b"hello";
        let mut parser = vt100::Parser::new(4, 20, 0);
        parser.process(leftover);
        assert!(parser.screen().contents().contains("hello"));
    }

    #[test]
    fn footer_hint_idle_is_mouse_sentence() {
        let idle = footer_hint(Mode::Terminal, "");
        assert!(!idle.contains("hjkl"));
        assert!(!idle.contains("^B q"));
        assert!(idle.contains("right-click"));
        assert!(idle.contains("Ctrl-b"));
        let prefix = footer_hint(Mode::Prefix, "ignored dump");
        assert!(prefix.contains("leave"));
        assert!(prefix.contains("Ctrl-b"));
        assert!(!prefix.contains("^B"));
        assert!(!prefix.contains("detach"));
        assert_eq!(footer_hint(Mode::Help, help_text()), " esc close keys");
        assert!(!footer_hint(Mode::Help, help_text()).contains('\n'));
        assert_eq!(footer_hint(Mode::Menu, "menu  1..  esc"), " 1.. run  esc cancel");
        assert_eq!(
            footer_hint(Mode::Picker, "workspace picker"),
            " j/k pick  enter  esc"
        );
        assert_eq!(footer_hint(Mode::Terminal, "copied"), "copied");
        assert_eq!(footer_hint(Mode::Terminal, "last pane kept"), "last pane kept");
        assert_eq!(
            footer_hint(Mode::Confirm, "ignored"),
            " 1 yes  2 no  esc"
        );
        assert_ne!(
            footer_hint(Mode::Confirm, "ignored"),
            confirm_ask(ConfirmKind::Pane)
        );
        let onboard = footer_hint(Mode::Onboard, "attach failed");
        assert!(onboard.contains("remember"));
        assert!(onboard.contains("esc"));
        assert!(onboard.contains("dismiss"));
        assert!(!onboard.contains("tiếp"));
        assert!(!onboard.contains("attach failed"));
    }

    #[test]
    fn onboard_helpers_use_injected_path() {
        let root = std::env::temp_dir().join(format!(
            "dory-onboard-{}-{}",
            std::process::id(),
            "helpers"
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let path = root.join("onboarded");
        assert!(!should_show_onboard(None, false));
        assert!(!should_show_onboard(Some(&path), true));
        assert!(should_show_onboard(Some(&path), false));
        assert!(matches!(initial_mode(Some(&path), false), Mode::Onboard));
        assert!(matches!(initial_mode(None, false), Mode::Terminal));
        mark_onboarded(&path).unwrap();
        assert!(path.is_file());
        assert!(!fs::read(&path).unwrap().is_empty());
        assert!(!should_show_onboard(Some(&path), false));
        assert!(matches!(initial_mode(Some(&path), false), Mode::Terminal));
        let blocker = root.join("not-a-dir");
        fs::write(&blocker, b"x").unwrap();
        let bad = blocker.join("onboarded");
        assert!(should_show_onboard(Some(&bad), false));
        assert!(mark_onboarded(&bad).is_err());
        assert!(!bad.exists());
        let as_dir = root.join("dir-flag");
        fs::create_dir_all(&as_dir).unwrap();
        assert!(!should_show_onboard(Some(&as_dir), false));
        assert!(mark_onboarded(&as_dir).is_err());
        let link = root.join("link-flag");
        std::os::unix::fs::symlink(&path, &link).unwrap();
        assert!(!should_show_onboard(Some(&link), false));
        assert!(mark_onboarded(&link).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn onboard_copy_and_input_table() {
        let lines = onboard_lines();
        let blob = lines.join("\n");
        assert!(blob.contains("right-click"));
        assert!(blob.contains("remember"));
        assert!(blob.contains("dismiss"));
        assert!(!blob.contains("không ghi nhớ"));
        assert!(!blob.contains("node bin/dory.js serve"));
        assert!(!blob.contains(":7380"));
        assert!(overlay_paints(Mode::Onboard));
        assert!(!overlay_paints(Mode::Terminal));
        assert_eq!(
            onboard_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            OnboardKey::Persist
        );
        assert_eq!(
            onboard_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)),
            OnboardKey::Persist
        );
        assert_eq!(
            onboard_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
            OnboardKey::Dismiss
        );
        assert_eq!(
            onboard_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
            OnboardKey::Eat
        );
        let ctrl_b = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL);
        assert_eq!(onboard_key(ctrl_b), OnboardKey::Prefix);
        assert_eq!(
            onboard_mouse(MouseEventKind::Down(MouseButton::Left)),
            OnboardMouse::Release
        );
        assert_eq!(
            onboard_mouse(MouseEventKind::Down(MouseButton::Right)),
            OnboardMouse::Release
        );
        assert_eq!(onboard_mouse(MouseEventKind::Moved), OnboardMouse::Eat);
    }

    #[test]
    fn parse_tree_reads_workspace_cwd() {
        let body = r#"{"ok":true,"result":{"focused":"w1:p1","items":[{"k":"w","id":"w1","cwd":"/home/u/flow"},{"k":"t","id":"w1:t1","cwd":"/home/u/flow"},{"k":"p","id":"w1:p1"}]}}"#;
        let (rows, _, ws, _) = parse_tree(body);
        assert_eq!(ws, "w1");
        assert_eq!(rows[0].cwd, "/home/u/flow");
        assert_eq!(rows[1].cwd, "/home/u/flow");
        assert_eq!(workspace_cwd_name("/home/u/flow").as_deref(), Some("flow"));
        assert_eq!(workspace_cwd_name("/").as_deref(), None);
        assert_eq!(workspace_label(&rows, "w1"), "flow");
        assert_eq!(tab_label(&rows, "w1", "w1:t1"), "flow");
    }

    #[test]
    fn sidebar_wide_card_uses_cwd_basename() {
        let mut rows = glance_rows();
        rows[0].cwd = "/home/u/flow".into();
        rows[4].cwd = "/tmp/other".into();
        let model = sidebar_model(&rows, "w1", SIDEBAR, 0);
        let cards: Vec<_> = model
            .iter()
            .filter(|h| matches!(h.kind, SideKind::Workspace))
            .collect();
        assert!(cards[0].text.contains("flow"));
        assert!(!cards[0].text.contains("w1"));
        assert!(cards[1].text.contains("other"));
        assert_eq!(cards[0].workspace, "w1");
        let fallback = sidebar_model(&glance_rows(), "w1", SIDEBAR, 0);
        assert!(fallback.iter().any(|h| h.text.contains("w1")));
        let picker = picker_lines(&rows, 0);
        assert!(picker.iter().any(|l| l.contains("flow")));
        assert!(!picker.iter().any(|l| l.contains("w1")));
    }

    #[test]
    fn title_and_tab_chips_use_folder_names() {
        let body = r#"{"ok":true,"result":{"focused":"w1:p1","items":[{"k":"w","id":"w1","cwd":"/home/u/flow"},{"k":"t","id":"w1:t1","cwd":"/home/u/flow"},{"k":"p","id":"w1:p1"},{"k":"t","id":"w1:t2","cwd":"/home/u/dory"},{"k":"p","id":"w1:p2"}]}}"#;
        let (rows, _, _, _) = parse_tree(body);
        assert_eq!(title_loc(&rows, "w1", "w1:t1", "w1:p1"), "  flow · flow · p1");
        assert_eq!(tab_chip_text(&rows, "w1", "w1:t1", "w1:t1"), "[flow]");
        assert_eq!(tab_chip_text(&rows, "w1", "w1:t2", "w1:t1"), " dory ");
        assert_eq!(
            tab_chip_at(&rows, "w1", SIDEBAR + 1, SIDEBAR).as_deref(),
            Some("w1:p1")
        );
        let next = SIDEBAR + 1 + display_width("[flow]") as u16;
        assert_eq!(tab_chip_at(&rows, "w1", next, SIDEBAR).as_deref(), Some("w1:p2"));
        let collide = r#"{"ok":true,"result":{"focused":"w1:p1","items":[{"k":"w","id":"w1","cwd":"/tmp/flow"},{"k":"t","id":"w1:t1","cwd":"/tmp/flow"},{"k":"p","id":"w1:p1"},{"k":"w","id":"w2","cwd":"/var/flow"},{"k":"t","id":"w2:t1","cwd":"/var/flow"},{"k":"p","id":"w2:p1"},{"k":"t","id":"w2:t2","cwd":"/var/flow"},{"k":"p","id":"w2:p2"}]}}"#;
        let (dup, _, _, _) = parse_tree(collide);
        assert_eq!(workspace_label(&dup, "w1"), "flow w1");
        assert_eq!(workspace_label(&dup, "w2"), "flow w2");
        assert_eq!(tab_label(&dup, "w2", "w2:t1"), "flow t1");
        assert_eq!(tab_label(&dup, "w2", "w2:t2"), "flow t2");
    }

    #[test]
    fn title_chips_mark_prefix_and_zoom() {
        assert_eq!(title_chips(Mode::Terminal, false), "");
        assert_eq!(title_chips(Mode::Prefix, false), "  Ctrl-b");
        assert_eq!(title_chips(Mode::Terminal, true), "  z");
        assert_eq!(title_chips(Mode::Prefix, true), "  Ctrl-b  z");
        let clipped = bar_line(" dory  flow · flow · p1  Ctrl-b  z", 20);
        assert_eq!(display_width(&clipped), 19);
        assert!(!clipped.contains('\n'));
    }

    #[test]
    fn divider_touches_only_focused_shared_edge() {
        let body = r#"{"ok":true,"result":{"focused":"w1:p2","cells":[{"id":"w1:p1","x":0,"y":0,"w":40,"h":22},{"id":"w1:p2","x":40,"y":0,"w":40,"h":22}]}}"#;
        let cells = parse_layout_cells(body);
        let edge = crate::layout::divider_at(&cells, 39, 1).expect("shared edge");
        assert!(edge.0 == "w1:p1" || edge.1 == "w1:p1");
        assert!(divider_touches_focus(&cells, 39, 1, "w1:p1"));
        assert!(divider_touches_focus(&cells, 39, 1, "w1:p2"));
        assert!(!divider_touches_focus(&cells, 39, 1, "w1:p9"));
    }

    #[test]
    fn overlay_box_menu_follows_pointer_and_clamps() {
        let lines = menu_lines(MenuKind::Workspace);
        let max_w = lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
        let fit = overlay_box(Mode::Menu, lines.len(), max_w, 80, 24, SIDEBAR, 2, Some((3, 4)));
        assert_eq!(fit.x, 3);
        assert_eq!(fit.y, 4);
        assert_eq!(fit.h, overlay_paint_rows(Mode::Menu, lines.len(), 21));
        assert!(fit.w >= 8);
        assert!(fit.x + fit.w + 1 <= 80);
        let edge = overlay_box(
            Mode::Menu,
            lines.len(),
            max_w,
            80,
            24,
            SIDEBAR,
            2,
            Some((75, 22)),
        );
        assert_eq!(edge.x, 80 - edge.w - 1);
        assert_eq!(edge.y, 24 - edge.h - 1);
        assert!(overlay_contains(fit, 3, 4, 24));
        assert!(!overlay_contains(fit, 3, 23, 24));
        let confirm = overlay_box(Mode::Confirm, 3, 20, 80, 24, SIDEBAR, 2, Some((3, 4)));
        let (ox, oy) = content_origin(SIDEBAR, 2);
        assert_eq!(confirm.x, ox);
        assert_eq!(confirm.y, oy);
        assert_ne!(confirm.x, 3);
        let cap = 80u16.saturating_sub(ox).saturating_sub(1);
        assert_eq!(confirm.w, 20u16.max(8).min(cap));
        let picker = overlay_box(Mode::Picker, 3, 20, 80, 24, SIDEBAR, 2, Some((3, 4)));
        assert_eq!(picker.x, ox);
        assert_eq!(picker.y, oy);
        assert_eq!(picker.w, 20u16.max(8).min(cap));
        for mode in [Mode::Help, Mode::Onboard] {
            let other = overlay_box(mode, 3, 20, 80, 24, SIDEBAR, 2, Some((3, 4)));
            assert_eq!(other.x, ox);
            assert_eq!(other.y, oy);
            assert_eq!(other.w, cap);
        }
    }

    #[test]
    fn overlay_grammar_copy_hits_and_keys() {
        assert_eq!(
            footer_hint(Mode::Confirm, "ignored"),
            " 1 yes  2 no  esc"
        );
        for kind in [ConfirmKind::Pane, ConfirmKind::Tab, ConfirmKind::Workspace] {
            let blob = confirm_lines(kind).join("\n");
            assert!(!blob.contains("y/n"));
            assert!(!confirm_ask(kind).contains("y/n"));
        }
        assert_eq!(picker_mouse_pick(1, 3), Some(0));
        assert_eq!(picker_mouse_pick(0, 3), None);
        assert_eq!(picker_mouse_pick(2, 3), Some(1));
        let lines = confirm_lines(ConfirmKind::Pane);
        let max_w = lines.iter().map(|l| display_width(l)).max().unwrap_or(0);
        let card = overlay_box(Mode::Confirm, lines.len(), max_w, 80, 24, SIDEBAR, 2, None);
        let (ox, oy) = content_origin(SIDEBAR, 2);
        assert_eq!((card.x, card.y), (ox, oy));
        assert!(overlay_contains(card, card.x, card.y + 1, 24));
        assert!(!overlay_contains(
            card,
            card.x.saturating_add(card.w),
            card.y + 1,
            24
        ));
        assert!(!PREFIX_FOOTER.contains('—'));
        assert!(!onboard_lines()[1].contains('—'));
        assert_eq!(empty_dash(""), "—");
        assert_eq!(last_room_copy(ConfirmKind::Workspace), "last window kept");
        assert_eq!(last_room_copy(ConfirmKind::Pane), "last pane kept");
        assert_eq!(last_room_copy(ConfirmKind::Tab), "last pane kept");
        assert_eq!(confirm_key(KeyCode::Char('y')), ConfirmPick::Yes);
        assert_eq!(confirm_key(KeyCode::Char('Y')), ConfirmPick::Yes);
        assert_eq!(confirm_key(KeyCode::Char('1')), ConfirmPick::Yes);
        assert_eq!(confirm_key(KeyCode::Enter), ConfirmPick::Yes);
        assert_eq!(confirm_key(KeyCode::Char('n')), ConfirmPick::No);
        assert_eq!(confirm_key(KeyCode::Char('N')), ConfirmPick::No);
        assert_eq!(confirm_key(KeyCode::Char('2')), ConfirmPick::No);
        assert_eq!(confirm_key(KeyCode::Esc), ConfirmPick::No);
    }

    fn glance_tmp(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "dory-flow-glance-{}-{tag}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join(".dory").join("sessions")).unwrap();
        root
    }

    fn glance_journal(root: &Path) -> PathBuf {
        root.join(".dory").join("sessions").join("s1.jsonl")
    }

    #[test]
    fn last_flow_result_skips_missing_invoke_and_broken() {
        let missing = PathBuf::from("/no/such/dory-flow-glance.jsonl");
        assert!(last_flow_result(&missing).is_none());
        assert!(last_flow_result_bytes(b"{\"type\":\"flow/invoke\",\"args\":[\"code\"]}\n").is_none());
        assert!(last_flow_result_bytes(b"{not json\n").is_none());
        assert!(last_flow_result_bytes(b"{\"type\":\"flow/result\",\"error\":\"unterminated\n").is_none());
    }

    #[test]
    fn last_flow_result_picks_final_code_and_ignores_stdout_keys() {
        let two = concat!(
            r#"{"type":"flow/result","args":["first"],"code":1}"#,
            "\n",
            r#"{"type":"flow/result","args":["code"],"code":0}"#,
            "\n"
        );
        let g = last_flow_result_bytes(two.as_bytes()).unwrap();
        assert_eq!(g.code, Some(0));
        assert_eq!(flow_glance_line(&g), "Flow 0. code");
        let seven = last_flow_result_bytes(br#"{"type":"flow/result","code":7}"#).unwrap();
        assert_eq!(seven.code, Some(7));
        assert_eq!(flow_glance_line(&seven), "Flow 7.");
        let nested = last_flow_result_bytes(
            br#"{"type":"flow/result","stdout":"{\"code\":99}","code":0}"#,
        )
        .unwrap();
        assert_eq!(nested.code, Some(0));
        assert_eq!(flow_glance_line(&nested), "Flow 0.");
        let pwn = last_flow_result_bytes(
            br#"{"type":"flow/result","stdout":"{\"error\":\"pwn\"}","code":0}"#,
        )
        .unwrap();
        let line = flow_glance_line(&pwn);
        assert_eq!(line, "Flow 0.");
        assert!(!line.contains("lỗi"));
        let timeout = last_flow_result_bytes(
            br#"{"type":"flow/result","code":null,"error":"timed out after 15000ms"}"#,
        )
        .unwrap();
        assert_eq!(timeout.code, None);
        assert_eq!(
            flow_glance_line(&timeout),
            "Flow error. timed out after 15000ms"
        );
        let obj = r#"{"type":"flow/result","args":["code"],"code":0}"#;
        assert_eq!(top_json_first_arg(obj).as_deref(), Some("code"));
        assert_eq!(top_json_i32(obj, "code"), Some(0));
        assert_eq!(top_json_string(obj, "type").as_deref(), Some("flow/result"));
        assert_eq!(top_json_i32(r#"{"code":null}"#, "code"), None);
    }

    #[test]
    fn clip_glance_keeps_vietnamese_and_strips_controls() {
        let nl = last_flow_result_bytes(
            br#"{"type":"flow/result","args":["a\nb"],"code":0}"#,
        )
        .unwrap();
        let nl_line = flow_glance_line(&nl);
        assert!(!nl_line.contains('\n'));
        let esc = last_flow_result_bytes(
            br#"{"type":"flow/result","code":null,"error":"\u001b[2J"}"#,
        )
        .unwrap();
        let esc_line = flow_glance_line(&esc);
        assert!(!esc_line.contains('\u{1b}'));
        let loi = last_flow_result_bytes(
            "{\"type\":\"flow/result\",\"code\":null,\"error\":\"lỗi\"}".as_bytes(),
        )
        .unwrap();
        assert_eq!(flow_glance_line(&loi), "Flow error. lỗi");
        let da = last_flow_result_bytes(
            "{\"type\":\"flow/result\",\"args\":[\"đã\"],\"code\":0}".as_bytes(),
        )
        .unwrap();
        assert_eq!(flow_glance_line(&da), "Flow 0. đã");
        let mixed = format!(
            "{{\"type\":\"flow/result\",\"code\":null,\"error\":\"lỗi{}{}{}\"}}",
            '\u{2028}', '\u{009B}', '\u{202E}'
        );
        let g = last_flow_result_bytes(mixed.as_bytes()).unwrap();
        let line = flow_glance_line(&g);
        assert!(line.contains("lỗi"));
        assert!(!line.contains('\u{2028}'));
        assert!(!line.contains('\u{009B}'));
        assert!(!line.contains('\u{202E}'));
        assert!(line.contains(' '));
    }

    #[test]
    fn last_flow_result_reads_small_and_tail_and_rejects_special_files() {
        let root = glance_tmp("io");
        let path = glance_journal(&root);
        fs::write(
            &path,
            concat!(
                r#"{"type":"flow/invoke","args":["code"]}"#,
                "\n",
                r#"{"type":"flow/result","args":["code"],"code":0}"#,
                "\n"
            ),
        )
        .unwrap();
        let small = last_flow_result(&path).unwrap();
        assert_eq!(flow_glance_line(&small), "Flow 0. code");
        let mut big = Vec::new();
        while big.len() < 70_000 {
            big.extend_from_slice(b"{\"type\":\"flow/invoke\",\"args\":[]}\n");
        }
        big.extend_from_slice(br#"{"type":"flow/result","args":["tail"],"code":7}"#);
        big.push(b'\n');
        fs::write(&path, &big).unwrap();
        let tail = last_flow_result(&path).unwrap();
        assert_eq!(flow_glance_line(&tail), "Flow 7. tail");
        assert!(last_flow_result(&path).is_some());
        fs::remove_file(&path).unwrap();
        assert!(last_flow_result(&path).is_none());
        fs::write(&path, br#"{"type":"flow/result","code":0}"#).unwrap();
        let mut glance = last_flow_result(&path).map(|g| flow_glance_line(&g));
        let mut mtime = fs::symlink_metadata(&path)
            .ok()
            .and_then(|m| m.modified().ok());
        let mut cached = Some(path.clone());
        assert!(glance.is_some());
        fs::remove_file(&path).unwrap();
        assert!(poll_flow_glance(
            Some(&path),
            &mut glance,
            &mut mtime,
            &mut cached
        ));
        assert!(glance.is_none());
        fs::write(&path, br#"{"type":"flow/result","code":0}"#).unwrap();
        let link = root.join("link.jsonl");
        std::os::unix::fs::symlink(&path, &link).unwrap();
        assert!(last_flow_result(&link).is_none());
        let fifo = root.join("fifo.jsonl");
        let st = std::process::Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap();
        assert!(st.success());
        assert!(last_flow_result(&fifo).is_none());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn footer_line_glance_idle_status_and_overlays() {
        let idle = " right-click menu  drag≥2 copy  Ctrl-b prefix";
        assert_eq!(footer_hint(Mode::Terminal, ""), idle);
        assert_eq!(
            footer_line(Mode::Terminal, "", Some("Flow 0. code")),
            "Flow 0. code"
        );
        assert_eq!(footer_line(Mode::Terminal, "", None), idle);
        assert_eq!(
            footer_line(Mode::Terminal, "copied", Some("Flow 0. code")),
            "copied"
        );
        assert_eq!(
            footer_line(Mode::Confirm, "", Some("Flow 0. code")),
            " 1 yes  2 no  esc"
        );
        assert_eq!(
            footer_line(Mode::Prefix, "", Some("Flow 0. code")),
            PREFIX_FOOTER
        );
        assert_eq!(
            footer_line(Mode::Help, "", Some("Flow 0. code")),
            " esc close keys"
        );
        assert_eq!(
            footer_line(Mode::Picker, "", Some("Flow 0. code")),
            " j/k pick  enter  esc"
        );
        assert_eq!(
            footer_line(Mode::Menu, "", Some("Flow 0. code")),
            " 1.. run  esc cancel"
        );
        assert_eq!(
            footer_line(Mode::Onboard, "", Some("Flow 0. code")),
            " enter remember  esc dismiss  Ctrl-b q leave"
        );
        let long = format!("Flow 0. {}", "x".repeat(200));
        let painted = bar_line(&format!(" {long}"), 40);
        assert_eq!(display_width(&painted), 39);
        assert!(display_width(&painted) <= 39);
        assert_eq!(
            flow_journal_path("/live/after/cd"),
            PathBuf::from("/live/after/cd/.dory/sessions/s1.jsonl")
        );
        let rows = empty_shell_rows("/live/after/cd");
        assert_eq!(workspace_cwd(&rows, "w1"), Some("/live/after/cd"));
        assert!(!pane_wipe_on_tile_draw(false));
    }

    #[test]
    fn error_toasts_and_empty_tile_are_english() {
        for toast in [
            "no window",
            "tab failed",
            "tab: no pane",
            "window failed",
            "window: no pane",
            "no pane",
            "split failed",
            "split: no pane",
            "not closed",
            "close failed",
            "copied",
            "could not remember",
        ] {
            assert_eq!(footer_hint(Mode::Terminal, toast), toast);
        }
        let blob = format!(
            "{}\n{}\n{}\n{}\n{}\n empty pane  Ctrl-b c tab · v/- split",
            help_text(),
            onboard_lines().join("\n"),
            PREFIX_FOOTER,
            footer_hint(Mode::Terminal, ""),
            confirm_lines(ConfirmKind::Pane).join("\n"),
        );
        for vi in [
            "thẻ lỗi",
            "tách lỗi",
            "không đóng",
            "đóng lỗi",
            "không có ô",
            "không có cửa sổ",
            "đã chép",
            "Tách phải",
            "chuột phải",
            "ô trống",
            "ô cuối giữ",
        ] {
            assert!(!blob.contains(vi), "{vi}");
        }
        assert!(blob.contains("empty pane"));
        assert!(blob.contains("1 yes"));
        assert!(flow_glance_line(&FlowGlance {
            arg0: String::new(),
            code: None,
            error: Some("x".into()),
        })
        .starts_with("Flow error. "));
    }
}
