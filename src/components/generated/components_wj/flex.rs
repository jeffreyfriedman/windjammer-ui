use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Flex {
    children: Vec<String>,
    direction: FlexDirection,
    gap: String,
    padding: String,
    background_color: String,
}

impl Flex {
    #[inline]
    pub fn new() -> Flex {
        Flex {
            children: Vec::new(),
            direction: FlexDirection::Row,
            gap: "8px".to_string().to_string(),
            padding: "".to_string().to_string(),
            background_color: "".to_string().to_string(),
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
    fn render(&self) -> String {
        let direction_str = match self.direction {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::Column => "column".to_string(),
        };
        let mut style = "display: flex; flex-direction: ".to_string()
            + &direction_str.to_string()
            + &"; gap: "
            + &self.gap.clone()
            + &";";
        if self.padding != "" {
            style = format!("{}{}{}{}", style, " padding: ", self.padding.clone(), ";");
        }
        if self.background_color != "" {
            style = format!(
                "{}{}{}{}",
                style,
                " background-color: ",
                self.background_color.clone(),
                ";"
            );
        }
        let children_html = self.children.join("\n  ");
        format!(
            "<div class='wj-flex' style='{}'>\n  {}\n</div>",
            style, children_html
        )
    }
}

fn main() {
    let flex_row = Flex::new()
        .direction(FlexDirection::Row)
        .gap("16px".to_string())
        .child("<button>First</button>".to_string())
        .child("<button>Second</button>".to_string())
        .child("<button>Third</button>".to_string());
    let flex_col = Flex::new()
        .direction(FlexDirection::Column)
        .gap("12px".to_string())
        .padding("20px".to_string())
        .background_color("#f5f5f5".to_string())
        .children(vec![
            "<h2>Title</h2>".to_string(),
            "<p>Paragraph 1</p>".to_string(),
            "<p>Paragraph 2</p>".to_string(),
        ]);
    println!("Flex Row:\n{}", flex_row.render());
    println!("\nFlex Column:\n{}", flex_col.render());
}
