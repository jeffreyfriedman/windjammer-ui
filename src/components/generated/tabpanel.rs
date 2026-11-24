#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct TabPanelTab {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl TabPanelTab {
#[inline]
pub fn new(id: String, title: String, content: String) -> TabPanelTab {
        TabPanelTab { id, title, content }
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct TabPanel {
    pub tabs: Vec<TabPanelTab>,
    pub active: String,
    pub orientation: String,
}

impl TabPanel {
#[inline]
pub fn new() -> TabPanel {
        TabPanel { tabs: Vec::new(), active: String::new(), orientation: String::from("horizontal") }
}
#[inline]
pub fn tab(mut self, tab: TabPanelTab) -> TabPanel {
        self.tabs.push(tab);
        self
}
#[inline]
pub fn active(mut self, id: impl Into<String>) -> TabPanel {
        self.active = id.into();
        self
}
#[inline]
pub fn orientation(mut self, orientation: impl Into<String>) -> TabPanel {
        self.orientation = orientation.into();
        self
}
}

impl Renderable for TabPanel {

fn render(&self) -> String {
        let flex_direction: String = {
            if self.orientation == String::from("vertical") {
                String::from("row")
            } else {
                String::from("column")
            }
        };
        let mut tabs_html = format!("<div class='wj-tab-panel-tabs wj-tab-panel-{}'>\n", self.orientation);
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
            tabs_html = format!("{}  <button class='wj-tab-panel-tab{}' data-id='{}'>{}</button>\n", tabs_html, active_class, tab.id, tab.title);
            i = i.wrapping_add(1);
        }
        tabs_html = format!("{}</div>\n", tabs_html);
        let mut content_html: String = String::from("<div class='wj-tab-panel-content'>\n");
        let mut j = 0_usize;
        while j < self.tabs.len() {
            let tab = &self.tabs[j];
            let display: String = {
                if tab.id == self.active {
                    String::from("block")
                } else {
                    String::from("none")
                }
            };
            { use std::fmt::Write as _; write!(content_html, "  <div class='wj-tab-panel-pane' data-id='{}' style='display: {};'>\n    {}\n  </div>\n", tab.id, display, tab.content).expect("a formatting trait implementation returned an error"); };
            j = j.wrapping_add(1);
        }
        { use std::fmt::Write as _; write!(content_html, "</div>").expect("a formatting trait implementation returned an error"); };
        format!("<div class='wj-tab-panel' style='display: flex; flex-direction: {};'>\n{}{}\n</div>", flex_direction, tabs_html, content_html)
}
}

