use std::fmt::Write;
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FileNode {
    pub name: String,
    pub is_directory: bool,
    pub children: Vec<FileNode>,
    pub expanded: bool,
}

impl FileNode {
#[inline]
pub fn new(name: String, is_directory: bool) -> FileNode {
        FileNode { name, is_directory, children: Vec::new(), expanded: false }
}
#[inline]
pub fn child(mut self, node: FileNode) -> FileNode {
        self.children.push(node);
        self
}
#[inline]
pub fn expanded(mut self, expanded: bool) -> FileNode {
        self.expanded = expanded;
        self
}
#[inline]
pub fn render(&self, depth: i32) -> String {
        let indent = "  ".repeat(depth as usize);
        let icon: String = {
            if self.is_directory {
                if self.expanded {
                    String::from("📂")
                } else {
                    String::from("📁")
                }
            } else {
                String::from("📄")
            }
        };
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "{}{} {}\n", indent, icon, self.name.clone()).unwrap();
            __s
        };
        if self.is_directory && self.expanded {
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

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FileTree {
    pub root: FileNode,
}

impl FileTree {
#[inline]
pub fn new(root: FileNode) -> FileTree {
        FileTree { root }
}
}

impl Renderable for FileTree {
#[inline]
fn render(self) -> String {
        format!("<div class='wj-file-tree'>\n{}</div>", self.root.render(0_i32))
}
}

