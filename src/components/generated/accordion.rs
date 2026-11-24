#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct AccordionItem {
    pub title: String,
    pub content: String,
    pub open: bool,
}

impl AccordionItem {
#[inline]
pub fn new(title: String, content: String) -> AccordionItem {
        AccordionItem { title, content, open: false }
}
#[inline]
pub fn open(mut self, open: bool) -> AccordionItem {
        self.open = open;
        self
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Accordion {
    pub items: Vec<AccordionItem>,
    pub allow_multiple: bool,
}

impl Accordion {
#[inline]
pub fn new() -> Accordion {
        Accordion { items: Vec::new(), allow_multiple: false }
}
#[inline]
pub fn item(mut self, item: AccordionItem) -> Accordion {
        self.items.push(item);
        self
}
#[inline]
pub fn allow_multiple(mut self, allow: bool) -> Accordion {
        self.allow_multiple = allow;
        self
}
}

impl Renderable for Accordion {

fn render(&self) -> String {
        let mut html: String = String::from("<div class='wj-accordion'>");
        let mut i = 0_usize;
        while i < self.items.len() {
            let item = &self.items[i];
            let open_attr = {
                if item.open {
                    String::from(" open")
                } else {
                    String::new()
                }
            };
            { use std::fmt::Write as _; write!(html, "<details class='wj-accordion-item'{}>\n  <summary class='wj-accordion-title'>{}</summary>\n  <div class='wj-accordion-content'>\n    {}\n  </div>\n</details>", open_attr, item.title, item.content).expect("a formatting trait implementation returned an error"); };
            i = i.wrapping_add(1);
        }
        format!("{}</div>", html)
}
}

