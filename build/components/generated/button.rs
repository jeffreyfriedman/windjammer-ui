#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

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
#[repr(C)]
pub struct Button {
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    click_handler: String,
}

impl Button {
    #[inline]
    pub fn new(label: String) -> Button {
        Button {
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            click_handler: String::new(),
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
    /// Set click handler by name (looked up in event dispatcher at render time)
    #[inline]
    pub fn on_click(mut self, handler: String) -> Button {
        self.click_handler = handler;
        self
    }
    #[inline]
    pub fn get_variant_class(&self) -> String {
        match self.variant {
            ButtonVariant::Primary => String::from("wj-button-primary").to_string(),
            ButtonVariant::Secondary => String::from("wj-button-secondary").to_string(),
            ButtonVariant::Success => String::from("wj-button-success").to_string(),
            ButtonVariant::Danger => String::from("wj-button-danger").to_string(),
            ButtonVariant::Warning => String::from("wj-button-warning").to_string(),
            ButtonVariant::Ghost => String::from("wj-button-ghost").to_string(),
        }
    }
    #[inline]
    pub fn get_size_class(&self) -> String {
        match self.size {
            ButtonSize::Small => String::from("wj-button-sm").to_string(),
            ButtonSize::Medium => String::from("wj-button-md").to_string(),
            ButtonSize::Large => String::from("wj-button-lg").to_string(),
        }
    }
    #[inline]
    pub fn get_style(&self) -> String {
        let base = "border: none; border-radius: 4px; cursor: pointer; font-weight: 500; transition: all 0.2s;".to_string();
        let size_style: String = match self.size {
            ButtonSize::Small => String::from(" padding: 4px 8px; font-size: 12px;"),
            ButtonSize::Medium => String::from(" padding: 8px 16px; font-size: 14px;"),
            ButtonSize::Large => String::from(" padding: 12px 24px; font-size: 16px;"),
        };
        let variant_style: String = match self.variant {
            ButtonVariant::Primary => String::from(" background: #4A9EFF; color: white;"),
            ButtonVariant::Secondary => {
                String::from(" background: #333; color: #e0e0e0; border: 1px solid #555;")
            }
            ButtonVariant::Success => String::from(" background: #44AA44; color: white;"),
            ButtonVariant::Danger => String::from(" background: #FF4444; color: white;"),
            ButtonVariant::Warning => String::from(" background: #FFAA44; color: white;"),
            ButtonVariant::Ghost => String::from(" background: transparent; color: #4A9EFF;"),
        };
        let disabled_style = {
            if self.disabled {
                String::from(" opacity: 0.5; cursor: not-allowed;")
            } else {
                String::new()
            }
        };
        format!("{}{}{}{}", base, size_style, variant_style, disabled_style)
    }
}

impl RenderableVNode for Button {
    #[inline]
    fn to_vnode(&self) -> VNode {
        let mut node = VNode::button()
            .add_class("wj-button")
            .add_class(&self.get_variant_class())
            .add_class(&self.get_size_class())
            .add_style(&self.get_style())
            .set_disabled(self.disabled)
            .add_text(&self.label);
        if !self.click_handler.is_empty() {
            node = node.on_click(&self.click_handler);
        }
        node
    }
}

impl Renderable for Button {
    #[inline]
    fn render(self) -> String {
        let variant_class: String = match self.variant {
            ButtonVariant::Primary => String::from("wj-button-primary"),
            ButtonVariant::Secondary => String::from("wj-button-secondary"),
            ButtonVariant::Success => String::from("wj-button-success"),
            ButtonVariant::Danger => String::from("wj-button-danger"),
            ButtonVariant::Warning => String::from("wj-button-warning"),
            ButtonVariant::Ghost => String::from("wj-button-ghost"),
        };
        let size_class: String = match self.size {
            ButtonSize::Small => String::from("wj-button-sm"),
            ButtonSize::Medium => String::from("wj-button-md"),
            ButtonSize::Large => String::from("wj-button-lg"),
        };
        let disabled_attr = {
            if self.disabled {
                String::from(" disabled='true'")
            } else {
                String::new()
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
