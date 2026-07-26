#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum IconName {
    Home,
    Money,
    Invoice,
    Bill,
    Books,
    Report,
    Settings,
    Sync,
    Search,
    Check,
    Alert,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Icon {
    pub name: IconName,
    pub size: i32,
    pub label: String,
}

impl Icon {
    #[inline]
    pub fn new(name: IconName) -> Icon {
        Icon {
            name,
            size: 18_i32,
            label: "".to_string(),
        }
    }
    #[inline]
    pub fn size(mut self, size: i32) -> Icon {
        self.size = size;
        self
    }
    #[inline]
    pub fn label(mut self, label: String) -> Icon {
        self.label = label;
        self
    }
}

#[inline]
fn path_for(name: IconName) -> String {
    match name {
        IconName::Home => {
            String::from("M3 10.5L12 3l9 7.5V21a1 1 0 0 1-1 1h-5v-7H9v7H4a1 1 0 0 1-1-1v-10.5z")
        }
        IconName::Money => String::from("M4 7h16v10H4V7zm2 2v6h12V9H6zm3 2h6v2H9v-2z"),
        IconName::Invoice => {
            String::from("M6 3h12v18l-3-2-3 2-3-2-3 2V3zm3 4h6v2H9V7zm0 4h6v2H9v-2z")
        }
        IconName::Bill => String::from("M5 4h14v16H5V4zm3 3h8v2H8V7zm0 4h8v2H8v-2zm0 4h5v2H8v-2z"),
        IconName::Books => String::from("M4 4h6v16H4V4zm10 0h6v16h-6V4zM6 7h2v2H6V7zm10 0h2v2h-2V7z"),
        IconName::Report => String::from("M5 19V9h3v10H5zm5 0V5h3v14h-3zm5 0v-7h3v7h-3z"),
        IconName::Settings => String::from(
            "M12 8a4 4 0 1 1 0 8 4 4 0 0 1 0-8zm0-5l1.2 2.4 2.7.4-1.9 2.1.5 2.7L12 11.8 9.5 13.6l.5-2.7-1.9-2.1 2.7-.4L12 3z",
        ),
        IconName::Sync => String::from(
            "M4 12a8 8 0 0 1 13.5-5.8L20 4v6h-6l2.2-2.2A6 6 0 1 0 18 12h2a8 8 0 0 1-16 0z",
        ),
        IconName::Search => String::from(
            "M10 4a6 6 0 1 1 0 12 6 6 0 0 1 0-12zm0 2a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm6.7 10.3l3 3-1.4 1.4-3-3 1.4-1.4z",
        ),
        IconName::Check => String::from("M5 12l4 4L19 6"),
        IconName::Alert => String::from("M12 3l10 18H2L12 3zm0 5v6h0V8zm0 8v2h0v-2z"),
    }
}

impl Renderable for Icon {
    #[inline]
    fn render(&self) -> String {
        let d = path_for(self.name);
        let aria: String = {
            if self.label.is_empty() {
                String::from("aria-hidden='true'")
            } else {
                format!("role='img' aria-label='{}'", self.label)
            }
        };
        let stroke: String = match self.name {
            IconName::Check => String::from(
                "stroke='currentColor' stroke-width='2' fill='none' stroke-linecap='round' stroke-linejoin='round'",
            ),
            _ => String::from("fill='currentColor'"),
        };
        format!(
            "<svg class='wj-icon' width='{}' height='{}' viewBox='0 0 24 24' {} {}><path d='{}'/></svg>",
            self.size, self.size, aria, stroke, d
        )
    }
}
