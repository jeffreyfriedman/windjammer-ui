pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Ghost,
}

pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

pub struct Button {
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
}

impl Button {
    #[inline]
    pub fn new(label: String) -> Button {
        Button {
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
        }
    }
    #[inline]
    pub fn variant(mut self, variant: ButtonVariant) -> Button {
        self.variant = variant;
        self
    }
    #[inline]
    pub fn size(mut self, size: ButtonSize) -> Button {
        self.size = size;
        self
    }
    #[inline]
    pub fn disabled(mut self, disabled: bool) -> Button {
        self.disabled = disabled;
        self
    }
    #[inline]
    pub fn render(mut self) -> String {
        let variant_class = match self.variant {
            ButtonVariant::Primary => "wj-button-primary",
            ButtonVariant::Secondary => "wj-button-secondary",
            ButtonVariant::Success => "wj-button-success",
            ButtonVariant::Danger => "wj-button-danger",
            ButtonVariant::Ghost => "wj-button-ghost",
        };
        let size_class = match self.size {
            ButtonSize::Small => "wj-button-sm",
            ButtonSize::Medium => "wj-button-md",
            ButtonSize::Large => "wj-button-lg",
        };
        let disabled_attr = {
            if self.disabled {
                " disabled='true'"
            } else {
                ""
            }
        };
        format!(
            "<button class='wj-button {} {}'{}>{}</button>",
            variant_class, size_class, disabled_attr, self.label
        )
    }
}
