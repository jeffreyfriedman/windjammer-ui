#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

pub struct FileNode {
    name: String,
    is_directory: bool,
    children: Vec<FileNode>,
    expanded: bool,
}

impl FileNode {
    #[inline]
    pub fn new(name: String, is_directory: bool) -> FileNode {
        FileNode {
            name,
            is_directory,
            children: Vec::new(),
            expanded: false,
        }
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
                    "📂"
                } else {
                    "📁"
                }
            } else {
                "📄"
            }
        };
        let mut html = {
            let mut __s = String::with_capacity(64);
            write!(
                &mut __s,
                "{}{} {}
",
                indent, icon, self.name
            )
            .unwrap();
            __s
        };
        if self.is_directory && self.expanded {
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

pub struct FileTree {
    root: FileNode,
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
        format!(
            "<div class='wj-file-tree'>
{}</div>",
            self.root.render(0)
        )
    }
}
