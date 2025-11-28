#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

pub struct SplitPanel {
    left: String,
    right: String,
    direction: SplitDirection,
    initial_size: i32,
}

impl SplitPanel {
    #[inline]
    pub fn new(left: String, right: String) -> SplitPanel {
        SplitPanel {
            left,
            right,
            direction: SplitDirection::Vertical,
            initial_size: 50,
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
    fn render(self) -> String {
        let flex_direction = match self.direction {
            SplitDirection::Horizontal => "column",
            SplitDirection::Vertical => "row",
        };
        format!(
            "<div class='wj-split-panel' style='display: flex; flex-direction: {}; height: 100%;'>
  <div class='wj-split-pane' style='flex: {}%;'>
    {}
  </div>
  <div class='wj-split-divider'></div>
  <div class='wj-split-pane' style='flex: {}%;'>
    {}
  </div>
</div>",
            flex_direction,
            self.initial_size,
            self.left,
            100 - self.initial_size,
            self.right
        )
    }
}
