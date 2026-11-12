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
        Input { value: String::new(), placeholder: String::new(), input_type: String::from("text"), change_handler: String::new(), input_handler: String::new() }
}
#[inline]
pub fn value(mut self, value: impl Into<String>) -> Input {
        self.value = value.into();
        self
}
#[inline]
pub fn placeholder(mut self, placeholder: impl Into<String>) -> Input {
        self.placeholder = placeholder.into();
        self
}
#[inline]
pub fn input_type(mut self, input_type: impl Into<String>) -> Input {
        self.input_type = input_type.into();
        self
}
#[inline]
pub fn on_change(mut self, handler: impl Into<String>) -> Input {
        self.change_handler = handler.into();
        self
}
#[inline]
pub fn on_input(mut self, handler: impl Into<String>) -> Input {
        self.input_handler = handler.into();
        self
}
}

impl RenderableVNode for Input {
#[inline]
fn to_vnode(&self) -> VNode {
        let mut node = VNode::input().add_class("wj-input".to_string()).set_type(self.input_type.clone()).set_value(self.value.clone()).set_placeholder(self.placeholder.clone());
        if !self.change_handler.is_empty() {
            node = node.on_change(self.change_handler.clone());
        }
        if !self.input_handler.is_empty() {
            node = node.on_input(self.input_handler.clone());
        }
        node
}
}

impl Renderable for Input {
#[inline]
fn render(&self) -> String {
        format!("<input class='wj-input' type='{}' value='{}' placeholder='{}'/>", self.input_type, self.value, self.placeholder)
}
}

