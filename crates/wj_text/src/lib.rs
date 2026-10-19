//! `wj-text` (layer L2): Parley shaping and line breaking, with an embedded default font so that
//! startup never waits on system font enumeration and output is identical on every machine.
//!
//! One [`TextEngine`] implements both seams: [`wj_layout::TextMeasure`] (sizes for layout) and
//! [`wj_paint::TextLayouter`] (glyph runs for painting). Layouts are cached per
//! `(text, size, width)`. Editing, IME and selection are not part of this crate yet (M4).
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use parley::fontique::{Blob, Collection, CollectionOptions};
use parley::{FontContext, FontFamily, Layout, LayoutContext, PositionedLayoutItem, StyleProperty};
use wj_layout::TextMeasure;
use wj_paint::{FontData, Glyph, GlyphRun, TextLayout, TextLayouter};
use wj_scene::Color;

/// Inter Regular, SIL Open Font License 1.1 (see `fonts/Inter-LICENSE.txt`).
pub const DEFAULT_FONT: &[u8] = include_bytes!("../fonts/Inter-Regular.ttf");
const FAMILY: &str = "Inter";

type Brush = [u8; 4];

pub struct TextEngine {
    fcx: FontContext,
    lcx: LayoutContext<Brush>,
    font_blob: Arc<Vec<u8>>,
    cache: HashMap<(String, u32, Option<u32>), Arc<TextLayout>>,
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEngine {
    /// Engine with only the embedded font (no system font scan).
    pub fn new() -> Self {
        let collection = Collection::new(CollectionOptions { shared: false, system_fonts: false });
        let mut fcx = FontContext { collection, source_cache: Default::default() };
        fcx.collection.register_fonts(Blob::new(Arc::new(DEFAULT_FONT.to_vec())), None);
        TextEngine { fcx, lcx: LayoutContext::new(), font_blob: Arc::new(DEFAULT_FONT.to_vec()), cache: HashMap::new() }
    }

    /// Engine that also sees system fonts (fallback for scripts the embedded font lacks).
    /// Discovery is lazy in Parley but the first layout that needs a fallback pays for it.
    pub fn with_system_fonts() -> Self {
        let mut e = Self::new();
        e.fcx = FontContext::new();
        e.fcx.collection.register_fonts(Blob::new(Arc::new(DEFAULT_FONT.to_vec())), None);
        e
    }

    fn build(&mut self, text: &str, size: f32, max_width: Option<f32>) -> Layout<Brush> {
        let mut b = self.lcx.ranged_builder(&mut self.fcx, text, 1.0, true);
        b.push_default(StyleProperty::FontSize(size));
        b.push_default(FontFamily::named(FAMILY));
        let mut l = b.build(text);
        l.break_all_lines(max_width);
        l
    }

    /// Number of cached layouts (for cache-behaviour tests).
    pub fn cached(&self) -> usize {
        self.cache.len()
    }

    fn shape(&mut self, text: &str, size: f32, max_width: Option<f32>) -> Arc<TextLayout> {
        let key = (text.to_string(), size.to_bits(), max_width.map(|w| w.round().max(0.0) as u32));
        if let Some(l) = self.cache.get(&key) {
            return l.clone();
        }
        let layout = self.build(text, size, max_width.map(|w| w.max(1.0)));
        let mut runs = vec![];
        for line in layout.lines() {
            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(run) = item {
                    let r = run.run();
                    let font = r.font();
                    let data = font.data.data();
                    let fd = FontData { id: font.data.id(), bytes: if data.len() == self.font_blob.len() { self.font_blob.clone() } else { Arc::new(data.to_vec()) }, index: font.index };
                    runs.push(GlyphRun {
                        font: fd,
                        size: r.font_size(),
                        x: 0.0,
                        y: run.baseline(),
                        color: Color::BLACK,
                        glyphs: run.positioned_glyphs().map(|g| Glyph { id: g.id as u32, x: g.x, y: g.y - run.baseline() }).collect(),
                    });
                    // Parley positions glyphs relative to the paragraph; keep x in the glyphs.
                }
            }
        }
        let out = Arc::new(TextLayout { width: layout.full_width(), height: layout.height(), runs });
        self.cache.insert(key, out.clone());
        out
    }
}

impl TextLayouter for TextEngine {
    fn layout(&mut self, text: &str, size: f32, max_width: Option<f32>) -> Arc<TextLayout> {
        self.shape(text, size, max_width)
    }
}

/// Interior-mutable handle so the immutable `TextMeasure` seam can share the engine's cache.
pub struct SharedText(pub RefCell<TextEngine>);

impl TextMeasure for SharedText {
    fn measure(&self, text: &str, size: f32, max_width: Option<f32>) -> (f32, f32) {
        let l = self.0.borrow_mut().shape(text, size, max_width);
        (l.width, l.height)
    }
}
