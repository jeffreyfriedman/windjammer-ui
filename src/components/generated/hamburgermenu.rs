#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct HamburgerMenuItem {
    label: String,
    href: String,
}

impl HamburgerMenuItem {
    #[inline]
    pub fn new(label: String, href: String) -> HamburgerMenuItem {
        HamburgerMenuItem { label, href }
    }
}

pub struct HamburgerMenu {
    items: Vec<HamburgerMenuItem>,
    open: bool,
}

impl HamburgerMenu {
    #[inline]
    pub fn new() -> HamburgerMenu {
        HamburgerMenu {
            items: Vec::new(),
            open: false,
        }
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
        let mut items_html = Vec::new();
        for item in &self.items {
            items_html.push(format!(
                "<a href='{}' class='wj-hamburger-item'>{}</a>",
                item.href, item.label
            ));
        }
        let open_class = {
            if self.open {
                " wj-hamburger-open"
            } else {
                ""
            }
        };
        format!("<div class='wj-hamburger-menu{}'>
                <button class='wj-hamburger-button' onclick='this.parentElement.classList.toggle(\"wj-hamburger-open\")'>
                    <span></span>
                    <span></span>
                    <span></span>
                </button>
                <div class='wj-hamburger-drawer'>
                    {}
                </div>
            </div>", open_class, items_html.join(""))
    }
}
