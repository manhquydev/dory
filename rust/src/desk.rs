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
    DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode,
};
use crossterm::{execute, queue};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::time::{Duration, Instant};

const SIDEBAR: u16 = 22;
const SIDEBAR_DOTS: u16 = 4;
const TAB_ROW: u16 = 1;
const CTRL_B: u8 = 0x02;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Terminal,
    Prefix,
    Picker,
    Confirm,
    Help,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConfirmKind {
    Pane,
    Tab,
    Workspace,
}
const TITLE_BG: Color = Color::Rgb {
    r: 24,
    g: 36,
    b: 52,
};
const SIDE_BG: Color = Color::Rgb {
    r: 14,
    g: 18,
    b: 26,
};
const PANE_BG: Color = Color::Rgb { r: 8, g: 10, b: 14 };
const ACCENT: Color = Color::Rgb {
    r: 80,
    g: 196,
    b: 214,
};
const MUTED: Color = Color::Rgb {
    r: 120,
    g: 132,
    b: 148,
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

struct Row {
    kind: char,
    id: String,
    #[allow(dead_code)]
    focus_pane: String,
    occ: String,
    st: String,
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
    confirm: Option<ConfirmKind>,
    sel_from: Option<(u16, u16)>,
    sel_to: Option<(u16, u16)>,
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
            sel_from: None,
            sel_to: None,
        };
        desk.refresh_tree();
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
                self.chrome_dirty = true;
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
            if self.chrome_dirty || self.tiles_dirty {
                self.draw(out)?;
                self.chrome_dirty = false;
                self.tiles_dirty = false;
            }
        }
    }

    fn tree_sig(&self) -> String {
        let mut s = self.focused.clone();
        for row in &self.rows {
            s.push('|');
            s.push_str(&row.id);
            s.push('/');
            s.push_str(&row.st);
            s.push('/');
            s.push_str(&row.occ);
        }
        s
    }

    fn refresh_tree(&mut self) {
        self.last_tree = Instant::now();
        let Ok(body) = server::rpc_line_quiet(r#"{"op":"desk.tree"}"#) else {
            return;
        };
        if !body.contains("\"ok\":true") {
            return;
        }
        let (rows, focused, workspace, tab) = parse_tree(&body);
        self.rows = rows;
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
        if self.workspace.is_empty() {
            if let Some(w) = workspace_of(&self.rows, &self.focused) {
                self.workspace = w;
            }
        }
        if self.tab.is_empty() {
            if let Some(t) = tab_of(&self.rows, &self.focused) {
                self.tab = t;
            }
        }
    }

    fn focus_tile(&mut self, id: &str) {
        if id.is_empty() {
            return;
        }
        let _ = server::rpc_line_quiet(&format!(r#"{{"op":"pane.focus","pane":"{id}"}}"#));
        self.focused = id.to_string();
        self.refresh_tree();
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
            Mode::Prefix => {
                self.mode = Mode::Terminal;
                self.prefix_cmd(key)
            }
            Mode::Terminal => {
                if is_ctrl_b(&key) {
                    self.mode = Mode::Prefix;
                    self.prefix_at = Instant::now();
                    self.status =
                        "Ctrl-b — q detach  c tab  n/p tabs  w pick  Shift-n ws  x close"
                            .to_string();
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
        match key.code {
            KeyCode::Char('y' | 'Y') => {
                let kind = self.confirm.take();
                self.mode = Mode::Terminal;
                if let Some(kind) = kind {
                    self.run_close(kind);
                }
                self.chrome_dirty = true;
            }
            KeyCode::Char('n' | 'N') | KeyCode::Esc => {
                self.confirm = None;
                self.mode = Mode::Terminal;
                self.status.clear();
                self.chrome_dirty = true;
            }
            _ => {}
        }
        false
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
            KeyCode::Enter => {
                if let Some(ws) = spaces.get(self.picker_idx) {
                    if let Some(pane) = first_pane_of(&self.rows, ws) {
                        self.zoomed = false;
                        self.focus_tile(&pane);
                        self.reconcile_tiles();
                    }
                }
                self.mode = Mode::Terminal;
                self.status.clear();
                self.chrome_dirty = true;
            }
            KeyCode::Esc => {
                self.mode = Mode::Terminal;
                self.status.clear();
                self.chrome_dirty = true;
            }
            _ => {}
        }
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
                self.status = help_text().to_string();
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
                        if id != self.focused {
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
            _ => {}
        }
        false
    }

    fn new_tab(&mut self) {
        if self.workspace.is_empty() {
            self.refresh_tree();
        }
        if self.workspace.is_empty() {
            self.status = "no workspace".to_string();
            return;
        }
        let body = match server::rpc_line_quiet(&format!(
            r#"{{"op":"tab.create","workspace":"{}"}}"#,
            self.workspace
        )) {
            Ok(b) => b,
            Err(_) => {
                self.status = "tab.create failed".to_string();
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
            self.status = "tab.create: no pane".to_string();
            self.refresh_tree();
            self.reconcile_tiles();
        }
    }

    fn new_workspace(&mut self) {
        let body = match server::rpc_line_quiet(r#"{"op":"workspace.create"}"#) {
            Ok(b) => b,
            Err(_) => {
                self.status = "workspace.create failed".to_string();
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
            self.status = "workspace.create: no pane".to_string();
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
        self.status = "workspace picker  j/k  enter  esc".to_string();
        self.chrome_dirty = true;
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

    fn ask_confirm(&mut self, kind: ConfirmKind) {
        self.confirm = Some(kind);
        self.mode = Mode::Confirm;
        self.status = match kind {
            ConfirmKind::Pane => "close pane? y/n".to_string(),
            ConfirmKind::Tab => "close tab? y/n".to_string(),
            ConfirmKind::Workspace => "close workspace? y/n".to_string(),
        };
        self.chrome_dirty = true;
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
        self.run_close(ConfirmKind::Pane);
    }

    fn run_close(&mut self, kind: ConfirmKind) {
        let line = match kind {
            ConfirmKind::Pane => format!(r#"{{"op":"pane.close","pane":"{}"}}"#, self.focused),
            ConfirmKind::Tab => format!(r#"{{"op":"tab.close","tab":"{}"}}"#, self.tab),
            ConfirmKind::Workspace => {
                format!(r#"{{"op":"workspace.close","workspace":"{}"}}"#, self.workspace)
            }
        };
        match server::rpc_line_quiet(&line) {
            Ok(body) if body.contains("\"ok\":true") => {
                self.zoomed = false;
                self.drag = None;
                self.refresh_tree();
                self.reconcile_tiles();
                self.status.clear();
            }
            Ok(body) => {
                self.status = if body.contains("last live pane") {
                    "last pane stays".to_string()
                } else {
                    "close refused".to_string()
                };
            }
            Err(_) => self.status = "close failed".to_string(),
        }
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
        queue!(out, Hide, ResetColor)?;
        if self.chrome_dirty {
            self.draw_title(out, cols)?;
            self.draw_tab_bar(out, cols)?;
            self.draw_sidebar(out, rows)?;
            self.draw_footer(out, cols, rows)?;
        }
        if self.tiles_dirty {
            self.draw_tiles(out, cols, rows)?;
        }
        if matches!(self.mode, Mode::Help | Mode::Picker) {
            self.draw_overlay(out, cols, rows)?;
        }
        self.place_cursor(out)?;
        out.flush()
    }

    fn draw_title(&self, out: &mut io::Stdout, cols: u16) -> io::Result<()> {
        let loc = format!(
            "  {} · {} · {}",
            empty_dash(&self.workspace),
            empty_dash(&self.tab),
            empty_dash(&self.focused)
        );
        let left = " dory";
        let line = bar_line(&format!("{left}{loc}"), cols);
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
            let short = id.rsplit(':').next().unwrap_or(&id);
            if id == self.tab {
                line.push_str(&format!("[{short}]"));
            } else {
                line.push_str(&format!(" {short} "));
            }
        }
        if self.mode == Mode::Picker {
            line.push_str("  pick");
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
        let lines = sidebar_lines(&self.rows, &self.workspace, &self.focused, side);
        for y in 0..height {
            let screen_y = y + self.top_rows;
            queue!(out, MoveTo(0, screen_y), SetBackgroundColor(SIDE_BG))?;
            let text = lines.get(y as usize).cloned().unwrap_or_else(|| " ".repeat(side as usize));
            let mut padded = text;
            clip_to(&mut padded, side as usize);
            while display_width(&padded) < side as usize {
                padded.push(' ');
            }
            queue!(out, SetForegroundColor(TEXT), Print(padded))?;
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
        let fill = width
            .min(cols.saturating_sub(origin_x).saturating_sub(1))
            .max(1);
        for y in 0..height {
            queue!(
                out,
                MoveTo(origin_x, y + origin_y),
                SetBackgroundColor(PANE_BG),
                SetForegroundColor(PANE_BG),
                Print(" ".repeat(fill as usize))
            )?;
        }
        if self.tiles.is_empty() {
            let msg = if self.status.is_empty() {
                " no live pane "
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
                            queue!(
                                out,
                                MoveTo(origin_x + x + w, origin_y + row),
                                SetBackgroundColor(TITLE_BG),
                                SetForegroundColor(MUTED),
                                Print("│")
                            )?;
                        }
                    }
                    if h < cell.h && y + h < height {
                        queue!(
                            out,
                            MoveTo(origin_x + x, origin_y + y + h),
                            SetBackgroundColor(TITLE_BG),
                            SetForegroundColor(MUTED),
                            Print("─".repeat(w.min(width.saturating_sub(x)) as usize))
                        )?;
                    }
                }
            }
        }
        queue!(out, ResetColor)?;
        Ok(())
    }

    fn draw_footer(&self, out: &mut io::Stdout, cols: u16, rows: u16) -> io::Result<()> {
        let hint = if self.mode == Mode::Prefix {
            " ^B — q detach  c tab  n/p tabs  w pick  Shift-n ws  x close  ? help"
        } else if !self.status.is_empty() {
            self.status.as_str()
        } else {
            " ^B q detach  w pick  n/p tabs  hjkl  z  x close  drag≥2 copy"
        };
        let line = bar_line(&format!(" {hint}"), cols);
        queue!(
            out,
            MoveTo(0, rows.saturating_sub(1)),
            SetBackgroundColor(TITLE_BG),
            SetForegroundColor(MUTED),
            Print(line),
            ResetColor
        )
    }

    fn draw_overlay(&self, out: &mut io::Stdout, cols: u16, rows: u16) -> io::Result<()> {
        let (origin_x, origin_y) = content_origin(self.sidebar_cols, self.top_rows);
        let width = cols.saturating_sub(origin_x).saturating_sub(1);
        let height = rows.saturating_sub(self.top_rows + 1);
        if width == 0 || height == 0 {
            return Ok(());
        }
        let lines: Vec<String> = match self.mode {
            Mode::Help => help_text().lines().map(|s| s.to_string()).collect(),
            Mode::Picker => picker_lines(&self.rows, self.picker_idx),
            _ => return Ok(()),
        };
        for y in 0..height {
            let mut text = lines
                .get(y as usize)
                .cloned()
                .unwrap_or_else(|| String::new());
            clip_to(&mut text, width as usize);
            while display_width(&text) < width as usize {
                text.push(' ');
            }
            queue!(
                out,
                MoveTo(origin_x, origin_y + y),
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

fn parse_item(obj: &str) -> Option<Item> {
    let k = attach::json_string_field(obj, "k")?;
    let id = attach::json_string_field(obj, "id")?;
    Some(Item {
        k: k.chars().next().unwrap_or('p'),
        id,
        occ: attach::json_string_field(obj, "occ").unwrap_or_default(),
        st: attach::json_string_field(obj, "st").unwrap_or_default(),
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
    ws
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
    tab
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

fn rollup_of(rows: &[Row], ws: &str) -> &'static str {
    let mut best = "unknown";
    let mut best_r = 0u8;
    let mut in_ws = false;
    for row in rows {
        if row.kind == 'w' {
            in_ws = row.id == ws;
            continue;
        }
        if in_ws && row.kind == 'p' {
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

struct SideHit {
    text: String,
    pane: Option<String>,
}

fn sidebar_model(rows: &[Row], workspace: &str, side: u16) -> Vec<SideHit> {
    let width = side as usize;
    if width == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let push = |out: &mut Vec<SideHit>, text: String, pane: Option<String>| {
        out.push(SideHit {
            text: pad_cols(text, width),
            pane,
        });
    };
    if width <= SIDEBAR_DOTS as usize {
        for ws in workspaces_of(rows) {
            let ch = match rollup_of(rows, &ws) {
                "blocked" => 'B',
                "working" => 'W',
                "done" => 'D',
                "idle" => 'I',
                _ => 'U',
            };
            push(&mut out, format!(" {ch}"), first_pane_of(rows, &ws));
        }
        push(&mut out, "──".into(), None);
        for pane in agents_from(rows) {
            let ch = pane.occ.chars().next().unwrap_or('·');
            push(&mut out, format!(" {ch}"), Some(pane.id.clone()));
        }
        return out;
    }
    push(&mut out, " Spaces".into(), None);
    for ws in workspaces_of(rows) {
        let st = rollup_of(rows, &ws);
        let mark = if ws == workspace { "●" } else { "○" };
        push(
            &mut out,
            format!(" {mark} {ws} {st}"),
            first_pane_of(rows, &ws),
        );
    }
    push(&mut out, " ─".into(), None);
    push(&mut out, " Agents".into(), None);
    for pane in agents_from(rows) {
        let st = normalize_st(&pane.st);
        let short = pane.id.rsplit(':').next().unwrap_or(&pane.id);
        push(
            &mut out,
            format!(" {} {st} {short}", pane.occ),
            Some(pane.id.clone()),
        );
    }
    out
}

fn sidebar_lines(rows: &[Row], workspace: &str, _focused: &str, side: u16) -> Vec<String> {
    sidebar_model(rows, workspace, side)
        .into_iter()
        .map(|h| h.text)
        .collect()
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
    sidebar_model(rows, workspace, side)
        .get(idx)
        .and_then(|h| h.pane.clone())
}

fn tab_chip_at(rows: &[Row], workspace: &str, column: u16, side: u16) -> Option<String> {
    let mut x = if side == 0 { 0 } else { side + 1 };
    for (id, pane) in tabs_of(rows, workspace) {
        let short = id.rsplit(':').next().unwrap_or(&id);
        let width = (short.len() + 2) as u16;
        if column >= x && column < x + width {
            return Some(pane);
        }
        x = x.saturating_add(width);
    }
    None
}

fn help_text() -> &'static str {
    "Ctrl-b prefix\n\
     q / d       detach (PTY stays)\n\
     Shift-d     close workspace\n\
     c           new tab\n\
     v / -       split right / down\n\
     n / p       next / prev tab (this workspace)\n\
     1-9         tab\n\
     hjkl        pane\n\
     w           workspace picker (not create)\n\
     Shift-n     new workspace\n\
     x           close pane\n\
     Shift-x     close tab\n\
     z           zoom (streams stay)\n\
     b           collapse sidebar\n\
     Ctrl-b      send C-b to pane\n\
     ?           this help\n\
     drag >=2    copy (OSC 52)\n\
     Esc / q / ? leave help"
}

fn picker_lines(rows: &[Row], idx: usize) -> Vec<String> {
    let mut lines = vec![" workspace picker  j/k  enter  esc".to_string()];
    for (i, ws) in workspaces_of(rows).into_iter().enumerate() {
        let mark = if i == idx { '>' } else { ' ' };
        lines.push(format!("{mark} {ws}"));
    }
    lines
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
            },
            Row {
                kind: 't',
                id: "w1:t1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: String::new(),
            },
            Row {
                kind: 'p',
                id: "w1:p1".into(),
                focus_pane: "w1:p1".into(),
                occ: String::new(),
                st: "working".into(),
            },
            Row {
                kind: 'p',
                id: "w1:p2".into(),
                focus_pane: "w1:p2".into(),
                occ: "coder".into(),
                st: "blocked".into(),
            },
            Row {
                kind: 'w',
                id: "w2".into(),
                focus_pane: "w2:p1".into(),
                occ: String::new(),
                st: String::new(),
            },
            Row {
                kind: 't',
                id: "w2:t1".into(),
                focus_pane: "w2:p1".into(),
                occ: String::new(),
                st: String::new(),
            },
            Row {
                kind: 'p',
                id: "w2:p1".into(),
                focus_pane: "w2:p1".into(),
                occ: "reviewer".into(),
                st: "unknown".into(),
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
        assert!(help_text().contains("workspace picker"));
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
}
