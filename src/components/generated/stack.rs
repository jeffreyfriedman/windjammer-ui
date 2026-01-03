use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum StackAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum StackJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    pub direction: StackDirection,
    pub gap: String,
    pub align: StackAlign,
    pub justify: StackJustify,
    pub children: Vec<String>,
    pub padding: String,
    pub width: String,
    pub height: String,
}

impl Stack {
    #[inline]
    pub fn new() -> Stack {
        Stack {
            direction: StackDirection::Vertical,
            gap: "8px".to_string(),
            align: StackAlign::Stretch,
            justify: StackJustify::Start,
            children: Vec::new(),
            padding: "0".to_string(),
            width: "auto".to_string(),
            height: "auto".to_string(),
        }
    }
    #[inline]
    pub fn vertical() -> Stack {
        Stack::new()
    }
    #[inline]
    pub fn horizontal() -> Stack {
        let mut stack = Stack::new();
        stack.direction = StackDirection::Horizontal;
        stack
    }
    #[inline]
    pub fn direction(mut self, dir: StackDirection) -> Stack {
        self.direction = dir;
        self
    }
    #[inline]
    pub fn gap(mut self, gap: String) -> Stack {
        self.gap = gap;
        self
    }
    #[inline]
    pub fn align(mut self, align: StackAlign) -> Stack {
        self.align = align;
        self
    }
    #[inline]
    pub fn justify(mut self, justify: StackJustify) -> Stack {
        self.justify = justify;
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Stack {
        self.padding = padding;
        self
    }
    #[inline]
    pub fn width(mut self, width: String) -> Stack {
        self.width = width;
        self
    }
    #[inline]
    pub fn height(mut self, height: String) -> Stack {
        self.height = height;
        self
    }
    #[inline]
    pub fn child(mut self, child: String) -> Stack {
        self.children.push(child);
        self
    }
}

impl Renderable for Stack {
    #[inline]
    fn render(self) -> String {
        let flex_direction = match self.direction {
            StackDirection::Vertical => "column".to_string(),
            StackDirection::Horizontal => "row".to_string(),
        };
        let align_items = match self.align {
            StackAlign::Start => "flex-start".to_string(),
            StackAlign::Center => "center".to_string(),
            StackAlign::End => "flex-end".to_string(),
            StackAlign::Stretch => "stretch".to_string(),
        };
        let justify_content = match self.justify {
            StackJustify::Start => "flex-start".to_string(),
            StackJustify::Center => "center".to_string(),
            StackJustify::End => "flex-end".to_string(),
            StackJustify::SpaceBetween => "space-between".to_string(),
            StackJustify::SpaceAround => "space-around".to_string(),
            StackJustify::SpaceEvenly => "space-evenly".to_string(),
        };
        let mut html = String::new();
        html.push_str("<div style='display: flex; flex-direction: ");
        html.push_str(&flex_direction);
        html.push_str("; gap: ");
        html.push_str(&self.gap);
        html.push_str("; align-items: ");
        html.push_str(&align_items);
        html.push_str("; justify-content: ");
        html.push_str(&justify_content);
        html.push_str("; padding: ");
        html.push_str(&self.padding);
        html.push_str("; width: ");
        html.push_str(&self.width);
        html.push_str("; height: ");
        html.push_str(&self.height);
        html.push_str(";'>");
        for child in &self.children {
            html.push_str(child);
        }
        html.push_str("</div>");
        html
    }
}
