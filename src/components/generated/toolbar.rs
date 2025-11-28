#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct Toolbar {
    items: Vec<String>,
    position: String,
}

impl Toolbar {
    #[inline]
    pub fn new() -> Toolbar {
        Toolbar {
            items: Vec::new(),
            position: "top".to_string(),
        }
    }
    #[inline]
    pub fn item(mut self, item: String) -> Toolbar {
        self.items.push(item);
        self
    }
    #[inline]
    pub fn position(mut self, position: String) -> Toolbar {
        self.position = position;
        self
    }
}

impl Renderable for Toolbar {
    #[inline]
    fn render(self) -> String {
        let items_html = self.items.join(
            "
",
        );
        format!(
            "<div class='wj-toolbar wj-toolbar-{}'>
  {}
</div>",
            self.position, items_html
        )
    }
}
