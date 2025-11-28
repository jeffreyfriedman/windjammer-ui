#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct Tooltip {
    text: String,
    position: TooltipPosition,
    child: String,
}

impl Tooltip {
    #[inline]
    pub fn new(text: String, child: String) -> Tooltip {
        Tooltip {
            text,
            position: TooltipPosition::Top,
            child,
        }
    }
    #[inline]
    pub fn position(mut self, position: TooltipPosition) -> Tooltip {
        self.position = position;
        self
    }
}

impl Renderable for Tooltip {
    #[inline]
    fn render(self) -> String {
        let position_class = match self.position {
            TooltipPosition::Top => "wj-tooltip-top",
            TooltipPosition::Bottom => "wj-tooltip-bottom",
            TooltipPosition::Left => "wj-tooltip-left",
            TooltipPosition::Right => "wj-tooltip-right",
        };
        format!(
            "<div class='wj-tooltip-container {}'>{}<span class='wj-tooltip-text'>{}</span></div>",
            position_class, self.child, self.text
        )
    }
}
