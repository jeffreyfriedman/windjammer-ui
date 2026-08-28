#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Mirrors `components_wj/daystoclosemetric.wj` — Windjammer is source of truth.

use super::traits::Renderable;

/// Close-checklist days-to-close metric (finance dogfood).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct DaysToCloseMetric {
    pub days: i64,
}

impl DaysToCloseMetric {
    #[inline]
    pub fn new(days: i64) -> DaysToCloseMetric {
        DaysToCloseMetric { days }
    }
}

impl Renderable for DaysToCloseMetric {
    #[inline]
    fn render(&self) -> String {
        let n = self.days.to_string();
        format!(
            "<p class='wj-days-to-close close-days-to-close' role='status' data-wj-days-to-close='{}'>Days to close: <strong>{}</strong></p>",
            n, n
        )
    }
}
