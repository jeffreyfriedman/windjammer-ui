#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::badge::{Badge, BadgeSize, BadgeVariant};
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StatusChip {
    pub status: String,
}

impl StatusChip {
    #[inline]
    pub fn new(status: String) -> StatusChip {
        StatusChip { status }
    }
}

#[inline]
fn variant_for(status: &str) -> BadgeVariant {
    let s = status.to_lowercase();
    if s == "paid" || s == "matched" || s == "posted" || s == "balanced" {
        BadgeVariant::Success
    } else if s == "open" || s == "partial" || s == "suggested" {
        BadgeVariant::Warning
    } else if s == "unmatched" || s == "overdue" || s == "void" || s == "failed" {
        BadgeVariant::Danger
    } else if s == "draft" {
        BadgeVariant::Info
    } else {
        BadgeVariant::Default
    }
}

impl Renderable for StatusChip {
    #[inline]
    fn render(&self) -> String {
        Badge::new(self.status.clone())
            .variant(variant_for(&self.status))
            .size(BadgeSize::Small)
            .render()
    }
}
