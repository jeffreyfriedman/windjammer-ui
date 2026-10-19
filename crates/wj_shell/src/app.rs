use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use wj_a11y::AccessKitTree;
use wj_layout::{LayoutEngine, TextMeasure};
use wj_paint::{build_display_list, TextLayout, TextLayouter};
use wj_reactive::{state, Signal};
use wj_scene::{Dirty, Scene};
use wj_text::TextEngine;

use crate::clock::Clock;
use crate::shell::{Shell, ShellEvent, Theme};

/// Window facts as signals, so views can react to size, scale and theme.
#[derive(Clone, Copy)]
pub struct WindowInfo {
    pub width: Signal<f32>,
    pub height: Signal<f32>,
    pub scale: Signal<f32>,
    pub dark: Signal<bool>,
}

/// What one frame did. Tests and the perf gates read this.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameReport {
    /// Nothing changed; no layout, paint or accessibility work was done.
    pub skipped: bool,
    pub events: usize,
    pub dirty_entries: usize,
    pub rects_changed: usize,
    pub display_list_len: usize,
    pub a11y_nodes_sent: usize,
    pub close_requested: bool,
    /// Wall-clock per stage (ms) from the supplied clock.
    pub layout_ms: f64,
    pub paint_build_ms: f64,
    pub present_ms: f64,
}

struct Text(Rc<RefCell<TextEngine>>);
impl TextMeasure for Text {
    fn measure(&self, t: &str, size: f32, mw: Option<f32>) -> (f32, f32) {
        let l = self.0.borrow_mut().layout(t, size, mw);
        (l.width, l.height)
    }
}
impl TextLayouter for Text {
    fn layout(&mut self, t: &str, size: f32, mw: Option<f32>) -> Arc<TextLayout> {
        self.0.borrow_mut().layout(t, size, mw)
    }
}

pub struct App {
    pub scene: Scene,
    pub window: WindowInfo,
    layout: LayoutEngine,
    text: Rc<RefCell<TextEngine>>,
    a11y: AccessKitTree,
    first: bool,
    resized: bool,
}

impl App {
    /// A new app with an empty scene on the current thread's reactive runtime.
    pub fn new() -> Self {
        let window = WindowInfo { width: state(0.0), height: state(0.0), scale: state(1.0), dark: state(false) };
        App { scene: Scene::new(), window, layout: LayoutEngine::new(), text: Rc::new(RefCell::new(TextEngine::new())), a11y: AccessKitTree::new(), first: true, resized: false }
    }

    /// Run one frame: deliver events, lay out, paint, forward accessibility changes.
    pub fn frame(&mut self, shell: &mut dyn Shell, clock: &dyn Clock) -> FrameReport {
        let mut rep = FrameReport::default();
        let events = shell.poll_events();
        rep.events = events.len();
        for e in events {
            match e {
                ShellEvent::Input(ev) => {
                    self.scene.dispatch(ev);
                }
                ShellEvent::Resize { width, height, scale } => {
                    wj_reactive::batch(|| {
                        self.window.width.set(width as f32 / scale);
                        self.window.height.set(height as f32 / scale);
                        self.window.scale.set(scale);
                    });
                    self.resized = true;
                }
                ShellEvent::Theme(t) => self.window.dark.set(t == Theme::Dark),
                ShellEvent::CloseRequested => rep.close_requested = true,
            }
        }
        let (pw, ph) = shell.size();
        let scale = shell.scale();
        let (lw, lh) = (pw as f32 / scale, ph as f32 / scale);
        if self.first {
            wj_reactive::batch(|| {
                self.window.width.set(lw);
                self.window.height.set(lh);
                self.window.scale.set(scale);
            });
        }

        let mut entries = self.scene.drain_dirty();
        if entries.is_empty() && !self.first && !self.resized {
            rep.skipped = true;
            return rep;
        }
        let t0 = clock.now_ms();
        let measure = Text(self.text.clone());
        rep.rects_changed = self.layout.update(self.scene, &entries, (lw, lh), &measure);
        rep.layout_ms = clock.now_ms() - t0;
        entries.extend(self.scene.drain_dirty()); // rect changes raised paint/semantics flags
        rep.dirty_entries = entries.len();

        let t1 = clock.now_ms();
        let mut layouter = Text(self.text.clone());
        let (list, _stats) = build_display_list(self.scene, (lw, lh), &mut layouter);
        rep.display_list_len = list.len();
        rep.paint_build_ms = clock.now_ms() - t1;
        let t2 = clock.now_ms();
        shell.present(&list, pw, ph);
        rep.present_ms = clock.now_ms() - t2;

        if self.first || entries.iter().any(|e| e.flags.contains(Dirty::SEMANTICS)) {
            let tree = self.scene.semantic_tree();
            if let Some(u) = self.a11y.update(&tree, self.scene.focus()) {
                rep.a11y_nodes_sent = u.nodes.len();
                shell.a11y_update(u);
            }
        }
        self.first = false;
        self.resized = false;
        rep
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
