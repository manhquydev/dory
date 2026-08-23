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
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, queue};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::time::{Duration, Instant};

const SIDEBAR: u16 = 22;
const CTRL_B: u8 = 0x02;
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
    prefix: bool,
    prefix_at: Instant,
    last_tree: Instant,
    cols: u16,
    rows_n: u16,
    pty_cols: u16,
    pty_rows: u16,
    dirty: bool,
    drag: Option<Drag>,
    zoomed: bool,
}

impl Desk {
    fn open(start_pane: Option<&str>) -> io::Result<Self> {
        let (cols, rows_n) = term_size();
        let (pty_cols, pty_rows) = pane_size(cols, rows_n);
        let mut desk = Self {
            tiles: Vec::new(),
            cells: Vec::new(),
            rows: Vec::new(),
            focused: String::new(),
            workspace: String::new(),
            tab: String::new(),
            status: String::new(),
            prefix: false,
            prefix_at: Instant::now(),
            last_tree: Instant::now() - Duration::from_secs(2),
            cols,
            rows_n,
            pty_cols,
            pty_rows,
            dirty: true,
            drag: None,
            zoomed: false,
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
            let mut progressed = self.pump_pty();
            if self.last_tree.elapsed() >= Duration::from_millis(400) {
                let before = self.tree_sig();
                self.refresh_tree();
                let focused_painted = self.tiles.iter().any(|t| t.id == self.focused);
                if self.tree_sig() != before || !focused_painted {
                    self.reconcile_tiles();
                    progressed = true;
                }
            }
            if self.prefix && self.prefix_at.elapsed() > Duration::from_secs(2) {
                self.prefix = false;
                progressed = true;
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
                        progressed = true;
                    }
                    Event::Mouse(mouse) => {
                        if self.handle_mouse(mouse) {
                            return Ok(());
                        }
                        progressed = true;
                    }
                    Event::Resize(cols, rows) => {
                        self.resize(cols, rows);
                        progressed = true;
                    }
                    Event::Paste(text) => {
                        self.write_pty(text.as_bytes());
                        progressed = true;
                    }
                    Event::FocusGained | Event::FocusLost => {}
                }
            }
            if progressed || self.dirty {
                self.draw(out)?;
                self.dirty = false;
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
        self.dirty = true;
    }

    fn reconcile_tiles(&mut self) {
        let (cols, rows) = if self.zoomed {
            (self.pty_cols, self.pty_rows)
        } else {
            (self.pty_cols, self.pty_rows)
        };
        let body = match server::rpc_line_quiet(&format!(
            r#"{{"op":"desk.layout","cols":{cols},"rows":{rows}}}"#
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
        let wanted: Vec<(String, u16, u16, u16, u16)> = self
            .cells
            .iter()
            .filter_map(|c| {
                let (x, y, w, h) = if self.zoomed {
                    if c.id != self.focused {
                        return None;
                    }
                    (0, 0, self.pty_cols.max(1), self.pty_rows.max(1))
                } else {
                    crate::layout::inset(&self.cells, &c.id).unwrap_or((c.x, c.y, c.w, c.h))
                };
                Some((c.id.clone(), x, y, w.max(1), h.max(1)))
            })
            .collect();
        self.tiles
            .retain(|t| wanted.iter().any(|(id, ..)| id == &t.id));
        for (id, x, y, w, h) in &wanted {
            if let Some(tile) = self.tiles.iter_mut().find(|t| t.id == *id) {
                if tile.w != *w || tile.h != *h {
                    tile.parser.set_size(*h, *w);
                    let _ = server::rpc_line_quiet(&format!(
                        r#"{{"op":"pane.resize","pane":"{id}","cols":{w},"rows":{h}}}"#
                    ));
                }
                tile.x = *x;
                tile.y = *y;
                tile.w = *w;
                tile.h = *h;
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
        self.dirty = true;
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows_n = rows;
        let (pty_cols, pty_rows) = pane_size(cols, rows);
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
        if self.prefix {
            self.prefix = false;
            return self.prefix_cmd(key);
        }
        if is_ctrl_b(&key) {
            self.prefix = true;
            self.prefix_at = Instant::now();
            self.status = "Ctrl-b — q detach  c tab  v/- split  hjkl  z zoom  n/p  w".to_string();
            return false;
        }
        if let Some(bytes) = encode_key(key) {
            self.write_pty(&bytes);
        }
        false
    }

    fn prefix_cmd(&mut self, key: KeyEvent) -> bool {
        self.status.clear();
        match key.code {
            KeyCode::Char('q' | 'Q' | 'd' | 'D') => return true,
            KeyCode::Char('c' | 'C') => self.new_tab(),
            KeyCode::Char('v' | 'V') => self.split("right"),
            KeyCode::Char('-' | '_') => self.split("down"),
            KeyCode::Char('n' | 'N') => self.neighbor("next"),
            KeyCode::Char('p' | 'P') => self.neighbor("prev"),
            KeyCode::Char('h' | 'H') => self.neighbor("left"),
            KeyCode::Char('j' | 'J') => self.neighbor("down"),
            KeyCode::Char('k' | 'K') => self.neighbor("up"),
            KeyCode::Char('l' | 'L') => self.neighbor("right"),
            KeyCode::Char('z' | 'Z') => {
                self.zoomed = !self.zoomed;
                self.drag = None;
                self.reconcile_tiles();
            }
            KeyCode::Char('w' | 'W') => self.new_workspace(),
            KeyCode::Char('b') | KeyCode::Char('\u{2}') => self.write_pty(&[CTRL_B]),
            KeyCode::Esc => {}
            _ => {
                if is_ctrl_b(&key) {
                    self.write_pty(&[CTRL_B]);
                }
            }
        }
        false
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
        let content_x = mouse.column.saturating_sub(SIDEBAR + 1);
        let content_y = mouse.row.saturating_sub(1);
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if mouse.row == 0 || mouse.row + 1 >= self.rows_n {
                    return false;
                }
                if mouse.column < SIDEBAR {
                    self.drag = None;
                    let idx = mouse.row.saturating_sub(1) as usize;
                    if let Some(row) = self.rows.get(idx) {
                        if !row.focus_pane.is_empty() {
                            let id = row.focus_pane.clone();
                            if id != self.focused {
                                self.zoomed = false;
                                self.focus_tile(&id);
                                self.reconcile_tiles();
                            }
                        }
                    }
                    return false;
                }
                if self.zoomed {
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
                    return false;
                }
                if let Some(cell) = crate::layout::cell_at(&self.cells, content_x, content_y) {
                    let id = cell.id.clone();
                    if id != self.focused {
                        self.focus_tile(&id);
                    }
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
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
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
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
                }
            }
            MouseEventKind::ScrollUp if mouse.column >= SIDEBAR => {
                self.write_pty(b"\x1b[A");
            }
            MouseEventKind::ScrollDown if mouse.column >= SIDEBAR => {
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
        let rows = self.rows_n.max(2);
        queue!(out, Hide, ResetColor)?;
        self.draw_title(out, cols)?;
        self.draw_sidebar(out, rows)?;
        self.draw_tiles(out, cols, rows)?;
        self.draw_footer(out, cols, rows)?;
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
        let mut line = format!("{left}{loc}");
        while display_width(&line) < cols as usize {
            line.push(' ');
        }
        clip_to(&mut line, cols as usize);
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

    fn draw_sidebar(&self, out: &mut io::Stdout, rows: u16) -> io::Result<()> {
        let height = rows.saturating_sub(2);
        for y in 0..height {
            let screen_y = y + 1;
            let idx = y as usize;
            queue!(out, MoveTo(0, screen_y), SetBackgroundColor(SIDE_BG))?;
            if let Some(row) = self.rows.get(idx) {
                self.draw_tree_row(out, row)?;
            } else {
                queue!(
                    out,
                    SetForegroundColor(MUTED),
                    Print(" ".repeat(SIDEBAR as usize))
                )?;
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

    fn draw_tree_row(&self, out: &mut io::Stdout, row: &Row) -> io::Result<()> {
        let focused = row.kind == 'p' && row.id == self.focused
            || (row.kind != 'p' && row.focus_pane == self.focused);
        let (indent, mark) = match row.kind {
            'w' => ("", ""),
            't' => ("  ", ""),
            _ => (
                "    ",
                if row.id == self.focused {
                    "● "
                } else {
                    "○ "
                },
            ),
        };
        let name = match row.kind {
            'w' => row.id.clone(),
            't' => row.id.rsplit(':').next().unwrap_or(&row.id).to_string(),
            _ => row.id.rsplit(':').next().unwrap_or(&row.id).to_string(),
        };
        let mut extra = String::new();
        if !row.occ.is_empty() {
            extra.push(' ');
            extra.push_str(&row.occ);
        }
        if !row.st.is_empty() && row.st != "unknown" {
            extra.push(' ');
            extra.push_str(&row.st);
        }
        let mut label = format!("{indent}{mark}{name}{extra}");
        clip_to(&mut label, SIDEBAR as usize);
        while display_width(&label) < SIDEBAR as usize {
            label.push(' ');
        }
        let fg = match (focused, row.st.as_str()) {
            (true, _) => FOCUSED_FG,
            (_, "working") => Color::Yellow,
            (_, "blocked") => Color::Red,
            (_, "idle" | "done") => Color::Green,
            _ => TEXT,
        };
        let attr = if row.kind == 'w' || focused {
            Attribute::Bold
        } else {
            Attribute::Reset
        };
        queue!(
            out,
            SetForegroundColor(fg),
            SetAttribute(attr),
            Print(&label),
            SetAttribute(Attribute::Reset),
            SetBackgroundColor(SIDE_BG)
        )?;
        Ok(())
    }

    fn draw_tiles(&self, out: &mut io::Stdout, cols: u16, rows: u16) -> io::Result<()> {
        let origin_x = SIDEBAR + 1;
        let width = cols.saturating_sub(origin_x);
        let height = rows.saturating_sub(2);
        if width == 0 || height == 0 {
            return Ok(());
        }
        for y in 0..height {
            queue!(
                out,
                MoveTo(origin_x, y + 1),
                SetBackgroundColor(PANE_BG),
                SetForegroundColor(PANE_BG),
                Print(" ".repeat(width as usize))
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
                MoveTo(origin_x, 1),
                SetBackgroundColor(PANE_BG),
                SetForegroundColor(MUTED),
                Print(format!(" {msg}"))
            )?;
            queue!(out, ResetColor)?;
            return Ok(());
        }
        for tile in &self.tiles {
            let screen = tile.parser.screen();
            for y in 0..tile.h {
                if y >= height {
                    break;
                }
                queue!(out, MoveTo(origin_x + tile.x, y + 1 + tile.y))?;
                for x in 0..tile.w {
                    if tile.x + x >= width {
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
                                MoveTo(origin_x + x + w, 1 + row),
                                SetBackgroundColor(TITLE_BG),
                                SetForegroundColor(MUTED),
                                Print("│")
                            )?;
                        }
                    }
                    if h < cell.h && y + h < height {
                        queue!(
                            out,
                            MoveTo(origin_x + x, 1 + y + h),
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
        let hint = if self.prefix {
            " ^B — q detach  c tab  v/- split  hjkl  z  n/p  drag divider"
        } else if !self.status.is_empty() {
            self.status.as_str()
        } else {
            " ^B q detach  c tab  v split  hjkl  z  click tile  drag divider"
        };
        let mut line = format!(" {hint}");
        clip_to(&mut line, cols as usize);
        while display_width(&line) < cols as usize {
            line.push(' ');
        }
        queue!(
            out,
            MoveTo(0, rows.saturating_sub(1)),
            SetBackgroundColor(TITLE_BG),
            SetForegroundColor(MUTED),
            Print(line),
            ResetColor
        )
    }

    fn place_cursor(&self, out: &mut io::Stdout) -> io::Result<()> {
        let Some(tile) = self.tiles.iter().find(|t| t.id == self.focused) else {
            queue!(out, Hide)?;
            return Ok(());
        };
        if tile.stream.is_none() || self.prefix {
            queue!(out, Hide)?;
            return Ok(());
        }
        let (cy, cx) = tile.parser.screen().cursor_position();
        let x = SIDEBAR + 1 + tile.x + cx;
        let y = 1 + tile.y + cy;
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

fn pane_size(cols: u16, rows: u16) -> (u16, u16) {
    let w = cols.saturating_sub(SIDEBAR + 1).max(8);
    let h = rows.saturating_sub(2).max(3);
    (w, h)
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
