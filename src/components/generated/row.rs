#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Row {
    pub children: Vec<String>,
    pub gap: String,
    pub align: RowAlign,
    pub justify: RowJustify,
    pub wrap: bool,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum RowAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum RowJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Row {
    #[inline]
    pub fn new() -> Row {
        Row {
            children: Vec::new(),
            gap: "8px".to_string(),
            align: RowAlign::Start,
            justify: RowJustify::Start,
            wrap: false,
            class: String::new(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Row {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn gap(mut self, gap: String) -> Row {
        self.gap = gap;
        self
    }
    #[inline]
    pub fn align(mut self, align: RowAlign) -> Row {
        self.align = align;
        self
    }
    #[inline]
    pub fn justify(mut self, justify: RowJustify) -> Row {
        self.justify = justify;
        self
    }
    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Row {
        self.wrap = wrap;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Row {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let align_str: String = match self.align {
            RowAlign::Start => String::from("flex-start"),
            RowAlign::Center => String::from("center"),
            RowAlign::End => String::from("flex-end"),
            RowAlign::Stretch => String::from("stretch"),
        };
        let justify_str: String = match self.justify {
            RowJustify::Start => String::from("flex-start"),
            RowJustify::Center => String::from("center"),
            RowJustify::End => String::from("flex-end"),
            RowJustify::SpaceBetween => String::from("space-between"),
            RowJustify::SpaceAround => String::from("space-around"),
            RowJustify::SpaceEvenly => String::from("space-evenly"),
        };
        let wrap_str: String = {
            if self.wrap {
                String::from("wrap")
            } else {
                String::from("nowrap")
            }
        };
        let mut html = String::new();
        html.push_str(&"<div class=\"wj-row ");
        html.push_str(&self.class);
        html.push_str(&"\" style=\"display: flex; flex-direction: row; gap: ");
        html.push_str(&self.gap);
        html.push_str(&"; align-items: ");
        html.push_str(&align_str);
        html.push_str(&"; justify-content: ");
        html.push_str(&justify_str);
        html.push_str(&"; flex-wrap: ");
        html.push_str(&wrap_str);
        html.push_str(&";\">");
        for child in &self.children {
            html.push_str(child);
        }
        html.push_str(&"</div>");
        html
    }
}
