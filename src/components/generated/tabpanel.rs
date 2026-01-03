#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct TabPanel {
    pub tabs: Vec<TabPanelTab>,
    pub active: String,
    pub orientation: String,
}

impl TabPanel {
    #[inline]
    pub fn new() -> TabPanel {
        TabPanel {
            tabs: Vec::new(),
            active: "".to_string(),
            orientation: "horizontal".to_string(),
        }
    }
    #[inline]
    pub fn tab(mut self, tab: TabPanelTab) -> TabPanel {
        self.tabs.push(tab);
        self
    }
    #[inline]
    pub fn active(mut self, id: String) -> TabPanel {
        self.active = id;
        self
    }
    #[inline]
    pub fn orientation(mut self, orientation: String) -> TabPanel {
        self.orientation = orientation;
        self
    }
}

impl Renderable for TabPanel {
    #[inline]
    fn render(self) -> String {
        let flex_direction = {
            if self.orientation == "vertical" {
                "row".to_string()
            } else {
                "column".to_string()
            }
        };
        let mut tabs_html = {
            let mut __s = String::with_capacity(64);
            write!(
                &mut __s,
                "<div class='wj-tab-panel-tabs wj-tab-panel-{}'>
",
                self.orientation
            )
            .unwrap();
            __s
        };
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
            tabs_html = format!(
                "{}  <button class='wj-tab-panel-tab{}' data-id='{}'>{}</button>
",
                tabs_html, active_class, tab.id, tab.title
            );
            i += 1;
        }
        tabs_html = format!(
            "{}</div>
",
            tabs_html
        );
        let mut content_html = "<div class='wj-tab-panel-content'>
"
        .to_string();
        let mut j = 0;
        while j < (self.tabs.len() as i64) {
            let tab = &self.tabs[j as usize];
            let display = {
                if tab.id == self.active {
                    "block".to_string()
                } else {
                    "none".to_string()
                }
            };
            content_html = format!(
                "{}  <div class='wj-tab-panel-pane' data-id='{}' style='display: {};'>
    {}
  </div>
",
                content_html, tab.id, display, tab.content
            );
            j += 1;
        }
        content_html = format!("{}</div>", content_html);
        format!(
            "<div class='wj-tab-panel' style='display: flex; flex-direction: {};'>
{}{}
</div>",
            flex_direction, tabs_html, content_html
        )
    }
}
