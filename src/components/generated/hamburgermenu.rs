#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct HamburgerMenuItem {
    pub label: String,
    pub href: String,
}

impl HamburgerMenuItem {
#[inline]
pub fn new(label: String, href: String) -> HamburgerMenuItem {
        HamburgerMenuItem { label, href }
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct HamburgerMenu {
    pub items: Vec<HamburgerMenuItem>,
    pub open: bool,
}

impl HamburgerMenu {
#[inline]
pub fn new() -> HamburgerMenu {
        HamburgerMenu { items: Vec::new(), open: false }
}
#[inline]
pub fn item(mut self, item: HamburgerMenuItem) -> HamburgerMenu {
        self.items.push(item);
        self
}
#[inline]
pub fn open(mut self, open: bool) -> HamburgerMenu {
        self.open = open;
        self
}
}

impl Renderable for HamburgerMenu {
#[inline]
fn render(self) -> String {
        let mut items_html: Vec<String> = Vec::new();
        for item in self.items {
            { let _temp0 = format!("<a href='{}' class='wj-hamburger-item'>{}</a>", item.href, item.label); items_html.push(_temp0) };
        }
        let open_class = {
            if self.open {
                String::from(" wj-hamburger-open")
            } else {
                String::new()
            }
        };
        format!("<div class='wj-hamburger-menu{}'>\n                <button class='wj-hamburger-button' onclick='this.parentElement.classList.toggle(\"wj-hamburger-open\")'>\n                    <span></span>\n                    <span></span>\n                    <span></span>\n                </button>\n                <div class='wj-hamburger-drawer'>\n                    {}\n                </div>\n            </div>", open_class, items_html.join(""))
}
}

