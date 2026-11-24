#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Center {
    pub child: String,
    pub width: String,
    pub height: String,
    pub class: String,
}

impl Center {
#[inline]
pub fn new(child: String) -> Center {
        Center { child, width: String::from("100%"), height: String::from("100%"), class: String::new() }
}
#[inline]
pub fn width(mut self, width: &str) -> Center {
        self.width = width.to_string();
        self
}
#[inline]
pub fn height(mut self, height: &str) -> Center {
        self.height = height.to_string();
        self
}
#[inline]
pub fn class(mut self, class: &str) -> Center {
        self.class = class.to_string();
        self
}
#[inline]
pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class=\"wj-center ");
        html.push_str(&self.class);
        html.push_str("\" style=\"display: flex; align-items: center; justify-content: center; width: ");
        html.push_str(&self.width);
        html.push_str("; height: ");
        html.push_str(&self.height);
        html.push_str(";\">");
        html.push_str(&self.child);
        html.push_str("</div>");
        html
}
}

