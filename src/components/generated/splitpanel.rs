#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SplitPanel {
    pub left: String,
    pub right: String,
    pub direction: SplitDirection,
    pub initial_size: i32,
    pub class_name: String,
}

impl SplitPanel {
    #[inline]
    pub fn new(left: String, right: String) -> SplitPanel {
        SplitPanel {
            left,
            right,
            direction: SplitDirection::Vertical,
            initial_size: 50_i32,
            class_name: "wj-split-panel".to_string(),
        }
    }
    #[inline]
    pub fn direction(mut self, direction: SplitDirection) -> SplitPanel {
        self.direction = direction;
        self
    }
    #[inline]
    pub fn initial_size(mut self, size: i32) -> SplitPanel {
        self.initial_size = size;
        self
    }
    #[inline]
    pub fn class_name(mut self, class_name: String) -> SplitPanel {
        self.class_name = class_name;
        self
    }
}

impl Renderable for SplitPanel {
    #[inline]
    fn render(&self) -> String {
        let flex_direction: String = match self.direction {
            SplitDirection::Horizontal => String::from("column"),
            SplitDirection::Vertical => String::from("row"),
        };
        let right_flex = 100_i32 - self.initial_size;
        format!(
            "<div class='{}' style='display: flex; flex-direction: {}; height: 100%;'><div class='wj-split-pane' style='flex: {}%;'>{}</div><div class='wj-split-divider'></div><div class='wj-split-pane' style='flex: {}%;'>{}</div></div>",
            self.class_name.clone(),
            flex_direction,
            self.initial_size,
            self.left.clone(),
            right_flex,
            self.right.clone()
        )
    }
}
