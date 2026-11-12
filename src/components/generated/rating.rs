#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum RatingSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Rating {
    pub value: f32,
    pub max: i32,
    pub size: RatingSize,
    pub readonly: bool,
    pub color: String,
}

impl Rating {
#[inline]
pub fn new(value: f32) -> Rating {
        Rating { value, max: 5_i32, size: RatingSize::Medium, readonly: true, color: String::from("#fbbf24") }
}
#[inline]
pub fn max(mut self, max: i32) -> Rating {
        self.max = max;
        self
}
#[inline]
pub fn size(mut self, size: RatingSize) -> Rating {
        self.size = size;
        self
}
#[inline]
pub fn readonly(mut self, readonly: bool) -> Rating {
        self.readonly = readonly;
        self
}
#[inline]
pub fn color(mut self, color: impl Into<String>) -> Rating {
        self.color = color.into();
        self
}
}

impl Renderable for Rating {

fn render(&self) -> String {
        let star_size: String = match &self.size {
            RatingSize::Small => String::from("16px"),
            RatingSize::Medium => String::from("24px"),
            RatingSize::Large => String::from("32px"),
        };
        let mut html = String::new();
        html.push_str("<div style='display: inline-flex; gap: 4px;'>");
        let mut i = 1_i32;
        while i <= self.max {
            let filled = i as f32 <= self.value as f32;
            let half_filled = i as f32 - 0.5_f32 <= self.value && (i as f32) > (self.value as f32);
            let star_color = {
                if filled || half_filled {
                    self.color.clone()
                } else {
                    String::from("#e2e8f0")
                }
            };
            let cursor: String = {
                if self.readonly {
                    String::from("default")
                } else {
                    String::from("pointer")
                }
            };
            let _ = html.push_str("<span style='font-size: ");
            let _ = html.push_str(&star_size);
            let _ = html.push_str("; color: ");
            let _ = html.push_str(&star_color);
            let _ = html.push_str("; cursor: ");
            let _ = html.push_str(&cursor);
            let _ = html.push_str(";'>");
            if half_filled {
                let _ = html.push('⯨');
            } else {
                let _ = html.push('★');
            }
            let _ = html.push_str("</span>");
            i += 1;
        }
        html.push_str("</div>");
        html
}
}

