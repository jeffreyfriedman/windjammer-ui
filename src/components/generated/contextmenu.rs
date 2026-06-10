#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ContextMenuItem {
    label: String,
    icon: String,
    action: String,
    disabled: bool,
}

impl ContextMenuItem {
    #[inline]
    pub fn new(label: String) -> ContextMenuItem {
        ContextMenuItem {
            label,
            icon: String::new(),
            action: String::new(),
            disabled: false,
        }
    }
    #[inline]
    pub fn icon(mut self, icon: String) -> ContextMenuItem {
        self.icon = icon;
        self
    }
    #[inline]
    pub fn action(mut self, action: String) -> ContextMenuItem {
        self.action = action;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> ContextMenuItem {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    trigger_id: String,
}

impl ContextMenu {
    #[inline]
    pub fn new(trigger_id: String) -> ContextMenu {
        ContextMenu {
            items: Vec::new(),
            trigger_id,
        }
    }
    #[inline]
    pub fn item(mut self, item: ContextMenuItem) -> ContextMenu {
        self.items.push(item);
        self
    }
}

impl Renderable for ContextMenu {
    #[inline]
    fn render(self) -> String {
        let mut items_html: Vec<String> = Vec::new();
        for item in self.items {
            let icon_html = {
                if item.icon.len() > 0 {
                    format!("<span class='wj-context-icon'>{}</span>", item.icon)
                } else {
                    String::new()
                }
            };
            let disabled_class = {
                if item.disabled {
                    String::from(" wj-context-item-disabled")
                } else {
                    String::new()
                }
            };
            let disabled_attr = {
                if item.disabled {
                    String::from(" disabled")
                } else {
                    String::new()
                }
            };
            {
                let _temp0 = format!("<button class='wj-context-item{}' onclick='{}'{}>\n                    {}\n                    <span>{}</span>\n                </button>", disabled_class, item.action, disabled_attr, icon_html, item.label);
                items_html.push(_temp0)
            };
        }
        format!("<div class='wj-context-menu' id='context-{}' style='display: none'>\n                {}\n            </div>", self.trigger_id, items_html.join(""))
    }
}
