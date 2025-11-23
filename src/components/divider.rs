//! Divider Component
//! 
//! A visual separator between sections or content areas.
//! Can be horizontal or vertical, with customizable color and thickness.

use crate::to_vnode::ToVNode;
use crate::simple_vnode::{VAttr, VNode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

/// Visual divider/separator
pub struct Divider {
    pub orientation: DividerOrientation,
    pub color: Option<String>,
    pub thickness: Option<String>,
    pub margin: Option<String>,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            color: Some("#3E3E3E".to_string()), // Default border color from design system
            thickness: Some("1px".to_string()),
            margin: Some("0".to_string()),
        }
    }
    
    pub fn horizontal() -> Self {
        Self::new()
    }
    
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            ..Self::new()
        }
    }
    
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
    
    pub fn thickness(mut self, thickness: impl Into<String>) -> Self {
        self.thickness = Some(thickness.into());
        self
    }
    
    pub fn margin(mut self, margin: impl Into<String>) -> Self {
        self.margin = Some(margin.into());
        self
    }
    
    pub fn render(&self) -> VNode {
        let color = self.color.as_ref().map(|s| s.as_str()).unwrap_or("#3E3E3E");
        let thickness = self.thickness.as_ref().map(|s| s.as_str()).unwrap_or("1px");
        let margin = self.margin.as_ref().map(|s| s.as_str()).unwrap_or("0");
        
        let style = match self.orientation {
            DividerOrientation::Horizontal => {
                format!(
                    "width: 100%; height: {}; background: {}; margin: {} 0;",
                    thickness, color, margin
                )
            }
            DividerOrientation::Vertical => {
                format!(
                    "width: {}; height: 100%; background: {}; margin: 0 {};",
                    thickness, color, margin
                )
            }
        };
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                (
                    "class".to_string(),
                    VAttr::Static(format!(
                        "wj-divider wj-divider-{}",
                        if self.orientation == DividerOrientation::Horizontal {
                            "horizontal"
                        } else {
                            "vertical"
                        }
                    )),
                ),
                ("style".to_string(), VAttr::Static(style)),
            ],
            children: vec![],
        }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl ToVNode for Divider {
    fn to_vnode(self) -> VNode {
        self.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divider_new() {
        let divider = Divider::new();
        assert_eq!(divider.orientation, DividerOrientation::Horizontal);
        assert_eq!(divider.color, Some("#3E3E3E".to_string()));
        assert_eq!(divider.thickness, Some("1px".to_string()));
    }
    
    #[test]
    fn test_divider_horizontal() {
        let divider = Divider::horizontal();
        assert_eq!(divider.orientation, DividerOrientation::Horizontal);
    }
    
    #[test]
    fn test_divider_vertical() {
        let divider = Divider::vertical();
        assert_eq!(divider.orientation, DividerOrientation::Vertical);
    }
    
    #[test]
    fn test_divider_color() {
        let divider = Divider::new().color("#FF0000");
        assert_eq!(divider.color, Some("#FF0000".to_string()));
    }
    
    #[test]
    fn test_divider_thickness() {
        let divider = Divider::new().thickness("2px");
        assert_eq!(divider.thickness, Some("2px".to_string()));
    }
    
    #[test]
    fn test_divider_render_horizontal() {
        let divider = Divider::horizontal();
        let vnode = divider.render();
        
        match vnode {
            VNode::Element { tag, attrs, children } => {
                assert_eq!(tag, "div");
                assert!(children.is_empty());
                assert!(attrs.iter().any(|(key, attr)| {
                    key == "class" && matches!(attr, VAttr::Static(v) if v.contains("wj-divider-horizontal"))
                }));
                assert!(attrs.iter().any(|(key, attr)| {
                    key == "style" && matches!(attr, VAttr::Static(v) if v.contains("width: 100%"))
                }));
            }
            _ => panic!("Expected element node"),
        }
    }
    
    #[test]
    fn test_divider_render_vertical() {
        let divider = Divider::vertical();
        let vnode = divider.render();
        
        match vnode {
            VNode::Element { attrs, .. } => {
                assert!(attrs.iter().any(|(key, attr)| {
                    key == "class" && matches!(attr, VAttr::Static(v) if v.contains("wj-divider-vertical"))
                }));
                assert!(attrs.iter().any(|(key, attr)| {
                    key == "style" && matches!(attr, VAttr::Static(v) if v.contains("height: 100%"))
                }));
            }
            _ => panic!("Expected element node"),
        }
    }
}

