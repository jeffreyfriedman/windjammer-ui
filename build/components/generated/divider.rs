#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Divider {
    orientation: DividerOrientation,
    color: String,
    thickness: String,
    margin: String,
}

impl Divider {
    #[inline]
    pub fn new() -> Divider {
        Divider {
            orientation: DividerOrientation::Horizontal,
            color: "#3E3E3E".to_string(),
            thickness: "1px".to_string(),
            margin: "0".to_string(),
        }
    }
    #[inline]
    pub fn horizontal() -> Divider {
        Divider::new()
    }
    #[inline]
    pub fn vertical() -> Divider {
        Divider {
            orientation: DividerOrientation::Vertical,
            color: "#3E3E3E".to_string(),
            thickness: "1px".to_string(),
            margin: "0".to_string(),
        }
    }
    #[inline]
    pub fn color(mut self, color: String) -> Divider {
        self.color = color;
        self
    }
    #[inline]
    pub fn thickness(mut self, thickness: String) -> Divider {
        self.thickness = thickness;
        self
    }
    #[inline]
    pub fn margin(mut self, margin: String) -> Divider {
        self.margin = margin;
        self
    }
}

impl Renderable for Divider {
    #[inline]
    fn render(self) -> String {
        let orientation_class: String = match self.orientation {
            DividerOrientation::Horizontal => String::from("wj-divider-horizontal"),
            DividerOrientation::Vertical => String::from("wj-divider-vertical"),
        };
        let style: String = match self.orientation {
            DividerOrientation::Horizontal => {
                format!(
                    "width: 100%; height: {}; background: {}; margin: {} 0;",
                    self.thickness, self.color, self.margin
                )
            }
            DividerOrientation::Vertical => {
                format!(
                    "width: {}; height: 100%; background: {}; margin: 0 {};",
                    self.thickness, self.color, self.margin
                )
            }
        };
        format!(
            "<div class='wj-divider {}' style='{}'></div>",
            orientation_class, style
        )
    }
}
