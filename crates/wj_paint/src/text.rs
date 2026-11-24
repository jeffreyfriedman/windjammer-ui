use std::sync::Arc;

use wj_scene::Color;

/// Font bytes shared between layouts and painters. `id` is unique per distinct font blob so painters can cache.
#[derive(Clone, Debug)]
pub struct FontData {
    pub id: u64,
    pub bytes: Arc<Vec<u8>>,
    pub index: u32,
}

impl PartialEq for FontData {
    fn eq(&self, o: &Self) -> bool {
        self.id == o.id && self.index == o.index
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph {
    pub id: u32,
    /// Offset from the run origin (pen position at the baseline).
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphRun {
    pub font: FontData,
    pub size: f32,
    /// Baseline origin in scene space.
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub glyphs: Vec<Glyph>,
}

/// A shaped, line-broken paragraph: runs positioned relative to the paragraph's top-left.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLayout {
    pub width: f32,
    pub height: f32,
    pub runs: Vec<GlyphRun>,
}

/// Source of text layouts (implemented over Parley by `wj-text`).
pub trait TextLayouter {
    fn layout(&mut self, text: &str, size: f32, max_width: Option<f32>) -> Arc<TextLayout>;
}

/// Layouter that produces no glyphs: for tests that only look at boxes.
#[derive(Default)]
pub struct NullTextLayouter;
impl TextLayouter for NullTextLayouter {
    fn layout(&mut self, _text: &str, _size: f32, _max_width: Option<f32>) -> Arc<TextLayout> {
        Arc::new(TextLayout { width: 0.0, height: 0.0, runs: vec![] })
    }
}
