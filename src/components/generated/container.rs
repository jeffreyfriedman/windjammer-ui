#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Container {
    pub children: Vec<String>,
    pub max_width: String,
    pub max_height: String,
    pub padding: String,
    pub background_color: String,
}

impl Container {
    #[inline]
    pub fn new() -> Container {
        Container {
            children: Vec::new(),
            max_width: "".to_string(),
            max_height: "".to_string(),
            padding: "16px".to_string(),
            background_color: "".to_string(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Container {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn children(mut self, children: Vec<String>) -> Container {
        self.children = children;
        self
    }
    #[inline]
    pub fn max_width(mut self, width: String) -> Container {
        self.max_width = width;
        self
    }
    #[inline]
    pub fn max_height(mut self, height: String) -> Container {
        self.max_height = height;
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Container {
        self.padding = padding;
        self
    }
    #[inline]
    pub fn background_color(mut self, color: String) -> Container {
        self.background_color = color;
        self
    }
}

impl Renderable for Container {
    #[inline]
    fn render(self) -> String {
        let mut style = "margin: 0 auto; ".to_string();
        if self.max_width != "" {
            style = format!("{}{}{}{}", style, "max-width: ", self.max_width, "; ");
        }
        if self.max_height != "" {
            style = format!("{}{}{}{}", style, "max-height: ", self.max_height, "; ");
        }
        if self.padding != "" {
            style = format!("{}{}{}{}", style, "padding: ", self.padding, "; ");
        }
        if self.background_color != "" {
            style = format!(
                "{}{}{}{}",
                style, "background-color: ", self.background_color, "; "
            );
        }
        let children_html = self.children.join(
            "
  ",
        );
        format!(
            "<div class='wj-container' style='{}'>
  {}
</div>",
            style, children_html
        )
    }
}
