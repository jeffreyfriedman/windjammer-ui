#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct PanelSectionHead {
    pub title: String,
    pub actions: String,
}

impl PanelSectionHead {
#[inline]
pub fn new(title: String) -> PanelSectionHead {
        PanelSectionHead { title, actions: String::new() }
}
/// Append action HTML after the h3 (AuthFetch refresh buttons).
#[inline]
pub fn actions(mut self, html: impl Into<String>) -> PanelSectionHead {
        self.actions = html.into();
        self
}
}

impl Renderable for PanelSectionHead {
#[inline]
fn render(&self) -> String {
        format!("{}{}{}{}{}", "<div class=\"panel-head\"><h3>", self.title, "</h3>", self.actions, "</div>")
}
}

