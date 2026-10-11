//! Simple UI renderer for basic elements
//! This provides a minimal but functional UI rendering system

use crate::vdom::{VElement, VNode};

/// Simple UI renderer that converts VNodes to console output
/// This is a minimal implementation for testing and demos
pub struct SimpleRenderer {
    output: Vec<String>,
}

impl SimpleRenderer {
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }

    /// Render a VNode tree to console output
    pub fn render(&mut self, vnode: &VNode) {
        self.output.clear();
        self.render_node(vnode, 0);
    }

    fn render_node(&mut self, vnode: &VNode, depth: usize) {
        let indent = "  ".repeat(depth);

        match vnode {
            VNode::Element(element) => {
                self.render_element(element, depth);
            }
            VNode::Text(text) => {
                self.output
                    .push(format!("{}Text: {}", indent, text.content));
            }
            VNode::Component(component) => {
                self.output
                    .push(format!("{}Component: {}", indent, component.name));
            }
            VNode::Empty => {
                // Empty nodes don't render anything
            }
        }
    }

    fn render_element(&mut self, element: &VElement, depth: usize) {
        let indent = "  ".repeat(depth);

        // Render opening tag
        let mut tag_str = format!("{}<{}", indent, element.tag);

        // Add attributes
        for (key, value) in &element.attrs {
            tag_str.push_str(&format!(" {}=\"{}\"", key, value));
        }
        tag_str.push('>');

        self.output.push(tag_str);

        // Render children
        for child in &element.children {
            self.render_node(child, depth + 1);
        }

        // Render closing tag
        self.output.push(format!("{}</{}>", indent, element.tag));
    }

    /// Get the rendered output as a string
    pub fn get_output(&self) -> String {
        self.output.join("\n")
    }

    /// Print the rendered output
    pub fn print(&self) {
        println!("{}", self.get_output());
    }
}

impl Default for SimpleRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vdom::{VComponent, VElement, VText};
    use std::collections::HashMap;

    #[test]
    fn test_render_text() {
        let mut renderer = SimpleRenderer::new();
        let vnode = VNode::Text(VText::new("Hello, World!"));

        renderer.render(&vnode);
        let output = renderer.get_output();

        assert!(output.contains("Hello, World!"));
    }

    #[test]
    fn test_render_element() {
        let mut renderer = SimpleRenderer::new();
        let element = VElement {
            tag: "div".to_string(),
            attrs: HashMap::new(),
            children: vec![VNode::Text(VText::new("Content"))],
        };

        renderer.render(&VNode::Element(element));
        let output = renderer.get_output();

        assert!(output.contains("<div>"));
        assert!(output.contains("Content"));
        assert!(output.contains("</div>"));
    }

    #[test]
    fn test_render_component() {
        let mut renderer = SimpleRenderer::new();
        let component = VComponent {
            name: "Counter".to_string(),
            props: HashMap::new(),
        };

        renderer.render(&VNode::Component(component));
        let output = renderer.get_output();

        assert!(output.contains("Component: Counter"));
    }
}

/// Void elements never get a closing tag.
pub fn is_void_tag(tag: &str) -> bool {
    matches!(
        tag,
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta"
            | "param" | "source" | "track" | "wbr"
    )
}

/// Render a VNode to an HTML string (WASM first paint + host tests).
///
/// Escaped by default: text and attribute values are escaped, script-capable URLs are
/// neutralized, invalid tags/attribute names and inline `on*` attributes are dropped.
/// Only `VNode::RawHtml` is emitted verbatim.
pub fn render_to_html(vnode: &crate::simple_vnode::VNode) -> String {
    let mut out = String::new();
    write_html(&vnode.sanitized(), &mut out);
    out
}

/// Serialize an already-sanitized tree.
fn write_html(vnode: &crate::simple_vnode::VNode, out: &mut String) {
    use crate::escape::{escape_attr, escape_text};
    use crate::simple_vnode::{VAttr, VNode};

    match vnode {
        VNode::Element { tag, attrs, children } => {
            out.push('<');
            out.push_str(tag);
            for (key, value) in attrs {
                if let VAttr::Static(v) | VAttr::Dynamic(v) = value {
                    out.push_str(&format!(" {}=\"{}\"", key, escape_attr(v)));
                }
            }
            out.push('>');
            if is_void_tag(tag) {
                return;
            }
            for child in children {
                write_html(child, out);
            }
            out.push_str("</");
            out.push_str(tag);
            out.push('>');
        }
        VNode::Text(text) => out.push_str(escape_text(text).as_str()),
        VNode::RawHtml(html) => out.push_str(html),
    }
}

#[cfg(test)]
mod render_to_html_tests {
    use super::render_to_html;
    use crate::simple_vnode::{VAttr, VNode};

    #[test]
    fn render_to_html_emits_raw_html_unchanged() {
        let vnode = VNode::raw_html("<div id=\"taskMount\">ok</div>");
        assert_eq!(render_to_html(&vnode), "<div id=\"taskMount\">ok</div>");
    }

    #[test]
    fn render_to_html_wraps_element_children() {
        let vnode = VNode::element(
            "div",
            vec![("class", VAttr::Static("panel".into()))],
            vec![VNode::raw_html("<span>kpi</span>")],
        );
        let html = render_to_html(&vnode);
        assert!(html.contains("class=\"panel\""));
        assert!(html.contains("<span>kpi</span>"));
    }
}

#[cfg(test)]
mod xss_tests {
    use super::render_to_html;
    use crate::simple_vnode::{VAttr, VNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn el(tag: &str, attrs: Vec<(&str, VAttr)>, kids: Vec<VNode>) -> VNode {
        VNode::element(tag, attrs, kids)
    }
    fn st(v: &str) -> VAttr {
        VAttr::Static(v.into())
    }

    #[test]
    fn text_script_payload_is_escaped() {
        let h = render_to_html(&el("div", vec![], vec![VNode::text("<script>alert(1)</script>")]));
        assert_eq!(h, "<div>&lt;script&gt;alert(1)&lt;/script&gt;</div>");
    }

    #[test]
    fn attribute_breakout_is_contained() {
        let h = render_to_html(&el("input", vec![("value", st("\" onmouseover=\"alert(1)"))], vec![]));
        assert_eq!(h, "<input value=\"&quot; onmouseover=&quot;alert(1)\">");
        let h = render_to_html(&el("a", vec![("title", st("' onfocus='x"))], vec![]));
        assert!(h.contains("&#39; onfocus=&#39;x"));
    }

    #[test]
    fn javascript_urls_are_neutralized_in_url_attributes() {
        for (attr, val) in [("href", "javascript:alert(1)"), ("src", " JaVa\tScRiPt:alert(1)"), ("action", "vbscript:x"), ("formaction", "data:text/html,x")] {
            let h = render_to_html(&el("a", vec![(attr, st(val))], vec![]));
            assert!(h.contains(&format!("{attr}=\"#\"")), "{h}");
        }
        let ok = render_to_html(&el("a", vec![("href", st("https://example.com/?a=1&b=2"))], vec![]));
        assert_eq!(ok, "<a href=\"https://example.com/?a=1&amp;b=2\"></a>");
    }

    #[test]
    fn nested_elements_escape_every_level() {
        let tree = el("ul", vec![("class", st("\"x"))], vec![
            el("li", vec![], vec![VNode::text("<b>1</b>"), el("em", vec![], vec![VNode::text("'&\"")])]),
        ]);
        assert_eq!(
            render_to_html(&tree),
            "<ul class=\"&quot;x\"><li>&lt;b&gt;1&lt;/b&gt;<em>&#39;&amp;&quot;</em></li></ul>"
        );
    }

    #[test]
    fn unicode_is_preserved_verbatim() {
        let h = render_to_html(&el("p", vec![("title", st("日本 🚀"))], vec![VNode::text("héllo ✓")]));
        assert_eq!(h, "<p title=\"日本 🚀\">héllo ✓</p>");
    }

    #[test]
    fn hostile_tag_and_attribute_names_are_dropped() {
        assert_eq!(render_to_html(&el("div onclick=alert(1)", vec![], vec![])), "");
        assert_eq!(render_to_html(&el("script", vec![], vec![VNode::text("alert(1)")])), "");
        let h = render_to_html(&el("div", vec![("x onmouseover=alert(1) y", st("1")), ("ok", st("2"))], vec![]));
        assert_eq!(h, "<div ok=\"2\"></div>");
    }

    #[test]
    fn inline_handler_attributes_are_dropped_but_event_attrs_are_not_serialized() {
        let ev = VAttr::Event(Rc::new(RefCell::new(|| {})));
        let h = render_to_html(&el("button", vec![("onclick", st("alert(1)")), ("on_click", ev)], vec![]));
        assert_eq!(h, "<button></button>");
    }

    #[test]
    fn raw_html_is_the_only_unescaped_path() {
        let h = render_to_html(&el("div", vec![], vec![VNode::raw(crate::escape::RawHtml::new("<b>trusted</b>"))]));
        assert_eq!(h, "<div><b>trusted</b></div>");
    }

    #[test]
    fn void_elements_have_no_closing_tag() {
        assert_eq!(render_to_html(&el("br", vec![], vec![])), "<br>");
    }

    #[test]
    fn top_level_text_is_escaped() {
        assert_eq!(render_to_html(&VNode::text("a<b")), "a&lt;b");
    }
}
