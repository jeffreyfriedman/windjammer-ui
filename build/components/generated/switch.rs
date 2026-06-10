#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Switch {
    checked: bool,
    disabled: bool,
    size: SwitchSize,
    label: String,
    class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SwitchSize {
    Small,
    Medium,
    Large,
}

impl Switch {
    #[inline]
    pub fn new() -> Switch {
        Switch {
            checked: false,
            disabled: false,
            size: SwitchSize::Medium,
            label: String::new(),
            class: String::new(),
        }
    }
    #[inline]
    pub fn checked(mut self, checked: bool) -> Switch {
        self.checked = checked;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Switch {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn size(mut self, size: SwitchSize) -> Switch {
        self.size = size;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> Switch {
        self.label = label;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Switch {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let (width, height, thumb_size) = match self.size {
            SwitchSize::Small => (
                String::from("32px"),
                String::from("18px"),
                String::from("14px"),
            ),
            SwitchSize::Medium => (
                String::from("44px"),
                String::from("24px"),
                String::from("20px"),
            ),
            SwitchSize::Large => (
                String::from("56px"),
                String::from("32px"),
                String::from("28px"),
            ),
        };
        let bg_color: String = {
            if self.checked {
                String::from("#3b82f6")
            } else {
                String::from("#d1d5db")
            }
        };
        let thumb_pos = {
            if self.checked {
                match self.size {
                    SwitchSize::Small => String::from("16px"),
                    SwitchSize::Medium => String::from("22px"),
                    SwitchSize::Large => String::from("26px"),
                }
            } else {
                String::from("2px")
            }
        };
        let disabled_style: String = {
            if self.disabled {
                String::from(" opacity: 0.5; cursor: not-allowed;")
            } else {
                String::from(" cursor: pointer;")
            }
        };
        let disabled_attr = {
            if self.disabled {
                String::from(" disabled")
            } else {
                String::new()
            }
        };
        let mut html = String::new();
        html.push_str("<label class=\"wj-switch ");
        html.push_str(&self.class.clone());
        html.push_str("\" style=\"display: inline-flex; align-items: center; gap: 8px;");
        html.push_str(&disabled_style);
        html.push_str("\">");
        html.push_str("<input type=\"checkbox\"");
        if self.checked {
            html.push_str(" checked");
        }
        html.push_str(&disabled_attr);
        html.push_str(" style=\"position: absolute; opacity: 0; width: 0; height: 0;\">");
        html.push_str("<span style=\"position: relative; display: inline-block; width: ");
        html.push_str(&width);
        html.push_str("; height: ");
        html.push_str(&height);
        html.push_str("; background-color: ");
        html.push_str(&bg_color);
        html.push_str("; border-radius: 999px; transition: background-color 0.2s;\">");
        html.push_str("<span style=\"position: absolute; top: 2px; left: ");
        html.push_str(&thumb_pos);
        html.push_str("; width: ");
        html.push_str(&thumb_size.clone());
        html.push_str("; height: ");
        html.push_str(&thumb_size);
        html.push_str("; background-color: white; border-radius: 50%; transition: left 0.2s; box-shadow: 0 2px 4px rgba(0,0,0,0.2);\"></span>");
        html.push_str("</span>");
        if !self.label.is_empty() {
            html.push_str("<span style=\"font-size: 14px;\">");
            html.push_str(&self.label.clone());
            html.push_str("</span>");
        }
        html.push_str("</label>");
        html
    }
}
