//! Simple VNode implementation for generated component code
//!
//! This is a minimal virtual DOM implementation specifically for compiled components.
//! It's designed to be simple and transparent, not a full-featured virtual DOM.

use std::cell::RefCell;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Document, Element, Event};

/// A simple virtual DOM node
#[derive(Clone)]
pub enum VNode {
    Element {
        tag: String,
        attrs: Vec<(String, VAttr)>,
        children: Vec<VNode>,
    },
    Text(String),
    /// Trusted HTML fragment (DownstreamApp Home pilot: finance-ui body into `#main`).
    RawHtml(String),
}

/// Attribute value
#[derive(Clone)]
pub enum VAttr {
    Static(String),
    Dynamic(String),
    Event(Rc<RefCell<dyn FnMut()>>),
}

impl VNode {
    /// Create an element node
    pub fn element(tag: &str, attrs: Vec<(&str, VAttr)>, children: Vec<VNode>) -> Self {
        VNode::Element {
            tag: tag.to_string(),
            attrs: attrs.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
            children,
        }
    }

    /// Create a text node
    pub fn text(content: &str) -> Self {
        VNode::Text(content.to_string())
    }

    /// Trusted HTML island (scripts/markup already sanitized by the adapter).
    /// This is the only way to bypass escaping; prefer [`VNode::raw`] with an
    /// explicit [`crate::escape::RawHtml`].
    pub fn raw_html(html: impl Into<String>) -> Self {
        VNode::RawHtml(html.into())
    }

    /// Explicit opt-out of escaping.
    pub fn raw(html: crate::escape::RawHtml) -> Self {
        VNode::RawHtml(html.0)
    }

    /// Element key used for keyed reordering (`key` attribute), if any.
    pub fn key(&self) -> Option<&str> {
        match self {
            VNode::Element { attrs, .. } => attrs.iter().rev().find_map(|(k, v)| match v {
                VAttr::Static(s) | VAttr::Dynamic(s) if k == "key" => Some(s.as_str()),
                _ => None,
            }),
            _ => None,
        }
    }

    /// Return a copy that is safe to serialize or patch into a DOM:
    /// invalid/blocked tags become empty text, invalid attribute names and inline
    /// `on*` attributes are dropped, URL attributes are sanitized, and duplicate
    /// attribute names collapse (last wins). `RawHtml` is left untouched.
    pub fn sanitized(&self) -> VNode {
        match self {
            VNode::Element { tag, attrs, children } => {
                if !crate::escape::is_valid_tag(tag) {
                    return VNode::Text(String::new());
                }
                VNode::Element {
                    tag: tag.clone(),
                    attrs: sanitize_attrs(attrs),
                    children: children.iter().map(VNode::sanitized).collect(),
                }
            }
            other => other.clone(),
        }
    }

    /// Render this VNode to the DOM
    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, document: &Document) -> Result<web_sys::Node, JsValue> {
        match self {
            VNode::Element {
                tag,
                attrs,
                children,
            } => {
                let element = document.create_element(tag)?;

                // Set attributes
                for (name, value) in attrs {
                    match value {
                        VAttr::Static(v) | VAttr::Dynamic(v) => {
                            if name == "class" {
                                element.set_attribute("class", v)?;
                            } else if name.starts_with("on_") {
                                // Event handlers are handled separately
                            } else {
                                element.set_attribute(name, v)?;
                            }
                        }
                        VAttr::Event(handler) => {
                            if let Some(event_name) = name.strip_prefix("on_") {
                                let handler_clone = handler.clone();
                                let closure = Closure::wrap(Box::new(move |_event: Event| {
                                    handler_clone.borrow_mut()();
                                })
                                    as Box<dyn FnMut(Event)>);

                                element.add_event_listener_with_callback(
                                    event_name,
                                    closure.as_ref().unchecked_ref(),
                                )?;
                                closure.forget(); // Keep the closure alive
                            }
                        }
                    }
                }

                // Render children
                for child in children {
                    let child_node = child.render(document)?;
                    element.append_child(&child_node)?;
                }

                Ok(element.into())
            }
            VNode::Text(content) => {
                let text_node = document.create_text_node(content);
                Ok(text_node.into())
            }
            VNode::RawHtml(html) => {
                let wrap = document.create_element("div")?;
                wrap.set_inner_html(html);
                Ok(wrap.into())
            }
        }
    }

    /// Mount this VNode to a parent element
    #[cfg(target_arch = "wasm32")]
    pub fn mount(&self, parent: &Element) -> Result<(), JsValue> {
        let document = parent
            .owner_document()
            .ok_or_else(|| JsValue::from_str("No document"))?;
        let node = self.render(&document)?;
        parent.append_child(&node)?;
        Ok(())
    }
}

fn sanitize_attrs(attrs: &[(String, VAttr)]) -> Vec<(String, VAttr)> {
    let mut out: Vec<(String, VAttr)> = Vec::with_capacity(attrs.len());
    for (name, value) in attrs {
        let cleaned = match value {
            VAttr::Event(_) => crate::escape::is_valid_attr_name(name).then(|| value.clone()),
            VAttr::Static(v) | VAttr::Dynamic(v) if name.starts_with("on_") => {
                let _ = v; // event attributes must be VAttr::Event
                None
            }
            VAttr::Static(v) | VAttr::Dynamic(v) => {
                crate::escape::safe_attr(name, v).map(VAttr::Static)
            }
        };
        if let Some(c) = cleaned {
            out.retain(|(n, v)| !(n == name && matches!(v, VAttr::Static(_)) == matches!(c, VAttr::Static(_))));
            out.push((name.clone(), c));
        }
    }
    out
}

/// Legacy host-side debug render: tags and text only, escaped. Use
/// `simple_renderer::render_to_html` for full output.
#[cfg(not(target_arch = "wasm32"))]
impl VNode {
    pub fn render(&self) -> Result<String, String> {
        match self {
            VNode::Element {
                tag,
                attrs: _,
                children,
            } => {
                let mut html = format!("<{}>", tag);
                for child in children {
                    html.push_str(&child.render()?);
                }
                html.push_str(&format!("</{}>", tag));
                Ok(html)
            }
            VNode::Text(content) => Ok(crate::escape::escape_text(content).into_string()),
            VNode::RawHtml(html) => Ok(html.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_html_preserves_trusted_fragment() {
        let vnode = VNode::raw_html("<section class=\"kpi\">42</section>");
        let html = vnode.render().expect("host render");
        assert_eq!(html, "<section class=\"kpi\">42</section>");
    }
}
