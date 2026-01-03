#[derive(Debug, Clone, PartialEq)]
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
        Row { children: Vec::new(), gap: "8px".to_string(), align: RowAlign::Start, justify: RowJustify::Start, wrap: false, class: String::new() }
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
        let align_str = match self.align {
            RowAlign::Start => "flex-start".to_string(),
            RowAlign::Center => "center".to_string(),
            RowAlign::End => "flex-end".to_string(),
            RowAlign::Stretch => "stretch".to_string(),
        };
        let justify_str = match self.justify {
            RowJustify::Start => "flex-start".to_string(),
            RowJustify::Center => "center".to_string(),
            RowJustify::End => "flex-end".to_string(),
            RowJustify::SpaceBetween => "space-between".to_string(),
            RowJustify::SpaceAround => "space-around".to_string(),
            RowJustify::SpaceEvenly => "space-evenly".to_string(),
        };
        let wrap_str = {
            if self.wrap {
                "wrap".to_string()
            } else {
                "nowrap".to_string()
            }
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-row ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"display: flex; flex-direction: row; gap: ");
        html.push_str(&self.gap.as_str());
        html.push_str("; align-items: ");
        html.push_str(&align_str);
        html.push_str("; justify-content: ");
        html.push_str(&justify_str);
        html.push_str("; flex-wrap: ");
        html.push_str(&wrap_str);
        html.push_str(";\">");
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</div>");
        html
}
}

