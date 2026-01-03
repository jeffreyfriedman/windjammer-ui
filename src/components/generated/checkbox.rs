#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Checkbox {
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    pub size: CheckboxSize,
}

impl Checkbox {
    #[inline]
    pub fn new(label: String) -> Checkbox {
        Checkbox {
            label,
            checked: false,
            disabled: false,
            size: CheckboxSize::Medium,
        }
    }
    #[inline]
    pub fn checked(mut self, checked: bool) -> Checkbox {
        self.checked = checked;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Checkbox {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn size(mut self, size: CheckboxSize) -> Checkbox {
        self.size = size;
        self
    }
}

impl Renderable for Checkbox {
    #[inline]
    fn render(self) -> String {
        let size_class = match self.size {
            CheckboxSize::Small => "sm".to_string(),
            CheckboxSize::Medium => "md".to_string(),
            CheckboxSize::Large => "lg".to_string(),
        };
        let checked_attr = {
            if self.checked {
                " checked".to_string()
            } else {
                "".to_string()
            }
        };
        let disabled_attr = {
            if self.disabled {
                " disabled".to_string()
            } else {
                "".to_string()
            }
        };
        format!("<label class='wj-checkbox wj-checkbox-{}'><input type='checkbox'{}{}/><span>{}</span></label>", size_class, checked_attr, disabled_attr, self.label)
    }
}
