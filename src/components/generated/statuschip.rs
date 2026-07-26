#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::badge::{Badge, BadgeSize, BadgeVariant};
use super::traits::Renderable;

/// Mirrors `components_wj/statuschip.wj`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct StatusChip {
    pub status: String,
}

impl StatusChip {
    #[inline]
    pub fn new(status: impl Into<String>) -> StatusChip {
        StatusChip {
            status: status.into(),
        }
    }
}

impl Renderable for StatusChip {
    #[inline]
    fn render(&self) -> String {
        let status = self.status.clone();
        let v = variant_for(&status);
        Badge::new(status)
            .variant(v)
            .size(BadgeSize::Small)
            .render()
    }
}

#[inline]
pub fn variant_for(status: &str) -> BadgeVariant {
    let s = status.to_lowercase();
    if s == "paid" || s == "matched" || s == "posted" || s == "balanced" {
        BadgeVariant::Success
    } else if s == "open" || s == "partial" || s == "suggested" || s == "customer" {
        BadgeVariant::Warning
    } else if s == "unmatched" || s == "overdue" || s == "void" || s == "failed" {
        BadgeVariant::Danger
    } else if s == "draft" || s == "vendor" || s == "employee" {
        BadgeVariant::Info
    } else {
        BadgeVariant::Default
    }
}
