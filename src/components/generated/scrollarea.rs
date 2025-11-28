#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

pub struct ScrollArea {
    children: Vec<String>,
    direction: ScrollDirection,
    height: String,
    width: String,
}

impl ScrollArea {
    #[inline]
    pub fn new() -> ScrollArea {
        ScrollArea {
            children: Vec::new(),
            direction: ScrollDirection::Vertical,
            height: "300px".to_string(),
            width: "100%".to_string(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> ScrollArea {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn direction(mut self, direction: ScrollDirection) -> ScrollArea {
        self.direction = direction;
        self
    }
    #[inline]
    pub fn height(mut self, height: String) -> ScrollArea {
        self.height = height;
        self
    }
    #[inline]
    pub fn width(mut self, width: String) -> ScrollArea {
        self.width = width;
        self
    }
}

impl Renderable for ScrollArea {
    #[inline]
    fn render(self) -> String {
        let overflow_style = match self.direction {
            ScrollDirection::Vertical => "overflow-y: auto; overflow-x: hidden;",
            ScrollDirection::Horizontal => "overflow-x: auto; overflow-y: hidden;",
            ScrollDirection::Both => "overflow: auto;",
        };
        let children_html = self.children.join(
            "
",
        );
        format!(
            "<div class='wj-scroll-area' style='height: {}; width: {}; {}'>
  {}
</div>",
            self.height, self.width, overflow_style, children_html
        )
    }
}
