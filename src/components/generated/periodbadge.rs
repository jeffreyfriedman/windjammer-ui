#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::badge::{Badge, BadgeSize, BadgeVariant};
use super::traits::Renderable;

/// Mirrors `components_wj/periodbadge.wj`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct PeriodBadge {
    pub state: String,
    pub label: String,
}

impl PeriodBadge {
    #[inline]
    pub fn new(state: impl Into<String>) -> PeriodBadge {
        PeriodBadge {
            state: state.into(),
            label: String::new(),
        }
    }

    #[inline]
    pub fn label(mut self, label: impl Into<String>) -> PeriodBadge {
        self.label = label.into();
        self
    }
}

#[inline]
pub fn period_variant_for(state: &str) -> BadgeVariant {
    let s = state.to_lowercase();
    if s == "open" {
        BadgeVariant::Success
    } else if s == "locked" {
        BadgeVariant::Danger
    } else if s == "closed" {
        BadgeVariant::Default
    } else {
        BadgeVariant::Info
    }
}

#[inline]
fn state_class(state: &str) -> &'static str {
    let s = state.to_lowercase();
    if s == "open" {
        "wj-period-open"
    } else if s == "locked" {
        "wj-period-locked"
    } else if s == "closed" {
        "wj-period-closed"
    } else {
        "wj-period-unknown"
    }
}

#[inline]
fn display_text(state: &str, label: &str) -> String {
    if label.is_empty() {
        state.to_string()
    } else {
        format!("{} · {}", label, state)
    }
}

impl Renderable for PeriodBadge {
    #[inline]
    fn render(&self) -> String {
        let state = self.state.clone();
        let v = period_variant_for(&state);
        let cls = state_class(&state);
        let text = display_text(&state, &self.label);
        let badge = Badge::new(text)
            .variant(v)
            .size(BadgeSize::Small)
            .render();
        format!(
            "<span class='wj-period-badge {}' data-period-state='{}'>{}</span>",
            cls, state, badge
        )
    }
}
