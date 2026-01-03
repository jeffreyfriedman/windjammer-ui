#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Div {
    pub children: Vec<String>,
    pub class: String,
    pub style: String,
    pub id: String,
}

impl Div {
    #[inline]
    pub fn new() -> Div {
        Div {
            children: Vec::new(),
            class: String::new(),
            style: String::new(),
            id: String::new(),
        }
    }
    #[inline]
    pub fn child<T: Renderable>(mut self, component: T) -> Div {
        self.children.push(component.render());
        self
    }
    #[inline]
    pub fn text(mut self, text: String) -> Div {
        self.children.push(text);
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Div {
        self.class = class;
        self
    }
    #[inline]
    pub fn style(mut self, style: String) -> Div {
        self.style = style;
        self
    }
    #[inline]
    pub fn id(mut self, id: String) -> Div {
        self.id = id;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<div");
        if !self.id.is_empty() {
            html.push_str(" id=\"");
            html.push_str(&self.id.as_str());
            html.push('"')
        }
        if !self.class.is_empty() {
            html.push_str(" class=\"");
            html.push_str(&self.class.as_str());
            html.push('"')
        }
        if !self.style.is_empty() {
            html.push_str(" style=\"");
            html.push_str(&self.style.as_str());
            html.push('"')
        }
        html.push('>');
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</div>");
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Span {
    pub children: Vec<String>,
    pub class: String,
    pub style: String,
}

impl Span {
    #[inline]
    pub fn new() -> Span {
        Span {
            children: Vec::new(),
            class: String::new(),
            style: String::new(),
        }
    }
    #[inline]
    pub fn child<T: Renderable>(mut self, component: T) -> Span {
        self.children.push(component.render());
        self
    }
    #[inline]
    pub fn text(mut self, text: String) -> Span {
        self.children.push(text);
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Span {
        self.class = class;
        self
    }
    #[inline]
    pub fn style(mut self, style: String) -> Span {
        self.style = style;
        self
    }
}

impl Renderable for Span {
    #[inline]
    fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<span");
        if !self.class.is_empty() {
            html.push_str(" class=\"");
            html.push_str(&self.class.as_str());
            html.push('"')
        }
        if !self.style.is_empty() {
            html.push_str(" style=\"");
            html.push_str(&self.style.as_str());
            html.push('"')
        }
        html.push('>');
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</span>");
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct P {
    pub children: Vec<String>,
    pub class: String,
    pub style: String,
}

impl P {
    #[inline]
    pub fn new() -> P {
        P {
            children: Vec::new(),
            class: String::new(),
            style: String::new(),
        }
    }
    #[inline]
    pub fn child<T: Renderable>(mut self, component: T) -> P {
        self.children.push(component.render());
        self
    }
    #[inline]
    pub fn text(mut self, text: String) -> P {
        self.children.push(text);
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> P {
        self.class = class;
        self
    }
    #[inline]
    pub fn style(mut self, style: String) -> P {
        self.style = style;
        self
    }
}

impl Renderable for P {
    #[inline]
    fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<p");
        if !self.class.is_empty() {
            html.push_str(" class=\"");
            html.push_str(&self.class.as_str());
            html.push('"')
        }
        if !self.style.is_empty() {
            html.push_str(" style=\"");
            html.push_str(&self.style.as_str());
            html.push('"')
        }
        html.push('>');
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</p>");
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct H1 {
    pub text: String,
    pub class: String,
    pub style: String,
}

impl H1 {
    #[inline]
    pub fn new(text: String) -> H1 {
        H1 {
            text,
            class: String::new(),
            style: String::new(),
        }
    }
    #[inline]
    pub fn class(mut self, class: String) -> H1 {
        self.class = class;
        self
    }
    #[inline]
    pub fn style(mut self, style: String) -> H1 {
        self.style = style;
        self
    }
}

impl Renderable for H1 {
    #[inline]
    fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<h1");
        if !self.class.is_empty() {
            html.push_str(" class=\"");
            html.push_str(&self.class.as_str());
            html.push('"')
        }
        if !self.style.is_empty() {
            html.push_str(" style=\"");
            html.push_str(&self.style.as_str());
            html.push('"')
        }
        html.push('>');
        html.push_str(&self.text.as_str());
        html.push_str("</h1>");
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct H2 {
    pub text: String,
    pub class: String,
    pub style: String,
}

impl H2 {
    #[inline]
    pub fn new(text: String) -> H2 {
        H2 {
            text,
            class: String::new(),
            style: String::new(),
        }
    }
    #[inline]
    pub fn class(mut self, class: String) -> H2 {
        self.class = class;
        self
    }
    #[inline]
    pub fn style(mut self, style: String) -> H2 {
        self.style = style;
        self
    }
}

impl Renderable for H2 {
    #[inline]
    fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<h2");
        if !self.class.is_empty() {
            html.push_str(" class=\"");
            html.push_str(&self.class.as_str());
            html.push('"')
        }
        if !self.style.is_empty() {
            html.push_str(" style=\"");
            html.push_str(&self.style.as_str());
            html.push('"')
        }
        html.push('>');
        html.push_str(&self.text.as_str());
        html.push_str("</h2>");
        html
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct H3 {
    pub text: String,
    pub class: String,
    pub style: String,
}

impl H3 {
    #[inline]
    pub fn new(text: String) -> H3 {
        H3 {
            text,
            class: String::new(),
            style: String::new(),
        }
    }
    #[inline]
    pub fn class(mut self, class: String) -> H3 {
        self.class = class;
        self
    }
    #[inline]
    pub fn style(mut self, style: String) -> H3 {
        self.style = style;
        self
    }
}

impl Renderable for H3 {
    #[inline]
    fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<h3");
        if !self.class.is_empty() {
            html.push_str(" class=\"");
            html.push_str(&self.class.as_str());
            html.push('"')
        }
        if !self.style.is_empty() {
            html.push_str(" style=\"");
            html.push_str(&self.style.as_str());
            html.push('"')
        }
        html.push('>');
        html.push_str(&self.text.as_str());
        html.push_str("</h3>");
        html
    }
}
