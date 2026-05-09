#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct CollapsibleSection {
    pub title: String,
    pub content: String,
    pub open: bool,
}

impl CollapsibleSection {
#[inline]
pub fn new(title: String, content: String) -> CollapsibleSection {
        CollapsibleSection { title: title.to_string(), content: content.to_string(), open: false }
}
#[inline]
pub fn open(mut self, open: bool) -> CollapsibleSection {
        self.open = open;
        self
}
}

impl Renderable for CollapsibleSection {
#[inline]
fn render(&self) -> String {
        let icon = {
            if self.open {
                "▼".to_string()
            } else {
                "▶".to_string()
            }
        };
        let content_style = {
            if self.open {
                "display: block;".to_string()
            } else {
                "display: none;".to_string()
            }
        };
        format!("<div class='wj-collapsible'>\n  <div class='wj-collapsible-header'>\n    <span class='wj-collapsible-icon'>{}</span>\n    <span>{}</span>\n  </div>\n  <div class='wj-collapsible-content' style='{}'>\n    {}\n  </div>\n</div>", icon, self.title.clone(), content_style, self.content.clone())
}
}

