#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Panel {
    pub title: String,
    pub children: Vec<String>,
    pub collapsible: bool,
    pub collapsed: bool,
    pub padding: String,
}

impl Panel {
    #[inline]
    pub fn new(title: String) -> Panel {
        Panel {
            title,
            children: Vec::new(),
            collapsible: false,
            collapsed: false,
            padding: "16px".to_string(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Panel {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn collapsible(mut self, collapsible: bool) -> Panel {
        self.collapsible = collapsible;
        self
    }
    #[inline]
    pub fn collapsed(mut self, collapsed: bool) -> Panel {
        self.collapsed = collapsed;
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Panel {
        self.padding = padding;
        self
    }
}

impl Renderable for Panel {
    #[inline]
    fn render(&self) -> String {
        let header_class: String = {
            if self.collapsible {
                String::from("wj-panel-header-collapsible")
            } else {
                String::from("wj-panel-header")
            }
        };
        let icon = {
            if self.collapsible {
                if self.collapsed {
                    String::from("▶")
                } else {
                    String::from("▼")
                }
            } else {
                String::new()
            }
        };
        let content_style: String = {
            if self.collapsed {
                String::from("display: none;")
            } else {
                String::from("display: block;")
            }
        };
        let children_html = self.children.join(&"\n");
        format!("<div class='wj-panel'>\n  <div class='{}'>\n    <span>{}</span>\n    <h3>{}</h3>\n  </div>\n  <div class='wj-panel-content' style='{}padding: {};'>\n    {}\n  </div>\n</div>", header_class, icon, self.title.clone(), content_style, self.padding.clone(), children_html)
    }
}
