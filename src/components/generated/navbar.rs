#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NavbarPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct NavbarItem {
    label: String,
    href: String,
}

impl NavbarItem {
    #[inline]
    pub fn new(label: String, href: String) -> NavbarItem {
        NavbarItem { label, href }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Navbar {
    brand: String,
    items: Vec<NavbarItem>,
    position: NavbarPosition,
    sticky: bool,
}

impl Navbar {
    #[inline]
    pub fn new() -> Navbar {
        Navbar {
            brand: String::new(),
            items: Vec::new(),
            position: NavbarPosition::Top,
            sticky: false,
        }
    }
    #[inline]
    pub fn brand(mut self, brand: String) -> Navbar {
        self.brand = brand;
        self
    }
    #[inline]
    pub fn item(mut self, item: NavbarItem) -> Navbar {
        self.items.push(item);
        self
    }
    #[inline]
    pub fn position(mut self, pos: NavbarPosition) -> Navbar {
        self.position = pos;
        self
    }
    #[inline]
    pub fn sticky(mut self, sticky: bool) -> Navbar {
        self.sticky = sticky;
        self
    }
}

impl Renderable for Navbar {
    #[inline]
    fn render(self) -> String {
        let mut items_html: Vec<String> = Vec::new();
        for item in self.items {
            {
                let _temp0 = format!(
                    "<a href='{}' class='wj-navbar-item'>{}</a>",
                    item.href, item.label
                );
                items_html.push(_temp0)
            };
        }
        let position_class: String = match self.position {
            NavbarPosition::Top => String::from("wj-navbar-top"),
            NavbarPosition::Bottom => String::from("wj-navbar-bottom"),
        };
        let sticky_class = {
            if self.sticky {
                String::from(" wj-navbar-sticky")
            } else {
                String::new()
            }
        };
        let brand_html = {
            if !self.brand.is_empty() {
                format!("<div class='wj-navbar-brand'>{}</div>", self.brand)
            } else {
                String::new()
            }
        };
        format!(
            "<nav class='wj-navbar {} {}'>{}<div class='wj-navbar-items'>{}</div></nav>",
            position_class,
            sticky_class,
            brand_html,
            items_html.join("")
        )
    }
}
