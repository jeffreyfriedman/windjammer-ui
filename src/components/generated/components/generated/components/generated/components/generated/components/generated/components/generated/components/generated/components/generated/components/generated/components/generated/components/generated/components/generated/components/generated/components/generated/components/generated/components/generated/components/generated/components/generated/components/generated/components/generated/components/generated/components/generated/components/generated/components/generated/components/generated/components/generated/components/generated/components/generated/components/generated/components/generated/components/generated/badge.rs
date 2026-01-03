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
pub struct Badge {
    pub text: String,
    pub variant: BadgeVariant,
    pub size: BadgeSize,
}

impl Badge {
#[inline]
pub fn new(text: String) -> Badge {
        Badge { text, variant: BadgeVariant::Default, size: BadgeSize::Medium }
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
            BadgeVariant::Default => "wj-badge-default".to_string(),
            BadgeVariant::Primary => "wj-badge-primary".to_string(),
            BadgeVariant::Success => "wj-badge-success".to_string(),
            BadgeVariant::Warning => "wj-badge-warning".to_string(),
            BadgeVariant::Danger => "wj-badge-danger".to_string(),
            BadgeVariant::Error => "wj-badge-danger".to_string(),
            BadgeVariant::Info => "wj-badge-info".to_string(),
        };
        let size_class = match self.size {
            BadgeSize::Small => "wj-badge-sm".to_string(),
            BadgeSize::Medium => "wj-badge-md".to_string(),
            BadgeSize::Large => "wj-badge-lg".to_string(),
        };
        format!("<span class='wj-badge {} {}'>{}</span>", variant_class, size_class, self.text)
}
}

