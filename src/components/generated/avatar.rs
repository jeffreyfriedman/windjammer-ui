#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Avatar {
    pub src: String,
    pub alt: String,
    pub size: AvatarSize,
    pub shape: AvatarShape,
    pub fallback: String,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AvatarShape {
    Circle,
    Square,
    Rounded,
}

impl Avatar {
    #[inline]
    pub fn new(src: String) -> Avatar {
        Avatar {
            src,
            alt: "Avatar".to_string(),
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            fallback: String::new(),
            class: String::new(),
        }
    }
    #[inline]
    pub fn alt(mut self, alt: String) -> Avatar {
        self.alt = alt;
        self
    }
    #[inline]
    pub fn size(mut self, size: AvatarSize) -> Avatar {
        self.size = size;
        self
    }
    #[inline]
    pub fn shape(mut self, shape: AvatarShape) -> Avatar {
        self.shape = shape;
        self
    }
    #[inline]
    pub fn fallback(mut self, fallback: String) -> Avatar {
        self.fallback = fallback;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Avatar {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let size_px: String = match self.size {
            AvatarSize::Small => String::from("32px"),
            AvatarSize::Medium => String::from("48px"),
            AvatarSize::Large => String::from("64px"),
            AvatarSize::XLarge => String::from("96px"),
        };
        let border_radius: String = match self.shape {
            AvatarShape::Circle => String::from("50%"),
            AvatarShape::Square => String::from("0"),
            AvatarShape::Rounded => String::from("8px"),
        };
        let mut html = String::new();
        if self.src.is_empty() && !self.fallback.is_empty() {
            html.push_str("<div class=\"wj-avatar wj-avatar-fallback ");
            html.push_str(&self.class);
            html.push_str("\" style=\"width: ");
            html.push_str(&size_px);
            html.push_str("; height: ");
            html.push_str(&size_px);
            html.push_str("; border-radius: ");
            html.push_str(&border_radius);
            html.push_str("; background-color: #3b82f6; color: white; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: ");
            let font_size: String = match self.size {
                AvatarSize::Small => String::from("12px"),
                AvatarSize::Medium => String::from("16px"),
                AvatarSize::Large => String::from("20px"),
                AvatarSize::XLarge => String::from("28px"),
            };
            html.push_str(&font_size);
            html.push_str(";\">");
            html.push_str(&self.fallback);
            html.push_str("</div>")
        } else {
            html.push_str("<img class=\"wj-avatar ");
            html.push_str(&self.class);
            html.push_str("\" src=\"");
            html.push_str(&self.src);
            html.push_str("\" alt=\"");
            html.push_str(&self.alt);
            html.push_str("\" style=\"width: ");
            html.push_str(&size_px);
            html.push_str("; height: ");
            html.push_str(&size_px);
            html.push_str("; border-radius: ");
            html.push_str(&border_radius);
            html.push_str("; object-fit: cover;\">")
        }
        html
    }
}
