#![allow(clippy::all)]
#![allow(noop_method_call)]
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
        Container {
            children: Vec::new(),
            vnode_children: Vec::new(),
            max_width: "".to_string(),
            max_height: "".to_string(),
            padding: "16px".to_string(),
            background_color: "".to_string(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Container {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn children(mut self, children: Vec<String>) -> Container {
        self.children = children;
        self
    }
    #[inline]
    pub fn max_width(mut self, width: String) -> Container {
        self.max_width = width;
        self
    }
    #[inline]
    pub fn max_height(mut self, height: String) -> Container {
        self.max_height = height;
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Container {
        self.padding = padding;
        self
    }
    #[inline]
    pub fn background_color(mut self, color: String) -> Container {
        self.background_color = color;
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
    #[inline]
    fn to_vnode(&self) -> VNode {
        let mut style = "margin: 0 auto;".to_string();
        if !self.max_width.is_empty() {
            style = format!("{} max-width: {};", style, self.max_width.clone());
        }
        if !self.max_height.is_empty() {
            style = format!("{} max-height: {};", style, self.max_height.clone());
        }
        if !self.padding.is_empty() {
            style = format!("{} padding: {};", style, self.padding.clone());
        }
        if !self.background_color.is_empty() {
            style = format!(
                "{} background-color: {};",
                style,
                self.background_color.clone()
            );
        }
        let mut node = VNode::div().add_class("wj-container").add_style(&style);
        let mut i: u32 = 0_u32;
        while i < (self.vnode_children.len() as u32) {
            node = node.child(self.vnode_children[i as usize]);
            i += 1_u32;
        }
        node
    }
}

impl Renderable for Container {
    #[inline]
    fn render(&self) -> String {
        let mut style = "margin: 0 auto; ".to_string();
        if self.max_width != "" {
            style = format!("{}{}{}{}", style, "max-width: ", self.max_width, "; ");
        }
        if self.max_height != "" {
            style = format!("{}{}{}{}", style, "max-height: ", self.max_height, "; ");
        }
        if self.padding != "" {
            style = format!("{}{}{}{}", style, "padding: ", self.padding, "; ");
        }
        if self.background_color != "" {
            style = format!(
                "{}{}{}{}",
                style, "background-color: ", self.background_color, "; "
            );
        }
        let children_html = self.children.join(&"\n  ");
        format!(
            "<div class='wj-container' style='{}'>\n  {}\n</div>",
            style, children_html
        )
    }
}
