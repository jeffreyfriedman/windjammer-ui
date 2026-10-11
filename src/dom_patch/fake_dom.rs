//! In-memory DOM implementing [`DomPort`], for host tests. Also models the pieces of
//! browser state patching must not disturb: focus, input value and caret.

use super::apply::Canon;
use super::port::DomPort;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FakeNode(pub usize);

#[derive(Debug)]
enum Kind {
    Element { tag: String, attrs: BTreeMap<String, String> },
    Text(String),
    Raw(String),
}

#[derive(Debug)]
struct Entry {
    kind: Kind,
    parent: Option<usize>,
    children: Vec<usize>,
    /// Live `value` property (diverges from the attribute once the user types).
    value: Option<String>,
    caret: usize,
}

#[derive(Debug)]
pub struct FakeDom {
    nodes: Vec<Entry>,
    focused: Option<usize>,
    pub listening: BTreeSet<String>,
    /// Count of `create_*` calls; lets tests assert nodes were reused, not rebuilt.
    pub created: usize,
    /// Model a host with `Element.moveBefore` (state-preserving move).
    pub supports_move_before: bool,
    /// Number of moves done atomically (focus never lost).
    pub atomic_moves: usize,
}

impl Default for FakeDom {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeDom {
    pub fn new() -> Self {
        let root = Entry {
            kind: Kind::Element { tag: "#container".into(), attrs: BTreeMap::new() },
            parent: None,
            children: vec![],
            value: None,
            caret: 0,
        };
        FakeDom { nodes: vec![root], focused: None, listening: BTreeSet::new(), created: 0, supports_move_before: false, atomic_moves: 0 }
    }

    fn add(&mut self, kind: Kind) -> FakeNode {
        self.created += 1;
        self.nodes.push(Entry { kind, parent: None, children: vec![], value: None, caret: 0 });
        FakeNode(self.nodes.len() - 1)
    }

    fn contains(&self, ancestor: usize, mut n: usize) -> bool {
        loop {
            if n == ancestor {
                return true;
            }
            match self.nodes[n].parent {
                Some(p) => n = p,
                None => return false,
            }
        }
    }

    /// Browser behaviour: removing a subtree that holds focus blurs it.
    fn detach(&mut self, n: usize) {
        if let Some(f) = self.focused {
            if self.contains(n, f) {
                self.focused = None;
            }
        }
        if let Some(p) = self.nodes[n].parent.take() {
            self.nodes[p].children.retain(|&c| c != n);
        }
    }

    /// Canonical snapshot of the mounted tree (container's first child).
    pub fn snapshot(&self) -> Option<Canon> {
        self.nodes[0].children.first().map(|&c| self.canon(c))
    }

    fn canon(&self, n: usize) -> Canon {
        match &self.nodes[n].kind {
            Kind::Text(t) => Canon::Text(t.clone()),
            Kind::Raw(h) => Canon::Raw(h.clone()),
            Kind::Element { tag, attrs } => Canon::El {
                tag: tag.clone(),
                attrs: attrs.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
                kids: self.nodes[n].children.iter().map(|&c| self.canon(c)).collect(),
            },
        }
    }

    // --- user-simulation helpers ---
    pub fn focus(&mut self, n: FakeNode) {
        self.focused = Some(n.0);
    }
    pub fn focused(&self) -> Option<FakeNode> {
        self.focused.map(FakeNode)
    }
    /// Simulate typing: sets the live value property and caret.
    pub fn type_value(&mut self, n: FakeNode, value: &str, caret: usize) {
        self.nodes[n.0].value = Some(value.to_string());
        self.nodes[n.0].caret = caret;
    }
    pub fn value(&self, n: FakeNode) -> Option<String> {
        self.nodes[n.0].value.clone()
    }
    pub fn caret(&self, n: FakeNode) -> usize {
        self.nodes[n.0].caret
    }
    /// Find the first element with attribute `name == value` (depth-first).
    pub fn find_by_attr(&self, name: &str, value: &str) -> Option<FakeNode> {
        (0..self.nodes.len()).find_map(|i| match &self.nodes[i].kind {
            Kind::Element { attrs, .. }
                if attrs.get(name).map(String::as_str) == Some(value) && self.attached(i) =>
            {
                Some(FakeNode(i))
            }
            _ => None,
        })
    }
    fn attached(&self, mut n: usize) -> bool {
        while let Some(p) = self.nodes[n].parent {
            n = p;
        }
        n == 0
    }
    /// Concatenated text of the subtree.
    pub fn text_content(&self, n: FakeNode) -> String {
        match &self.nodes[n.0].kind {
            Kind::Text(t) => t.clone(),
            Kind::Raw(h) => h.clone(),
            Kind::Element { .. } => self.nodes[n.0]
                .children
                .iter()
                .map(|&c| self.text_content(FakeNode(c)))
                .collect(),
        }
    }
}

impl DomPort for FakeDom {
    type Node = FakeNode;

    fn container(&self) -> FakeNode {
        FakeNode(0)
    }
    fn create_element(&mut self, tag: &str) -> FakeNode {
        self.add(Kind::Element { tag: tag.into(), attrs: BTreeMap::new() })
    }
    fn create_text(&mut self, text: &str) -> FakeNode {
        self.add(Kind::Text(text.into()))
    }
    fn create_raw(&mut self, html: &str) -> FakeNode {
        self.add(Kind::Raw(html.into()))
    }
    fn set_attr(&mut self, node: &FakeNode, name: &str, value: &str) {
        let e = &mut self.nodes[node.0];
        if let Kind::Element { attrs, .. } = &mut e.kind {
            attrs.insert(name.into(), value.into());
        }
        // Like the browser: writing `value` only touches the property when it differs.
        if name == "value" && e.value.as_deref() != Some(value) {
            e.value = Some(value.into());
            e.caret = value.len();
        }
    }
    fn remove_attr(&mut self, node: &FakeNode, name: &str) {
        if let Kind::Element { attrs, .. } = &mut self.nodes[node.0].kind {
            attrs.remove(name);
        }
    }
    fn set_text(&mut self, node: &FakeNode, text: &str) {
        self.nodes[node.0].kind = Kind::Text(text.into());
    }
    fn child_at(&self, parent: &FakeNode, index: usize) -> Option<FakeNode> {
        self.nodes[parent.0].children.get(index).map(|&c| FakeNode(c))
    }
    fn parent(&self, node: &FakeNode) -> Option<FakeNode> {
        self.nodes[node.0].parent.map(FakeNode)
    }
    fn index_in_parent(&self, node: &FakeNode) -> Option<usize> {
        let p = self.nodes[node.0].parent?;
        self.nodes[p].children.iter().position(|&c| c == node.0)
    }
    fn insert_child(&mut self, parent: &FakeNode, index: usize, child: &FakeNode) {
        self.detach(child.0);
        self.nodes[parent.0].children.insert(index, child.0);
        self.nodes[child.0].parent = Some(parent.0);
    }
    fn remove(&mut self, node: &FakeNode) {
        self.detach(node.0);
        if self.focused == Some(node.0) {
            self.focused = None;
        }
    }
    fn replace(&mut self, old: &FakeNode, new: &FakeNode) {
        let (p, i) = (self.parent(old).expect("attached"), self.index_in_parent(old).unwrap());
        self.remove(old);
        self.insert_child(&p, i, new);
    }
    fn focused_within(&self, node: &FakeNode) -> Option<FakeNode> {
        self.focused.filter(|&f| self.contains(node.0, f)).map(FakeNode)
    }
    fn selection_of(&self, node: &FakeNode) -> Option<(usize, usize)> {
        Some((self.nodes[node.0].caret, self.nodes[node.0].caret))
    }
    fn focus_node(&mut self, node: &FakeNode, selection: Option<(usize, usize)>) {
        self.focused = Some(node.0);
        if let Some((_, end)) = selection {
            self.nodes[node.0].caret = end;
        }
    }
    fn move_child(&mut self, parent: &FakeNode, index: usize, child: &FakeNode) {
        if self.supports_move_before {
            // Atomic: detach without blurring.
            let keep = self.focused;
            self.insert_child(parent, index, child);
            self.focused = keep;
            self.atomic_moves += 1;
        } else {
            let had = self.focused_within(child).map(|f| (self.selection_of(&f), f));
            self.insert_child(parent, index, child);
            if let Some((sel, f)) = had {
                self.focus_node(&f, sel);
            }
        }
    }
    fn listen(&mut self, event: &str) {
        self.listening.insert(event.into());
    }
}
