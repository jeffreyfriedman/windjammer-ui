#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Mirrors `components_wj/panelhead.wj` — Windjammer is source of truth.

use super::traits::Renderable;

/// Finance panel chrome — hub kicker, title, optional lede, and action slot.
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
    pub fn new(kicker: impl Into<String>, title: impl Into<String>) -> PanelHead {
        PanelHead {
            kicker: kicker.into(),
            title: title.into(),
            lede: String::new(),
            actions: String::new(),
            actions_row: false,
            inline: false,
        }
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

    #[inline]
    pub fn actions(mut self, html: impl Into<String>) -> PanelHead {
        self.actions = html.into();
        self.actions_row = false;
        self
    }

    #[inline]
    pub fn actions_row(mut self, html: impl Into<String>) -> PanelHead {
        self.actions = html.into();
        self.actions_row = true;
        self
    }
}

impl Renderable for PanelHead {
    #[inline]
    fn render(&self) -> String {
        let lede_class = if self.inline { "muted" } else { "lede" };
        let lede_html = if !self.lede.is_empty() {
            format!("<p class=\"{lede_class}\">{}</p>", self.lede)
        } else {
            String::new()
        };
        if self.inline {
            return format!(
                "<p class=\"hub-kicker\">{}</p><h2>{}</h2>{lede_html}",
                self.kicker, self.title
            );
        }
        let actions_html = if self.actions.is_empty() {
            String::new()
        } else if self.actions_row {
            format!("<div class=\"row\">{}</div>", self.actions)
        } else {
            self.actions.clone()
        };
        format!(
            "<div class=\"panel-head\"><div><p class=\"hub-kicker\">{}</p><h2>{}</h2>{lede_html}</div>{actions_html}</div>",
            self.kicker, self.title
        )
    }
}
