#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use super::traits::RenderableVNode;
use super::vnode::VNode;
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Container {
    pub children: Vec<String>,
    pub vnode_children: Vec<VNode>,
    pub max_width: String,
    pub max_height: String,
    pub padding: String,
    pub background_color: String,
}

impl Container {
#[inline]
pub fn new() -> Container {
        Container { children: Vec::new(), vnode_children: Vec::new(), max_width: String::new(), max_height: String::new(), padding: String::from("16px"), background_color: String::new() }
}
#[inline]
pub fn child(mut self, child: impl Into<String>) -> Container {
        self.children.push(child.into());
        self
}
#[inline]
pub fn children(mut self, children: Vec<String>) -> Container {
        self.children = children;
        self
}
#[inline]
pub fn max_width(mut self, width: impl Into<String>) -> Container {
        self.max_width = width.into();
        self
}
#[inline]
pub fn max_height(mut self, height: impl Into<String>) -> Container {
        self.max_height = height.into();
        self
}
#[inline]
pub fn padding(mut self, padding: impl Into<String>) -> Container {
        self.padding = padding.into();
        self
}
#[inline]
pub fn background_color(mut self, color: impl Into<String>) -> Container {
        self.background_color = color.into();
        self
}
/// Add a VNode child for cross-platform rendering
#[inline]
pub fn add_child(mut self, child: VNode) -> Container {
        self.vnode_children.push(child);
        self
}
}

impl RenderableVNode for Container {

fn to_vnode(&self) -> VNode {
        let mut style: String = String::from("margin: 0 auto;");
        if !self.max_width.is_empty() {
            { use std::fmt::Write as _; write!(style, " max-width: {};", self.max_width).expect("a formatting trait implementation returned an error"); };
        }
        if !self.max_height.is_empty() {
            { use std::fmt::Write as _; write!(style, " max-height: {};", self.max_height).expect("a formatting trait implementation returned an error"); };
        }
        if !self.padding.is_empty() {
            { use std::fmt::Write as _; write!(style, " padding: {};", self.padding).expect("a formatting trait implementation returned an error"); };
        }
        if !self.background_color.is_empty() {
            { use std::fmt::Write as _; write!(style, " background-color: {};", self.background_color).expect("a formatting trait implementation returned an error"); };
        }
        let mut node = VNode::div().add_class("wj-container".to_string()).add_style(style.to_string());
        let mut i: u32 = 0_u32;
        while i < (self.vnode_children.len() as u32) {
            node = node.child(self.vnode_children[i as usize]);
            i = i.wrapping_add(1);
        }
        node
}
}

impl Renderable for Container {

fn render(&self) -> String {
        let mut style: String = String::from("margin: 0 auto; ");
        if self.max_width != "" {
            { use std::fmt::Write as _; write!(style, "{}{}{}", "max-width: ", self.max_width, "; ").expect("a formatting trait implementation returned an error"); };
        }
        if self.max_height != "" {
            { use std::fmt::Write as _; write!(style, "{}{}{}", "max-height: ", self.max_height, "; ").expect("a formatting trait implementation returned an error"); };
        }
        if self.padding != "" {
            { use std::fmt::Write as _; write!(style, "{}{}{}", "padding: ", self.padding, "; ").expect("a formatting trait implementation returned an error"); };
        }
        if self.background_color != "" {
            { use std::fmt::Write as _; write!(style, "{}{}{}", "background-color: ", self.background_color, "; ").expect("a formatting trait implementation returned an error"); };
        }
        let children_html = self.children.join("\n  ");
        format!("<div class='wj-container' style='{}'>\n  {}\n</div>", style, children_html)
}
}

