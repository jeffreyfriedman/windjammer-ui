#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Form {
    pub id: String,
    pub action: String,
    pub method: String,
    pub children: Vec<String>,
    pub on_submit: String,
}

impl Form {
#[inline]
pub fn new(id: String) -> Form {
        Form { id, action: String::from("#"), method: String::from("POST"), children: Vec::new(), on_submit: String::from("return false;") }
}
#[inline]
pub fn action(mut self, action: impl Into<String>) -> Form {
        self.action = action.into();
        self
}
#[inline]
pub fn method(mut self, method: impl Into<String>) -> Form {
        self.method = method.into();
        self
}
#[inline]
pub fn on_submit(mut self, handler: impl Into<String>) -> Form {
        self.on_submit = handler.into();
        self
}
#[inline]
pub fn child(mut self, child: impl Into<String>) -> Form {
        self.children.push(child.into());
        self
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct FormField {
    pub label: String,
    pub input: String,
    pub error: String,
    pub required: bool,
    pub help_text: String,
}

impl FormField {
#[inline]
pub fn new(label: String, input: String) -> FormField {
        FormField { label, input, error: String::new(), required: false, help_text: String::new() }
}
#[inline]
pub fn required(mut self, required: bool) -> FormField {
        self.required = required;
        self
}
#[inline]
pub fn error(mut self, error: impl Into<String>) -> FormField {
        self.error = error.into();
        self
}
#[inline]
pub fn help_text(mut self, text: impl Into<String>) -> FormField {
        self.help_text = text.into();
        self
}

pub fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class='wj-form-field' style='margin-bottom: 16px;'>");
        html.push_str("<label class='wj-form-field-label' style='display: block; margin-bottom: 4px; font-weight: 500; color: #333;'>");
        html.push_str(&self.label);
        if self.required {
            let _ = html.push_str(" <span style='color: #e53e3e;'>*</span>");
        }
        html.push_str("</label>");
        html.push_str(&self.input);
        if !self.help_text.is_empty() {
            let _ = html.push_str("<div class='wj-form-field-hint' style='margin-top: 4px; font-size: 12px; color: #718096;'>");
            let _ = html.push_str(&self.help_text);
            let _ = html.push_str("</div>");
        }
        if !self.error.is_empty() {
            let _ = html.push_str("<div class='wj-form-field-error' role='alert' style='margin-top: 4px; font-size: 12px; color: #e53e3e;'>");
            let _ = html.push_str(&self.error);
            let _ = html.push_str("</div>");
        }
        html.push_str("</div>");
        html
}
}

impl Renderable for Form {
#[inline]
fn render(&self) -> String {
        let mut html = String::new();
        html.push_str("<form id='");
        html.push_str(&self.id);
        html.push_str("' action='");
        html.push_str(&self.action);
        html.push_str("' method='");
        html.push_str(&self.method);
        html.push_str("' onsubmit='");
        html.push_str(&self.on_submit);
        html.push_str("'>");
        for child in &self.children {
            html.push_str(child);
        }
        html.push_str("</form>");
        html
}
}

