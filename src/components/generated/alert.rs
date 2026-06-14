#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AlertVariant {
    Error,
    Warning,
    Info,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Alert {
    pub message: String,
    pub variant: AlertVariant,
}

impl Alert {
#[inline]
pub fn error(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Error }
}
#[inline]
pub fn warning(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Warning }
}
#[inline]
pub fn info(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Info }
}
#[inline]
pub fn success(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Success }
}
}

impl Renderable for Alert {
#[inline]
fn render(self) -> String {
        let variant_class: String = match self.variant {
            AlertVariant::Error => String::from("wj-alert-error"),
            AlertVariant::Warning => String::from("wj-alert-warning"),
            AlertVariant::Info => String::from("wj-alert-info"),
            AlertVariant::Success => String::from("wj-alert-success"),
        };
        let icon: String = match self.variant {
            AlertVariant::Error => String::from("❌"),
            AlertVariant::Warning => String::from("⚠️"),
            AlertVariant::Info => String::from("ℹ️"),
            AlertVariant::Success => String::from("✅"),
        };
        format!("<div class='wj-alert {}'>{} {}</div>", variant_class, icon, self.message)
}
}

