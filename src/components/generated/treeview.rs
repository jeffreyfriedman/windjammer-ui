#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct TreeItem {
    pub label: String,
    pub children: Vec<TreeItem>,
    pub expanded: bool,
}

impl TreeItem {
#[inline]
pub fn new(label: String) -> TreeItem {
        TreeItem { label, children: Vec::new(), expanded: false }
}
#[inline]
pub fn child(mut self, item: TreeItem) -> TreeItem {
        self.children.push(item);
        self
}
#[inline]
pub fn expanded(mut self, expanded: bool) -> TreeItem {
        self.expanded = expanded;
        self
}

pub fn render(&self, depth: i32) -> String {
        let indent_px = depth * 20_i32;
        let icon: String = {
            if !self.children.is_empty() {
                if self.expanded {
                    String::from("▼")
                } else {
                    String::from("▶")
                }
            } else {
                String::from("•")
            }
        };
        let mut html = format!("<div class='wj-tree-item' style='padding-left: {}px;'>\n  <span class='wj-tree-icon'>{}</span>\n  <span>{}</span>\n</div>\n", indent_px, icon, self.label);
        if self.expanded {
            let mut i = 0_usize;
            while i < self.children.len() {
                let child = &self.children[i];
                html = format!("{}{}", html, child.render(depth + 1_i32));
                i = i.wrapping_add(1);
            }
        }
        html
}
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct TreeView {
    pub items: Vec<TreeItem>,
}

impl TreeView {
#[inline]
pub fn new() -> TreeView {
        TreeView { items: Vec::new() }
}
#[inline]
pub fn item(mut self, item: TreeItem) -> TreeView {
        self.items.push(item);
        self
}
}

impl Renderable for TreeView {
#[inline]
fn render(&self) -> String {
        let mut html: String = String::from("<div class='wj-tree-view'>\n");
        let mut i = 0_usize;
        while i < self.items.len() {
            let item = &self.items[i];
            { use std::fmt::Write as _; write!(html, "{}", item.render(0_i32)).expect("a formatting trait implementation returned an error"); };
            i = i.wrapping_add(1);
        }
        format!("{}</div>", html)
}
}

