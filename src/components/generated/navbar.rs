#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NavbarPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NavbarItem {
    pub label: String,
    pub href: String,
}

impl NavbarItem {
    #[inline]
    pub fn new(label: String, href: String) -> NavbarItem {
        NavbarItem { label, href }
    }
}

#[derive(Debug, Clone)]
pub struct Navbar {
    pub brand: String,
    pub items: Vec<NavbarItem>,
    pub position: NavbarPosition,
    pub sticky: bool,
}

impl Navbar {
    #[inline]
    pub fn new() -> Navbar {
        Navbar {
            brand: String::from(""),
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
        let mut items_html = Vec::new();
        for item in &self.items {
            items_html.push(format!(
                "<a href='{}' class='wj-navbar-item'>{}</a>",
                item.href.clone(),
                item.label.clone()
            ));
        }
        let position_class = match self.position {
            NavbarPosition::Top => "wj-navbar-top".to_string(),
            NavbarPosition::Bottom => "wj-navbar-bottom".to_string(),
        };
        let sticky_class = {
            if self.sticky {
                " wj-navbar-sticky".to_string()
            } else {
                "".to_string()
            }
        };
        let brand_html = {
            if self.brand.len() > (0 as usize) {
                format!("<div class='wj-navbar-brand'>{}</div>", self.brand)
            } else {
                String::from("")
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
