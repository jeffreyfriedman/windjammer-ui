enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Ghost,
}

enum ButtonSize {
    Small,
    Medium,
    Large,
}

struct Button {
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
}

impl Button {
#[inline]
fn new(label: String) -> Button {
        Button { label, variant: ButtonVariant::Primary, size: ButtonSize::Medium, disabled: false }
}
#[inline]
fn variant(mut self, variant: ButtonVariant) -> Button {
        self.variant = variant;
        self
}
#[inline]
fn size(mut self, size: ButtonSize) -> Button {
        self.size = size;
        self
}
#[inline]
fn disabled(mut self, disabled: bool) -> Button {
        self.disabled = disabled;
        self
}
#[inline]
fn render(mut self) -> String {
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
        format!("<button class='wj-button {} {}'{}>{}</button>", variant_class, size_class, disabled_attr, self.label)
}
}

fn main() {
    let button1 = Button::new("Click Me".to_string()).variant(ButtonVariant::Primary).size(ButtonSize::Large);
    let button2 = Button::new("Cancel".to_string()).variant(ButtonVariant::Danger).disabled(true);
    println!("Button 1: {}", button1.render());
    println!("Button 2: {}", button2.render())
}

