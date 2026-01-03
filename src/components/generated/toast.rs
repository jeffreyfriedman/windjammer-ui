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
        Toast { message, variant: ToastVariant::Info, position: ToastPosition::TopRight, duration: 3000, show_close: true }
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
            ToastVariant::Success => "wj-toast-success".to_string(),
            ToastVariant::Error => "wj-toast-error".to_string(),
            ToastVariant::Warning => "wj-toast-warning".to_string(),
            ToastVariant::Info => "wj-toast-info".to_string(),
        };
        let position_class = match self.position {
            ToastPosition::TopRight => "wj-toast-top-right".to_string(),
            ToastPosition::TopLeft => "wj-toast-top-left".to_string(),
            ToastPosition::BottomRight => "wj-toast-bottom-right".to_string(),
            ToastPosition::BottomLeft => "wj-toast-bottom-left".to_string(),
            ToastPosition::TopCenter => "wj-toast-top-center".to_string(),
            ToastPosition::BottomCenter => "wj-toast-bottom-center".to_string(),
        };
        let icon = match self.variant {
            ToastVariant::Success => "✓".to_string(),
            ToastVariant::Error => "✗".to_string(),
            ToastVariant::Warning => "⚠".to_string(),
            ToastVariant::Info => "ℹ".to_string(),
        };
        let close_button = {
            if self.show_close {
                "<button class='wj-toast-close'>×</button>".to_string()
            } else {
                "".to_string()
            }
        };
        format!("<div class='wj-toast {} {}' data-duration='{}'>
  <span class='wj-toast-icon'>{}</span>
  <span class='wj-toast-message'>{}</span>
  {}
</div>", variant_class, position_class, self.duration, icon, self.message, close_button)
}
}

