#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
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
    fn render(&mut self) -> String {
        let position_class: String = match self.position {
            TooltipPosition::Top => String::from("wj-tooltip-top"),
            TooltipPosition::Bottom => String::from("wj-tooltip-bottom"),
            TooltipPosition::Left => String::from("wj-tooltip-left"),
            TooltipPosition::Right => String::from("wj-tooltip-right"),
        };
        format!(
            "<div class='wj-tooltip-container {}'>{}<span class='wj-tooltip-text'>{}</span></div>",
            position_class, self.child, self.text
        )
    }
}
