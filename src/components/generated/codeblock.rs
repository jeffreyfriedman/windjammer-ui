#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CodeBlock {
    code: String,
    language: String,
    show_line_numbers: bool,
    show_copy_button: bool,
}

impl CodeBlock {
    #[inline]
    pub fn new(code: String) -> CodeBlock {
        CodeBlock {
            code,
            language: String::from("".to_string()),
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
    fn render(self) -> String {
        let language_label = {
            if (self.language.len() as i32) > 0 {
                format!("<div class='wj-codeblock-language'>{}</div>", self.language)
            } else {
                String::from("".to_string())
            }
        };
        let copy_button = {
            if self.show_copy_button {
                format!("<button class='wj-codeblock-copy' onclick='navigator.clipboard.writeText(this.parentElement.querySelector(\"code\").textContent); this.textContent=\"✓ Copied!\"; setTimeout(() => this.textContent=\"📋 Copy\", 2000)'>
                    📋 Copy
                </button>")
            } else {
                String::from("".to_string())
            }
        };
        let line_number_class = {
            if self.show_line_numbers {
                " wj-codeblock-numbered".to_string()
            } else {
                "".to_string()
            }
        };
        format!(
            "<div class='wj-codeblock{}'>
                <div class='wj-codeblock-header'>
                    {}
                    {}
                </div>
                <pre class='wj-codeblock-pre'><code class='wj-codeblock-code'>{}</code></pre>
            </div>",
            line_number_class, language_label, copy_button, self.code
        )
    }
}
