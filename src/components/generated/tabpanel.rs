#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

pub struct TabPanelTab {
    id: String,
    title: String,
    content: String,
}

impl TabPanelTab {
    #[inline]
    pub fn new(id: String, title: String, content: String) -> TabPanelTab {
        TabPanelTab { id, title, content }
    }
}

pub struct TabPanel {
    tabs: Vec<TabPanelTab>,
    active: String,
    orientation: String,
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
    fn render(self) -> String {
        let flex_direction = {
            if self.orientation == "vertical" {
                "row"
            } else {
                "column"
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
        while i < self.tabs.len() {
            let tab = &self.tabs[i];
            let active_class = {
                if tab.id == self.active {
                    " wj-tab-active"
                } else {
                    ""
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
        while j < self.tabs.len() {
            let tab = &self.tabs[j];
            let display = {
                if tab.id == self.active {
                    "block"
                } else {
                    "none"
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
