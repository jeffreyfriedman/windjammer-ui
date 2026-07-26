#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct AppShell {
    pub brand: String,
    pub title: String,
    pub header_meta_html: String,
    pub nav_html: String,
    pub main_html: String,
    pub main_id: String,
}

impl AppShell {
    #[inline]
    pub fn new() -> AppShell {
        AppShell {
            brand: "App".to_string(),
            title: "".to_string(),
            header_meta_html: "".to_string(),
            nav_html: "".to_string(),
            main_html: "".to_string(),
            main_id: "main".to_string(),
        }
    }
    #[inline]
    pub fn brand(mut self, brand: String) -> AppShell {
        self.brand = brand;
        self
    }
    #[inline]
    pub fn title(mut self, title: String) -> AppShell {
        self.title = title;
        self
    }
    #[inline]
    pub fn header_meta_html(mut self, html: String) -> AppShell {
        self.header_meta_html = html;
        self
    }
    #[inline]
    pub fn nav_html(mut self, html: String) -> AppShell {
        self.nav_html = html;
        self
    }
    #[inline]
    pub fn main_html(mut self, html: String) -> AppShell {
        self.main_html = html;
        self
    }
    #[inline]
    pub fn main_id(mut self, id: String) -> AppShell {
        self.main_id = id;
        self
    }
}

impl Renderable for AppShell {
    #[inline]
    fn render(&self) -> String {
        let title_block = {
            if !self.title.is_empty() {
                format!(
                    "{}{}{}",
                    "<div class='shell-title'>",
                    self.title.clone(),
                    "</div>"
                )
            } else {
                "".to_string()
            }
        };
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "<div id=\"app\" class=\"wj-app-shell\">",
            "<header class=\"shell-header\">",
            "<div class=\"shell-brand\"><strong>",
            self.brand.clone(),
            "</strong></div>",
            "<div class=\"shell-header-meta\">",
            self.header_meta_html.clone(),
            title_block,
            "</div>",
            "</header>",
            self.nav_html.clone(),
            "<main class=\"shell-main\" id=\"",
            self.main_id.clone(),
            "\">",
            self.main_html.clone(),
            "</main>",
            "</div>"
        )
    }
}
