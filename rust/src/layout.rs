//! Tab geometry. Textbook BSP — not a Herdr copy.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitDir {
    Right,
    Down,
}

impl SplitDir {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "right" => Some(Self::Right),
            "down" => Some(Self::Down),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Layout {
    Leaf {
        pane: String,
    },
    Split {
        dir: SplitDir,
        ratio: f32,
        a: Box<Layout>,
        b: Box<Layout>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub id: String,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub const MIN_RATIO: f32 = 0.05;
pub const MAX_RATIO: f32 = 0.95;

pub fn clamp_ratio(ratio: f32) -> f32 {
    if !ratio.is_finite() {
        return 0.5;
    }
    ratio.clamp(MIN_RATIO, MAX_RATIO)
}

pub fn synthesize(ids: &[String]) -> Layout {
    match ids {
        [] => Layout::Leaf {
            pane: String::new(),
        },
        [id] => Layout::Leaf { pane: id.clone() },
        _ => {
            let mut layout = Layout::Leaf {
                pane: ids[0].clone(),
            };
            for id in &ids[1..] {
                layout = Layout::Split {
                    dir: SplitDir::Right,
                    ratio: 0.5,
                    a: Box::new(layout),
                    b: Box::new(Layout::Leaf { pane: id.clone() }),
                };
            }
            layout
        }
    }
}

pub fn leaves(layout: &Layout) -> Vec<String> {
    let mut out = Vec::new();
    walk_leaves(layout, &mut out);
    out
}

fn walk_leaves(layout: &Layout, out: &mut Vec<String>) {
    match layout {
        Layout::Leaf { pane } => {
            if !pane.is_empty() {
                out.push(pane.clone());
            }
        }
        Layout::Split { a, b, .. } => {
            walk_leaves(a, out);
            walk_leaves(b, out);
        }
    }
}

pub fn ensure_layout(layout: &Layout, pane_ids: &[String]) -> Layout {
    let mut have = leaves(layout);
    have.sort();
    let mut want = pane_ids.to_vec();
    want.sort();
    if have == want && !pane_ids.is_empty() {
        layout.clone()
    } else {
        synthesize(pane_ids)
    }
}

pub fn split_leaf(layout: &mut Layout, pane: &str, dir: SplitDir, new_id: &str) -> bool {
    match layout {
        Layout::Leaf { pane: id } if id == pane => {
            *layout = Layout::Split {
                dir,
                ratio: 0.5,
                a: Box::new(Layout::Leaf {
                    pane: pane.to_string(),
                }),
                b: Box::new(Layout::Leaf {
                    pane: new_id.to_string(),
                }),
            };
            true
        }
        Layout::Split { a, b, .. } => {
            split_leaf(a, pane, dir, new_id) || split_leaf(b, pane, dir, new_id)
        }
        _ => false,
    }
}

pub fn set_ratio(layout: &mut Layout, a_id: &str, b_id: &str, ratio: f32) -> bool {
    set_ratio_inner(layout, a_id, b_id, clamp_ratio(ratio))
}

fn set_ratio_inner(layout: &mut Layout, a_id: &str, b_id: &str, ratio: f32) -> bool {
    match layout {
        Layout::Leaf { .. } => false,
        Layout::Split {
            ratio: slot, a, b, ..
        } => {
            let in_a_first = contains(a, a_id) && contains(b, b_id);
            let in_b_first = contains(a, b_id) && contains(b, a_id);
            if in_a_first {
                *slot = ratio;
                return true;
            }
            if in_b_first {
                *slot = clamp_ratio(1.0 - ratio);
                return true;
            }
            set_ratio_inner(a, a_id, b_id, ratio) || set_ratio_inner(b, a_id, b_id, ratio)
        }
    }
}

fn contains(layout: &Layout, id: &str) -> bool {
    match layout {
        Layout::Leaf { pane } => pane == id,
        Layout::Split { a, b, .. } => contains(a, id) || contains(b, id),
    }
}

pub fn tiles(layout: &Layout, x: u16, y: u16, w: u16, h: u16) -> Vec<Cell> {
    let mut out = Vec::new();
    tiles_inner(layout, x, y, w, h, &mut out);
    out
}

fn tiles_inner(layout: &Layout, x: u16, y: u16, w: u16, h: u16, out: &mut Vec<Cell>) {
    match layout {
        Layout::Leaf { pane } => {
            if !pane.is_empty() && w > 0 && h > 0 {
                out.push(Cell {
                    id: pane.clone(),
                    x,
                    y,
                    w,
                    h,
                });
            }
        }
        Layout::Split { dir, ratio, a, b } => match dir {
            SplitDir::Right => {
                if w < 2 {
                    tiles_inner(a, x, y, w, h, out);
                    return;
                }
                let aw = split_span(w, *ratio);
                tiles_inner(a, x, y, aw, h, out);
                tiles_inner(b, x.saturating_add(aw), y, w.saturating_sub(aw), h, out);
            }
            SplitDir::Down => {
                if h < 2 {
                    tiles_inner(a, x, y, w, h, out);
                    return;
                }
                let ah = split_span(h, *ratio);
                tiles_inner(a, x, y, w, ah, out);
                tiles_inner(b, x, y.saturating_add(ah), w, h.saturating_sub(ah), out);
            }
        },
    }
}

fn split_span(span: u16, ratio: f32) -> u16 {
    let raw = (ratio * span as f32).round() as i32;
    raw.clamp(1, i32::from(span) - 1) as u16
}

pub fn cell_at(cells: &[Cell], x: u16, y: u16) -> Option<&Cell> {
    cells.iter().find(|c| {
        x >= c.x && y >= c.y && x < c.x.saturating_add(c.w) && y < c.y.saturating_add(c.h)
    })
}

/// Shared exclusive edge: last col/row of A abuts B.
pub fn divider_at(cells: &[Cell], x: u16, y: u16) -> Option<(String, String, SplitDir)> {
    for i in 0..cells.len() {
        for j in 0..cells.len() {
            if i == j {
                continue;
            }
            let a = &cells[i];
            let b = &cells[j];
            if a.x.saturating_add(a.w) == b.x && overlap(a.y, a.h, b.y, b.h) {
                let edge_x = b.x.saturating_sub(1);
                if x == edge_x
                    && y >= a.y.max(b.y)
                    && y < a.y.saturating_add(a.h).min(b.y.saturating_add(b.h))
                {
                    return Some((a.id.clone(), b.id.clone(), SplitDir::Right));
                }
            }
            if a.y.saturating_add(a.h) == b.y && overlap(a.x, a.w, b.x, b.w) {
                let edge_y = b.y.saturating_sub(1);
                if y == edge_y
                    && x >= a.x.max(b.x)
                    && x < a.x.saturating_add(a.w).min(b.x.saturating_add(b.w))
                {
                    return Some((a.id.clone(), b.id.clone(), SplitDir::Down));
                }
            }
        }
    }
    None
}

fn overlap(a0: u16, asz: u16, b0: u16, bsz: u16) -> bool {
    let a1 = a0.saturating_add(asz);
    let b1 = b0.saturating_add(bsz);
    a0 < b1 && b0 < a1
}

pub fn neighbor(cells: &[Cell], from: &str, step: SplitDir) -> Option<String> {
    let src = cells.iter().find(|c| c.id == from)?;
    let mut best: Option<(&Cell, u32)> = None;
    for c in cells {
        if c.id == from {
            continue;
        }
        let ok = match step {
            SplitDir::Right => {
                c.x >= src.x.saturating_add(src.w) && overlap(src.y, src.h, c.y, c.h)
            }
            SplitDir::Down => c.y >= src.y.saturating_add(src.h) && overlap(src.x, src.w, c.x, c.w),
        };
        // left / up encoded by flipping: caller maps left→search cells where src is to the right
        if !ok {
            continue;
        }
        let dist = match step {
            SplitDir::Right => u32::from(c.x.saturating_sub(src.x.saturating_add(src.w))),
            SplitDir::Down => u32::from(c.y.saturating_sub(src.y.saturating_add(src.h))),
        };
        if best.is_none_or(|(_, d)| dist < d) {
            best = Some((c, dist));
        }
    }
    best.map(|(c, _)| c.id.clone())
}

pub fn neighbor_step(cells: &[Cell], from: &str, step: &str) -> Option<String> {
    match step {
        "right" => neighbor(cells, from, SplitDir::Right),
        "down" => neighbor(cells, from, SplitDir::Down),
        "left" => cells.iter().find(|c| c.id == from).and_then(|src| {
            let mut best: Option<(&Cell, u32)> = None;
            for c in cells {
                if c.id == from {
                    continue;
                }
                if c.x.saturating_add(c.w) <= src.x && overlap(src.y, src.h, c.y, c.h) {
                    let dist = u32::from(src.x.saturating_sub(c.x.saturating_add(c.w)));
                    if best.is_none_or(|(_, d)| dist < d) {
                        best = Some((c, dist));
                    }
                }
            }
            best.map(|(c, _)| c.id.clone())
        }),
        "up" => cells.iter().find(|c| c.id == from).and_then(|src| {
            let mut best: Option<(&Cell, u32)> = None;
            for c in cells {
                if c.id == from {
                    continue;
                }
                if c.y.saturating_add(c.h) <= src.y && overlap(src.x, src.w, c.x, c.w) {
                    let dist = u32::from(src.y.saturating_sub(c.y.saturating_add(c.h)));
                    if best.is_none_or(|(_, d)| dist < d) {
                        best = Some((c, dist));
                    }
                }
            }
            best.map(|(c, _)| c.id.clone())
        }),
        _ => None,
    }
}

/// Painted rect: steal last col of A on a vertical join, last row on a horizontal join.
pub fn inset(cells: &[Cell], id: &str) -> Option<(u16, u16, u16, u16)> {
    let c = cells.iter().find(|c| c.id == id)?;
    let x = c.x;
    let y = c.y;
    let mut w = c.w;
    let mut h = c.h;
    let steal_w = cells
        .iter()
        .any(|o| o.id != c.id && c.x.saturating_add(c.w) == o.x && overlap(c.y, c.h, o.y, o.h));
    let steal_h = cells
        .iter()
        .any(|o| o.id != c.id && c.y.saturating_add(c.h) == o.y && overlap(c.x, c.w, o.x, o.w));
    if steal_w && w > 1 {
        w -= 1;
    }
    if steal_h && h > 1 {
        h -= 1;
    }
    let _ = (x, y);
    Some((x, y, w, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(n: &[&str]) -> Vec<String> {
        n.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn synthesize_one_is_leaf() {
        let l = synthesize(&ids(&["w1:p1"]));
        assert_eq!(leaves(&l), vec!["w1:p1".to_string()]);
    }

    #[test]
    fn split_leaf_right_two_tiles_abut() {
        let mut l = synthesize(&ids(&["w1:p1"]));
        assert!(split_leaf(&mut l, "w1:p1", SplitDir::Right, "w1:p2"));
        let cells = tiles(&l, 0, 0, 80, 22);
        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0].x + cells[0].w, cells[1].x);
        assert_eq!(cells[0].w + cells[1].w, 80);
        assert_eq!(cells[0].y, 0);
        assert_eq!(cells[1].h, 22);
        assert!(cell_at(&cells, 0, 0).unwrap().id == "w1:p1");
        assert!(cell_at(&cells, 40, 0).unwrap().id == "w1:p2");
    }

    #[test]
    fn split_down_on_left_is_2x2_or_l() {
        let mut l = synthesize(&ids(&["w1:p1"]));
        split_leaf(&mut l, "w1:p1", SplitDir::Right, "w1:p2");
        split_leaf(&mut l, "w1:p1", SplitDir::Down, "w1:p3");
        let cells = tiles(&l, 0, 0, 80, 22);
        assert_eq!(cells.len(), 3);
        let sum: u32 = cells.iter().map(|c| u32::from(c.w) * u32::from(c.h)).sum();
        assert_eq!(sum, 80 * 22);
        let left = neighbor_step(&cells, "w1:p2", "left");
        assert!(left.as_deref() == Some("w1:p1") || left.as_deref() == Some("w1:p3"));
        assert_eq!(neighbor_step(&cells, "w1:p2", "right"), None);
    }

    #[test]
    fn divider_moves_after_set_ratio() {
        let mut l = synthesize(&ids(&["w1:p1"]));
        split_leaf(&mut l, "w1:p1", SplitDir::Right, "w1:p2");
        let before = tiles(&l, 0, 0, 80, 10);
        assert!(set_ratio(&mut l, "w1:p1", "w1:p2", 0.25));
        let after = tiles(&l, 0, 0, 80, 10);
        assert!(after[0].w < before[0].w);
        assert_eq!(after[0].w + after[1].w, 80);
        let edge = after[0].w.saturating_sub(1);
        let hit = divider_at(&after, edge, 1);
        assert!(hit.is_some());
    }

    #[test]
    fn clamp_and_ensure() {
        assert_eq!(clamp_ratio(0.0), MIN_RATIO);
        assert_eq!(clamp_ratio(2.0), MAX_RATIO);
        let l = synthesize(&ids(&["w1:p1"]));
        let fixed = ensure_layout(&l, &ids(&["w1:p1", "w1:p2"]));
        assert_eq!(leaves(&fixed).len(), 2);
    }

    #[test]
    fn inset_steals_trailing_col() {
        let mut l = synthesize(&ids(&["w1:p1"]));
        split_leaf(&mut l, "w1:p1", SplitDir::Right, "w1:p2");
        let cells = tiles(&l, 0, 0, 80, 10);
        let (_x, _y, w, h) = inset(&cells, "w1:p1").unwrap();
        assert_eq!(w, cells[0].w - 1);
        assert_eq!(h, 10);
        let r = inset(&cells, "w1:p2").unwrap();
        assert_eq!(r.2, cells[1].w);
    }

    #[test]
    fn neighbor_no_wrap() {
        let mut l = synthesize(&ids(&["w1:p1"]));
        split_leaf(&mut l, "w1:p1", SplitDir::Right, "w1:p2");
        let cells = tiles(&l, 0, 0, 40, 10);
        assert_eq!(
            neighbor_step(&cells, "w1:p1", "right").as_deref(),
            Some("w1:p2")
        );
        assert_eq!(neighbor_step(&cells, "w1:p1", "left"), None);
        assert_eq!(
            neighbor_step(&cells, "w1:p2", "left").as_deref(),
            Some("w1:p1")
        );
    }
}
