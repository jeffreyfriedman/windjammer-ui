#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrollArea {
    pub children: Vec<String>,
    pub direction: ScrollDirection,
    pub height: String,
    pub width: String,
}

impl ScrollArea {
#[inline]
pub fn new() -> ScrollArea {
        ScrollArea { children: Vec::new(), direction: ScrollDirection::Vertical, height: String::from("300px"), width: String::from("100%") }
}
#[inline]
pub fn child(mut self, child: impl Into<String>) -> ScrollArea {
        self.children.push(child.into());
        self
}
#[inline]
pub fn direction(mut self, direction: ScrollDirection) -> ScrollArea {
        self.direction = direction;
        self
}
#[inline]
pub fn height(mut self, height: impl Into<String>) -> ScrollArea {
        self.height = height.into();
        self
}
#[inline]
pub fn width(mut self, width: impl Into<String>) -> ScrollArea {
        self.width = width.into();
        self
}
}

impl Renderable for ScrollArea {
#[inline]
fn render(&self) -> String {
        let overflow_style: String = match &self.direction {
            ScrollDirection::Vertical => String::from("overflow-y: auto; overflow-x: hidden;"),
            ScrollDirection::Horizontal => String::from("overflow-x: auto; overflow-y: hidden;"),
            ScrollDirection::Both => String::from("overflow: auto;"),
        };
        let children_html = self.children.join("\n");
        format!("<div class='wj-scroll-area' style='height: {}; width: {}; {}'>\n  {}\n</div>", self.height, self.width, overflow_style, children_html)
}
}

