#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

#[derive(Debug, Clone, Default)]
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
        self.items.push(item);
        self
    }
}

impl Renderable for Dropdown {
    #[inline]
    fn render(self) -> String {
        let mut items_html = "".to_string();
        let mut i = 0;
        while i < (self.items.len() as i64) {
            let item = &self.items[i as usize];
            let disabled_class = {
                if item.disabled {
                    " wj-dropdown-item-disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            items_html = format!(
                "{}<a class='wj-dropdown-item{}' data-value='{}'>{}</a>",
                items_html, disabled_class, item.value, item.label
            );
            i += 1;
        }
        format!(
            "<div class='wj-dropdown'>
  <button class='wj-dropdown-toggle'>{} ▼</button>
  <div class='wj-dropdown-menu'>
    {}
  </div>
</div>",
            self.label, items_html
        )
    }
}
