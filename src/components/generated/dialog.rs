#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct Dialog {
    title: String,
    content: String,
    open: bool,
    width: String,
}

impl Dialog {
    #[inline]
    pub fn new(title: String, content: String) -> Dialog {
        Dialog {
            title,
            content,
            open: false,
            width: "500px".to_string(),
        }
    }
    #[inline]
    pub fn open(mut self, open: bool) -> Dialog {
        self.open = open;
        self
    }
    #[inline]
    pub fn width(mut self, width: String) -> Dialog {
        self.width = width;
        self
    }
}

impl Renderable for Dialog {
    #[inline]
    fn render(self) -> String {
        let display_style = {
            if self.open {
                "display: flex;"
            } else {
                "display: none;"
            }
        };
        format!(
            "<div class='wj-dialog-overlay' style='{}'>
  <div class='wj-dialog' style='max-width: {}; width: 90%;'>
    <div class='wj-dialog-header'>
      <h2>{}</h2>
      <button class='wj-dialog-close'>×</button>
    </div>
    <div class='wj-dialog-content'>
      {}
    </div>
  </div>
</div>",
            display_style, self.width, self.title, self.content
        )
    }
}
