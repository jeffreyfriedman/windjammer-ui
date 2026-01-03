#[derive(Debug, Clone, PartialEq)]
pub struct Scroll {
    pub children: Vec<String>,
    pub direction: ScrollDir,
    pub height: String,
    pub width: String,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ScrollDir {
    Vertical,
    Horizontal,
    Both,
    None,
}

impl Scroll {
#[inline]
pub fn new() -> Scroll {
        Scroll { children: Vec::new(), direction: ScrollDir::Vertical, height: "400px".to_string(), width: "100%".to_string(), class: String::new() }
}
#[inline]
pub fn child(mut self, child: String) -> Scroll {
        self.children.push(child);
        self
}
#[inline]
pub fn direction(mut self, direction: ScrollDir) -> Scroll {
        self.direction = direction;
        self
}
#[inline]
pub fn height(mut self, height: String) -> Scroll {
        self.height = height;
        self
}
#[inline]
pub fn width(mut self, width: String) -> Scroll {
        self.width = width;
        self
}
#[inline]
pub fn class(mut self, class: String) -> Scroll {
        self.class = class;
        self
}
#[inline]
pub fn render(&self) -> String {
        let overflow = match self.direction {
            ScrollDir::Vertical => "overflow-x: hidden; overflow-y: auto".to_string(),
            ScrollDir::Horizontal => "overflow-x: auto; overflow-y: hidden".to_string(),
            ScrollDir::Both => "overflow: auto".to_string(),
            ScrollDir::None => "overflow: hidden".to_string(),
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-scroll ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"");
        html.push_str(&overflow);
        html.push_str("; height: ");
        html.push_str(&self.height.as_str());
        html.push_str("; width: ");
        html.push_str(&self.width.as_str());
        html.push_str(";\">");
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</div>");
        html
}
}

