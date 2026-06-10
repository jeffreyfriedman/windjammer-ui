#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct AdvancedCodeEditor {
    code: String,
    language: String,
    theme: String,
    line_numbers: bool,
    minimap: bool,
    autocomplete: bool,
}

impl AdvancedCodeEditor {
    #[inline]
    pub fn new(code: String) -> AdvancedCodeEditor {
        AdvancedCodeEditor {
            code,
            language: "rust".to_string(),
            theme: "monokai".to_string(),
            line_numbers: true,
            minimap: true,
            autocomplete: true,
        }
    }
    #[inline]
    pub fn language(mut self, language: String) -> AdvancedCodeEditor {
        self.language = language;
        self
    }
    #[inline]
    pub fn theme(mut self, theme: String) -> AdvancedCodeEditor {
        self.theme = theme;
        self
    }
    #[inline]
    pub fn line_numbers(mut self, show: bool) -> AdvancedCodeEditor {
        self.line_numbers = show;
        self
    }
    #[inline]
    pub fn minimap(mut self, show: bool) -> AdvancedCodeEditor {
        self.minimap = show;
        self
    }
    #[inline]
    pub fn autocomplete(mut self, enable: bool) -> AdvancedCodeEditor {
        self.autocomplete = enable;
        self
    }
}

impl Renderable for AdvancedCodeEditor {
    #[inline]
    fn render(self) -> String {
        let features_class = {
            if self.minimap {
                String::from(" wj-editor-with-minimap")
            } else {
                String::new()
            }
        };
        let line_class = {
            if self.line_numbers {
                String::from(" wj-editor-with-lines")
            } else {
                String::new()
            }
        };
        format!("<div class='wj-advanced-editor wj-editor-{} wj-editor-theme-{}{}{}'>\n  <div class='wj-editor-toolbar'>\n    <span>Language: {}</span>\n    <span>Theme: {}</span>\n  </div>\n  <div class='wj-editor-main'>\n    <textarea class='wj-editor-textarea'>\n{}</textarea>\n    {}\n  </div>\n</div>", self.language, self.theme, features_class, line_class, self.language, self.theme, self.code, {
            if self.minimap {
                "<div class='wj-editor-minimap'></div>"
            } else {
                ""
            }
        })
    }
}
