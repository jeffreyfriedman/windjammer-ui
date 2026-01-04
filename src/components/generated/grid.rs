#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub children: Vec<String>,
    pub columns: i32,
    pub gap: String,
    pub padding: String,
}

impl Grid {
    #[inline]
    pub fn new(columns: i32) -> Grid {
        Grid {
            children: Vec::new(),
            columns,
            gap: "16px".to_string(),
            padding: "0".to_string(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Grid {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn children(mut self, children: Vec<String>) -> Grid {
        self.children = children;
        self
    }
    #[inline]
    pub fn gap(mut self, gap: String) -> Grid {
        self.gap = gap;
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Grid {
        self.padding = padding;
        self
    }
}

impl Renderable for Grid {
    #[inline]
    fn render(&self) -> String {
        let style = {
            let mut __s = String::with_capacity(64);
            write!(
                &mut __s,
                "display: grid; grid-template-columns: repeat({}, 1fr); gap: {}; padding: {};",
                self.columns, self.gap, self.padding
            )
            .unwrap();
            __s
        };
        let children_html = self.children.join(
            "
",
        );
        format!(
            "<div class='wj-grid' style='{}'>
{}
</div>",
            style, children_html
        )
    }
}
