#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

pub struct TreeItem {
    label: String,
    children: Vec<TreeItem>,
    expanded: bool,
}

impl TreeItem {
    #[inline]
    pub fn new(label: String) -> TreeItem {
        TreeItem {
            label,
            children: Vec::new(),
            expanded: false,
        }
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
        let indent_px = depth * 20;
        let icon = {
            if self.children.len() > 0 {
                if self.expanded {
                    "▼"
                } else {
                    "▶"
                }
            } else {
                "•"
            }
        };
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(
                &mut __s,
                "<div class='wj-tree-item' style='padding-left: {}px;'>
  <span class='wj-tree-icon'>{}</span>
  <span>{}</span>
</div>
",
                indent_px, icon, self.label
            )
            .unwrap();
            __s
        };
        if self.expanded {
            let mut i = 0;
            while i < self.children.len() {
                let child = &self.children[i];
                html = format!("{}{}", html, child.render(depth + 1));
                i += 1;
            }
        }
        html
    }
}

pub struct TreeView {
    items: Vec<TreeItem>,
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
        let mut html = "<div class='wj-tree-view'>
"
        .to_string();
        let mut i = 0;
        while i < self.items.len() {
            let item = &self.items[i];
            html = format!("{}{}", html, item.render(0));
            i += 1;
        }
        format!("{}</div>", html)
    }
}
