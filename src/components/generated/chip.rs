#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum ChipVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChipSize {
    Small,
    Medium,
    Large,
}

pub struct Chip {
    label: String,
    variant: ChipVariant,
    size: ChipSize,
    removable: bool,
    icon: String,
}

impl Chip {
    #[inline]
    pub fn new(label: String) -> Chip {
        Chip {
            label,
            variant: ChipVariant::Default,
            size: ChipSize::Medium,
            removable: false,
            icon: String::new(),
        }
    }
    #[inline]
    pub fn variant(mut self, variant: ChipVariant) -> Chip {
        self.variant = variant;
        self
    }
    #[inline]
    pub fn size(mut self, size: ChipSize) -> Chip {
        self.size = size;
        self
    }
    #[inline]
    pub fn removable(mut self, removable: bool) -> Chip {
        self.removable = removable;
        self
    }
    #[inline]
    pub fn icon(mut self, icon: String) -> Chip {
        self.icon = icon;
        self
    }
}

impl Renderable for Chip {
    fn render(self) -> String {
        let bg_color = match self.variant {
            ChipVariant::Default => "#e2e8f0",
            ChipVariant::Primary => "#3b82f6",
            ChipVariant::Success => "#10b981",
            ChipVariant::Warning => "#f59e0b",
            ChipVariant::Danger => "#ef4444",
            ChipVariant::Info => "#06b6d4",
        };
        let text_color = match self.variant {
            ChipVariant::Default => "#2d3748",
            ChipVariant::Primary => "white",
            ChipVariant::Success => "white",
            ChipVariant::Warning => "white",
            ChipVariant::Danger => "white",
            ChipVariant::Info => "white",
        };
        let border_color = match self.variant {
            ChipVariant::Default => "#cbd5e0",
            ChipVariant::Primary => "#2563eb",
            ChipVariant::Success => "#059669",
            ChipVariant::Warning => "#d97706",
            ChipVariant::Danger => "#dc2626",
            ChipVariant::Info => "#0891b2",
        };
        let padding = match self.size {
            ChipSize::Small => "4px 8px",
            ChipSize::Medium => "6px 12px",
            ChipSize::Large => "8px 16px",
        };
        let font_size = match self.size {
            ChipSize::Small => "12px",
            ChipSize::Medium => "14px",
            ChipSize::Large => "16px",
        };
        let mut html = String::new();
        html.push_str(
            "<span style='display: inline-flex; align-items: center; gap: 6px; padding: ",
        );
        html.push_str(padding);
        html.push_str("; font-size: ");
        html.push_str(font_size);
        html.push_str("; font-weight: 500; border-radius: 16px; background: ");
        html.push_str(bg_color);
        html.push_str("; color: ");
        html.push_str(text_color);
        html.push_str("; border: 1px solid ");
        html.push_str(border_color);
        html.push_str(";'>");
        if self.icon.len() > 0 {
            html.push_str("<span>");
            html.push_str(&self.icon);
            html.push_str("</span>")
        }
        html.push_str("<span>");
        html.push_str(&self.label);
        html.push_str("</span>");
        if self.removable {
            html.push_str("<button onclick='this.parentElement.remove()' style='background: none; border: none; cursor: pointer; padding: 0; margin: 0; display: flex; align-items: center; color: ");
            html.push_str(text_color);
            html.push_str("; opacity: 0.7; font-size: 18px; line-height: 1;' onmouseover='this.style.opacity=\"1\"' onmouseout='this.style.opacity=\"0.7\"'>&times;</button>")
        }
        html.push_str("</span>");
        html
    }
}
