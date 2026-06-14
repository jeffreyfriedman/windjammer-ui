use std::fmt::Write;
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
#[inline]
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
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "<div class='wj-tree-item' style='padding-left: {}px;'>\n  <span class='wj-tree-icon'>{}</span>\n  <span>{}</span>\n</div>\n", indent_px, icon, self.label.clone()).unwrap();
            __s
        };
        if self.expanded {
            let mut i = 0;
            while i < self.children.len() {
                let child = &self.children[i];
                html = format!("{}{}", html, child.render(depth + 1_i32));
                i += 1;
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
fn render(self) -> String {
        let mut html = "<div class='wj-tree-view'>\n".to_string();
        let mut i = 0;
        while i < self.items.len() {
            let item = &self.items[i];
            html = format!("{}{}", html, item.render(0));
            i += 1;
        }
        format!("{}</div>", html)
}
}

