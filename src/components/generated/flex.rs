use super::traits::Renderable;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flex {
    pub children: Vec<String>,
    pub direction: FlexDirection,
    pub gap: String,
    pub padding: String,
    pub background_color: String,
}

impl Flex {
    #[inline]
    pub fn new() -> Flex {
        Flex {
            children: Vec::new(),
            direction: FlexDirection::Row,
            gap: "8px".to_string(),
            padding: "".to_string(),
            background_color: "".to_string(),
        }
    }
    #[inline]
    pub fn direction(mut self, direction: FlexDirection) -> Flex {
        self.direction = direction;
        self
    }
    #[inline]
    pub fn child(mut self, child: String) -> Flex {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn children(mut self, children: Vec<String>) -> Flex {
        self.children = children;
        self
    }
    #[inline]
    pub fn gap(mut self, gap: String) -> Flex {
        self.gap = gap;
        self
    }
    #[inline]
    pub fn gap_px(mut self, gap: i32) -> Flex {
        self.gap = format!("{}px", gap);
        self
    }
    #[inline]
    pub fn padding(mut self, padding: String) -> Flex {
        self.padding = padding;
        self
    }
    #[inline]
    pub fn background_color(mut self, color: String) -> Flex {
        self.background_color = color;
        self
    }
}

impl Renderable for Flex {
    #[inline]
    fn render(self) -> String {
        let direction_str = match self.direction {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::Column => "column".to_string(),
        };
        let mut style = format!(
            "{}{}{}{}{}",
            "display: flex; flex-direction: ".to_string(),
            direction_str,
            "; gap: ",
            self.gap,
            ";"
        );
        if self.padding != "" {
            style = format!("{}{}{}{}", style, " padding: ", self.padding, ";");
        }
        if self.background_color != "" {
            style = format!(
                "{}{}{}{}",
                style, " background-color: ", self.background_color, ";"
            );
        }
        let children_html = self.children.join(
            "
  ",
        );
        format!(
            "<div class='wj-flex' style='{}'>
  {}
</div>",
            style, children_html
        )
    }
}
