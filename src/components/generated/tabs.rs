#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub content: String,
    pub disabled: bool,
}

impl Tab {
#[inline]
pub fn new(id: String, label: String, content: String) -> Tab {
        Tab { id, label, content, disabled: false }
}
#[inline]
pub fn disabled(mut self, disabled: bool) -> Tab {
        self.disabled = disabled;
        self
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub active: String,
}

impl Tabs {
#[inline]
pub fn new() -> Tabs {
        Tabs { tabs: Vec::new(), active: String::new() }
}
#[inline]
pub fn tab(mut self, tab: Tab) -> Tabs {
        self.tabs.push(tab);
        self
}
#[inline]
pub fn active(mut self, id: impl Into<String>) -> Tabs {
        self.active = id.into();
        self
}
}

impl Renderable for Tabs {

fn render(&self) -> String {
        let mut tabs_html: String = String::from("<div class='wj-tabs-header'>");
        let mut i = 0_usize;
        while i < self.tabs.len() {
            let tab = &self.tabs[i];
            let active_class = {
                if tab.id == self.active {
                    String::from(" wj-tab-active")
                } else {
                    String::new()
                }
            };
            let disabled_class = {
                if tab.disabled {
                    String::from(" wj-tab-disabled")
                } else {
                    String::new()
                }
            };
            { use std::fmt::Write as _; write!(tabs_html, "<button class='wj-tab{}{}' data-tab-id='{}'>{}</button>", active_class, disabled_class, tab.id, tab.label).expect("a formatting trait implementation returned an error"); };
            i += 1;
        }
        { use std::fmt::Write as _; write!(tabs_html, "</div>").expect("a formatting trait implementation returned an error"); };
        let mut content_html: String = String::from("<div class='wj-tabs-content'>");
        let mut j = 0_usize;
        while j < self.tabs.len() {
            let tab = &self.tabs[j];
            let display_style: String = {
                if tab.id == self.active {
                    String::from("display: block;")
                } else {
                    String::from("display: none;")
                }
            };
            { use std::fmt::Write as _; write!(content_html, "<div class='wj-tab-panel' data-tab-id='{}' style='{}'>{}</div>", tab.id, display_style, tab.content).expect("a formatting trait implementation returned an error"); };
            j += 1;
        }
        { use std::fmt::Write as _; write!(content_html, "</div>").expect("a formatting trait implementation returned an error"); };
        format!("<div class='wj-tabs'>{}{}</div>", tabs_html, content_html)
}
}

