//! Server-Side Rendering (SSR) for web targets

use crate::component::Component;
use crate::vdom::{VElement, VNode, VText};
use std::collections::HashMap;

/// SSR renderer that converts VNodes to HTML strings
pub struct SSRRenderer {
    /// Generated HTML
    html: String,
    /// Component state for hydration
    state: HashMap<String, String>,
    /// Hydration script
    hydration_script: String,
}

impl SSRRenderer {
    /// Create a new SSR renderer
    pub fn new() -> Self {
        Self {
            html: String::new(),
            state: HashMap::new(),
            hydration_script: String::new(),
        }
    }

    /// Render a component to HTML string
    pub fn render_to_string<C: Component>(&mut self, component: C) -> String {
        let vnode = component.render();
        self.render_vnode(&vnode);
        self.html.clone()
    }

    /// Render a component to full HTML document with hydration
    pub fn render_to_document<C: Component>(&mut self, component: C, title: &str) -> String {
        let body_html = self.render_to_string(component);

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <script id="__WINDJAMMER_STATE__" type="application/json">
    {}
    </script>
</head>
<body>
    <div id="app">{}</div>
    <script>
        {}
    </script>
</body>
</html>"#,
            Self::escape_html(title),
            Self::state_json_for_script(&self.state),
            body_html,
            self.get_hydration_script()
        )
    }

    /// JSON for an inline `<script type="application/json">`: `<`, `>`, `&` and the JS
    /// line separators are `\u`-escaped so state values cannot close the tag or start a
    /// comment. `JSON.parse` / serde decode the escapes back to the original text.
    fn state_json_for_script(state: &HashMap<String, String>) -> String {
        let json = serde_json::to_string(state).unwrap_or_default();
        let mut out = String::with_capacity(json.len());
        for c in json.chars() {
            match c {
                '<' => out.push_str("\\u003c"),
                '>' => out.push_str("\\u003e"),
                '&' => out.push_str("\\u0026"),
                '\u{2028}' => out.push_str("\\u2028"),
                '\u{2029}' => out.push_str("\\u2029"),
                c => out.push(c),
            }
        }
        out
    }

    /// Get the hydration script
    pub fn get_hydration_script(&self) -> String {
        if !self.hydration_script.is_empty() {
            return self.hydration_script.clone();
        }

        // Default hydration script
        r#"
        (function() {
            // Hydration: Attach event listeners to server-rendered HTML
            const state = JSON.parse(document.getElementById('__WINDJAMMER_STATE__').textContent);
            
            // Store state for client-side hydration
            window.__WINDJAMMER_HYDRATION_STATE__ = state;
            
            // Mark as hydrated
            document.getElementById('app').setAttribute('data-hydrated', 'true');
            
            console.log('Windjammer: Hydration complete', state);
        })();
        "#
        .to_string()
    }

    /// Render a VNode to HTML
    fn render_vnode(&mut self, vnode: &VNode) {
        match vnode {
            VNode::Element(element) => self.render_element(element),
            VNode::Text(text) => self.render_text(text),
            VNode::Component(_) => {
                // Components should be expanded before SSR
                self.html.push_str("<!-- Component not expanded -->");
            }
            VNode::Empty => {}
        }
    }

    /// Render an element
    fn render_element(&mut self, element: &VElement) {
        if !crate::escape::is_valid_tag(&element.tag) {
            return; // invalid or blocked tag: emit nothing (children dropped too)
        }
        // Opening tag
        self.html.push('<');
        self.html.push_str(&element.tag);

        // Attributes
        for (key, value) in &element.attrs {
            if let Some(attr) = Self::attribute_html(key, value) {
                self.html.push(' ');
                self.html.push_str(&attr);
            }
        }

        // Self-closing tags
        if element.children.is_empty() && Self::is_void_element(&element.tag) {
            self.html.push_str(" />");
            return;
        }

        self.html.push('>');

        // Children
        for child in &element.children {
            self.render_vnode(child);
        }

        // Closing tag
        self.html.push_str("</");
        self.html.push_str(&element.tag);
        self.html.push('>');
    }

    /// Render text
    fn render_text(&mut self, text: &VText) {
        self.html.push_str(&Self::escape_html(&text.content));
    }

    /// Check if element is self-closing (void element)
    fn is_void_element(tag: &str) -> bool {
        matches!(
            tag,
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        )
    }

    /// Escape HTML special characters
    fn escape_html(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Escape attribute values
    fn escape_attribute(text: &str) -> String {
        crate::escape::escape_attr(text).into_string()
    }

    /// `name="value"` for a safe attribute (valid name, no inline handler, sanitized URL).
    fn attribute_html(key: &str, value: &str) -> Option<String> {
        let value = crate::escape::safe_attr(key, value)?;
        Some(format!("{}=\"{}\"", key, Self::escape_attribute(&value)))
    }

    /// Add component state for hydration
    pub fn add_state(&mut self, key: String, value: String) {
        self.state.insert(key, value);
    }

    /// Set custom hydration script
    pub fn set_hydration_script(&mut self, script: String) {
        self.hydration_script = script;
    }
}

impl Default for SSRRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming SSR renderer for large pages
pub struct StreamingSSRRenderer {
    chunk_size: usize,
    chunks: Vec<String>,
}

impl StreamingSSRRenderer {
    /// Create a new streaming SSR renderer
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            chunks: Vec::new(),
        }
    }

    /// Render a VNode and return chunks
    pub fn render_vnode(&mut self, vnode: &VNode) -> Vec<String> {
        self.chunks.clear();
        let mut buffer = String::new();

        self.render_vnode_to_buffer(vnode, &mut buffer);

        // Split into chunks of at most `chunk_size` bytes, never inside a character
        // (a single character wider than the limit gets its own chunk).
        let mut rest = buffer.as_str();
        while !rest.is_empty() {
            let mut end = self.chunk_size.max(1).min(rest.len());
            while !rest.is_char_boundary(end) {
                end += 1;
            }
            let (head, tail) = rest.split_at(end);
            self.chunks.push(head.to_string());
            rest = tail;
        }

        self.chunks.clone()
    }

    #[allow(clippy::only_used_in_recursion)]
    fn render_vnode_to_buffer(&self, vnode: &VNode, buffer: &mut String) {
        match vnode {
            VNode::Element(element) => {
                if !crate::escape::is_valid_tag(&element.tag) {
                    return;
                }
                buffer.push('<');
                buffer.push_str(&element.tag);

                for (key, value) in &element.attrs {
                    if let Some(attr) = SSRRenderer::attribute_html(key, value) {
                        buffer.push(' ');
                        buffer.push_str(&attr);
                    }
                }

                if element.children.is_empty() && SSRRenderer::is_void_element(&element.tag) {
                    buffer.push_str(" />");
                } else {
                    buffer.push('>');
                    for child in &element.children {
                        self.render_vnode_to_buffer(child, buffer);
                    }
                    buffer.push_str("</");
                    buffer.push_str(&element.tag);
                    buffer.push('>');
                }
            }
            VNode::Text(text) => {
                buffer.push_str(&SSRRenderer::escape_html(&text.content));
            }
            VNode::Component(_) => {
                buffer.push_str("<!-- Component -->");
            }
            VNode::Empty => {}
        }
    }
}

/// Hydration helper for client-side
pub struct Hydration {
    state: HashMap<String, String>,
}

impl Hydration {
    /// Create from serialized state
    pub fn from_state(state_json: &str) -> Result<Self, String> {
        let state: HashMap<String, String> =
            serde_json::from_str(state_json).map_err(|e| e.to_string())?;
        Ok(Self { state })
    }

    /// Get state value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.state.get(key)
    }

    /// Check if element is hydrated
    pub fn is_hydrated(&self) -> bool {
        !self.state.is_empty()
    }

    /// Helper extension for render_to_document_with_html
    #[allow(dead_code)]
    fn render_to_document_with_html(&self, title: &str, body_html: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    {}
</body>
</html>"#,
            SSRRenderer::escape_html(title),
            body_html
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssr_render_simple_element() {
        let mut renderer = SSRRenderer::new();
        let vnode = VNode::Element(VElement::new("div").child(VNode::Text(VText::new("Hello"))));

        renderer.render_vnode(&vnode);
        assert_eq!(renderer.html, "<div>Hello</div>");
    }

    #[test]
    fn test_ssr_render_with_attributes() {
        let mut renderer = SSRRenderer::new();
        let vnode = VNode::Element(
            VElement::new("div")
                .attr("class", "container")
                .child(VNode::Text(VText::new("Content"))),
        );

        renderer.render_vnode(&vnode);
        assert!(renderer.html.contains("class=\"container\""));
        assert!(renderer.html.contains("Content"));
    }

    #[test]
    fn test_ssr_render_void_element() {
        let mut renderer = SSRRenderer::new();
        let vnode = VNode::Element(VElement::new("br"));

        renderer.render_vnode(&vnode);
        assert_eq!(renderer.html, "<br />");
    }

    #[test]
    fn test_ssr_escape_html() {
        let mut renderer = SSRRenderer::new();
        let vnode = VNode::Element(
            VElement::new("div").child(VNode::Text(VText::new("<script>alert('xss')</script>"))),
        );

        renderer.render_vnode(&vnode);
        assert!(renderer.html.contains("&lt;script&gt;"));
        assert!(!renderer.html.contains("<script>"));
    }

    #[test]
    fn test_ssr_nested_elements() {
        let mut renderer = SSRRenderer::new();
        let vnode = VNode::Element(
            VElement::new("div")
                .child(VNode::Element(
                    VElement::new("p").child(VNode::Text(VText::new("Nested"))),
                ))
                .child(VNode::Element(
                    VElement::new("span").child(VNode::Text(VText::new("Content"))),
                )),
        );

        renderer.render_vnode(&vnode);
        assert!(renderer.html.contains("<div>"));
        assert!(renderer.html.contains("<p>Nested</p>"));
        assert!(renderer.html.contains("<span>Content</span>"));
        assert!(renderer.html.contains("</div>"));
    }

    #[test]
    fn test_ssr_document_generation() {
        let mut renderer = SSRRenderer::new();

        // Create a simple component render result
        let vnode = VNode::Element(VElement::new("h1").child(VNode::Text(VText::new("Hello SSR"))));
        renderer.render_vnode(&vnode);

        // Test that HTML was generated
        assert!(renderer.html.contains("<h1>Hello SSR</h1>"));
    }

    #[test]
    fn test_streaming_ssr() {
        let mut renderer = StreamingSSRRenderer::new(10); // Small chunk size for testing
        let vnode = VNode::Element(
            VElement::new("div").child(VNode::Text(VText::new("This is a longer text"))),
        );

        let chunks = renderer.render_vnode(&vnode);
        assert!(!chunks.is_empty());

        // Recombine chunks
        let combined: String = chunks.into_iter().collect();
        assert!(combined.contains("<div>"));
        assert!(combined.contains("This is a longer text"));
        assert!(combined.contains("</div>"));
    }

    #[test]
    fn test_hydration_state() {
        let mut renderer = SSRRenderer::new();
        renderer.add_state("count".to_string(), "42".to_string());
        renderer.add_state("name".to_string(), "Alice".to_string());

        let state_json = serde_json::to_string(&renderer.state).unwrap();
        let hydration = Hydration::from_state(&state_json).unwrap();

        assert_eq!(hydration.get("count"), Some(&"42".to_string()));
        assert_eq!(hydration.get("name"), Some(&"Alice".to_string()));
        assert!(hydration.is_hydrated());
    }

    #[test]
    fn test_attribute_escaping() {
        let mut renderer = SSRRenderer::new();
        let vnode = VNode::Element(
            VElement::new("input")
                .attr("value", "Test \"quoted\" value")
                .attr("data-test", "<script>"),
        );

        renderer.render_vnode(&vnode);
        assert!(renderer.html.contains("&quot;"));
        assert!(renderer.html.contains("&lt;script&gt;"));
    }

    fn render(v: &VNode) -> String {
        let mut r = SSRRenderer::new();
        r.render_vnode(v);
        r.html
    }

    #[test]
    fn url_attributes_are_sanitized_for_every_url_attr() {
        for name in ["href", "src", "action", "formaction", "poster", "data", "cite", "background", "ping", "xlink:href"] {
            for bad in ["javascript:alert(1)", "JaVaScRiPt:alert(1)", " \tjava\nscript:alert(1)", "vbscript:x", "data:text/html,<script>alert(1)</script>"] {
                let out = render(&VNode::Element(VElement::new("a").attr(name, bad)));
                assert!(!out.to_lowercase().contains("script:") && !out.contains("data:text"), "{name}={bad:?} -> {out}");
                assert!(out.contains(&format!("{name}=\"#\"")), "{name}={bad:?} -> {out}");
            }
        }
    }

    #[test]
    fn safe_urls_pass_through_and_are_attribute_escaped() {
        let out = render(&VNode::Element(VElement::new("a").attr("href", "https://x.test/?a=1&b=\"2\"")));
        assert!(out.contains("href=\"https://x.test/?a=1&amp;b=&quot;2&quot;\""), "{out}");
        for ok in ["/rel/path", "#frag", "mailto:a@b.test", "tel:+1555", "data:image/png;base64,AAAA"] {
            let out = render(&VNode::Element(VElement::new("a").attr("href", ok)));
            assert!(out.contains(&format!("href=\"{ok}\"")), "{ok} -> {out}");
        }
    }

    #[test]
    fn inline_handlers_and_bad_attribute_names_are_dropped() {
        let out = render(&VNode::Element(
            VElement::new("div").attr("onclick", "alert(1)").attr("x\" onfocus=\"alert(1)", "v").attr("id", "ok"),
        ));
        assert!(!out.contains("onclick") && !out.contains("onfocus"), "{out}");
        assert!(out.contains("id=\"ok\""));
    }

    #[test]
    fn hostile_tag_names_cannot_inject_markup() {
        for tag in ["script", "img src=x onerror=alert(1)", "div><script>alert(1)</script", "iframe"] {
            let out = render(&VNode::Element(VElement::new(tag).child(VText::new("t").into())));
            assert!(!out.contains("<script") && !out.contains("onerror") && !out.contains("<iframe"), "{tag:?} -> {out}");
        }
    }

    #[test]
    fn streaming_renderer_applies_the_same_sanitization() {
        let v = VNode::Element(VElement::new("a").attr("href", "javascript:alert(1)").attr("onclick", "x").child(VText::new("<b>").into()));
        let joined = StreamingSSRRenderer::new(7).render_vnode(&v).concat();
        assert!(joined.contains("href=\"#\"") && !joined.contains("onclick") && joined.contains("&lt;b&gt;"), "{joined}");
        let bad_tag = VNode::Element(VElement::new("script"));
        assert!(!StreamingSSRRenderer::new(7).render_vnode(&bad_tag).concat().contains("<script"));
    }

    #[test]
    fn streaming_chunks_never_split_multibyte_characters() {
        let v = VNode::Text(VText::new("héllo wörld ✓ 日本語"));
        let chunks = StreamingSSRRenderer::new(3).render_vnode(&v);
        assert!(chunks.iter().all(|c| !c.contains('\u{FFFD}')), "{chunks:?}");
        assert_eq!(chunks.concat(), "héllo wörld ✓ 日本語");
    }

    #[test]
    fn hydration_state_cannot_break_out_of_the_json_script_tag() {
        let mut r = SSRRenderer::new();
        r.add_state("k".into(), "</script><img src=x onerror=alert(1)>".into());
        r.add_state("\u{2028}".into(), "<!--".into());
        let doc = r.render_to_document(Hello, "t</title><script>alert(1)</script>");
        assert_eq!(doc.matches("</script>").count(), 2, "only the two real script tags: {doc}");
        assert!(!doc.contains("<img src=x"), "{doc}");
        assert!(!doc.contains("<script>alert"), "{doc}");
        // The state must still round-trip through JSON.parse semantics.
        let start = doc.find("application/json\">").unwrap() + "application/json\">".len();
        let end = doc[start..].find("</script>").unwrap() + start;
        let parsed: HashMap<String, String> = serde_json::from_str(doc[start..end].trim()).unwrap();
        assert_eq!(parsed["k"], "</script><img src=x onerror=alert(1)>");
    }

    struct Hello;
    impl crate::component::Component for Hello {
        fn render(&self) -> VNode {
            VNode::Text(VText::new("hi"))
        }
    }
}
