#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct CodeBlock {
    pub code: String,
    pub language: String,
    pub show_line_numbers: bool,
    pub show_copy_button: bool,
}

impl CodeBlock {
    #[inline]
    pub fn new(code: String) -> CodeBlock {
        CodeBlock {
            code: code.to_string(),
            language: String::from(""),
            show_line_numbers: false,
            show_copy_button: true,
        }
    }
    #[inline]
    pub fn language(mut self, language: String) -> CodeBlock {
        self.language = language;
        self
    }
    #[inline]
    pub fn show_line_numbers(mut self, show: bool) -> CodeBlock {
        self.show_line_numbers = show;
        self
    }
    #[inline]
    pub fn show_copy_button(mut self, show: bool) -> CodeBlock {
        self.show_copy_button = show;
        self
    }
}

impl Renderable for CodeBlock {
    #[inline]
    fn render(&self) -> String {
        let language_label = {
            if !self.language.is_empty() {
                format!(
                    "<div class='wj-codeblock-language'>{}</div>",
                    self.language.clone()
                )
            } else {
                String::from("")
            }
        };
        let copy_button = {
            if self.show_copy_button {
                format!("<button class='wj-codeblock-copy' onclick='navigator.clipboard.writeText(this.parentElement.querySelector(\"code\").textContent); this.textContent=\"✓ Copied!\"; setTimeout(() => this.textContent=\"📋 Copy\", 2000)'>\n                    📋 Copy\n                </button>")
            } else {
                String::from("")
            }
        };
        let line_number_class = {
            if self.show_line_numbers {
                " wj-codeblock-numbered".to_string()
            } else {
                "".to_string()
            }
        };
        format!("<div class='wj-codeblock{}'>\n                <div class='wj-codeblock-header'>\n                    {}\n                    {}\n                </div>\n                <pre class='wj-codeblock-pre'><code class='wj-codeblock-code'>{}</code></pre>\n            </div>", line_number_class, language_label, copy_button, self.code.clone())
    }
}
