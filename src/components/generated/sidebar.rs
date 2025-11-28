#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub enum SidebarPosition {
    Left,
    Right,
}

pub struct SidebarItem {
    label: String,
    icon: String,
    href: String,
}

impl SidebarItem {
    #[inline]
    pub fn new(label: String) -> SidebarItem {
        SidebarItem {
            label,
            icon: String::from("".to_string()),
            href: String::from("#".to_string()),
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

pub struct Sidebar {
    items: Vec<SidebarItem>,
    position: SidebarPosition,
    width: String,
    collapsed: bool,
}

impl Sidebar {
    #[inline]
    pub fn new() -> Sidebar {
        Sidebar {
            items: Vec::new(),
            position: SidebarPosition::Left,
            width: String::from("250px".to_string()),
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
                if item.icon.len() > 0 {
                    format!("<span class='wj-sidebar-icon'>{}</span>", item.icon)
                } else {
                    String::from("".to_string())
                }
            };
            items_html.push(format!("<a href='{}' class='wj-sidebar-item'>{}<span class='wj-sidebar-label'>{}</span></a>", item.href, icon_html, item.label));
        }
        let position_class = match self.position {
            SidebarPosition::Left => "wj-sidebar-left",
            SidebarPosition::Right => "wj-sidebar-right",
        };
        let collapsed_class = {
            if self.collapsed {
                " wj-sidebar-collapsed"
            } else {
                ""
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
