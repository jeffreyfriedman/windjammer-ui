#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct CollapsibleSection {
    title: String,
    content: String,
    open: bool,
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
                "▼"
            } else {
                "▶"
            }
        };
        let content_style = {
            if self.open {
                "display: block;"
            } else {
                "display: none;"
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
