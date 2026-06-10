#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Column {
    children: Vec<String>,
    gap: String,
    align: ColumnAlign,
    justify: ColumnJustify,
    class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ColumnAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ColumnJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Column {
    #[inline]
    pub fn new() -> Column {
        Column {
            children: Vec::new(),
            gap: "8px".to_string(),
            align: ColumnAlign::Start,
            justify: ColumnJustify::Start,
            class: String::new(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Column {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn gap(mut self, gap: String) -> Column {
        self.gap = gap;
        self
    }
    #[inline]
    pub fn align(mut self, align: ColumnAlign) -> Column {
        self.align = align;
        self
    }
    #[inline]
    pub fn justify(mut self, justify: ColumnJustify) -> Column {
        self.justify = justify;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Column {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let align_str: String = match self.align {
            ColumnAlign::Start => String::from("flex-start"),
            ColumnAlign::Center => String::from("center"),
            ColumnAlign::End => String::from("flex-end"),
            ColumnAlign::Stretch => String::from("stretch"),
        };
        let justify_str: String = match self.justify {
            ColumnJustify::Start => String::from("flex-start"),
            ColumnJustify::Center => String::from("center"),
            ColumnJustify::End => String::from("flex-end"),
            ColumnJustify::SpaceBetween => String::from("space-between"),
            ColumnJustify::SpaceAround => String::from("space-around"),
            ColumnJustify::SpaceEvenly => String::from("space-evenly"),
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-column ");
        html.push_str(&self.class.clone());
        html.push_str("\" style=\"display: flex; flex-direction: column; gap: ");
        html.push_str(&self.gap.clone());
        html.push_str("; align-items: ");
        html.push_str(&align_str);
        html.push_str("; justify-content: ");
        html.push_str(&justify_str);
        html.push_str(";\">");
        for child in &self.children {
            html.push_str(child.clone());
        }
        html.push_str("</div>");
        html
    }
}
