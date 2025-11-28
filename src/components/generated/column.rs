#![allow(clippy::all)]
#![allow(noop_method_call)]
pub struct Column {
    children: Vec<String>,
    gap: String,
    align: ColumnAlign,
    justify: ColumnJustify,
    class: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn render(&self) -> String {
        let align_str = match self.align {
            ColumnAlign::Start => "flex-start",
            ColumnAlign::Center => "center",
            ColumnAlign::End => "flex-end",
            ColumnAlign::Stretch => "stretch",
        };
        let justify_str = match self.justify {
            ColumnJustify::Start => "flex-start",
            ColumnJustify::Center => "center",
            ColumnJustify::End => "flex-end",
            ColumnJustify::SpaceBetween => "space-between",
            ColumnJustify::SpaceAround => "space-around",
            ColumnJustify::SpaceEvenly => "space-evenly",
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-column ");
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"display: flex; flex-direction: column; gap: ");
        html.push_str(self.gap.as_str());
        html.push_str("; align-items: ");
        html.push_str(align_str);
        html.push_str("; justify-content: ");
        html.push_str(justify_str);
        html.push_str(";\">");
        for child in self.children.iter() {
            html.push_str(child.as_str());
        }
        html.push_str("</div>");
        html
    }
}
