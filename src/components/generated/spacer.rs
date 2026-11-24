#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Spacer {
    pub width: String,
    pub height: String,
    pub flex: bool,
}

impl Spacer {
#[inline]
pub fn new() -> Spacer {
        Spacer { width: String::new(), height: String::new(), flex: false }
}
#[inline]
pub fn horizontal(width: String) -> Spacer {
        Spacer { width, height: String::new(), flex: false }
}
#[inline]
pub fn vertical(height: String) -> Spacer {
        Spacer { width: String::new(), height, flex: false }
}
#[inline]
pub fn flexible() -> Spacer {
        Spacer { width: String::new(), height: String::new(), flex: true }
}
#[inline]
pub fn width(mut self, width: impl Into<String>) -> Spacer {
        self.width = width.into();
        self
}
#[inline]
pub fn height(mut self, height: impl Into<String>) -> Spacer {
        self.height = height.into();
        self
}
#[inline]
pub fn xxs() -> Spacer {
        Spacer::vertical("4px".to_string())
}
#[inline]
pub fn xs() -> Spacer {
        Spacer::vertical("8px".to_string())
}
#[inline]
pub fn sm() -> Spacer {
        Spacer::vertical("12px".to_string())
}
#[inline]
pub fn md() -> Spacer {
        Spacer::vertical("16px".to_string())
}
#[inline]
pub fn lg() -> Spacer {
        Spacer::vertical("24px".to_string())
}
#[inline]
pub fn xl() -> Spacer {
        Spacer::vertical("32px".to_string())
}
#[inline]
pub fn xxl() -> Spacer {
        Spacer::vertical("48px".to_string())
}
}

impl Renderable for Spacer {
#[inline]
fn render(&self) -> String {
        let mut style = String::new();
        if self.flex {
            style = String::from("flex: 1; ");
        }
        if self.width != "" {
            { use std::fmt::Write as _; write!(style, "width: {}; ", self.width).expect("a formatting trait implementation returned an error"); };
        }
        if self.height != "" {
            { use std::fmt::Write as _; write!(style, "height: {}; ", self.height).expect("a formatting trait implementation returned an error"); };
        }
        format!("<div class='wj-spacer' style='{}'></div>", style)
}
}

