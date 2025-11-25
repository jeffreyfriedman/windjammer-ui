//! Input component
use crate::event_handler::StringEventHandler;
use crate::simple_vnode::{VAttr, VNode};

pub struct Input {
    pub value: String,
    pub placeholder: String,
    pub on_change: Option<StringEventHandler>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            on_change: None,
        }
    }

    /// Set the input value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the change handler
    pub fn on_change<F: FnMut(String) + 'static>(mut self, handler: F) -> Self {
        use std::cell::RefCell;
        use std::rc::Rc;
        self.on_change = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn render(&self) -> VNode {
        let attrs = vec![
            ("class".to_string(), VAttr::Static("wj-input".to_string())),
            ("value".to_string(), VAttr::Static(self.value.clone())),
            (
                "placeholder".to_string(),
                VAttr::Static(self.placeholder.clone()),
            ),
        ];

        // Note: Event handling with parameters (on_change with String) needs
        // to be implemented at the renderer level. For now, we just render
        // the input without the event handler. The desktop renderer can
        // connect this using egui's TextEdit widget directly.

        VNode::Element {
            tag: "input".to_string(),
            attrs,
            children: vec![],
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
