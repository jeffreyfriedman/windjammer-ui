#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Grid {
    pub children: Vec<String>,
    pub columns: i32,
    pub gap: String,
    pub padding: String,
}

impl Grid {
#[inline]
pub fn new(columns: i32) -> Grid {
        Grid { children: Vec::new(), columns, gap: String::from("16px"), padding: String::from("0") }
}
#[inline]
pub fn child(mut self, child: impl Into<String>) -> Grid {
        self.children.push(child.into());
        self
}
#[inline]
pub fn children(mut self, children: Vec<String>) -> Grid {
        self.children = children;
        self
}
#[inline]
pub fn gap(mut self, gap: impl Into<String>) -> Grid {
        self.gap = gap.into();
        self
}
#[inline]
pub fn padding(mut self, padding: impl Into<String>) -> Grid {
        self.padding = padding.into();
        self
}
}

impl Renderable for Grid {
#[inline]
fn render(&self) -> String {
        let style = format!("display: grid; grid-template-columns: repeat({}, 1fr); gap: {}; padding: {};", self.columns, self.gap, self.padding);
        let children_html = self.children.join("\n");
        format!("<div class='wj-grid' style='{}'>\n{}\n</div>", style, children_html)
}
}

