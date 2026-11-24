#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Scroll {
    pub children: Vec<String>,
    pub direction: ScrollDir,
    pub height: String,
    pub width: String,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ScrollDir {
    Vertical,
    Horizontal,
    Both,
    None,
}

impl Scroll {
#[inline]
pub fn new() -> Scroll {
        Scroll { children: Vec::new(), direction: ScrollDir::Vertical, height: String::from("400px"), width: String::from("100%"), class: String::new() }
}
#[inline]
pub fn child(mut self, child: String) -> Scroll {
        self.children.push(child);
        self
}
#[inline]
pub fn direction(mut self, direction: ScrollDir) -> Scroll {
        self.direction = direction;
        self
}
#[inline]
pub fn height(mut self, height: &str) -> Scroll {
        self.height = height.to_string();
        self
}
#[inline]
pub fn width(mut self, width: &str) -> Scroll {
        self.width = width.to_string();
        self
}
#[inline]
pub fn class(mut self, class: &str) -> Scroll {
        self.class = class.to_string();
        self
}

pub fn render(&self) -> String {
        let overflow: String = match &self.direction {
            ScrollDir::Vertical => String::from("overflow-x: hidden; overflow-y: auto"),
            ScrollDir::Horizontal => String::from("overflow-x: auto; overflow-y: hidden"),
            ScrollDir::Both => String::from("overflow: auto"),
            ScrollDir::None => String::from("overflow: hidden"),
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-scroll ");
        html.push_str(&self.class);
        html.push_str("\" style=\"");
        html.push_str(&overflow);
        html.push_str("; height: ");
        html.push_str(&self.height);
        html.push_str("; width: ");
        html.push_str(&self.width);
        html.push_str(";\">");
        for child in &self.children {
            html.push_str(child);
        }
        html.push_str("</div>");
        html
}
}

