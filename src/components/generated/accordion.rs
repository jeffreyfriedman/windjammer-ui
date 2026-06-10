#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct AccordionItem {
    title: String,
    content: String,
    open: bool,
}

impl AccordionItem {
    #[inline]
    pub fn new(title: String, content: String) -> AccordionItem {
        AccordionItem {
            title,
            content,
            open: false,
        }
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
    items: Vec<AccordionItem>,
    allow_multiple: bool,
}

impl Accordion {
    #[inline]
    pub fn new() -> Accordion {
        Accordion {
            items: Vec::new(),
            allow_multiple: false,
        }
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
    #[inline]
    fn render(self) -> String {
        let mut html = "<div class='wj-accordion'>".to_string();
        let mut i = 0;
        while i < self.items.len() {
            let item = &self.items[i];
            let open_attr = {
                if item.open {
                    String::from(" open")
                } else {
                    String::new()
                }
            };
            html = format!("{}<details class='wj-accordion-item'{}>\n  <summary class='wj-accordion-title'>{}</summary>\n  <div class='wj-accordion-content'>\n    {}\n  </div>\n</details>", html, open_attr, item.title, item.content);
            i += 1;
        }
        format!("{}</div>", html)
    }
}
