#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct List {
    pub items: Vec<String>,
    pub ordered: bool,
    pub class: String,
}

impl List {
    #[inline]
    pub fn new() -> List {
        List {
            items: Vec::new(),
            ordered: false,
            class: String::new(),
        }
    }
    #[inline]
    pub fn item(mut self, item: String) -> List {
        self.items.push(item);
        self
    }
    #[inline]
    pub fn ordered(mut self, ordered: bool) -> List {
        self.ordered = ordered;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> List {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let tag = {
            if self.ordered {
                "ol".to_string()
            } else {
                "ul".to_string()
            }
        };
        let mut html = String::new();
        html.push('<');
        html.push_str(&tag.clone());
        html.push_str(" class=\"wj-list ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"list-style-position: inside; padding-left: 0;\">");
        for item in &self.items {
            html.push_str("<li style=\"padding: 8px 0;\">");
            html.push_str(&item.as_str());
            html.push_str("</li>");
        }
        html.push_str("</");
        html.push_str(&tag);
        html.push('>');
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ListItem {
    pub content: String,
    pub class: String,
}

impl ListItem {
    #[inline]
    pub fn new(content: String) -> ListItem {
        ListItem {
            content,
            class: String::new(),
        }
    }
    #[inline]
    pub fn class(mut self, class: String) -> ListItem {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<li class=\"wj-list-item ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"padding: 8px 0;\">");
        html.push_str(&self.content.as_str());
        html.push_str("</li>");
        html
    }
}
