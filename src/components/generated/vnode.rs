#![allow(clippy::all)]
#![allow(noop_method_call)]
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
        VNode {
            handle: (unsafe {
                vnode_ffi::vnode_element(windjammer_runtime::ffi::string_to_ffi("div".to_string()))
            }),
        }
    }
    /// Create a new span element
    #[inline]
    pub fn span() -> VNode {
        VNode {
            handle: (unsafe {
                vnode_ffi::vnode_element(windjammer_runtime::ffi::string_to_ffi("span".to_string()))
            }),
        }
    }
    /// Create a new button element
    #[inline]
    pub fn button() -> VNode {
        VNode {
            handle: (unsafe {
                vnode_ffi::vnode_element(windjammer_runtime::ffi::string_to_ffi(
                    "button".to_string(),
                ))
            }),
        }
    }
    /// Create a new input element
    #[inline]
    pub fn input() -> VNode {
        VNode {
            handle: (unsafe {
                vnode_ffi::vnode_element(windjammer_runtime::ffi::string_to_ffi(
                    "input".to_string(),
                ))
            }),
        }
    }
    /// Create a new text node
    #[inline]
    pub fn text(content: &str) -> VNode {
        VNode {
            handle: (unsafe {
                vnode_ffi::vnode_text(windjammer_runtime::ffi::string_to_ffi(content.to_string()))
            }),
        }
    }
    /// Create any HTML element by tag name
    #[inline]
    pub fn element(tag: &str) -> VNode {
        VNode {
            handle: (unsafe {
                vnode_ffi::vnode_element(windjammer_runtime::ffi::string_to_ffi(tag.to_string()))
            }),
        }
    }
    /// Add a CSS class (takes owned String)
    #[inline]
    pub fn add_class(self, class: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_class(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi(class.to_string()),
            )
        });
        self
    }
    /// Add inline style (takes owned String)
    #[inline]
    pub fn add_style(self, style: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_style(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi(style.to_string()),
            )
        });
        self
    }
    /// Add an attribute (takes owned Strings)
    #[inline]
    pub fn add_attr(self, name: &str, value: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_attr(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi(name.to_string()),
                windjammer_runtime::ffi::string_to_ffi(value.to_string()),
            )
        });
        self
    }
    /// Add a child VNode
    #[inline]
    pub fn child(self, child: VNode) -> VNode {
        (unsafe { vnode_ffi::vnode_child(self.handle, child.handle) });
        self
    }
    /// Add text content (takes owned String)
    #[inline]
    pub fn add_text(self, text: &str) -> VNode {
        let text_node = VNode::text(text);
        (unsafe { vnode_ffi::vnode_child(self.handle, text_node.handle) });
        self
    }
    /// Set id attribute
    #[inline]
    pub fn set_id(self, id: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_attr(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi("id".to_string()),
                windjammer_runtime::ffi::string_to_ffi(id.to_string()),
            )
        });
        self
    }
    /// Set placeholder (for inputs)
    #[inline]
    pub fn set_placeholder(self, text: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_attr(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi("placeholder".to_string()),
                windjammer_runtime::ffi::string_to_ffi(text.to_string()),
            )
        });
        self
    }
    /// Set type attribute (for inputs/buttons)
    #[inline]
    pub fn set_type(self, t: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_attr(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi("type".to_string()),
                windjammer_runtime::ffi::string_to_ffi(t.to_string()),
            )
        });
        self
    }
    /// Set value attribute
    #[inline]
    pub fn set_value(self, v: &str) -> VNode {
        (unsafe {
            vnode_ffi::vnode_attr(
                self.handle,
                windjammer_runtime::ffi::string_to_ffi("value".to_string()),
                windjammer_runtime::ffi::string_to_ffi(v.to_string()),
            )
        });
        self
    }
    /// Set disabled attribute
    #[inline]
    pub fn set_disabled(self, d: bool) -> VNode {
        if d {
            (unsafe {
                vnode_ffi::vnode_attr(
                    self.handle,
                    windjammer_runtime::ffi::string_to_ffi("disabled".to_string()),
                    windjammer_runtime::ffi::string_to_ffi("true".to_string()),
                )
            });
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
    VNode::div()
        .add_class("wj-container")
        .add_style("padding: 16px")
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
