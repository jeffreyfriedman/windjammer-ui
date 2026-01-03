#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SidebarPosition {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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
            icon: String::from(""),
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

#[derive(Debug, Clone)]
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
    fn render(self) -> String {
        let mut items_html = Vec::new();
        for item in &self.items {
            let icon_html = {
                if item.icon.clone().len() > 0 {
                    format!("<span class='wj-sidebar-icon'>{}</span>", item.icon.clone())
                } else {
                    String::from("")
                }
            };
            items_html.push(format!("<a href='{}' class='wj-sidebar-item'>{}<span class='wj-sidebar-label'>{}</span></a>", item.href.clone(), icon_html, item.label.clone()));
        }
        let position_class = match self.position {
            SidebarPosition::Left => "wj-sidebar-left".to_string(),
            SidebarPosition::Right => "wj-sidebar-right".to_string(),
        };
        let collapsed_class = {
            if self.collapsed {
                " wj-sidebar-collapsed".to_string()
            } else {
                "".to_string()
            }
        };
        format!("<aside class='wj-sidebar {} {}' style='width: {}'>
                <div class='wj-sidebar-toggle' onclick='this.parentElement.classList.toggle(\"wj-sidebar-collapsed\")'>
                    <span class='wj-sidebar-toggle-icon'>☰</span>
                </div>
                <nav class='wj-sidebar-nav'>{}</nav>
            </aside>", position_class, collapsed_class, self.width, items_html.join(""))
    }
}
