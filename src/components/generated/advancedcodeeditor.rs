#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdvancedCodeEditor {
    pub code: String,
    pub language: String,
    pub theme: String,
    pub line_numbers: bool,
    pub minimap: bool,
    pub autocomplete: bool,
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
                " wj-editor-with-minimap".to_string()
            } else {
                "".to_string()
            }
        };
        let line_class = {
            if self.line_numbers {
                " wj-editor-with-lines".to_string()
            } else {
                "".to_string()
            }
        };
        format!(
            "<div class='wj-advanced-editor wj-editor-{} wj-editor-theme-{}{}{}'>
  <div class='wj-editor-toolbar'>
    <span>Language: {}</span>
    <span>Theme: {}</span>
  </div>
  <div class='wj-editor-main'>
    <textarea class='wj-editor-textarea'>
{}</textarea>
    {}
  </div>
</div>",
            self.language,
            self.theme,
            features_class,
            line_class,
            self.language,
            self.theme,
            self.code,
            {
                if self.minimap {
                    "<div class='wj-editor-minimap'></div>".to_string()
                } else {
                    "".to_string()
                }
            }
        )
    }
}
