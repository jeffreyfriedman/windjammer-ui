#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum TextSize {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextWeight {
    Normal,
    Bold,
}

pub struct Text {
    content: String,
    size: TextSize,
    weight: TextWeight,
    color: String,
}

impl Text {
    #[inline]
    pub fn new(content: String) -> Text {
        Text {
            content,
            size: TextSize::Medium,
            weight: TextWeight::Normal,
            color: "".to_string(),
        }
    }
    #[inline]
    pub fn size(mut self, size: TextSize) -> Text {
        self.size = size;
        self
    }
    #[inline]
    pub fn bold(mut self) -> Text {
        self.weight = TextWeight::Bold;
        self
    }
    #[inline]
    pub fn color(mut self, color: String) -> Text {
        self.color = color;
        self
    }
}

impl Renderable for Text {
    #[inline]
    fn render(self) -> String {
        let size_class = match self.size {
            TextSize::Small => "sm",
            TextSize::Medium => "md",
            TextSize::Large => "lg",
            TextSize::XLarge => "xl",
        };
        let weight_class = match self.weight {
            TextWeight::Normal => "normal",
            TextWeight::Bold => "bold",
        };
        let style = {
            if self.color != "" {
                format!(" style='color: {};'", self.color)
            } else {
                "".to_string()
            }
        };
        format!(
            "<span class='wj-text {} {}'{}>{}</span>",
            size_class, weight_class, style, self.content
        )
    }
}
