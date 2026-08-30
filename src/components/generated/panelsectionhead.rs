#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Mirrors `components_wj/panelsectionhead.wj` — Windjammer is source of truth.

use super::traits::Renderable;

/// In-panel h3 subsection chrome with optional action slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct PanelSectionHead {
    pub title: String,
    pub actions: String,
}

impl PanelSectionHead {
    #[inline]
    pub fn new(title: impl Into<String>) -> PanelSectionHead {
        PanelSectionHead {
            title: title.into(),
            actions: String::new(),
        }
    }

    #[inline]
    pub fn actions(mut self, html: impl Into<String>) -> PanelSectionHead {
        self.actions = html.into();
        self
    }
}

impl Renderable for PanelSectionHead {
    #[inline]
    fn render(&self) -> String {
        format!(
            "<div class=\"panel-head\"><h3>{}</h3>{}</div>",
            self.title, self.actions
        )
    }
}
