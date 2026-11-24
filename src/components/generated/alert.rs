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
    pub title: String,
}

impl Alert {
#[inline]
pub fn error(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Error, title: String::new() }
}
#[inline]
pub fn warning(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Warning, title: String::new() }
}
#[inline]
pub fn info(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Info, title: String::new() }
}
#[inline]
pub fn success(message: String) -> Alert {
        Alert { message, variant: AlertVariant::Success, title: String::new() }
}
#[inline]
pub fn title(mut self, title: impl Into<String>) -> Alert {
        self.title = title.into();
        self
}
}

impl Renderable for Alert {

fn render(&self) -> String {
        let variant_class: String = match &self.variant {
            AlertVariant::Error => String::from("wj-alert-error"),
            AlertVariant::Warning => String::from("wj-alert-warning"),
            AlertVariant::Info => String::from("wj-alert-info"),
            AlertVariant::Success => String::from("wj-alert-success"),
        };
        let mark = mark_for(self.variant);
        let heading = {
            if self.title.is_empty() {
                String::new()
            } else {
                format!("<strong class='wj-alert-title'>{}</strong>", self.title)
            }
        };
        format!("<div class='wj-alert {}' role='status'><span class='wj-alert-mark'>{}</span>{} <span class='wj-alert-msg'>{}</span></div>", variant_class, mark, heading, self.message)
}
}

#[inline]
fn mark_for(variant: AlertVariant) -> String {
    match variant {
        AlertVariant::Error => String::from("Error"),
        AlertVariant::Warning => String::from("Warning"),
        AlertVariant::Info => String::from("Note"),
        AlertVariant::Success => String::from("Done"),
    }
}

