#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CollapsibleSection {
    pub title: String,
    pub content: String,
    pub open: bool,
}

impl CollapsibleSection {
    #[inline]
    pub fn new(title: String, content: String) -> CollapsibleSection {
        CollapsibleSection {
            title,
            content,
            open: false,
        }
    }
    #[inline]
    pub fn open(mut self, open: bool) -> CollapsibleSection {
        self.open = open;
        self
    }
}

impl Renderable for CollapsibleSection {
    #[inline]
    fn render(self) -> String {
        let icon = {
            if self.open {
                "▼".to_string()
            } else {
                "▶".to_string()
            }
        };
        let content_style = {
            if self.open {
                "display: block;".to_string()
            } else {
                "display: none;".to_string()
            }
        };
        format!(
            "<div class='wj-collapsible'>
  <div class='wj-collapsible-header'>
    <span class='wj-collapsible-icon'>{}</span>
    <span>{}</span>
  </div>
  <div class='wj-collapsible-content' style='{}'>
    {}
  </div>
</div>",
            icon, self.title, content_style, self.content
        )
    }
}
