#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use super::traits::RenderableVNode;
use super::vnode::VNode;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TextSize {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TextWeight {
    Normal,
    Bold,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Text {
    pub content: String,
    pub size: TextSize,
    pub weight: TextWeight,
    pub color: String,
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

impl RenderableVNode for Text {
    #[inline]
    fn to_vnode(&self) -> VNode {
        let size_class: String = match self.size {
            TextSize::Small => String::from("wj-text-sm"),
            TextSize::Medium => String::from("wj-text-md"),
            TextSize::Large => String::from("wj-text-lg"),
            TextSize::XLarge => String::from("wj-text-xl"),
        };
        let weight_class: String = match self.weight {
            TextWeight::Normal => String::from("wj-text-normal"),
            TextWeight::Bold => String::from("wj-text-bold"),
        };
        let mut node = VNode::span()
            .add_class("wj-text")
            .add_class(&size_class.to_string())
            .add_class(&weight_class.to_string())
            .add_text(self.content.clone());
        if !self.color.is_empty() {
            node = {
                let _temp0 = format!("color: {};", self.color.clone());
                node.add_style(&_temp0)
            };
        }
        node
    }
}

impl Renderable for Text {
    #[inline]
    fn render(&mut self) -> String {
        let size_class: String = match self.size {
            TextSize::Small => String::from("sm"),
            TextSize::Medium => String::from("md"),
            TextSize::Large => String::from("lg"),
            TextSize::XLarge => String::from("xl"),
        };
        let weight_class: String = match self.weight {
            TextWeight::Normal => String::from("normal"),
            TextWeight::Bold => String::from("bold"),
        };
        let style: String = {
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
