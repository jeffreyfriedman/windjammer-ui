#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub content: String,
    pub disabled: bool,
}

impl Tab {
    #[inline]
    pub fn new(id: String, label: String, content: String) -> Tab {
        Tab {
            id,
            label,
            content,
            disabled: false,
        }
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Tab {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub active: String,
}

impl Tabs {
    #[inline]
    pub fn new() -> Tabs {
        Tabs {
            tabs: Vec::new(),
            active: "".to_string(),
        }
    }
    #[inline]
    pub fn tab(mut self, tab: Tab) -> Tabs {
        self.tabs.push(tab);
        self
    }
    #[inline]
    pub fn active(mut self, id: String) -> Tabs {
        self.active = id;
        self
    }
}

impl Renderable for Tabs {
    #[inline]
    fn render(self) -> String {
        let mut tabs_html = "<div class='wj-tabs-header'>".to_string();
        let mut i = 0;
        while i < (self.tabs.len() as i64) {
            let tab = &self.tabs[i as usize];
            let active_class = {
                if tab.id == self.active {
                    " wj-tab-active".to_string()
                } else {
                    "".to_string()
                }
            };
            let disabled_class = {
                if tab.disabled {
                    " wj-tab-disabled".to_string()
                } else {
                    "".to_string()
                }
            };
            tabs_html = format!(
                "{}<button class='wj-tab{}{}' data-tab-id='{}'>{}</button>",
                tabs_html, active_class, disabled_class, tab.id, tab.label
            );
            i += 1;
        }
        tabs_html = format!("{}</div>", tabs_html);
        let mut content_html = "<div class='wj-tabs-content'>".to_string();
        let mut j = 0;
        while j < (self.tabs.len() as i64) {
            let tab = &self.tabs[j as usize];
            let display_style = {
                if tab.id == self.active {
                    "display: block;".to_string()
                } else {
                    "display: none;".to_string()
                }
            };
            content_html = format!(
                "{}<div class='wj-tab-panel' data-tab-id='{}' style='{}'>{}</div>",
                content_html, tab.id, display_style, tab.content
            );
            j += 1;
        }
        content_html = format!("{}</div>", content_html);
        format!("<div class='wj-tabs'>{}{}</div>", tabs_html, content_html)
    }
}
