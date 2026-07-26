#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct KpiTile {
    pub label: String,
    pub value_html: String,
}

impl KpiTile {
    #[inline]
    pub fn new(label: String) -> KpiTile {
        KpiTile {
            label,
            value_html: "".to_string(),
        }
    }

    #[inline]
    pub fn value_html(mut self, html: String) -> KpiTile {
        self.value_html = html;
        self
    }
}

impl Renderable for KpiTile {
    #[inline]
    fn render(&self) -> String {
        format!(
            "<div class='wj-kpi-tile kpi'><span class='kpi-label'>{}</span><div class='kpi-value'>{}</div></div>",
            self.label,
            self.value_html
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct KpiGrid {
    pub tiles: Vec<String>,
}

impl KpiGrid {
    #[inline]
    pub fn new() -> KpiGrid {
        KpiGrid {
            tiles: Vec::new(),
        }
    }

    #[inline]
    pub fn tile(mut self, html: String) -> KpiGrid {
        self.tiles.push(html);
        self
    }
}

impl Renderable for KpiGrid {
    #[inline]
    fn render(&self) -> String {
        let mut body = String::new();
        for t in &self.tiles {
            body.push_str(t);
        }
        format!("<div class='kpi-grid wj-kpi-grid'>{}</div>", body)
    }
}
