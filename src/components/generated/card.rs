#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Card {
    pub title: String,
    pub children: Vec<String>,
    pub padding: String,
    pub background_color: String,
    pub border_color: String,
}

impl Card {
#[inline]
pub fn new() -> Card {
        Card { title: String::new(), children: Vec::new(), padding: String::from("16px"), background_color: String::from("#fff"), border_color: String::from("#e0e0e0") }
}
#[inline]
pub fn title(mut self, title: impl Into<String>) -> Card {
        self.title = title.into();
        self
}
#[inline]
pub fn child(mut self, child: impl Into<String>) -> Card {
        self.children.push(child.into());
        self
}
#[inline]
pub fn children(mut self, children: Vec<String>) -> Card {
        self.children = children;
        self
}
#[inline]
pub fn padding(mut self, padding: impl Into<String>) -> Card {
        self.padding = padding.into();
        self
}
#[inline]
pub fn background_color(mut self, color: impl Into<String>) -> Card {
        self.background_color = color.into();
        self
}
#[inline]
pub fn border_color(mut self, color: impl Into<String>) -> Card {
        self.border_color = color.into();
        self
}
}

impl Renderable for Card {

fn render(&self) -> String {
        let style = format!("padding: {}; background-color: {}; border: 1px solid {}; border-radius: 8px;", self.padding, self.background_color, self.border_color);
        let title_html = {
            if self.title != "" {
                format!("<div class='wj-card-title' style='font-weight: bold; margin-bottom: 12px; font-size: 1.25rem;'>{}</div>", self.title)
            } else {
                String::new()
            }
        };
        let children_html = self.children.join("\n");
        format!("<div class='wj-card' style='{}'>\n{}{}\n</div>", style, title_html, children_html)
}
}

