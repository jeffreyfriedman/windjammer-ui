#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Toolbar {
    pub items: Vec<String>,
    pub position: String,
}

impl Toolbar {
#[inline]
pub fn new() -> Toolbar {
        Toolbar { items: Vec::new(), position: String::from("top") }
}
#[inline]
pub fn item(mut self, item: impl Into<String>) -> Toolbar {
        self.items.push(item.into());
        self
}
#[inline]
pub fn position(mut self, position: impl Into<String>) -> Toolbar {
        self.position = position.into();
        self
}
}

impl Renderable for Toolbar {
#[inline]
fn render(&self) -> String {
        let items_html = self.items.join("\n");
        format!("<div class='wj-toolbar wj-toolbar-{}'>\n  {}\n</div>", self.position, items_html)
}
}

