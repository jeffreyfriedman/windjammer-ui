#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

use super::traits::RenderableVNode;

use super::vnode::VNode;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Ghost,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
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
    fn get_variant_class(&self) -> String {
        match self.variant {
            ButtonVariant::Primary => "wj-button-primary".to_string(),
            ButtonVariant::Secondary => "wj-button-secondary".to_string(),
            ButtonVariant::Success => "wj-button-success".to_string(),
            ButtonVariant::Danger => "wj-button-danger".to_string(),
            ButtonVariant::Warning => "wj-button-warning".to_string(),
            ButtonVariant::Ghost => "wj-button-ghost".to_string(),
        }
    }
    #[inline]
    fn get_size_class(&self) -> String {
        match self.size {
            ButtonSize::Small => "wj-button-sm".to_string(),
            ButtonSize::Medium => "wj-button-md".to_string(),
            ButtonSize::Large => "wj-button-lg".to_string(),
        }
    }
    #[inline]
    fn get_style(&self) -> String {
        let base = "border: none; border-radius: 4px; cursor: pointer; font-weight: 500; transition: all 0.2s;".to_string();
        let size_style = match self.size {
            ButtonSize::Small => " padding: 4px 8px; font-size: 12px;",
            ButtonSize::Medium => " padding: 8px 16px; font-size: 14px;",
            ButtonSize::Large => " padding: 12px 24px; font-size: 16px;",
        };
        let variant_style = match self.variant {
            ButtonVariant::Primary => " background: #4A9EFF; color: white;",
            ButtonVariant::Secondary => {
                " background: #333; color: #e0e0e0; border: 1px solid #555;"
            }
            ButtonVariant::Success => " background: #44AA44; color: white;",
            ButtonVariant::Danger => " background: #FF4444; color: white;",
            ButtonVariant::Warning => " background: #FFAA44; color: white;",
            ButtonVariant::Ghost => " background: transparent; color: #4A9EFF;",
        };
        let disabled_style = {
            if self.disabled {
                " opacity: 0.5; cursor: not-allowed;".to_string()
            } else {
                "".to_string()
            }
        };
        format!("{}{}{}{}", base, size_style, variant_style, disabled_style)
    }
}

impl RenderableVNode for Button {
    #[inline]
    fn to_vnode(&self) -> VNode {
        VNode::button()
            .add_class("wj-button".to_string())
            .add_class(self.get_variant_class())
            .add_class(self.get_size_class())
            .add_style(self.get_style())
            .set_disabled(self.disabled)
            .add_text(self.label.clone())
    }
}

impl Renderable for Button {
    #[inline]
    fn render(self) -> String {
        let variant_class = match self.variant {
            ButtonVariant::Primary => "wj-button-primary",
            ButtonVariant::Secondary => "wj-button-secondary",
            ButtonVariant::Success => "wj-button-success",
            ButtonVariant::Danger => "wj-button-danger",
            ButtonVariant::Warning => "wj-button-warning",
            ButtonVariant::Ghost => "wj-button-ghost",
        };
        let size_class = match self.size {
            ButtonSize::Small => "wj-button-sm",
            ButtonSize::Medium => "wj-button-md",
            ButtonSize::Large => "wj-button-lg",
        };
        let disabled_attr = {
            if self.disabled {
                " disabled='true'".to_string()
            } else {
                "".to_string()
            }
        };
        format!(
            "<button class='wj-button {} {}' style='{}'{}>{}</button>",
            variant_class,
            size_class,
            self.get_style(),
            disabled_attr,
            self.label
        )
    }
}
