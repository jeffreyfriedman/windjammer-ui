#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
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
    fn render(&self) -> String {
        let direction_str: String = match self.direction {
            FlexDirection::Row => String::from("row"),
            FlexDirection::Column => String::from("column"),
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
        let children_html = self.children.join(&"\n  ");
        format!(
            "<div class='wj-flex' style='{}'>\n  {}\n</div>",
            style, children_html
        )
    }
}
