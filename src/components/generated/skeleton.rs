#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Skeleton {
    pub variant: SkeletonVariant,
    pub width: String,
    pub height: String,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rectangle,
}

impl Skeleton {
#[inline]
pub fn new() -> Skeleton {
        Skeleton { variant: SkeletonVariant::Text, width: String::from("100%"), height: String::from("20px"), class: String::new() }
}
#[inline]
pub fn variant(mut self, variant: SkeletonVariant) -> Skeleton {
        self.variant = variant;
        self
}
#[inline]
pub fn width(mut self, width: &str) -> Skeleton {
        self.width = width.to_string();
        self
}
#[inline]
pub fn height(mut self, height: &str) -> Skeleton {
        self.height = height.to_string();
        self
}
#[inline]
pub fn class(mut self, class: &str) -> Skeleton {
        self.class = class.to_string();
        self
}

pub fn render(&self) -> String {
        let border_radius: String = match &self.variant {
            SkeletonVariant::Text => String::from("4px"),
            SkeletonVariant::Circle => String::from("50%"),
            SkeletonVariant::Rectangle => String::from("8px"),
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-skeleton ");
        html.push_str(&self.class);
        html.push_str("\" style=\"width: ");
        html.push_str(&self.width);
        html.push_str("; height: ");
        html.push_str(&self.height);
        html.push_str("; border-radius: ");
        html.push_str(&border_radius);
        html.push_str("; background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s ease-in-out infinite;\"></div>");
        html.push_str("<style>@keyframes skeleton-loading { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }</style>");
        html
}
}

