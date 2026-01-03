#![allow(clippy::all)]
#![allow(noop_method_call)]
#[derive(Debug, Clone, PartialEq)]
pub struct Loading {
    pub text: String,
    pub size: LoadingSize,
    pub overlay: bool,
    pub class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum LoadingSize {
    Small,
    Medium,
    Large,
}

impl Loading {
    #[inline]
    pub fn new() -> Loading {
        Loading {
            text: String::new(),
            size: LoadingSize::Medium,
            overlay: false,
            class: String::new(),
        }
    }
    #[inline]
    pub fn text(mut self, text: String) -> Loading {
        self.text = text;
        self
    }
    #[inline]
    pub fn size(mut self, size: LoadingSize) -> Loading {
        self.size = size;
        self
    }
    #[inline]
    pub fn overlay(mut self, overlay: bool) -> Loading {
        self.overlay = overlay;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Loading {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let spinner_size = match self.size {
            LoadingSize::Small => "24px".to_string(),
            LoadingSize::Medium => "40px".to_string(),
            LoadingSize::Large => "64px".to_string(),
        };
        let border_width = match self.size {
            LoadingSize::Small => "2px".to_string(),
            LoadingSize::Medium => "3px".to_string(),
            LoadingSize::Large => "4px".to_string(),
        };
        let mut html = String::new();
        if self.overlay {
            html.push_str("<div class=\"wj-loading-overlay\" style=\"position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 9999;\">")
        }
        html.push_str("<div class=\"wj-loading ");
        html.push_str(&self.class.as_str());
        html.push_str(
            "\" style=\"display: flex; flex-direction: column; align-items: center; gap: 12px;\">",
        );
        html.push_str("<div style=\"width: ");
        html.push_str(&spinner_size);
        html.push_str("; height: ");
        html.push_str(&spinner_size);
        html.push_str("; border: ");
        html.push_str(&border_width);
        html.push_str(" solid #f3f4f6; border-top-color: #3b82f6; border-radius: 50%; animation: spin 0.8s linear infinite;\"></div>");
        if !self.text.is_empty() {
            html.push_str("<span style=\"color: ");
            if self.overlay {
                html.push_str("white")
            } else {
                html.push_str("#6b7280")
            }
            html.push_str("; font-size: 14px;\">");
            html.push_str(&self.text.as_str());
            html.push_str("</span>")
        }
        html.push_str("</div>");
        if self.overlay {
            html.push_str("</div>")
        }
        html.push_str("<style>@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }</style>");
        html
    }
}
