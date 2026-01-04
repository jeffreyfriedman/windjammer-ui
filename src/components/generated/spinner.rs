#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SpinnerSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spinner {
    pub size: SpinnerSize,
    pub label: String,
}

impl Spinner {
    #[inline]
    pub fn new() -> Spinner {
        Spinner {
            size: SpinnerSize::Medium,
            label: "".to_string(),
        }
    }
    #[inline]
    pub fn size(mut self, size: SpinnerSize) -> Spinner {
        self.size = size;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> Spinner {
        self.label = label;
        self
    }
}

impl Renderable for Spinner {
    #[inline]
    fn render(&self) -> String {
        let size_class = match self.size {
            SpinnerSize::Small => "wj-spinner-sm".to_string(),
            SpinnerSize::Medium => "wj-spinner-md".to_string(),
            SpinnerSize::Large => "wj-spinner-lg".to_string(),
        };
        let spinner_html = {
            let mut __s = String::with_capacity(64);
            write!(&mut __s, "<div class='wj-spinner {}'></div>", size_class).unwrap();
            __s
        };
        if self.label != "" {
            format!("<div class='wj-spinner-container'>{}<span class='wj-spinner-label'>{}</span></div>", spinner_html, self.label)
        } else {
            spinner_html
        }
    }
}
