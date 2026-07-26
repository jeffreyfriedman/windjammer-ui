#![allow(clippy::all)]
#![allow(noop_method_call)]
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
        DatePicker {
            value: "".to_string(),
            label: "".to_string(),
            name: "date".to_string(),
            required: false,
        }
    }
    #[inline]
    pub fn value(mut self, value: String) -> DatePicker {
        self.value = value;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> DatePicker {
        self.label = label;
        self
    }
    #[inline]
    pub fn name(mut self, name: String) -> DatePicker {
        self.name = name;
        self
    }
    #[inline]
    pub fn required(mut self, required: bool) -> DatePicker {
        self.required = required;
        self
    }
}

impl Renderable for DatePicker {
    #[inline]
    fn render(&self) -> String {
        let req = {
            if self.required {
                String::from(" required")
            } else {
                String::new()
            }
        };
        let label_html: String = {
            if !self.label.is_empty() {
                format!(
                    "<label class='wj-datepicker-label' for='{}'>{}</label>",
                    self.name.clone(),
                    self.label.clone()
                )
            } else {
                "".to_string()
            }
        };
        format!("<div class='wj-datepicker'>{}<input class='wj-datepicker-input' type='date' id='{}' name='{}' value='{}'{}/></div>", label_html, self.name.clone(), self.name.clone(), self.value.clone(), req)
    }
}
