#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ContextMenuItem {
    pub label: String,
    pub icon: String,
    pub action: String,
    pub disabled: bool,
}

impl ContextMenuItem {
#[inline]
pub fn new(label: String) -> ContextMenuItem {
        ContextMenuItem { label: label.to_string(), icon: String::from(""), action: String::from(""), disabled: false }
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
    pub items: Vec<ContextMenuItem>,
    pub trigger_id: String,
}

impl ContextMenu {
#[inline]
pub fn new(trigger_id: String) -> ContextMenu {
        ContextMenu { items: Vec::new(), trigger_id: trigger_id.to_string() }
}
#[inline]
pub fn item(mut self, item: ContextMenuItem) -> ContextMenu {
        self.items.push(item);
        self
}
}

impl Renderable for ContextMenu {
#[inline]
fn render(&self) -> String {
        let mut items_html: Vec<String> = Vec::new();
        for item in &self.items {
            let icon_html = {
                if item.icon.len() > 0 {
                    format!("<span class='wj-context-icon'>{}</span>", item.icon.clone())
                } else {
                    String::from("")
                }
            };
            let disabled_class = {
                if item.disabled {
                    " wj-context-item-disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            let disabled_attr = {
                if item.disabled {
                    " disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            { let _temp0 = format!("<button class='wj-context-item{}' onclick='{}'{}>\n                    {}\n                    <span>{}</span>\n                </button>", disabled_class, item.action.clone(), disabled_attr, icon_html, item.label.clone()); items_html.push(_temp0) };
        }
        format!("<div class='wj-context-menu' id='context-{}' style='display: none'>\n                {}\n            </div>", self.trigger_id.clone(), items_html.join(""))
}
}

