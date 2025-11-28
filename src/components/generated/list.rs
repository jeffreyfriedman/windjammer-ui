#![allow(clippy::all)]
#![allow(noop_method_call)]
pub struct List {
    items: Vec<String>,
    ordered: bool,
    class: String,
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
    pub fn render(&self) -> String {
        let tag = {
            if self.ordered {
                "ol"
            } else {
                "ul"
            }
        };
        let mut html = String::new();
        html.push('<');
        html.push_str(tag.clone());
        html.push_str(" class=\"wj-list ");
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"list-style-position: inside; padding-left: 0;\">");
        for item in self.items.iter() {
            html.push_str("<li style=\"padding: 8px 0;\">");
            html.push_str(item.as_str());
            html.push_str("</li>");
        }
        html.push_str("</");
        html.push_str(tag);
        html.push('>');
        html
    }
}

pub struct ListItem {
    content: String,
    class: String,
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
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"padding: 8px 0;\">");
        html.push_str(self.content.as_str());
        html.push_str("</li>");
        html
    }
}
