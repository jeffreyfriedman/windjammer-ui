#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
    BottomCenter,
}

pub struct Toast {
    message: String,
    variant: ToastVariant,
    position: ToastPosition,
    duration: i32,
    show_close: bool,
}

impl Toast {
    #[inline]
    pub fn new(message: String) -> Toast {
        Toast {
            message,
            variant: ToastVariant::Info,
            position: ToastPosition::TopRight,
            duration: 3000,
            show_close: true,
        }
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
        let variant_class = match self.variant {
            ToastVariant::Success => "wj-toast-success",
            ToastVariant::Error => "wj-toast-error",
            ToastVariant::Warning => "wj-toast-warning",
            ToastVariant::Info => "wj-toast-info",
        };
        let position_class = match self.position {
            ToastPosition::TopRight => "wj-toast-top-right",
            ToastPosition::TopLeft => "wj-toast-top-left",
            ToastPosition::BottomRight => "wj-toast-bottom-right",
            ToastPosition::BottomLeft => "wj-toast-bottom-left",
            ToastPosition::TopCenter => "wj-toast-top-center",
            ToastPosition::BottomCenter => "wj-toast-bottom-center",
        };
        let icon = match self.variant {
            ToastVariant::Success => "✓",
            ToastVariant::Error => "✗",
            ToastVariant::Warning => "⚠",
            ToastVariant::Info => "ℹ",
        };
        let close_button = {
            if self.show_close {
                "<button class='wj-toast-close'>×</button>"
            } else {
                ""
            }
        };
        format!(
            "<div class='wj-toast {} {}' data-duration='{}'>
  <span class='wj-toast-icon'>{}</span>
  <span class='wj-toast-message'>{}</span>
  {}
</div>",
            variant_class, position_class, self.duration, icon, self.message, close_button
        )
    }
}
