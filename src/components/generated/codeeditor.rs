#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CodeEditor {
    code: String,
    language: String,
    theme: String,
    line_numbers: bool,
    readonly: bool,
}

impl CodeEditor {
    #[inline]
    pub fn new(code: String) -> CodeEditor {
        CodeEditor {
            code,
            language: "rust".to_string(),
            theme: "dark".to_string(),
            line_numbers: true,
            readonly: false,
        }
    }
    #[inline]
    pub fn language(mut self, language: String) -> CodeEditor {
        self.language = language;
        self
    }
    #[inline]
    pub fn theme(mut self, theme: String) -> CodeEditor {
        self.theme = theme;
        self
    }
    #[inline]
    pub fn line_numbers(mut self, show: bool) -> CodeEditor {
        self.line_numbers = show;
        self
    }
    #[inline]
    pub fn readonly(mut self, readonly: bool) -> CodeEditor {
        self.readonly = readonly;
        self
    }
}

impl Renderable for CodeEditor {
    #[inline]
    fn render(self) -> String {
        let readonly_attr = {
            if self.readonly {
                " readonly".to_string()
            } else {
                "".to_string()
            }
        };
        let line_numbers_class = {
            if self.line_numbers {
                " wj-editor-with-lines".to_string()
            } else {
                "".to_string()
            }
        };
        format!(
            "<div class='wj-code-editor wj-editor-{} wj-editor-theme-{}{}'>
  <textarea{}>
{}</textarea>
</div>",
            self.language, self.theme, line_numbers_class, readonly_attr, self.code
        )
    }
}
