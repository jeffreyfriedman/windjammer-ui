#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct TypingIndicator {
    pub label: String,
}

impl TypingIndicator {
#[inline]
pub fn new() -> TypingIndicator {
        TypingIndicator { label: String::from("AI is typing") }
}
#[inline]
pub fn label(mut self, label: impl Into<String>) -> TypingIndicator {
        self.label = label.into();
        self
}
}

impl Renderable for TypingIndicator {
#[inline]
fn render(&self) -> String {
        format!("<div class='wj-typing-indicator'>\n                <div class='wj-typing-dots'>\n                    <span class='wj-typing-dot'></span>\n                    <span class='wj-typing-dot'></span>\n                    <span class='wj-typing-dot'></span>\n                </div>\n                <span class='wj-typing-label'>{}</span>\n            </div>", self.label)
}
}

