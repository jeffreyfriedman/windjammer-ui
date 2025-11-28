#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Error,
    Info,
}

pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

pub struct Badge {
    text: String,
    variant: BadgeVariant,
    size: BadgeSize,
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
    fn render(self) -> String {
        let variant_class = match self.variant {
            BadgeVariant::Default => "wj-badge-default",
            BadgeVariant::Primary => "wj-badge-primary",
            BadgeVariant::Success => "wj-badge-success",
            BadgeVariant::Warning => "wj-badge-warning",
            BadgeVariant::Danger => "wj-badge-danger",
            BadgeVariant::Error => "wj-badge-danger",
            BadgeVariant::Info => "wj-badge-info",
        };
        let size_class = match self.size {
            BadgeSize::Small => "wj-badge-sm",
            BadgeSize::Medium => "wj-badge-md",
            BadgeSize::Large => "wj-badge-lg",
        };
        format!(
            "<span class='wj-badge {} {}'>{}</span>",
            variant_class, size_class, self.text
        )
    }
}
