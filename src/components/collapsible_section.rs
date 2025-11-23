//! Collapsible Section Component
//! 
//! A section with a header that can be expanded or collapsed to show/hide content.
//! Follows the Windjammer UI design system for professional, modern appearance.

use crate::to_vnode::ToVNode;
use crate::simple_vnode::{VAttr, VNode};
use std::rc::Rc;
use std::cell::RefCell;

/// Collapsible section with expand/collapse functionality
pub struct CollapsibleSection {
    pub title: String,
    pub children: Vec<VNode>,
    pub expanded: bool,
    pub on_toggle: Option<Rc<RefCell<dyn FnMut()>>>,
}

impl CollapsibleSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            children: Vec::new(),
            expanded: true, // Expanded by default
            on_toggle: None,
        }
    }
    
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
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
    
    pub fn on_toggle<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.on_toggle = Some(Rc::new(RefCell::new(handler)));
        self
    }
    
    pub fn render(&self) -> VNode {
        // Container for the entire collapsible section
        let mut section_children = Vec::new();
        
        // Header (clickable to toggle)
        let header = self.render_header();
        section_children.push(header);
        
        // Content (only if expanded)
        if self.expanded {
            let content = self.render_content();
            section_children.push(content);
        }
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                (
                    "class".to_string(),
                    VAttr::Static("wj-collapsible-section".to_string()),
                ),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "background: #252525; border: 1px solid #3E3E3E; border-radius: 4px; margin-bottom: 8px;".to_string()
                    ),
                ),
            ],
            children: section_children,
        }
    }
    
    fn render_header(&self) -> VNode {
        // Arrow indicator (rotates when expanded)
        let arrow = VNode::Element {
            tag: "span".to_string(),
            attrs: vec![
                (
                    "class".to_string(),
                    VAttr::Static("wj-collapsible-arrow".to_string()),
                ),
                (
                    "style".to_string(),
                    VAttr::Static(format!(
                        "display: inline-block; transition: transform 200ms ease-in-out; transform: rotate({}deg); margin-right: 8px; color: #9D9D9D; font-size: 12px;",
                        if self.expanded { 90 } else { 0 }
                    )),
                ),
            ],
            children: vec![VNode::Text("▶".to_string())],
        };
        
        // Title text
        let title_text = VNode::Element {
            tag: "span".to_string(),
            attrs: vec![
                (
                    "class".to_string(),
                    VAttr::Static("wj-collapsible-title".to_string()),
                ),
                (
                    "style".to_string(),
                    VAttr::Static("font-size: 14px; font-weight: 500; color: #D4D4D4;".to_string()),
                ),
            ],
            children: vec![VNode::Text(self.title.clone())],
        };
        
        // Header container (clickable)
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                (
                    "class".to_string(),
                    VAttr::Static("wj-collapsible-header".to_string()),
                ),
                (
                    "style".to_string(),
                    VAttr::Static("padding: 12px; cursor: pointer; user-select: none; display: flex; align-items: center; background: #252525; transition: background 100ms ease-out;".to_string()),
                ),
            ],
            children: vec![arrow, title_text],
        }
    }
    
    fn render_content(&self) -> VNode {
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                (
                    "class".to_string(),
                    VAttr::Static("wj-collapsible-content".to_string()),
                ),
                (
                    "style".to_string(),
                    VAttr::Static("padding: 12px; background: #2D2D2D; border-top: 1px solid #3E3E3E;".to_string()),
                ),
            ],
            children: self.children.clone(),
        }
    }
}

impl ToVNode for CollapsibleSection {
    fn to_vnode(self) -> VNode {
        self.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collapsible_section_new() {
        let section = CollapsibleSection::new("Test Section");
        assert_eq!(section.title, "Test Section");
        assert!(section.expanded);
        assert!(section.children.is_empty());
    }
    
    #[test]
    fn test_collapsible_section_expanded() {
        let section = CollapsibleSection::new("Test")
            .expanded(false);
        assert!(!section.expanded);
    }
    
    #[test]
    fn test_collapsible_section_children() {
        let section = CollapsibleSection::new("Test")
            .child(VNode::Text("Child 1".to_string()))
            .child(VNode::Text("Child 2".to_string()));
        assert_eq!(section.children.len(), 2);
    }
    
    #[test]
    fn test_collapsible_section_render_expanded() {
        let section = CollapsibleSection::new("Test")
            .expanded(true)
            .child(VNode::Text("Content".to_string()));
        
        let vnode = section.render();
        match vnode {
            VNode::Element { children, .. } => {
                // Should have header + content (2 children)
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected element node"),
        }
    }
    
    #[test]
    fn test_collapsible_section_render_collapsed() {
        let section = CollapsibleSection::new("Test")
            .expanded(false)
            .child(VNode::Text("Content".to_string()));
        
        let vnode = section.render();
        match vnode {
            VNode::Element { children, .. } => {
                // Should have only header (1 child)
                assert_eq!(children.len(), 1);
            }
            _ => panic!("Expected element node"),
        }
    }
}

