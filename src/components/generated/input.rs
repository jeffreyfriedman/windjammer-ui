#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use super::traits::RenderableVNode;
use super::vnode::VNode;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Input {
    pub value: String,
    pub placeholder: String,
    pub input_type: String,
    pub change_handler: String,
    pub input_handler: String,
}

impl Input {
    #[inline]
    pub fn new() -> Input {
        Input {
            value: "".to_string(),
            placeholder: "".to_string(),
            input_type: "text".to_string(),
            change_handler: String::new(),
            input_handler: String::new(),
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
    #[inline]
    pub fn on_change(mut self, handler: String) -> Input {
        self.change_handler = handler;
        self
    }
    #[inline]
    pub fn on_input(mut self, handler: String) -> Input {
        self.input_handler = handler;
        self
    }
}

impl RenderableVNode for Input {
    #[inline]
    fn to_vnode(&self) -> VNode {
        let mut node = VNode::input()
            .add_class("wj-input")
            .set_type(self.input_type.as_str())
            .set_value(self.value.as_str())
            .set_placeholder(self.placeholder.as_str());
        if !self.change_handler.is_empty() {
            node = node.on_change(self.change_handler.as_str());
        }
        if !self.input_handler.is_empty() {
            node = node.on_input(self.input_handler.as_str());
        }
        node
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
