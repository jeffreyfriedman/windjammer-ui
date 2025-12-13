#![allow(clippy::all)]
#![allow(noop_method_call)]
use std::fmt::Write;

use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Card {
    title: String,
    children: Vec<String>,
    padding: String,
    background_color: String,
    border_color: String,
}

impl Card {
    #[inline]
    pub fn new() -> Card {
        Card {
            title: "".to_string(),
            children: Vec::new(),
            padding: "16px".to_string(),
            background_color: "#fff".to_string(),
            border_color: "#e0e0e0".to_string(),
        }
    }
    #[inline]
    pub fn title(mut self, title: String) -> Card {
        self.title = title;
        self
    }
    #[inline]
    pub fn child(mut self, child: String) -> Card {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn children(mut self, children: Vec<String>) -> Card {
        self.children = children;
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Card {
        self.padding = padding;
        self
    }
    #[inline]
    pub fn background_color(mut self, color: String) -> Card {
        self.background_color = color;
        self
    }
    #[inline]
    pub fn border_color(mut self, color: String) -> Card {
        self.border_color = color;
        self
    }
}

impl Renderable for Card {
    #[inline]
    fn render(self) -> String {
        let style = {
            let mut __s = String::with_capacity(64);
            write!(
                &mut __s,
                "padding: {}; background-color: {}; border: 1px solid {}; border-radius: 8px;",
                self.padding, self.background_color, self.border_color
            )
            .unwrap();
            __s
        };
        let title_html = {
            if self.title != "" {
                {
                    let mut __s = String::with_capacity(64);
                    write!(&mut __s, "<div class='wj-card-title' style='font-weight: bold; margin-bottom: 12px; font-size: 1.25rem;'>{}</div>", self.title).unwrap();
                    __s
                }
            } else {
                "".to_string()
            }
        };
        let children_html = self.children.join(
            "
",
        );
        format!(
            "<div class='wj-card' style='{}'>
{}{}
</div>",
            style, title_html, children_html
        )
    }
}
