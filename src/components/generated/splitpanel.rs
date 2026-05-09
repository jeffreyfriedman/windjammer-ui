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
}

impl SplitPanel {
    #[inline]
    pub fn new(left: String, right: String) -> SplitPanel {
        SplitPanel {
            left: left.to_string(),
            right: right.to_string(),
            direction: SplitDirection::Vertical,
            initial_size: 50_i32,
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
}

impl Renderable for SplitPanel {
    #[inline]
    fn render(&self) -> String {
        let flex_direction = match self.direction {
            SplitDirection::Horizontal => "column".to_string(),
            SplitDirection::Vertical => "row".to_string(),
        };
        format!("<div class='wj-split-panel' style='display: flex; flex-direction: {}; height: 100%;'>\n  <div class='wj-split-pane' style='flex: {}%;'>\n    {}\n  </div>\n  <div class='wj-split-divider'></div>\n  <div class='wj-split-pane' style='flex: {}%;'>\n    {}\n  </div>\n</div>", flex_direction, self.initial_size, self.left.clone(), 100_i32 - self.initial_size, self.right.clone())
    }
}
