//! VNode FFI - Functions for constructing VNodes from Windjammer code
//!
//! This module provides a stable FFI interface that Windjammer components can use
//! to construct VNodes. This enables cross-platform UI rendering:
//! - Web: VNode → HTML (via simple_renderer)
//! - Desktop: VNode → egui (via desktop_renderer)
//!
//! The design uses opaque handles (u64) to reference VNodes stored in a thread-local
//! registry. This avoids complex memory management across the FFI boundary.

use crate::simple_vnode::{VAttr, VNode};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// Registry of VNodes created via FFI
    static VNODE_REGISTRY: RefCell<VNodeRegistry> = RefCell::new(VNodeRegistry::new());
}

/// Registry for storing VNodes with handles
struct VNodeRegistry {
    nodes: HashMap<u64, VNode>,
    next_handle: u64,
}

impl VNodeRegistry {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_handle: 1,
        }
    }

    fn insert(&mut self, node: VNode) -> u64 {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.nodes.insert(handle, node);
        handle
    }

    fn get(&self, handle: u64) -> Option<&VNode> {
        self.nodes.get(&handle)
    }

    fn take(&mut self, handle: u64) -> Option<VNode> {
        self.nodes.remove(&handle)
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.next_handle = 1;
    }
}

// ============================================================================
// VNode Creation Functions (called from Windjammer)
// ============================================================================

/// Create a new element VNode with a tag name
/// Returns a handle to the VNode
#[inline]
pub fn vnode_element(tag: impl AsRef<str>) -> u64 {
    let node = VNode::Element {
        tag: tag.as_ref().to_string(),
        attrs: Vec::new(),
        children: Vec::new(),
    };
    VNODE_REGISTRY.with(|registry| registry.borrow_mut().insert(node))
}

/// Create a text VNode
/// Returns a handle to the VNode
#[inline]
pub fn vnode_text(content: impl AsRef<str>) -> u64 {
    let node = VNode::Text(content.as_ref().to_string());
    VNODE_REGISTRY.with(|registry| registry.borrow_mut().insert(node))
}

/// Add a static attribute to a VNode
#[inline]
pub fn vnode_attr(handle: u64, name: impl AsRef<str>, value: impl AsRef<str>) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            attrs.push((
                name.as_ref().to_string(),
                VAttr::Static(value.as_ref().to_string()),
            ));
        }
    });
}

/// Add a child VNode to a parent VNode
/// The child is consumed (moved from registry to parent's children)
#[inline]
pub fn vnode_child(parent_handle: u64, child_handle: u64) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        // Take the child from the registry
        if let Some(child) = reg.nodes.remove(&child_handle) {
            // Add it to the parent's children
            if let Some(VNode::Element { children, .. }) = reg.nodes.get_mut(&parent_handle) {
                children.push(child);
            }
        }
    });
}

/// Add a class to a VNode (convenience for class attribute)
#[inline]
pub fn vnode_class(handle: u64, class: impl AsRef<str>) {
    let class = class.as_ref();
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            // Find existing class attribute or create new one
            let mut found = false;
            for (name, value) in attrs.iter_mut() {
                if name == "class" {
                    if let VAttr::Static(existing) = value {
                        *existing = format!("{} {}", existing, class);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                attrs.push(("class".to_string(), VAttr::Static(class.to_string())));
            }
        }
    });
}

/// Add inline style to a VNode
#[inline]
pub fn vnode_style(handle: u64, style: impl AsRef<str>) {
    let style = style.as_ref();
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            // Find existing style attribute or create new one
            let mut found = false;
            for (name, value) in attrs.iter_mut() {
                if name == "style" {
                    if let VAttr::Static(existing) = value {
                        *existing = format!("{}; {}", existing, style);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                attrs.push(("style".to_string(), VAttr::Static(style.to_string())));
            }
        }
    });
}

/// Attach an on_click handler name to a VNode
/// The renderer looks up this name in the event dispatcher to find the callback
#[inline]
pub fn vnode_on_click(handle: u64, handler_name: impl AsRef<str>) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            attrs.push((
                "on_click".to_string(),
                VAttr::Dynamic(handler_name.as_ref().to_string()),
            ));
        }
    });
}

/// Attach an on_change handler name to a VNode
/// The renderer looks up this name in the event dispatcher to find the callback
#[inline]
pub fn vnode_on_change(handle: u64, handler_name: impl AsRef<str>) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            attrs.push((
                "on_change".to_string(),
                VAttr::Dynamic(handler_name.as_ref().to_string()),
            ));
        }
    });
}

/// Attach an on_input handler name to a VNode
#[inline]
pub fn vnode_on_input(handle: u64, handler_name: impl AsRef<str>) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            attrs.push((
                "on_input".to_string(),
                VAttr::Dynamic(handler_name.as_ref().to_string()),
            ));
        }
    });
}

/// Get the VNode for a handle and remove it from the registry
/// This is called when the VNode tree is complete and ready to render
#[inline]
pub fn vnode_take(handle: u64) -> Option<VNode> {
    VNODE_REGISTRY.with(|registry| registry.borrow_mut().take(handle))
}

/// Get a reference to a VNode (without removing it)
#[inline]
pub fn vnode_get(handle: u64) -> Option<VNode> {
    VNODE_REGISTRY.with(|registry| registry.borrow().get(handle).cloned())
}

/// Clear all VNodes from the registry (for cleanup between renders)
#[inline]
pub fn vnode_clear() {
    VNODE_REGISTRY.with(|registry| registry.borrow_mut().clear());
}

// ============================================================================
// Callback FFI — function pointer event handlers
// ============================================================================

thread_local! {
    static CALLBACK_REGISTRY: RefCell<CallbackRegistry> = RefCell::new(CallbackRegistry::new());
}

struct CallbackRegistry {
    callbacks: HashMap<u64, Box<dyn Fn()>>,
    next_handle: u64,
}

impl CallbackRegistry {
    fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            next_handle: 1,
        }
    }
}

/// Register a callback function and return a handle.
/// The callback is stored and can be invoked later by handle.
#[inline]
pub fn callback_register(cb: impl Fn() + 'static) -> u64 {
    CALLBACK_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        let handle = reg.next_handle;
        reg.next_handle += 1;
        reg.callbacks.insert(handle, Box::new(cb));
        handle
    })
}

/// Invoke a callback by handle. Returns true if the callback was found and called.
#[inline]
pub fn callback_invoke(handle: u64) -> bool {
    CALLBACK_REGISTRY.with(|registry| {
        let reg = registry.borrow();
        if let Some(cb) = reg.callbacks.get(&handle) {
            cb();
            true
        } else {
            false
        }
    })
}

/// Remove a callback from the registry
#[inline]
pub fn callback_remove(handle: u64) {
    CALLBACK_REGISTRY.with(|registry| {
        registry.borrow_mut().callbacks.remove(&handle);
    });
}

/// Clear all callbacks
#[inline]
pub fn callback_clear() {
    CALLBACK_REGISTRY.with(|registry| {
        registry.borrow_mut().callbacks.clear();
    });
}

/// Attach a callback handle to a VNode's on_click event.
/// The callback handle is stored as a dynamic attribute prefixed with "cb:"
/// so the renderer can distinguish it from string-based handler names.
#[inline]
pub fn vnode_on_click_cb(handle: u64, callback_handle: u64) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            attrs.push((
                "on_click".to_string(),
                VAttr::Dynamic(format!("cb:{}", callback_handle)),
            ));
        }
    });
}

/// Attach a callback handle to a VNode's on_change event.
#[inline]
pub fn vnode_on_change_cb(handle: u64, callback_handle: u64) {
    VNODE_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        if let Some(VNode::Element { attrs, .. }) = reg.nodes.get_mut(&handle) {
            attrs.push((
                "on_change".to_string(),
                VAttr::Dynamic(format!("cb:{}", callback_handle)),
            ));
        }
    });
}

// ============================================================================
// Convenience builders for common elements
// ============================================================================

/// Create a div element with optional class and style
#[inline]
pub fn vnode_div(class: &str, style: &str) -> u64 {
    let handle = vnode_element("div");
    if !class.is_empty() {
        vnode_class(handle, class);
    }
    if !style.is_empty() {
        vnode_style(handle, style);
    }
    handle
}

/// Create a span element with text content
#[inline]
pub fn vnode_span(text: &str, class: &str, style: &str) -> u64 {
    let handle = vnode_element("span");
    if !class.is_empty() {
        vnode_class(handle, class);
    }
    if !style.is_empty() {
        vnode_style(handle, style);
    }
    let text_handle = vnode_text(text);
    vnode_child(handle, text_handle);
    handle
}

/// Create a button element
#[inline]
pub fn vnode_button(label: &str, class: &str, style: &str) -> u64 {
    let handle = vnode_element("button");
    if !class.is_empty() {
        vnode_class(handle, class);
    }
    if !style.is_empty() {
        vnode_style(handle, style);
    }
    let text_handle = vnode_text(label);
    vnode_child(handle, text_handle);
    handle
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_element() {
        vnode_clear();
        let handle = vnode_element("div");
        assert!(handle > 0);

        let node = vnode_take(handle);
        assert!(node.is_some());
        if let Some(VNode::Element { tag, .. }) = node {
            assert_eq!(tag, "div");
        }
    }

    #[test]
    fn test_create_text() {
        vnode_clear();
        let handle = vnode_text("Hello, World!");

        let node = vnode_take(handle);
        assert!(node.is_some());
        if let Some(VNode::Text(content)) = node {
            assert_eq!(content, "Hello, World!");
        }
    }

    #[test]
    fn test_add_child() {
        vnode_clear();
        let parent = vnode_element("div");
        let child = vnode_text("Child text");

        vnode_child(parent, child);

        let node = vnode_take(parent);
        if let Some(VNode::Element { children, .. }) = node {
            assert_eq!(children.len(), 1);
            if let VNode::Text(content) = &children[0] {
                assert_eq!(content, "Child text");
            }
        }
    }

    #[test]
    fn test_add_class() {
        vnode_clear();
        let handle = vnode_element("button");
        vnode_class(handle, "wj-button");
        vnode_class(handle, "wj-button-primary");

        let node = vnode_take(handle);
        if let Some(VNode::Element { attrs, .. }) = node {
            let class_attr = attrs.iter().find(|(n, _)| n == "class");
            assert!(class_attr.is_some());
            if let Some((_, VAttr::Static(classes))) = class_attr {
                assert!(classes.contains("wj-button"));
                assert!(classes.contains("wj-button-primary"));
            }
        }
    }

    #[test]
    fn test_on_click_event() {
        vnode_clear();
        let handle = vnode_element("button");
        vnode_on_click(handle, "increment");

        let node = vnode_take(handle);
        if let Some(VNode::Element { attrs, .. }) = node {
            let event_attr = attrs
                .iter()
                .find(|(n, _)| n == "on_click");
            assert!(event_attr.is_some());
            if let Some((_, VAttr::Dynamic(handler_name))) = event_attr {
                assert_eq!(handler_name, "increment");
            }
        }
    }

    #[test]
    fn test_on_change_event() {
        vnode_clear();
        let handle = vnode_element("input");
        vnode_on_change(handle, "update_value");

        let node = vnode_take(handle);
        if let Some(VNode::Element { attrs, .. }) = node {
            let event_attr = attrs
                .iter()
                .find(|(n, _)| n == "on_change");
            assert!(event_attr.is_some());
            if let Some((_, VAttr::Dynamic(handler_name))) = event_attr {
                assert_eq!(handler_name, "update_value");
            }
        }
    }

    #[test]
    fn test_callback_register_and_invoke() {
        callback_clear();
        let handle = callback_register(|| {});
        assert!(handle > 0);
        assert!(callback_invoke(handle));
        assert!(!callback_invoke(9999)); // nonexistent
        callback_remove(handle);
        assert!(!callback_invoke(handle)); // removed
    }

    #[test]
    fn test_vnode_on_click_cb() {
        vnode_clear();
        callback_clear();
        let btn = vnode_element("button");
        let cb = callback_register(|| {});
        vnode_on_click_cb(btn, cb);

        let node = vnode_take(btn);
        if let Some(VNode::Element { attrs, .. }) = node {
            let event_attr = attrs.iter().find(|(n, _)| n == "on_click");
            assert!(event_attr.is_some());
            if let Some((_, VAttr::Dynamic(val))) = event_attr {
                assert!(val.starts_with("cb:"));
            }
        }
    }

    #[test]
    fn test_convenience_builders() {
        vnode_clear();
        let button = vnode_button("Click me", "btn", "color: red");

        let node = vnode_take(button);
        if let Some(VNode::Element {
            tag,
            children,
            attrs,
            ..
        }) = node
        {
            assert_eq!(tag, "button");
            assert_eq!(children.len(), 1);

            // Check class
            let has_class = attrs
                .iter()
                .any(|(n, v)| n == "class" && matches!(v, VAttr::Static(s) if s.contains("btn")));
            assert!(has_class);
        }
    }
}
