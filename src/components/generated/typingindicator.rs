#![allow(clippy::all)]
#![allow(noop_method_call)]

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TypingIndicator {
    label: String,
}

impl TypingIndicator {
    #[inline]
    pub fn new() -> TypingIndicator {
        TypingIndicator {
            label: String::from("AI is typing".to_string()),
        }
    }
    #[inline]
    pub fn label(mut self, label: String) -> TypingIndicator {
        self.label = label;
        self
    }
}

impl Renderable for TypingIndicator {
    #[inline]
    fn render(self) -> String {
        format!(
            "<div class='wj-typing-indicator'>
                <div class='wj-typing-dots'>
                    <span class='wj-typing-dot'></span>
                    <span class='wj-typing-dot'></span>
                    <span class='wj-typing-dot'></span>
                </div>
                <span class='wj-typing-label'>{}</span>
            </div>",
            self.label
        )
    }
}
