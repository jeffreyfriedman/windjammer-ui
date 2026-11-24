use crate::geom::Color;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Dim {
    #[default]
    Auto,
    Px(f32),
    Pct(f32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Column,
    Row,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    Start,
    Center,
    End,
    #[default]
    Stretch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Edges {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Edges {
    pub fn all(v: f32) -> Self {
        Edges { left: v, top: v, right: v, bottom: v }
    }
}

/// Layout props. Flexbox subset, plus absolute positioning (used by virtual lists) and scroll containers.
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    pub direction: Direction,
    pub width: Dim,
    pub height: Dim,
    pub padding: Edges,
    pub margin: Edges,
    pub gap: f32,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub align_items: Align,
    pub justify: Justify,
    pub wrap: bool,
    /// `Some((left, top))`: positioned relative to the parent, out of flow.
    pub absolute: Option<(f32, f32)>,
    /// Content may exceed the box; the box clips and scrolls.
    pub scroll: bool,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            direction: Direction::Column,
            width: Dim::Auto,
            height: Dim::Auto,
            padding: Edges::default(),
            margin: Edges::default(),
            gap: 0.0,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            align_items: Align::Stretch,
            justify: Justify::Start,
            wrap: false,
            absolute: None,
            scroll: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Paint {
    pub background: Option<Color>,
    pub corner_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub opacity: f32,
    pub clip: bool,
}

impl Default for Paint {
    fn default() -> Self {
        Paint { background: None, corner_radius: 0.0, border_width: 0.0, border_color: Color::BLACK, opacity: 1.0, clip: false }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextProps {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
}

impl TextProps {
    pub fn new(content: impl Into<String>) -> Self {
        TextProps { content: content.into(), font_size: 14.0, color: Color::BLACK }
    }
}
