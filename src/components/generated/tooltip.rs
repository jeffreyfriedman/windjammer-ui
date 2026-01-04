#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tooltip {
    pub text: String,
    pub position: TooltipPosition,
    pub child: String,
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
    fn render(&self) -> String {
        let position_class = match self.position {
            TooltipPosition::Top => "wj-tooltip-top".to_string(),
            TooltipPosition::Bottom => "wj-tooltip-bottom".to_string(),
            TooltipPosition::Left => "wj-tooltip-left".to_string(),
            TooltipPosition::Right => "wj-tooltip-right".to_string(),
        };
        format!(
            "<div class='wj-tooltip-container {}'>{}<span class='wj-tooltip-text'>{}</span></div>",
            position_class, self.child, self.text
        )
    }
}
