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
        RadioOption { value, label, disabled: false }
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
        RadioGroup { name, options: Vec::new(), selected: String::new() }
}
#[inline]
pub fn option(mut self, option: RadioOption) -> RadioGroup {
        self.options.push(option);
        self
}
#[inline]
pub fn selected(mut self, value: impl Into<String>) -> RadioGroup {
        self.selected = value.into();
        self
}
}

impl Renderable for RadioGroup {

fn render(&self) -> String {
        let mut html = format!("<div class='wj-radio-group' data-name='{}'>", self.name);
        let mut i = 0_usize;
        while i < self.options.len() {
            let opt = &self.options[i];
            let checked_attr = {
                if opt.value == self.selected {
                    String::from(" checked")
                } else {
                    String::new()
                }
            };
            let disabled_attr = {
                if opt.disabled {
                    String::from(" disabled")
                } else {
                    String::new()
                }
            };
            html = format!("{}<label class='wj-radio'><input type='radio' name='{}' value='{}'{}{}><span>{}</span></label>", html, self.name, opt.value, checked_attr, disabled_attr, opt.label);
            i += 1;
        }
        format!("{}</div>", html)
}
}

