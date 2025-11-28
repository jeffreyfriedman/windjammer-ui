#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

pub struct RadioOption {
    value: String,
    label: String,
    disabled: bool,
}

impl RadioOption {
    #[inline]
    pub fn new(value: String, label: String) -> RadioOption {
        RadioOption {
            value,
            label,
            disabled: false,
        }
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> RadioOption {
        self.disabled = disabled;
        self
    }
}

pub struct RadioGroup {
    name: String,
    options: Vec<RadioOption>,
    selected: String,
}

impl RadioGroup {
    #[inline]
    pub fn new(name: String) -> RadioGroup {
        RadioGroup {
            name,
            options: Vec::new(),
            selected: "".to_string(),
        }
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
    fn render(self) -> String {
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(
                &mut __s,
                "<div class='wj-radio-group' data-name='{}'>",
                self.name
            )
            .unwrap();
            __s
        };
        let mut i = 0;
        while i < self.options.len() {
            let opt = &self.options[i];
            let checked_attr = {
                if opt.value == self.selected {
                    " checked"
                } else {
                    ""
                }
            };
            let disabled_attr = {
                if opt.disabled {
                    " disabled"
                } else {
                    ""
                }
            };
            html = {
                let mut __s = String::with_capacity(64);
                write!(&mut __s, "{}<label class='wj-radio'><input type='radio' name='{}' value='{}'{}{}><span>{}</span></label>", html, self.name, opt.value, checked_attr, disabled_attr, opt.label).unwrap();
                __s
            };
            i += 1;
        }
        format!("{}</div>", html)
    }
}
