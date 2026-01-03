#![allow(clippy::all)]
#![allow(noop_method_call)]
#[derive(Debug, Clone, PartialEq)]
pub struct Drawer {
    pub children: Vec<String>,
    pub position: DrawerPosition,
    pub width: String,
    pub open: bool,
    pub class: String,
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
                "left: 0; top: 0; bottom: 0;",
                format!("width: {};", self.width),
            ),
            DrawerPosition::Right => (
                "right: 0; top: 0; bottom: 0;",
                format!("width: {};", self.width),
            ),
            DrawerPosition::Top => (
                "top: 0; left: 0; right: 0;",
                format!("height: {};", self.width),
            ),
            DrawerPosition::Bottom => (
                "bottom: 0; left: 0; right: 0;",
                format!("height: {};", self.width),
            ),
        };
        let transform = {
            if self.open {
                "transform: translateX(0);".to_string()
            } else {
                match self.position {
                    DrawerPosition::Left => "transform: translateX(-100%);".to_string(),
                    DrawerPosition::Right => "transform: translateX(100%);".to_string(),
                    DrawerPosition::Top => "transform: translateY(-100%);".to_string(),
                    DrawerPosition::Bottom => "transform: translateY(100%);".to_string(),
                }
            }
        };
        let display = {
            if self.open {
                "display: block;".to_string()
            } else {
                "display: none;".to_string()
            }
        };
        let mut html = String::new();
        html.push_str("<div class=\"wj-drawer-backdrop\" style=\"");
        html.push_str(&display);
        html.push_str(" position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 999;\"></div>");
        html.push_str("<div class=\"wj-drawer ");
        html.push_str(&self.class.as_str());
        html.push_str("\" style=\"position: fixed; ");
        html.push_str(&position_style);
        html.push(' ');
        html.push_str(&size_prop.as_str());
        html.push(' ');
        html.push_str(&transform);
        html.push_str(" background: white; box-shadow: 0 0 20px rgba(0, 0, 0, 0.3); z-index: 1000; transition: transform 0.3s ease; overflow-y: auto; padding: 24px;\">");
        for child in &self.children {
            html.push_str(&child.as_str());
        }
        html.push_str("</div>");
        html
    }
}
