#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ColorPicker {
    pub value: String,
    pub label: String,
}

impl ColorPicker {
#[inline]
pub fn new() -> ColorPicker {
        ColorPicker { value: String::from("#000000"), label: String::new() }
}
#[inline]
pub fn value(mut self, value: impl Into<String>) -> ColorPicker {
        self.value = value.into();
        self
}
#[inline]
pub fn label(mut self, label: impl Into<String>) -> ColorPicker {
        self.label = label.into();
        self
}
}

impl Renderable for ColorPicker {
#[inline]
fn render(&self) -> String {
        let label_html = {
            if self.label != "" {
                format!("<label>{}</label>", self.label)
            } else {
                String::new()
            }
        };
        format!("<div class='wj-color-picker'>\n  {}\n  <input type='color' value='{}' class='wj-color-input'>\n  <span class='wj-color-value'>{}</span>\n</div>", label_html, self.value, self.value)
}
}

