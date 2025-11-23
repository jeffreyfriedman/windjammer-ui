//! Spacer Component
//! 
//! A flexible spacing component that can be used to add fixed or flexible space
//! between UI elements. Follows the 8px grid system from the design system.

use crate::to_vnode::ToVNode;
use crate::simple_vnode::{VAttr, VNode};

/// Spacer for adding space between elements
pub struct Spacer {
    pub width: Option<String>,
    pub height: Option<String>,
    pub flex: bool, // If true, takes up available space
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            flex: false,
        }
    }
    
    /// Create a horizontal spacer with fixed width
    pub fn horizontal(width: impl Into<String>) -> Self {
        Self {
            width: Some(width.into()),
            height: None,
            flex: false,
        }
    }
    
    /// Create a vertical spacer with fixed height
    pub fn vertical(height: impl Into<String>) -> Self {
        Self {
            width: None,
            height: Some(height.into()),
            flex: false,
        }
    }
    
    /// Create a flexible spacer that fills available space
    pub fn flexible() -> Self {
        Self {
            width: None,
            height: None,
            flex: true,
        }
    }
    
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
    
    pub fn height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }
    
    pub fn render(&self) -> VNode {
        let mut style = String::new();
        
        if self.flex {
            style.push_str("flex: 1; ");
        }
        
        if let Some(ref w) = self.width {
            style.push_str(&format!("width: {}; ", w));
        }
        
        if let Some(ref h) = self.height {
            style.push_str(&format!("height: {}; ", h));
        }
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-spacer".to_string())),
                ("style".to_string(), VAttr::Static(style)),
            ],
            children: vec![],
        }
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl ToVNode for Spacer {
    fn to_vnode(self) -> VNode {
        self.render()
    }
}

/// Common spacer sizes following the 8px grid
impl Spacer {
    /// Extra small spacing (4px)
    pub fn xxs() -> Self {
        Self::vertical("4px")
    }
    
    /// Extra small spacing (8px)
    pub fn xs() -> Self {
        Self::vertical("8px")
    }
    
    /// Small spacing (12px)
    pub fn sm() -> Self {
        Self::vertical("12px")
    }
    
    /// Medium spacing (16px)
    pub fn md() -> Self {
        Self::vertical("16px")
    }
    
    /// Large spacing (24px)
    pub fn lg() -> Self {
        Self::vertical("24px")
    }
    
    /// Extra large spacing (32px)
    pub fn xl() -> Self {
        Self::vertical("32px")
    }
    
    /// Extra extra large spacing (48px)
    pub fn xxl() -> Self {
        Self::vertical("48px")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spacer_new() {
        let spacer = Spacer::new();
        assert!(spacer.width.is_none());
        assert!(spacer.height.is_none());
        assert!(!spacer.flex);
    }
    
    #[test]
    fn test_spacer_horizontal() {
        let spacer = Spacer::horizontal("16px");
        assert_eq!(spacer.width, Some("16px".to_string()));
        assert!(spacer.height.is_none());
    }
    
    #[test]
    fn test_spacer_vertical() {
        let spacer = Spacer::vertical("16px");
        assert!(spacer.width.is_none());
        assert_eq!(spacer.height, Some("16px".to_string()));
    }
    
    #[test]
    fn test_spacer_flexible() {
        let spacer = Spacer::flexible();
        assert!(spacer.flex);
    }
    
    #[test]
    fn test_spacer_grid_sizes() {
        assert_eq!(Spacer::xxs().height, Some("4px".to_string()));
        assert_eq!(Spacer::xs().height, Some("8px".to_string()));
        assert_eq!(Spacer::sm().height, Some("12px".to_string()));
        assert_eq!(Spacer::md().height, Some("16px".to_string()));
        assert_eq!(Spacer::lg().height, Some("24px".to_string()));
        assert_eq!(Spacer::xl().height, Some("32px".to_string()));
        assert_eq!(Spacer::xxl().height, Some("48px".to_string()));
    }
    
    #[test]
    fn test_spacer_render() {
        let spacer = Spacer::vertical("16px");
        let vnode = spacer.render();
        
        match vnode {
            VNode::Element { tag, attrs, children } => {
                assert_eq!(tag, "div");
                assert!(children.is_empty());
                assert!(attrs.iter().any(|(key, attr)| {
                    key == "style" && matches!(attr, VAttr::Static(v) if v.contains("height: 16px"))
                }));
            }
            _ => panic!("Expected element node"),
        }
    }
}

