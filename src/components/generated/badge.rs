#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Error,
    Info,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Badge {
    pub text: String,
    pub variant: BadgeVariant,
    pub size: BadgeSize,
}

impl Badge {
    #[inline]
    pub fn new(text: String) -> Badge {
        Badge {
            text,
            variant: BadgeVariant::Default,
            size: BadgeSize::Medium,
        }
    }
    #[inline]
    pub fn variant(mut self, variant: BadgeVariant) -> Badge {
        self.variant = variant;
        self
    }
    #[inline]
    pub fn size(mut self, size: BadgeSize) -> Badge {
        self.size = size;
        self
    }
}

impl Renderable for Badge {
    #[inline]
    fn render(&self) -> String {
        let variant_class: String = match self.variant {
            BadgeVariant::Default => String::from("wj-badge-default"),
            BadgeVariant::Primary => String::from("wj-badge-primary"),
            BadgeVariant::Success => String::from("wj-badge-success"),
            BadgeVariant::Warning => String::from("wj-badge-warning"),
            BadgeVariant::Danger => String::from("wj-badge-danger"),
            BadgeVariant::Error => String::from("wj-badge-danger"),
            BadgeVariant::Info => String::from("wj-badge-info"),
        };
        let size_class: String = match self.size {
            BadgeSize::Small => String::from("wj-badge-sm"),
            BadgeSize::Medium => String::from("wj-badge-md"),
            BadgeSize::Large => String::from("wj-badge-lg"),
        };
        format!(
            "<span class='wj-badge {} {}'>{}</span>",
            variant_class,
            size_class,
            self.text.clone()
        )
    }
}
