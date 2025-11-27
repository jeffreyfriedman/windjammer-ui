#![allow(clippy::all)]
#![allow(noop_method_call)]
pub struct Popover {
    trigger: String,
    content: String,
    position: PopoverPosition,
    class: String,
}

pub enum PopoverPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Popover {
    #[inline]
    pub fn new(trigger: String, content: String) -> Popover {
        Popover {
            trigger,
            content,
            position: PopoverPosition::Bottom,
            class: String::new(),
        }
    }
    #[inline]
    pub fn position(mut self, position: PopoverPosition) -> Popover {
        self.position = position;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Popover {
        self.class = class;
        self
    }
    pub fn render(&self) -> String {
        let position_style = match self.position {
            PopoverPosition::Top => {
                "bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 8px;"
            }
            PopoverPosition::Bottom => {
                "top: 100%; left: 50%; transform: translateX(-50%); margin-top: 8px;"
            }
            PopoverPosition::Left => {
                "right: 100%; top: 50%; transform: translateY(-50%); margin-right: 8px;"
            }
            PopoverPosition::Right => {
                "left: 100%; top: 50%; transform: translateY(-50%); margin-left: 8px;"
            }
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-popover ");
        html.push_str(self.class.as_str());
        html.push_str("\" style=\"position: relative; display: inline-block;\">");
        html.push_str(self.trigger.as_str());
        html.push_str("<div class=\"wj-popover-content\" style=\"position: absolute; ");
        html.push_str(position_style);
        html.push_str(" background: white; border: 1px solid #e5e7eb; border-radius: 8px; padding: 12px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); min-width: 200px; z-index: 1000; display: none;\">");
        html.push_str(self.content.as_str());
        html.push_str("</div>");
        html.push_str("</div>");
        html.push_str("<style>.wj-popover:hover .wj-popover-content { display: block; }</style>");
        html
    }
}
