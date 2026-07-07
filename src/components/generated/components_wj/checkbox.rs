#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use super::traits::RenderableVNode;
use super::vnode::VNode;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Checkbox {
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    pub size: CheckboxSize,
    pub change_handler: String,
}

impl Checkbox {
    #[inline]
    pub fn new(label: String) -> Checkbox {
        Checkbox {
            label,
            checked: false,
            disabled: false,
            size: CheckboxSize::Medium,
            change_handler: String::new(),
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
    #[inline]
    pub fn on_change(mut self, handler: String) -> Checkbox {
        self.change_handler = handler;
        self
    }
}

impl RenderableVNode for Checkbox {
    #[inline]
    fn to_vnode(&self) -> VNode {
        let size_class: String = match self.size {
            CheckboxSize::Small => String::from("wj-checkbox-sm"),
            CheckboxSize::Medium => String::from("wj-checkbox-md"),
            CheckboxSize::Large => String::from("wj-checkbox-lg"),
        };
        let mut label_node = VNode::element("label".to_string())
            .add_class("wj-checkbox")
            .add_class(&size_class.to_string())
            .child(
                VNode::input()
                    .set_type("checkbox")
                    .add_attr(
                        "checked",
                        &{
                            if self.checked {
                                String::from("true")
                            } else {
                                String::from("false")
                            }
                        }
                        .to_string(),
                    )
                    .set_disabled(self.disabled),
            )
            .child(VNode::span().add_text(self.label.clone()));
        if !self.change_handler.is_empty() {
            label_node = label_node.on_change(self.change_handler.clone());
        }
        label_node
    }
}

impl Renderable for Checkbox {
    #[inline]
    fn render(&mut self) -> String {
        let size_class: String = match self.size {
            CheckboxSize::Small => String::from("sm"),
            CheckboxSize::Medium => String::from("md"),
            CheckboxSize::Large => String::from("lg"),
        };
        let checked_attr = {
            if self.checked {
                String::from(" checked")
            } else {
                String::new()
            }
        };
        let disabled_attr = {
            if self.disabled {
                String::from(" disabled")
            } else {
                String::new()
            }
        };
        format!("<label class='wj-checkbox wj-checkbox-{}'><input type='checkbox'{}{}/><span>{}</span></label>", size_class, checked_attr, disabled_attr, self.label)
    }
}
