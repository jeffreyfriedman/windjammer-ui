#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Dialog {
    pub title: String,
    pub content: String,
    pub open: bool,
    pub width: String,
}

impl Dialog {
#[inline]
pub fn new(title: String, content: String) -> Dialog {
        Dialog { title, content, open: false, width: "500px".to_string() }
}
#[inline]
pub fn open(mut self, open: bool) -> Dialog {
        self.open = open;
        self
}
#[inline]
pub fn width(mut self, width: String) -> Dialog {
        self.width = width;
        self
}
}

impl Renderable for Dialog {
#[inline]
fn render(self) -> String {
        let display_style: String = {
            if self.open {
                String::from("display: flex;")
            } else {
                String::from("display: none;")
            }
        };
        format!("<div class='wj-dialog-overlay' style='{}'>\n  <div class='wj-dialog' style='max-width: {}; width: 90%;'>\n    <div class='wj-dialog-header'>\n      <h2>{}</h2>\n      <button class='wj-dialog-close'>×</button>\n    </div>\n    <div class='wj-dialog-content'>\n      {}\n    </div>\n  </div>\n</div>", display_style, self.width, self.title, self.content)
}
}

