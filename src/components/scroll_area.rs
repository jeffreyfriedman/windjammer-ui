//! Scroll Area Component
//! 
//! A scrollable container for content that exceeds the visible area.
//! Supports both horizontal and vertical scrolling.

use crate::to_vnode::ToVNode;
use crate::simple_vnode::{VAttr, VNode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

/// Scrollable container
pub struct ScrollArea {
    pub children: Vec<VNode>,
    pub direction: ScrollDirection,
    pub max_height: Option<String>,
    pub max_width: Option<String>,
}

impl ScrollArea {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            direction: ScrollDirection::Vertical,
            max_height: None,
            max_width: None,
        }
    }
    
    pub fn vertical() -> Self {
        Self::new()
    }
    
    pub fn horizontal() -> Self {
        Self {
            direction: ScrollDirection::Horizontal,
            ..Self::new()
        }
    }
    
    pub fn both() -> Self {
        Self {
            direction: ScrollDirection::Both,
            ..Self::new()
        }
    }
    
    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }
    
    pub fn max_height(mut self, height: impl Into<String>) -> Self {
        self.max_height = Some(height.into());
        self
    }
    
    pub fn max_width(mut self, width: impl Into<String>) -> Self {
        self.max_width = Some(width.into());
        self
    }
    
    pub fn child(mut self, child: VNode) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn children(mut self, children: Vec<VNode>) -> Self {
        self.children = children;
        self
    }
    
    pub fn render(&self) -> VNode {
        let mut style = String::from("overflow: ");
        
        match self.direction {
            ScrollDirection::Vertical => style.push_str("hidden auto"),
            ScrollDirection::Horizontal => style.push_str("auto hidden"),
            ScrollDirection::Both => style.push_str("auto"),
        }
        
        style.push_str("; ");
        
        if let Some(ref height) = self.max_height {
            style.push_str(&format!("max-height: {}; ", height));
        }
        
        if let Some(ref width) = self.max_width {
            style.push_str(&format!("max-width: {}; ", width));
        }
        
        // Add scrollbar styling for modern look
        style.push_str(
            "scrollbar-width: thin; scrollbar-color: #3E3E3E #1E1E1E;"
        );
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-scroll-area".to_string())),
                ("style".to_string(), VAttr::Static(style)),
            ],
            children: self.children.clone(),
        }
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}

impl ToVNode for ScrollArea {
    fn to_vnode(self) -> VNode {
        self.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scroll_area_new() {
        let scroll = ScrollArea::new();
        assert_eq!(scroll.direction, ScrollDirection::Vertical);
        assert!(scroll.children.is_empty());
        assert!(scroll.max_height.is_none());
    }
    
    #[test]
    fn test_scroll_area_vertical() {
        let scroll = ScrollArea::vertical();
        assert_eq!(scroll.direction, ScrollDirection::Vertical);
    }
    
    #[test]
    fn test_scroll_area_horizontal() {
        let scroll = ScrollArea::horizontal();
        assert_eq!(scroll.direction, ScrollDirection::Horizontal);
    }
    
    #[test]
    fn test_scroll_area_both() {
        let scroll = ScrollArea::both();
        assert_eq!(scroll.direction, ScrollDirection::Both);
    }
    
    #[test]
    fn test_scroll_area_max_height() {
        let scroll = ScrollArea::new().max_height("400px");
        assert_eq!(scroll.max_height, Some("400px".to_string()));
    }
    
    #[test]
    fn test_scroll_area_children() {
        let scroll = ScrollArea::new()
            .child(VNode::Text("Child 1".to_string()))
            .child(VNode::Text("Child 2".to_string()));
        assert_eq!(scroll.children.len(), 2);
    }
    
    #[test]
    fn test_scroll_area_render() {
        let scroll = ScrollArea::new()
            .max_height("300px")
            .child(VNode::Text("Content".to_string()));
        
        let vnode = scroll.render();
        match vnode {
            VNode::Element { tag, attrs, children } => {
                assert_eq!(tag, "div");
                assert_eq!(children.len(), 1);
                assert!(attrs.iter().any(|(key, attr)| {
                    key == "style" && matches!(attr, VAttr::Static(v) if v.contains("max-height: 300px"))
                }));
            }
            _ => panic!("Expected element node"),
        }
    }
}

