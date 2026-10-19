use wj_scene::{Color, Rect};

use crate::text::GlyphRun;

/// One drawing command. All coordinates are absolute (scene space, pixels); clips and layers nest.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCmd {
    /// Intersect the clip with a rounded rect until the matching `PopClip`.
    PushClip { rect: Rect, radius: f32 },
    PopClip,
    /// Composite everything until `PopLayer` as a group at this opacity.
    PushLayer { opacity: f32 },
    PopLayer,
    FillRRect { rect: Rect, radius: f32, color: Color },
    /// Inside stroke of the given width (the stroke lies within `rect`).
    StrokeRRect { rect: Rect, radius: f32, width: f32, color: Color },
    Text { run: GlyphRun },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisplayList {
    pub cmds: Vec<DrawCmd>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, c: DrawCmd) {
        self.cmds.push(c);
    }
    pub fn len(&self) -> usize {
        self.cmds.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }
    /// Push/pop pairs balance (a malformed list is a painter-contract violation).
    pub fn is_balanced(&self) -> bool {
        let (mut clips, mut layers) = (0i32, 0i32);
        for c in &self.cmds {
            match c {
                DrawCmd::PushClip { .. } => clips += 1,
                DrawCmd::PopClip => clips -= 1,
                DrawCmd::PushLayer { .. } => layers += 1,
                DrawCmd::PopLayer => layers -= 1,
                _ => {}
            }
            if clips < 0 || layers < 0 {
                return false;
            }
        }
        clips == 0 && layers == 0
    }
}

/// RGBA8, straight (non-premultiplied) alpha, row-major, top-left origin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image { width, height, rgba: vec![0; (width * height * 4) as usize] }
    }
    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * self.width + x) * 4) as usize;
        [self.rgba[i], self.rgba[i + 1], self.rgba[i + 2], self.rgba[i + 3]]
    }
    /// Per-channel comparison. Returns (max channel difference, fraction of pixels whose largest
    /// channel difference exceeds `strict`).
    pub fn diff(&self, other: &Image, strict: u8) -> (u8, f64) {
        assert_eq!((self.width, self.height), (other.width, other.height), "image sizes differ");
        let mut max = 0u8;
        let mut over = 0usize;
        for (a, b) in self.rgba.chunks(4).zip(other.rgba.chunks(4)) {
            let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap();
            max = max.max(d);
            if d > strict {
                over += 1;
            }
        }
        (max, over as f64 / (self.width * self.height) as f64)
    }
}
