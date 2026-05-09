#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Input {
    pub value: String,
    pub placeholder: String,
    pub input_type: String,
}

impl Input {
    #[inline]
    pub fn new() -> Input {
        Input {
            value: "".to_string(),
            placeholder: "".to_string(),
            input_type: "text".to_string(),
        }
    }
    #[inline]
    pub fn value(mut self, value: String) -> Input {
        self.value = value;
        self
    }
    #[inline]
    pub fn placeholder(mut self, placeholder: String) -> Input {
        self.placeholder = placeholder;
        self
    }
    #[inline]
    pub fn input_type(mut self, input_type: String) -> Input {
        self.input_type = input_type;
        self
    }
}

impl Renderable for Input {
    #[inline]
    fn render(&self) -> String {
        format!(
            "<input class='wj-input' type='{}' value='{}' placeholder='{}'/>",
            self.input_type.clone(),
            self.value.clone(),
            self.placeholder.clone()
        )
    }
}
