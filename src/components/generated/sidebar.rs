#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SidebarPosition {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct SidebarItem {
    pub label: String,
    pub icon: String,
    pub href: String,
}

impl SidebarItem {
    #[inline]
    pub fn new(label: String) -> SidebarItem {
        SidebarItem {
            label,
            icon: String::new(),
            href: String::from("#"),
        }
    }
    #[inline]
    pub fn icon(mut self, icon: String) -> SidebarItem {
        self.icon = icon;
        self
    }
    #[inline]
    pub fn href(mut self, href: String) -> SidebarItem {
        self.href = href;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Sidebar {
    pub items: Vec<SidebarItem>,
    pub position: SidebarPosition,
    pub width: String,
    pub collapsed: bool,
}

impl Sidebar {
    #[inline]
    pub fn new() -> Sidebar {
        Sidebar {
            items: Vec::new(),
            position: SidebarPosition::Left,
            width: String::from("250px"),
            collapsed: false,
        }
    }
    #[inline]
    pub fn item(mut self, item: SidebarItem) -> Sidebar {
        self.items.push(item);
        self
    }
    #[inline]
    pub fn position(mut self, pos: SidebarPosition) -> Sidebar {
        self.position = pos;
        self
    }
    #[inline]
    pub fn width(mut self, width: String) -> Sidebar {
        self.width = width;
        self
    }
    #[inline]
    pub fn collapsed(mut self, collapsed: bool) -> Sidebar {
        self.collapsed = collapsed;
        self
    }
}

impl Renderable for Sidebar {
    #[inline]
    fn render(&self) -> String {
        let mut items_html: Vec<String> = Vec::new();
        for item in &self.items {
            let icon_html = {
                if item.icon.len() > 0 {
                    format!("<span class='wj-sidebar-icon'>{}</span>", item.icon.clone())
                } else {
                    String::new()
                }
            };
            {
                let _temp0 = format!("<a href='{}' class='wj-sidebar-item'>{}<span class='wj-sidebar-label'>{}</span></a>", item.href.clone(), icon_html, item.label.clone());
                items_html.push(_temp0)
            };
        }
        let position_class: String = match self.position {
            SidebarPosition::Left => String::from("wj-sidebar-left"),
            SidebarPosition::Right => String::from("wj-sidebar-right"),
        };
        let collapsed_class = {
            if self.collapsed {
                String::from(" wj-sidebar-collapsed")
            } else {
                String::new()
            }
        };
        format!("<aside class='wj-sidebar {} {}' style='width: {}'>\n                <div class='wj-sidebar-toggle' onclick='this.parentElement.classList.toggle(\"wj-sidebar-collapsed\")'>\n                    <span class='wj-sidebar-toggle-icon'>☰</span>\n                </div>\n                <nav class='wj-sidebar-nav'>{}</nav>\n            </aside>", position_class, collapsed_class, self.width.clone(), items_html.join(&""))
    }
}
