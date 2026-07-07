#![allow(clippy::all)]
#![allow(noop_method_call)]
use crate::vnode_ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VNode {
    pub handle: u64,
}

impl VNode {
    #[inline]
    pub fn div() -> VNode {
        VNode {
            handle: vnode_ffi::vnode_element("div"),
        }
    }
    #[inline]
    pub fn span() -> VNode {
        VNode {
            handle: vnode_ffi::vnode_element("span"),
        }
    }
    #[inline]
    pub fn button() -> VNode {
        VNode {
            handle: vnode_ffi::vnode_element("button"),
        }
    }
    #[inline]
    pub fn input() -> VNode {
        VNode {
            handle: vnode_ffi::vnode_element("input"),
        }
    }
    #[inline]
    pub fn text(content: impl AsRef<str>) -> VNode {
        VNode {
            handle: vnode_ffi::vnode_text(content.as_ref()),
        }
    }
    #[inline]
    pub fn element(tag: impl AsRef<str>) -> VNode {
        VNode {
            handle: vnode_ffi::vnode_element(tag.as_ref()),
        }
    }
    #[inline]
    pub fn add_class(self, class: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_class(self.handle, class.as_ref());
        self
    }
    #[inline]
    pub fn add_style(self, style: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_style(self.handle, style.as_ref());
        self
    }
    #[inline]
    pub fn add_attr(self, name: impl AsRef<str>, value: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_attr(self.handle, name.as_ref(), value.as_ref());
        self
    }
    #[inline]
    pub fn child(self, child: VNode) -> VNode {
        vnode_ffi::vnode_child(self.handle, child.handle);
        self
    }
    #[inline]
    pub fn add_text(self, text: impl AsRef<str>) -> VNode {
        let text_node = VNode::text(text.as_ref());
        vnode_ffi::vnode_child(self.handle, text_node.handle);
        self
    }
    #[inline]
    pub fn set_id(self, id: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "id", id.as_ref());
        self
    }
    #[inline]
    pub fn set_placeholder(self, text: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "placeholder", text.as_ref());
        self
    }
    #[inline]
    pub fn set_type(self, t: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "type", t.as_ref());
        self
    }
    #[inline]
    pub fn set_value(self, v: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_attr(self.handle, "value", v.as_ref());
        self
    }
    #[inline]
    pub fn set_disabled(self, d: bool) -> VNode {
        if d {
            vnode_ffi::vnode_attr(self.handle, "disabled", "true")
        }
        self
    }
    #[inline]
    pub fn on_click(self, handler_name: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_on_click(self.handle, handler_name.as_ref());
        self
    }
    #[inline]
    pub fn on_change(self, handler_name: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_on_change(self.handle, handler_name.as_ref());
        self
    }
    #[inline]
    pub fn on_input(self, handler_name: impl AsRef<str>) -> VNode {
        vnode_ffi::vnode_on_input(self.handle, handler_name.as_ref());
        self
    }
    #[inline]
    pub fn raw_handle(&self) -> u64 {
        self.handle
    }
}

#[inline]
pub fn container() -> VNode {
    VNode::div()
        .add_class("wj-container")
        .add_style("padding: 16px")
}

#[inline]
pub fn row() -> VNode {
    VNode::div().add_style("display: flex; flex-direction: row; gap: 8px")
}

#[inline]
pub fn column() -> VNode {
    VNode::div().add_style("display: flex; flex-direction: column; gap: 8px")
}

#[inline]
pub fn spacer() -> VNode {
    VNode::div().add_style("flex: 1")
}

#[inline]
pub fn divider() -> VNode {
    VNode::element("hr")
        .add_style("border: 0; border-top: 1px solid #333; margin: 8px 0")
}
