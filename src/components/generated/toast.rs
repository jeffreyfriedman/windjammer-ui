#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
    BottomCenter,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Toast {
    pub message: String,
    pub variant: ToastVariant,
    pub position: ToastPosition,
    pub duration: i32,
    pub show_close: bool,
}

impl Toast {
#[inline]
pub fn new(message: String) -> Toast {
        Toast { message, variant: ToastVariant::Info, position: ToastPosition::TopRight, duration: 3000_i32, show_close: true }
}
#[inline]
pub fn variant(mut self, variant: ToastVariant) -> Toast {
        self.variant = variant;
        self
}
#[inline]
pub fn position(mut self, position: ToastPosition) -> Toast {
        self.position = position;
        self
}
#[inline]
pub fn duration(mut self, duration: i32) -> Toast {
        self.duration = duration;
        self
}
#[inline]
pub fn show_close(mut self, show: bool) -> Toast {
        self.show_close = show;
        self
}
}

impl Renderable for Toast {
#[inline]
fn render(self) -> String {
        let variant_class: String = match self.variant {
            ToastVariant::Success => String::from("wj-toast-success"),
            ToastVariant::Error => String::from("wj-toast-error"),
            ToastVariant::Warning => String::from("wj-toast-warning"),
            ToastVariant::Info => String::from("wj-toast-info"),
        };
        let position_class: String = match self.position {
            ToastPosition::TopRight => String::from("wj-toast-top-right"),
            ToastPosition::TopLeft => String::from("wj-toast-top-left"),
            ToastPosition::BottomRight => String::from("wj-toast-bottom-right"),
            ToastPosition::BottomLeft => String::from("wj-toast-bottom-left"),
            ToastPosition::TopCenter => String::from("wj-toast-top-center"),
            ToastPosition::BottomCenter => String::from("wj-toast-bottom-center"),
        };
        let icon: String = match self.variant {
            ToastVariant::Success => String::from("✓"),
            ToastVariant::Error => String::from("✗"),
            ToastVariant::Warning => String::from("⚠"),
            ToastVariant::Info => String::from("ℹ"),
        };
        let close_button = {
            if self.show_close {
                String::from("<button class='wj-toast-close'>×</button>")
            } else {
                String::new()
            }
        };
        format!("<div class='wj-toast {} {}' data-duration='{}'>\n  <span class='wj-toast-icon'>{}</span>\n  <span class='wj-toast-message'>{}</span>\n  {}\n</div>", variant_class, position_class, self.duration, icon, self.message, close_button)
}
}

