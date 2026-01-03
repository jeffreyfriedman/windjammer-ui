use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ContextMenuItem {
    pub label: String,
    pub icon: String,
    pub action: String,
    pub disabled: bool,
}

impl ContextMenuItem {
#[inline]
pub fn new(label: String) -> ContextMenuItem {
        ContextMenuItem { label, icon: String::from(""), action: String::from(""), disabled: false }
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

#[derive(Debug, Clone, Default)]
pub struct ContextMenu {
    pub items: Vec<ContextMenuItem>,
    pub trigger_id: String,
}

impl ContextMenu {
#[inline]
pub fn new(trigger_id: String) -> ContextMenu {
        ContextMenu { items: Vec::new(), trigger_id }
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
        let mut items_html = Vec::new();
        for item in &self.items {
            let icon_html = {
                if item.icon.clone().len() > 0 {
                    format!("<span class='wj-context-icon'>{}</span>", item.icon.clone())
                } else {
                    String::from("")
                }
            };
            let disabled_class = {
                if item.disabled.clone() {
                    " wj-context-item-disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            let disabled_attr = {
                if item.disabled.clone() {
                    " disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            items_html.push(format!("<button class='wj-context-item{}' onclick='{}'{}>
                    {}
                    <span>{}</span>
                </button>", disabled_class, item.action.clone(), disabled_attr, icon_html, item.label.clone()));
        }
        format!("<div class='wj-context-menu' id='context-{}' style='display: none'>
                {}
            </div>", self.trigger_id, items_html.join(""))
}
}

