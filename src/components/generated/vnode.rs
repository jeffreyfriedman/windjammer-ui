#[allow(unused_imports)]
use super::*;

use crate::vnode_ffi;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct VNode {
    pub handle: u64,
}
impl VNode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut __bytes = Vec::with_capacity(8);
        __bytes.extend_from_slice(&self.handle.to_ne_bytes());
        __bytes
    }
}


impl VNode {
/// Create a new div element
#[inline]
pub fn div() -> VNode {
        VNode { handle: vnode_ffi::vnode_element("div") }
}
/// Create a new span element
#[inline]
pub fn span() -> VNode {
        VNode { handle: vnode_ffi::vnode_element("span") }
}
/// Create a new button element
#[inline]
pub fn button() -> VNode {
        VNode { handle: vnode_ffi::vnode_element("button") }
}
/// Create a new input element
#[inline]
pub fn input() -> VNode {
        VNode { handle: vnode_ffi::vnode_element("input") }
}
/// Create a new text node
#[inline]
pub fn text(content: &str) -> VNode {
        VNode { handle: vnode_ffi::vnode_text(content) }
}
/// Create any HTML element by tag name
#[inline]
pub fn element(tag: &str) -> VNode {
        VNode { handle: vnode_ffi::vnode_element(tag) }
}
/// Add a CSS class (takes owned String)
#[inline]
pub fn add_class(self, class: &str) -> VNode {
        vnode_ffi::vnode_class(self.handle, class);
        self
}
/// Add inline style (takes owned String)
#[inline]
pub fn add_style(self, style: &str) -> VNode {
        vnode_ffi::vnode_style(self.handle, style);
        self
}
/// Add an attribute (takes owned Strings)
#[inline]
pub fn add_attr(self, name: &str, value: &str) -> VNode {
        vnode_ffi::vnode_attr(self.handle, name, value);
        self
}
/// Add a child VNode
#[inline]
pub fn child(self, child: VNode) -> VNode {
        vnode_ffi::vnode_child(self.handle, child.handle);
        self
}
/// Add text content (takes owned String)
#[inline]
pub fn add_text(self, text: &str) -> VNode {
        let text_node = VNode::text(text);
        vnode_ffi::vnode_child(self.handle, text_node.handle);
        self
}
/// Set id attribute
#[inline]
pub fn set_id(self, id: &str) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "id", id);
        self
}
/// Set placeholder (for inputs)
#[inline]
pub fn set_placeholder(self, text: &str) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "placeholder", text);
        self
}
/// Set type attribute (for inputs/buttons)
#[inline]
pub fn set_type(self, t: &str) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "type", t);
        self
}
/// Set value attribute
#[inline]
pub fn set_value(self, v: &str) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "value", v);
        self
}
/// Set disabled attribute
#[inline]
pub fn set_disabled(self, d: bool) -> VNode {
        if d {
            vnode_ffi::vnode_attr(self.handle, "disabled", "true");
        }
        self
}
/// Get the raw handle (for interop with Rust code)
#[inline]
pub fn raw_handle(&self) -> u64 {
        self.handle
}
}

/// Create a container with padding
#[inline]
pub fn container() -> VNode {
    VNode::div().add_class("wj-container").add_style("padding: 16px")
}

/// Create a flex row
#[inline]
pub fn row() -> VNode {
    VNode::div().add_style("display: flex; flex-direction: row; gap: 8px")
}

/// Create a flex column
#[inline]
pub fn column() -> VNode {
    VNode::div().add_style("display: flex; flex-direction: column; gap: 8px")
}

/// Create a spacer element
#[inline]
pub fn spacer() -> VNode {
    VNode::div().add_style("flex: 1")
}

/// Create a horizontal divider
#[inline]
pub fn divider() -> VNode {
    VNode::element("hr").add_style("border: 0; border-top: 1px solid #333; margin: 8px 0")
}

