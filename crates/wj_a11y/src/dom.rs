use wj_scene::{Role, SemanticNode, SemanticTree};

/// A neutral element tree. The web host turns it into DOM (through the keyed patcher), SSR turns
/// it into a string with [`ProjectedNode::to_html`].
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedNode {
    pub tag: &'static str,
    pub attrs: Vec<(&'static str, String)>,
    pub text: Option<String>,
    pub children: Vec<ProjectedNode>,
}

fn escape(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
}

impl ProjectedNode {
    /// Serialize with escape-by-default for text and attribute values.
    pub fn to_html(&self) -> String {
        let mut s = String::new();
        self.write(&mut s);
        s
    }

    fn write(&self, out: &mut String) {
        out.push('<');
        out.push_str(self.tag);
        for (k, v) in &self.attrs {
            out.push(' ');
            out.push_str(k);
            out.push_str("=\"");
            escape(v, out);
            out.push('"');
        }
        out.push('>');
        if self.tag == "input" || self.tag == "img" {
            return; // void elements
        }
        if let Some(t) = &self.text {
            escape(t, out);
        }
        for c in &self.children {
            c.write(out);
        }
        out.push_str("</");
        out.push_str(self.tag);
        out.push('>');
    }
}

fn node(n: &SemanticNode) -> ProjectedNode {
    let name = n.name.clone().unwrap_or_default();
    let mut attrs: Vec<(&'static str, String)> = vec![("data-wj", format!("{}", id_str(n)))];
    attrs.push((
        "style",
        format!("position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;color:transparent;background:none;border:0;padding:0;margin:0", n.bounds.x, n.bounds.y, n.bounds.w, n.bounds.h),
    ));
    if n.focusable {
        attrs.push(("tabindex", "0".into()));
    }
    if n.disabled {
        attrs.push(("aria-disabled", "true".into()));
    }
    let (tag, text): (&'static str, Option<String>) = match n.role {
        Role::Button => {
            attrs.push(("type", "button".into()));
            ("button", Some(name.clone()))
        }
        Role::Checkbox => {
            attrs.push(("type", "checkbox".into()));
            attrs.push(("aria-label", name.clone()));
            if n.checked == Some(true) {
                attrs.push(("checked", "checked".into()));
            }
            ("input", None)
        }
        Role::TextField => {
            attrs.push(("type", "text".into()));
            attrs.push(("aria-label", name.clone()));
            attrs.push(("value", n.value.clone().unwrap_or_default()));
            ("input", None)
        }
        Role::Heading => ("h2", Some(name.clone())),
        Role::Text => ("span", Some(name.clone())),
        Role::List => ("ul", None),
        Role::ListItem => ("li", None),
        Role::Image => {
            attrs.push(("alt", name.clone()));
            ("img", None)
        }
        Role::ScrollView => {
            attrs.push(("role", "region".into()));
            if !name.is_empty() {
                attrs.push(("aria-label", name.clone()));
            }
            ("div", None)
        }
        Role::Group | Role::None => {
            attrs.push(("role", "group".into()));
            if !name.is_empty() {
                attrs.push(("aria-label", name.clone()));
            }
            ("div", None)
        }
    };
    ProjectedNode { tag, attrs, text, children: n.children.iter().map(node).collect() }
}

fn id_str(n: &SemanticNode) -> String {
    let (i, g) = n.id.parts();
    format!("{i}v{g}")
}

/// Project the semantic tree to a transparent, absolutely positioned DOM mirror.
pub fn project(tree: &SemanticTree) -> ProjectedNode {
    ProjectedNode {
        tag: "div",
        attrs: vec![("role", "application".into()), ("aria-label", "app".into()), ("style", "position:fixed;left:0;top:0;pointer-events:none".into())],
        text: None,
        children: tree.roots.iter().map(node).collect(),
    }
}
