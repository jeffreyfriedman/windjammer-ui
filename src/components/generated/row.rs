#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
pub struct Row {
    children: Vec<String>,
    gap: String,
    align: RowAlign,
    justify: RowJustify,
    wrap: bool,
    class: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RowAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn render(&self) -> String {
        let align_str = match self.align {
            RowAlign::Start => "flex-start",
            RowAlign::Center => "center",
            RowAlign::End => "flex-end",
            RowAlign::Stretch => "stretch",
        };
        let justify_str = match self.justify {
            RowJustify::Start => "flex-start",
            RowJustify::Center => "center",
            RowJustify::End => "flex-end",
            RowJustify::SpaceBetween => "space-between",
            RowJustify::SpaceAround => "space-around",
            RowJustify::SpaceEvenly => "space-evenly",
        };
        let wrap_str = {
            if self.wrap {
                "wrap"
            } else {
                "nowrap"
            }
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-row ");
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"display: flex; flex-direction: row; gap: ");
        html.push_str(self.gap.as_str());
        html.push_str("; align-items: ");
        html.push_str(align_str);
        html.push_str("; justify-content: ");
        html.push_str(justify_str);
        html.push_str("; flex-wrap: ");
        html.push_str(wrap_str);
        html.push_str(";\">");
        for child in self.children.iter() {
            html.push_str(child.as_str());
        }
        html.push_str("</div>");
        html
    }
}
