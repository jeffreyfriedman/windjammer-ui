#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub children: Vec<String>,
    pub gap: String,
    pub align: ColumnAlign,
    pub justify: ColumnJustify,
    pub class: String,
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
        Column { children: Vec::new(), gap: "8px".to_string(), align: ColumnAlign::Start, justify: ColumnJustify::Start, class: String::new() }
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
        let align_str = match self.align {
            ColumnAlign::Start => "flex-start".to_string(),
            ColumnAlign::Center => "center".to_string(),
            ColumnAlign::End => "flex-end".to_string(),
            ColumnAlign::Stretch => "stretch".to_string(),
        };
        let justify_str = match self.justify {
            ColumnJustify::Start => "flex-start".to_string(),
            ColumnJustify::Center => "center".to_string(),
            ColumnJustify::End => "flex-end".to_string(),
            ColumnJustify::SpaceBetween => "space-between".to_string(),
            ColumnJustify::SpaceAround => "space-around".to_string(),
            ColumnJustify::SpaceEvenly => "space-evenly".to_string(),
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-column ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"display: flex; flex-direction: column; gap: ");
        html.push_str(&self.gap.as_str());
        html.push_str("; align-items: ");
        html.push_str(&align_str);
        html.push_str("; justify-content: ");
        html.push_str(&justify_str);
        html.push_str(";\">");
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</div>");
        html
}
}

