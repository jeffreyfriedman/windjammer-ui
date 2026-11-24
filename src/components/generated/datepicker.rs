#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct DatePicker {
    pub value: String,
    pub label: String,
    pub name: String,
    pub required: bool,
}

impl DatePicker {
#[inline]
pub fn new() -> DatePicker {
        DatePicker { value: String::new(), label: String::new(), name: String::from("date"), required: false }
}
#[inline]
pub fn value(mut self, value: impl Into<String>) -> DatePicker {
        self.value = value.into();
        self
}
#[inline]
pub fn label(mut self, label: impl Into<String>) -> DatePicker {
        self.label = label.into();
        self
}
#[inline]
pub fn name(mut self, name: impl Into<String>) -> DatePicker {
        self.name = name.into();
        self
}
#[inline]
pub fn required(mut self, required: bool) -> DatePicker {
        self.required = required;
        self
}
}

impl Renderable for DatePicker {

fn render(&self) -> String {
        let req = {
            if self.required {
                String::from(" required")
            } else {
                String::new()
            }
        };
        let label_html = {
            if !self.label.is_empty() {
                format!("<label class='wj-datepicker-label' for='{}'>{}</label>", self.name, self.label)
            } else {
                String::new()
            }
        };
        format!("<div class='wj-datepicker'>{}<input class='wj-datepicker-input' type='date' id='{}' name='{}' value='{}'{}/></div>", label_html, self.name, self.name, self.value, req)
}
}

