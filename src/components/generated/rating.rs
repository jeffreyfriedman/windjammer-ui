#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum RatingSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
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
        Rating {
            value,
            max: 5,
            size: RatingSize::Medium,
            readonly: true,
            color: "#fbbf24".to_string(),
        }
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
    pub fn color(mut self, color: String) -> Rating {
        self.color = color;
        self
    }
}

impl Renderable for Rating {
    #[inline]
    fn render(self) -> String {
        let star_size = match self.size {
            RatingSize::Small => "16px".to_string(),
            RatingSize::Medium => "24px".to_string(),
            RatingSize::Large => "32px".to_string(),
        };
        let mut html = String::new();
        html.push_str("<div style='display: inline-flex; gap: 4px;'>");
        let mut i = 1;
        while i <= self.max {
            let filled = i as f32 <= self.value;
            let half_filled = i as f32 - 0.5 <= self.value && i as f32 > self.value;
            let star_color = {
                if filled || half_filled {
                    self.color.as_str()
                } else {
                    "#e2e8f0"
                }
            };
            let cursor = {
                if self.readonly {
                    "default".to_string()
                } else {
                    "pointer".to_string()
                }
            };
            html.push_str("<span style='font-size: ");
            html.push_str(&star_size);
            html.push_str("; color: ");
            html.push_str(&star_color);
            html.push_str("; cursor: ");
            html.push_str(&cursor);
            html.push_str(";'>");
            if half_filled {
                html.push('⯨')
            } else {
                html.push('★')
            }
            html.push_str("</span>");
            i += 1;
        }
        html.push_str("</div>");
        html
    }
}
