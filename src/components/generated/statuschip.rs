#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Mirrors `components_wj/statuschip.wj` — Windjammer is source of truth.

use super::badge::{Badge, BadgeSize, BadgeVariant};
use super::traits::Renderable;

/// Maps ledger / close-checklist statuses to Badge variants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct StatusChip {
    pub status: String,
    pub label: String,
}

impl StatusChip {
    #[inline]
    pub fn new(status: impl Into<String>) -> StatusChip {
        StatusChip {
            status: status.into(),
            label: String::new(),
        }
    }

    /// Display text (defaults to `status` when empty).
    #[inline]
    pub fn label(mut self, label: impl Into<String>) -> StatusChip {
        self.label = label.into();
        self
    }
}

#[inline]
pub fn variant_for(status: &str) -> BadgeVariant {
    let s = status.to_lowercase();
    if s == "paid" || s == "matched" || s == "posted" || s == "balanced" || s == "done" {
        BadgeVariant::Success
    } else if s == "open"
        || s == "partial"
        || s == "suggested"
        || s == "customer"
        || s == "progress"
    {
        BadgeVariant::Warning
    } else if s == "unmatched" || s == "overdue" || s == "void" || s == "failed" {
        BadgeVariant::Danger
    } else if s == "draft" || s == "vendor" || s == "employee" {
        BadgeVariant::Info
    } else {
        BadgeVariant::Default
    }
}

impl Renderable for StatusChip {
    #[inline]
    fn render(&self) -> String {
        let status = self.status.clone();
        let text = if self.label.is_empty() {
            status.clone()
        } else {
            self.label.clone()
        };
        let v = variant_for(&status);
        let badge = Badge::new(text)
            .variant(v)
            .size(BadgeSize::Small)
            .render();
        format!(
            "<span class='wj-status-chip' data-wj-status='{}'>{}</span>",
            status, badge
        )
    }
}
