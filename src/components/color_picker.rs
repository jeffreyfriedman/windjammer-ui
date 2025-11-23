//! Color Picker Component
//! 
//! A comprehensive color picker with:
//! - Color preview
//! - RGB/HSL/Hex input
//! - Alpha channel support
//! - Design system compliant

use crate::to_vnode::ToVNode;
use crate::simple_vnode::{VAttr, VNode};
use std::rc::Rc;
use std::cell::RefCell;

/// Color picker component
pub struct ColorPicker {
    pub color: [f32; 4], // RGBA, values 0.0-1.0
    pub show_alpha: bool,
    pub on_change: Option<Rc<RefCell<dyn FnMut([f32; 4])>>>,
}

impl ColorPicker {
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            color,
            show_alpha: true,
            on_change: None,
        }
    }
    
    pub fn show_alpha(mut self, show_alpha: bool) -> Self {
        self.show_alpha = show_alpha;
        self
    }
    
    pub fn on_change<F: FnMut([f32; 4]) + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Rc::new(RefCell::new(handler)));
        self
    }
    
    pub fn render(&self) -> VNode {
        let [r, g, b, a] = self.color;
        
        // Convert to CSS color
        let color_string = if self.show_alpha {
            format!("rgba({}, {}, {}, {})", 
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
                a
            )
        } else {
            format!("rgb({}, {}, {})", 
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8
            )
        };
        
        // Color preview
        let preview = VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-color-preview".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(format!(
                        "width: 48px; height: 48px; border-radius: 4px; background: {}; \
                         border: 1px solid #3E3E3E; cursor: pointer;",
                        color_string
                    )),
                ),
            ],
            children: vec![],
        };
        
        // Hex input
        let hex_value = format!("#{:02X}{:02X}{:02X}", 
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8
        );
        
        let hex_input = VNode::Element {
            tag: "input".to_string(),
            attrs: vec![
                ("type".to_string(), VAttr::Static("text".to_string())),
                ("value".to_string(), VAttr::Static(hex_value)),
                ("class".to_string(), VAttr::Static("wj-color-hex".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "background: #3C3C3C; color: #D4D4D4; border: 1px solid #3E3E3E; \
                         border-radius: 4px; padding: 4px 8px; font-size: 12px; \
                         font-family: monospace; width: 80px; margin-left: 8px;".to_string()
                    ),
                ),
            ],
            children: vec![],
        };
        
        // Container for preview and hex
        let preview_row = VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-color-picker-header".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static("display: flex; align-items: center; margin-bottom: 8px;".to_string()),
                ),
            ],
            children: vec![preview, hex_input],
        };
        
        // RGB sliders
        let rgb_sliders = self.render_rgb_sliders();
        
        // Alpha slider (if enabled)
        let mut children = vec![preview_row, rgb_sliders];
        
        if self.show_alpha {
            let alpha_slider = self.render_alpha_slider();
            children.push(alpha_slider);
        }
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-color-picker".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "padding: 12px; background: #2D2D2D; border: 1px solid #3E3E3E; \
                         border-radius: 4px;".to_string()
                    ),
                ),
            ],
            children,
        }
    }
    
    fn render_rgb_sliders(&self) -> VNode {
        let [r, g, b, _] = self.color;
        
        let r_slider = self.render_channel_slider("R", r, "#F48771");
        let g_slider = self.render_channel_slider("G", g, "#4EC9B0");
        let b_slider = self.render_channel_slider("B", b, "#007ACC");
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-color-rgb-sliders".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static("display: flex; flex-direction: column; gap: 8px;".to_string()),
                ),
            ],
            children: vec![r_slider, g_slider, b_slider],
        }
    }
    
    fn render_channel_slider(&self, label: &str, value: f32, color: &str) -> VNode {
        let label_node = VNode::Element {
            tag: "span".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-slider-label".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "font-size: 12px; color: #9D9D9D; width: 20px; font-weight: 500;".to_string()
                    ),
                ),
            ],
            children: vec![VNode::Text(label.to_string())],
        };
        
        let value_int = (value * 255.0) as u8;
        let value_text = VNode::Element {
            tag: "span".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-slider-value".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "font-size: 12px; color: #D4D4D4; width: 30px; text-align: right; \
                         font-family: monospace;".to_string()
                    ),
                ),
            ],
            children: vec![VNode::Text(value_int.to_string())],
        };
        
        let slider = VNode::Element {
            tag: "input".to_string(),
            attrs: vec![
                ("type".to_string(), VAttr::Static("range".to_string())),
                ("min".to_string(), VAttr::Static("0".to_string())),
                ("max".to_string(), VAttr::Static("255".to_string())),
                ("value".to_string(), VAttr::Static(value_int.to_string())),
                ("class".to_string(), VAttr::Static("wj-slider".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(format!(
                        "flex: 1; margin: 0 8px; height: 4px; border-radius: 2px; \
                         background: linear-gradient(to right, #3E3E3E 0%, {} 100%); \
                         outline: none; cursor: pointer;",
                        color
                    )),
                ),
            ],
            children: vec![],
        };
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-slider-row".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static("display: flex; align-items: center;".to_string()),
                ),
            ],
            children: vec![label_node, slider, value_text],
        }
    }
    
    fn render_alpha_slider(&self) -> VNode {
        let [_, _, _, a] = self.color;
        
        let label_node = VNode::Element {
            tag: "span".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-slider-label".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "font-size: 12px; color: #9D9D9D; width: 20px; font-weight: 500;".to_string()
                    ),
                ),
            ],
            children: vec![VNode::Text("A".to_string())],
        };
        
        let value_percent = (a * 100.0) as u8;
        let value_text = VNode::Element {
            tag: "span".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-slider-value".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "font-size: 12px; color: #D4D4D4; width: 30px; text-align: right; \
                         font-family: monospace;".to_string()
                    ),
                ),
            ],
            children: vec![VNode::Text(format!("{}%", value_percent))],
        };
        
        let slider = VNode::Element {
            tag: "input".to_string(),
            attrs: vec![
                ("type".to_string(), VAttr::Static("range".to_string())),
                ("min".to_string(), VAttr::Static("0".to_string())),
                ("max".to_string(), VAttr::Static("100".to_string())),
                ("value".to_string(), VAttr::Static(value_percent.to_string())),
                ("class".to_string(), VAttr::Static("wj-slider".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static(
                        "flex: 1; margin: 0 8px; height: 4px; border-radius: 2px; \
                         background: linear-gradient(to right, transparent 0%, #D4D4D4 100%); \
                         outline: none; cursor: pointer;".to_string()
                    ),
                ),
            ],
            children: vec![],
        };
        
        VNode::Element {
            tag: "div".to_string(),
            attrs: vec![
                ("class".to_string(), VAttr::Static("wj-slider-row".to_string())),
                (
                    "style".to_string(),
                    VAttr::Static("display: flex; align-items: center; margin-top: 8px;".to_string()),
                ),
            ],
            children: vec![label_node, slider, value_text],
        }
    }
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new([1.0, 1.0, 1.0, 1.0])
    }
}

impl ToVNode for ColorPicker {
    fn to_vnode(self) -> VNode {
        self.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_picker_new() {
        let picker = ColorPicker::new([1.0, 0.5, 0.25, 1.0]);
        assert_eq!(picker.color, [1.0, 0.5, 0.25, 1.0]);
        assert!(picker.show_alpha);
    }
    
    #[test]
    fn test_color_picker_show_alpha() {
        let picker = ColorPicker::new([1.0, 1.0, 1.0, 1.0])
            .show_alpha(false);
        assert!(!picker.show_alpha);
    }
    
    #[test]
    fn test_color_picker_default() {
        let picker = ColorPicker::default();
        assert_eq!(picker.color, [1.0, 1.0, 1.0, 1.0]);
    }
    
    #[test]
    fn test_color_picker_render() {
        let picker = ColorPicker::new([1.0, 0.0, 0.0, 1.0]);
        let vnode = picker.render();
        
        match vnode {
            VNode::Element { tag, children, .. } => {
                assert_eq!(tag, "div");
                // Should have preview row, RGB sliders, and alpha slider
                assert_eq!(children.len(), 3);
            }
            _ => panic!("Expected element node"),
        }
    }
    
    #[test]
    fn test_color_picker_no_alpha() {
        let picker = ColorPicker::new([1.0, 0.0, 0.0, 1.0])
            .show_alpha(false);
        let vnode = picker.render();
        
        match vnode {
            VNode::Element { children, .. } => {
                // Should have preview row and RGB sliders only (no alpha)
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected element node"),
        }
    }
}

