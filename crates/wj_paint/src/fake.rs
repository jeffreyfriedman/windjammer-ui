use wj_scene::Color;

use crate::display_list::{DisplayList, Image};
use crate::Painter;

/// Records every list it is asked to render and returns a blank image. The scene-level test double.
#[derive(Default)]
pub struct FakePainter {
    pub frames: Vec<DisplayList>,
}

impl Painter for FakePainter {
    fn render(&mut self, list: &DisplayList, width: u32, height: u32, clear: Color) -> Image {
        self.frames.push(list.clone());
        let mut img = Image::new(width, height);
        for px in img.rgba.chunks_mut(4) {
            px.copy_from_slice(&[clear.r, clear.g, clear.b, clear.a]);
        }
        img
    }
}
