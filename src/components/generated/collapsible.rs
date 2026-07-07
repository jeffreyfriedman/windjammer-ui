#![allow(clippy::all)]
#![allow(noop_method_call)]
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
        CollapsibleSection {
            title,
            content,
            open: false,
        }
    }
    #[inline]
    pub fn open(mut self, open: bool) -> CollapsibleSection {
        self.open = open;
        self
    }
}

impl Renderable for CollapsibleSection {
    #[inline]
    fn render(&mut self) -> String {
        let icon: String = {
            if self.open {
                String::from("▼")
            } else {
                String::from("▶")
            }
        };
        let content_style: String = {
            if self.open {
                String::from("display: block;")
            } else {
                String::from("display: none;")
            }
        };
        format!("<div class='wj-collapsible'>\n  <div class='wj-collapsible-header'>\n    <span class='wj-collapsible-icon'>{}</span>\n    <span>{}</span>\n  </div>\n  <div class='wj-collapsible-content' style='{}'>\n    {}\n  </div>\n</div>", icon, self.title, content_style, self.content)
    }
}
