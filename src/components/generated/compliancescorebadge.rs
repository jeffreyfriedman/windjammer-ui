#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Regenerated from `components_wj/compliancescorebadge.wj` — Windjammer is source of truth.
//! Note: avoid `use super::*` (ambiguous glob imports under wasm deny).

use std::fmt::Write;
use super::traits::Renderable;
use super::badge::{Badge, BadgeVariant, BadgeSize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct ComplianceScoreBadge {
    pub score: i64,
    pub risk_band: String,
}

impl ComplianceScoreBadge {
#[inline]
pub fn new(score: i64, risk_band: String) -> ComplianceScoreBadge {
        ComplianceScoreBadge { score, risk_band }
}
}

impl Renderable for ComplianceScoreBadge {
#[inline]
fn render(&self) -> String {
        let score = self.score;
        let band = self.risk_band.clone();
        let score_s = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "{}", score).unwrap();
            __s
        };
        let label = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "{} · {}", score_s, band).unwrap();
            __s
        };
        let badge = Badge::new(label).variant(band_badge_variant(&band)).size(BadgeSize::Medium).render();
        format!("<div class='wj-compliance-score' data-wj-compliance-score data-wj-compliance-band='{}' data-wj-compliance-value='{}'>{}</div>", band, score_s, badge)
}
}

#[inline]
pub fn band_badge_variant(band: &str) -> BadgeVariant {
    let b = band.to_lowercase();
    if b == "ready" {
        BadgeVariant::Success
    } else {
        if b == "watch" {
            BadgeVariant::Warning
        } else {
            if b == "gap" {
                BadgeVariant::Danger
            } else {
                BadgeVariant::Info
            }
        }
    }
}

