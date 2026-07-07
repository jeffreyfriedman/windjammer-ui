#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct DropdownItem {
    pub label: String,
    pub value: String,
    pub disabled: bool,
}

impl DropdownItem {
    #[inline]
    pub fn new(label: String, value: String) -> DropdownItem {
        DropdownItem {
            label,
            value,
            disabled: false,
        }
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> DropdownItem {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Dropdown {
    pub label: String,
    pub items: Vec<DropdownItem>,
}

impl Dropdown {
    #[inline]
    pub fn new(label: String) -> Dropdown {
        Dropdown {
            label,
            items: Vec::new(),
        }
    }
    #[inline]
    pub fn item(mut self, item: DropdownItem) -> Dropdown {
        self.items.push(item.clone());
        self
    }
}

impl Renderable for Dropdown {
    #[inline]
    fn render(&mut self) -> String {
        let mut items_html = "".to_string();
        let mut i = 0;
        while i < self.items.len() {
            let item = &self.items[i];
            let disabled_class = {
                if item.disabled {
                    String::from(" wj-dropdown-item-disabled")
                } else {
                    String::new()
                }
            };
            items_html = format!(
                "{}<a class='wj-dropdown-item{}' data-value='{}'>{}</a>",
                items_html, disabled_class, item.value, item.label
            );
            i += 1;
        }
        format!("<div class='wj-dropdown'>\n  <button class='wj-dropdown-toggle'>{} ▼</button>\n  <div class='wj-dropdown-menu'>\n    {}\n  </div>\n</div>", self.label, items_html)
    }
}
