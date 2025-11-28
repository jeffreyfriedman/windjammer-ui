#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub enum NavbarPosition {
    Top,
    Bottom,
}

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
            brand: String::from("".to_string()),
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
                item.href, item.label
            ));
        }
        let position_class = match self.position {
            NavbarPosition::Top => "wj-navbar-top",
            NavbarPosition::Bottom => "wj-navbar-bottom",
        };
        let sticky_class = {
            if self.sticky {
                " wj-navbar-sticky"
            } else {
                ""
            }
        };
        let brand_html = {
            if self.brand.len() > 0 {
                format!("<div class='wj-navbar-brand'>{}</div>", self.brand)
            } else {
                String::from("".to_string())
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
