use std::fmt::Write;

use super::traits::Renderable;

#[derive(Debug, Clone, Default)]
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
        let icon = {
            if self.is_directory {
                if self.expanded {
                    "📂".to_string()
                } else {
                    "📁".to_string()
                }
            } else {
                "📄".to_string()
            }
        };
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "{}{} {}
", indent, icon, self.name).unwrap();
            __s
        };
        if self.is_directory && self.expanded {
            let mut i = 0;
            while i < (self.children.len() as i64) {
                let child = &self.children[i as usize];
                html = format!("{}{}", html, child.render(depth + 1));
                i += 1;
            }
        }
        html
}
}

#[derive(Debug, Clone)]
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
        format!("<div class='wj-file-tree'>
{}</div>", self.root.render(0))
}
}

