use std::fmt::Write;
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl RadioOption {
#[inline]
pub fn new(value: String, label: String) -> RadioOption {
        RadioOption { value: value.to_string(), label: label.to_string(), disabled: false }
}
#[inline]
pub fn disabled(mut self, disabled: bool) -> RadioOption {
        self.disabled = disabled;
        self
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct RadioGroup {
    pub name: String,
    pub options: Vec<RadioOption>,
    pub selected: String,
}

impl RadioGroup {
#[inline]
pub fn new(name: String) -> RadioGroup {
        RadioGroup { name: name.to_string(), options: Vec::new(), selected: "".to_string() }
}
#[inline]
pub fn option(mut self, option: RadioOption) -> RadioGroup {
        self.options.push(option);
        self
}
#[inline]
pub fn selected(mut self, value: String) -> RadioGroup {
        self.selected = value;
        self
}
}

impl Renderable for RadioGroup {
#[inline]
fn render(&self) -> String {
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "<div class='wj-radio-group' data-name='{}'>", self.name.clone()).unwrap();
            __s
        };
        let mut i = 0;
        while i < self.options.len() {
            let opt = &self.options[i];
            let checked_attr = {
                if opt.value == self.selected {
                    " checked".to_string()
                } else {
                    "".to_string()
                }
            };
            let disabled_attr = {
                if opt.disabled {
                    " disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            html = format!("{}<label class='wj-radio'><input type='radio' name='{}' value='{}'{}{}><span>{}</span></label>", html, self.name.clone(), opt.value, checked_attr, disabled_attr, opt.label);
            i += 1;
        }
        format!("{}</div>", html)
}
}

