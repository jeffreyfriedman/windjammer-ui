#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ChipVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ChipSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Chip {
    pub label: String,
    pub variant: ChipVariant,
    pub size: ChipSize,
    pub removable: bool,
    pub icon: String,
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
    #[inline]
    fn render(&mut self) -> String {
        let bg_color: String = match self.variant {
            ChipVariant::Default => String::from("#e2e8f0"),
            ChipVariant::Primary => String::from("#3b82f6"),
            ChipVariant::Success => String::from("#10b981"),
            ChipVariant::Warning => String::from("#f59e0b"),
            ChipVariant::Danger => String::from("#ef4444"),
            ChipVariant::Info => String::from("#06b6d4"),
        };
        let text_color: String = match self.variant {
            ChipVariant::Default => String::from("#2d3748"),
            ChipVariant::Primary => String::from("white"),
            ChipVariant::Success => String::from("white"),
            ChipVariant::Warning => String::from("white"),
            ChipVariant::Danger => String::from("white"),
            ChipVariant::Info => String::from("white"),
        };
        let border_color: String = match self.variant {
            ChipVariant::Default => String::from("#cbd5e0"),
            ChipVariant::Primary => String::from("#2563eb"),
            ChipVariant::Success => String::from("#059669"),
            ChipVariant::Warning => String::from("#d97706"),
            ChipVariant::Danger => String::from("#dc2626"),
            ChipVariant::Info => String::from("#0891b2"),
        };
        let padding: String = match self.size {
            ChipSize::Small => String::from("4px 8px"),
            ChipSize::Medium => String::from("6px 12px"),
            ChipSize::Large => String::from("8px 16px"),
        };
        let font_size: String = match self.size {
            ChipSize::Small => String::from("12px"),
            ChipSize::Medium => String::from("14px"),
            ChipSize::Large => String::from("16px"),
        };
        let mut html = String::new();
        html.push_str(
            "<span style='display: inline-flex; align-items: center; gap: 6px; padding: ",
        );
        html.push_str(&padding);
        html.push_str("; font-size: ");
        html.push_str(&font_size);
        html.push_str("; font-weight: 500; border-radius: 16px; background: ");
        html.push_str(&bg_color);
        html.push_str("; color: ");
        html.push_str(&text_color);
        html.push_str("; border: 1px solid ");
        html.push_str(&border_color);
        html.push_str(";'>");
        if !self.icon.is_empty() {
            html.push_str("<span>");
            html.push_str(&self.icon);
            html.push_str("</span>");
        }
        html.push_str("<span>");
        html.push_str(&self.label);
        html.push_str("</span>");
        if self.removable {
            html.push_str("<button onclick='this.parentElement.remove()' style='background: none; border: none; cursor: pointer; padding: 0; margin: 0; display: flex; align-items: center; color: ");
            html.push_str(&text_color);
            html.push_str("; opacity: 0.7; font-size: 18px; line-height: 1;' onmouseover='this.style.opacity=\"1\"' onmouseout='this.style.opacity=\"0.7\"'>&times;</button>");
        }
        html.push_str("</span>");
        html
    }
}
