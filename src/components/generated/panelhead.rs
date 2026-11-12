#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct PanelHead {
    pub kicker: String,
    pub title: String,
    pub lede: String,
    pub actions: String,
    pub actions_row: bool,
    pub inline: bool,
}

impl PanelHead {
#[inline]
pub fn new(kicker: String, title: String) -> PanelHead {
        PanelHead { kicker, title, lede: String::new(), actions: String::new(), actions_row: false, inline: false }
}
#[inline]
pub fn lede(mut self, lede: impl Into<String>) -> PanelHead {
        self.lede = lede.into();
        self
}
/// Breadcrumb layout: kicker + h2 + muted lede without `panel-head` wrapper.
#[inline]
pub fn inline(mut self) -> PanelHead {
        self.inline = true;
        self
}
/// Append action HTML directly after the title block (AuthFetch refresh buttons).
#[inline]
pub fn actions(mut self, html: impl Into<String>) -> PanelHead {
        self.actions = html.into();
        self.actions_row = false;
        self
}
/// Wrap actions in `<div class="row">` (close hub, multi-button chrome).
#[inline]
pub fn actions_row(mut self, html: impl Into<String>) -> PanelHead {
        self.actions = html.into();
        self.actions_row = true;
        self
}
}

impl Renderable for PanelHead {

fn render(&self) -> String {
        let lede_class: String = {
            if self.inline {
                String::from("muted")
            } else {
                String::from("lede")
            }
        };
        let lede_html = {
            if !self.lede.is_empty() {
                format!("{}{}{}{}{}", "<p class=\"", lede_class, "\">", self.lede, "</p>")
            } else {
                String::new()
            }
        };
        if self.inline {
            let title_html = {
                if !self.title.is_empty() {
                    format!("{}{}{}", "<h2>", self.title, "</h2>")
                } else {
                    String::new()
                }
            };
            return format!("{}{}{}{}{}", "<p class=\"hub-kicker\">", self.kicker, "</p>", title_html, lede_html);
        }
        let actions_html = {
            if self.actions.is_empty() {
                String::new()
            } else {
                if self.actions_row {
                    format!("{}{}{}", "<div class=\"row\">", self.actions, "</div>")
                } else {
                    self.actions.clone()
                }
            }
        };
        format!("{}{}{}{}{}{}{}{}{}", "<div class=\"panel-head\"><div><p class=\"hub-kicker\">", self.kicker, "</p><h2>", self.title, "</h2>", lede_html, "</div>", actions_html, "</div>")
}
}

