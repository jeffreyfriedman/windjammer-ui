#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Drawer {
    children: Vec<String>,
    position: DrawerPosition,
    width: String,
    open: bool,
    class: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl Drawer {
    #[inline]
    pub fn new() -> Drawer {
        Drawer {
            children: Vec::new(),
            position: DrawerPosition::Right,
            width: "320px".to_string(),
            open: false,
            class: String::new(),
        }
    }
    #[inline]
    pub fn child(mut self, child: String) -> Drawer {
        self.children.push(child);
        self
    }
    #[inline]
    pub fn position(mut self, position: DrawerPosition) -> Drawer {
        self.position = position;
        self
    }
    #[inline]
    pub fn width(mut self, width: String) -> Drawer {
        self.width = width;
        self
    }
    #[inline]
    pub fn open(mut self, open: bool) -> Drawer {
        self.open = open;
        self
    }
    #[inline]
    pub fn class(mut self, class: String) -> Drawer {
        self.class = class;
        self
    }
    #[inline]
    pub fn render(&self) -> String {
        let (position_style, size_prop) = match self.position {
            DrawerPosition::Left => (
                String::from("left: 0; top: 0; bottom: 0;"),
                format!("width: {};", self.width.clone()),
            ),
            DrawerPosition::Right => (
                String::from("right: 0; top: 0; bottom: 0;"),
                format!("width: {};", self.width.clone()),
            ),
            DrawerPosition::Top => (
                String::from("top: 0; left: 0; right: 0;"),
                format!("height: {};", self.width.clone()),
            ),
            DrawerPosition::Bottom => (
                String::from("bottom: 0; left: 0; right: 0;"),
                format!("height: {};", self.width.clone()),
            ),
        };
        let transform = {
            if self.open {
                String::from("transform: translateX(0);")
            } else {
                match self.position {
                    DrawerPosition::Left => String::from("transform: translateX(-100%);"),
                    DrawerPosition::Right => String::from("transform: translateX(100%);"),
                    DrawerPosition::Top => String::from("transform: translateY(-100%);"),
                    DrawerPosition::Bottom => String::from("transform: translateY(100%);"),
                }
            }
        };
        let display: String = {
            if self.open {
                String::from("display: block;")
            } else {
                String::from("display: none;")
            }
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-drawer-backdrop\" style=\"");
        html.push_str(&display);
        html.push_str(" position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 999;\"></div>");
        html.push_str("<div class=\"wj-drawer ");
        html.push_str(&self.class.clone());
        html.push_str("\" style=\"position: fixed; ");
        html.push_str(&position_style);
        html.push(' ');
        html.push_str(&size_prop);
        html.push(' ');
        html.push_str(&transform);
        html.push_str(" background: white; box-shadow: 0 0 20px rgba(0, 0, 0, 0.3); z-index: 1000; transition: transform 0.3s ease; overflow-y: auto; padding: 24px;\">");
        for child in &self.children {
            html.push_str(child);
        }
        html.push_str("</div>");
        html
    }
}
