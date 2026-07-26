#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

/// Retained application chrome (ADR-001 WASM-first hybrid).
/// Hand-tuned `impl Into<String>` builders until WJ emits owned String params
/// (`codegen_string_param_to_owned_method_test.rs`). Source of truth: `components_wj/appshell.wj`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
            title: String::new(),
            header_meta_html: String::new(),
            nav_html: String::new(),
            main_html: String::new(),
            main_id: "main".to_string(),
        }
    }

    #[inline]
    pub fn brand(mut self, brand: impl Into<String>) -> AppShell {
        self.brand = brand.into();
        self
    }

    #[inline]
    pub fn title(mut self, title: impl Into<String>) -> AppShell {
        self.title = title.into();
        self
    }

    #[inline]
    pub fn header_meta_html(mut self, html: impl Into<String>) -> AppShell {
        self.header_meta_html = html.into();
        self
    }

    #[inline]
    pub fn nav_html(mut self, html: impl Into<String>) -> AppShell {
        self.nav_html = html.into();
        self
    }

    #[inline]
    pub fn main_html(mut self, html: impl Into<String>) -> AppShell {
        self.main_html = html.into();
        self
    }

    #[inline]
    pub fn main_id(mut self, id: impl Into<String>) -> AppShell {
        self.main_id = id.into();
        self
    }
}

impl Renderable for AppShell {
    #[inline]
    fn render(&self) -> String {
        let title_block = if self.title.is_empty() {
            String::new()
        } else {
            format!("<div class='shell-title'>{}</div>", self.title)
        };
        format!(
            "<div id=\"app\" class=\"wj-app-shell\"><header class=\"shell-header\"><div class=\"shell-brand\"><strong>{}</strong></div><div class=\"shell-header-meta\">{}{}</div></header>{}<main class=\"shell-main\" id=\"{}\">{}</main></div>",
            self.brand,
            self.header_meta_html,
            title_block,
            self.nav_html,
            self.main_id,
            self.main_html
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appshell_renders_retained_chrome_slots() {
        let html = AppShell::new()
            .brand("LedgerKit")
            .title("Home")
            .header_meta_html("<button type='button' class='cmd-trigger'>Search</button>")
            .nav_html("<nav class='shell-nav' id='shellNav'><a href='#/'>Home</a></nav>")
            .main_html("<div class='panel home-hero'>body</div>")
            .main_id("main")
            .render();
        assert!(html.contains("wj-app-shell"));
        assert!(html.contains("id=\"app\""));
        assert!(html.contains("LedgerKit"));
        assert!(html.contains("shell-header"));
        assert!(html.contains("shell-nav"));
        assert!(html.contains("id=\"main\""));
        assert!(html.contains("home-hero"));
        let app = html.find("id=\"app\"").expect("app");
        let main = html.find("id=\"main\"").expect("main");
        assert!(app < main);
    }
}
