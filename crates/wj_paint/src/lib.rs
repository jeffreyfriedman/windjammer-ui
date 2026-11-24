//! `wj-paint` (layer L3, backend-neutral half): the [`DisplayList`] is the contract between the
//! scene and every renderer. Backends (`wj-paint-cpu`, `wj-paint-vello`) implement [`Painter`];
//! tests compare display lists (not pixels) across targets, then pixels as a second check.

mod build;
mod display_list;
mod fake;
mod text;

pub use build::{build_display_list, BuildStats};
pub use display_list::{DisplayList, DrawCmd, Image};
pub use fake::FakePainter;
pub use text::{FontData, Glyph, GlyphRun, TextLayout, TextLayouter, NullTextLayouter};

use wj_scene::Color;

/// A renderer. `render` draws the whole list onto a `width` x `height` target cleared to `clear`
/// and returns the pixels (RGBA8, straight alpha), which is what golden tests compare.
pub trait Painter {
    fn render(&mut self, list: &DisplayList, width: u32, height: u32, clear: Color) -> Image;
}
