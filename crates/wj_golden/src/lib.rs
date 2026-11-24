//! The golden scene corpus: 240 deterministic scenes, generated from seeds so the corpus itself
//! needs no maintenance. Part one is raw display lists (geometry, clipping, layers, strokes); part
//! two builds real scenes through the whole stack (tree, layout, text, paint).
use std::cell::RefCell;

use wj_layout::LayoutEngine;
use wj_paint::{build_display_list, DisplayList, DrawCmd};
use wj_scene::{Align, Color, Dim, Direction, Edges, Justify, Paint, Rect, Scene, Style, TextProps};
use wj_text::TextEngine;

pub struct GoldenScene {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub clear: Color,
    pub list: DisplayList,
    /// True when the scene has no text (so an unrelated rasterizer can draw it too).
    pub vector_only: bool,
}

pub struct Rng(pub u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1)
    }
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    pub fn f(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (self.next() % 10_000) as f32 / 10_000.0 * (hi - lo)
    }
    pub fn color(&mut self, alpha: bool) -> Color {
        Color::rgba(self.below(256) as u8, self.below(256) as u8, self.below(256) as u8, if alpha && self.below(3) == 0 { 60 + self.below(180) as u8 } else { 255 })
    }
}

const W: u32 = 96;
const H: u32 = 64;

fn vector_scene(seed: u64) -> GoldenScene {
    let mut r = Rng::new(seed);
    let mut list = DisplayList::new();
    let mut stack: Vec<bool> = vec![]; // true = clip, false = layer
    let n = 4 + r.below(10);
    for _ in 0..n {
        match r.below(10) {
            0 | 1 if stack.len() < 3 => {
                let rect = Rect::new(r.f(0.0, 30.0), r.f(0.0, 20.0), r.f(20.0, 70.0), r.f(16.0, 50.0));
                list.push(DrawCmd::PushClip { rect, radius: if r.below(2) == 0 { 0.0 } else { r.f(1.0, 14.0) } });
                stack.push(true);
            }
            2 if stack.len() < 3 => {
                list.push(DrawCmd::PushLayer { opacity: r.f(0.2, 0.9) });
                stack.push(false);
            }
            3 if !stack.is_empty() => {
                list.push(if stack.pop().unwrap() { DrawCmd::PopClip } else { DrawCmd::PopLayer });
            }
            4 | 5 => {
                let rect = Rect::new(r.f(0.0, 70.0), r.f(0.0, 44.0), r.f(6.0, 40.0), r.f(6.0, 30.0));
                list.push(DrawCmd::StrokeRRect { rect, radius: r.f(0.0, 10.0), width: r.f(1.0, 4.0), color: r.color(true) });
            }
            _ => {
                let aligned = r.below(3) == 0;
                let mut rect = Rect::new(r.f(0.0, 80.0), r.f(0.0, 50.0), r.f(4.0, 60.0), r.f(4.0, 40.0));
                if aligned {
                    rect = Rect::new(rect.x.round(), rect.y.round(), rect.w.round(), rect.h.round());
                }
                let radius = if aligned || r.below(2) == 0 { 0.0 } else { r.f(1.0, 20.0) };
                list.push(DrawCmd::FillRRect { rect, radius, color: r.color(true) });
            }
        }
    }
    while let Some(c) = stack.pop() {
        list.push(if c { DrawCmd::PopClip } else { DrawCmd::PopLayer });
    }
    let clear = if r.below(2) == 0 { Color::WHITE } else { Color::rgb(24, 26, 32) };
    GoldenScene { name: format!("vector-{seed:03}"), width: W, height: H, clear, list, vector_only: true }
}

fn build_ui(seed: u64, text: &RefCell<TextEngine>) -> GoldenScene {
    let mut r = Rng::new(seed ^ 0xABCDEF);
    wj_reactive::reset();
    let s = Scene::new();
    let (w, h) = (128u32, 96u32);
    s.set_style(s.root(), Style { direction: if r.below(2) == 0 { Direction::Column } else { Direction::Row }, padding: Edges::all(r.f(2.0, 8.0)), gap: r.f(2.0, 8.0), ..Default::default() });
    let labels = ["Save", "Open file", "Hello, world", "Windjammer", "Lorem ipsum dolor", "OK", "42"];
    let mut with_text = false;
    for _ in 0..(2 + r.below(4)) {
        let card = s.create(s.root());
        s.set_style(card, Style { direction: if r.below(2) == 0 { Direction::Row } else { Direction::Column }, padding: Edges::all(r.f(2.0, 8.0)), gap: r.f(1.0, 5.0), flex_grow: r.below(3) as f32, align_items: [Align::Start, Align::Center, Align::Stretch][r.below(3)], justify: [Justify::Start, Justify::Center, Justify::SpaceBetween][r.below(3)], ..Default::default() });
        s.set_paint(card, Paint { background: Some(r.color(false)), corner_radius: r.f(0.0, 12.0), border_width: if r.below(3) == 0 { 1.5 } else { 0.0 }, border_color: r.color(false), opacity: if r.below(5) == 0 { 0.6 } else { 1.0 }, clip: r.below(2) == 0 });
        for _ in 0..(1 + r.below(3)) {
            let leaf = s.create(card);
            if r.below(3) == 0 {
                s.set_style(leaf, Style { width: Dim::Px(r.f(8.0, 30.0)), height: Dim::Px(r.f(8.0, 20.0)), ..Default::default() });
                s.set_paint(leaf, Paint { background: Some(r.color(true)), corner_radius: r.f(0.0, 8.0), ..Default::default() });
            } else {
                let mut tp = TextProps::new(labels[r.below(labels.len())]);
                tp.font_size = [10.0, 12.0, 14.0, 16.0][r.below(4)];
                tp.color = r.color(false);
                s.set_text(leaf, tp);
                with_text = true;
            }
        }
    }
    let mut engine = LayoutEngine::new();
    let d = s.drain_dirty();
    // Share the caller's engine for both measure and paint so layouts agree.
    struct Shared<'a>(&'a RefCell<TextEngine>);
    impl wj_layout::TextMeasure for Shared<'_> {
        fn measure(&self, t: &str, size: f32, mw: Option<f32>) -> (f32, f32) {
            use wj_paint::TextLayouter;
            let l = self.0.borrow_mut().layout(t, size, mw);
            (l.width, l.height)
        }
    }
    engine.update(s, &d, (w as f32, h as f32), &Shared(text));
    struct Lay<'a>(&'a RefCell<TextEngine>);
    impl wj_paint::TextLayouter for Lay<'_> {
        fn layout(&mut self, t: &str, size: f32, mw: Option<f32>) -> std::sync::Arc<wj_paint::TextLayout> {
            self.0.borrow_mut().layout(t, size, mw)
        }
    }
    let (list, _) = build_display_list(s, (w as f32, h as f32), &mut Lay(text));
    GoldenScene { name: format!("ui-{seed:03}"), width: w, height: h, clear: Color::WHITE, list, vector_only: !with_text }
}

/// The full corpus (`>= 200` scenes).
pub fn corpus() -> Vec<GoldenScene> {
    let text = RefCell::new(TextEngine::new());
    let mut v: Vec<GoldenScene> = (0..160).map(vector_scene).collect();
    v.extend((0..80).map(|i| build_ui(i, &text)));
    v
}
