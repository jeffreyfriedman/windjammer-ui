#![allow(clippy::all)]
#![allow(noop_method_call)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Label {
    text: String,
    for_id: String,
    required: bool,
    class: String,
}

impl Label {
    #[inline]
    pub fn new(text: String) -> Label {
        Label {
            text,
            for_id: String::new(),
            required: false,
            class: String::new(),
        }
    }
    #[inline]
    pub fn for_id(mut self, for_id: String) -> Label {
        self.for_id = for_id;
        self
    }
    #[inline]
    pub fn required(mut self, required: bool) -> Label {
        self.required = required;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Label {
        self.class = class;
        self
    }
    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<label class=\"wj-label ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"font-weight: 500; font-size: 14px; color: #374151; display: block; margin-bottom: 4px;\"");
        if !self.for_id.is_empty() {
            html.push_str(" for=\"");
            html.push_str(&self.for_id.as_str());
            html.push('"')
        }
        html.push('>');
        html.push_str(&self.text.as_str());
        if self.required {
            html.push_str("<span style=\"color: #ef4444; margin-left: 4px;\">*</span>")
        }
        html.push_str("</label>");
        html
    }
}
