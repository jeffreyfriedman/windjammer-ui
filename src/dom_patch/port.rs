//! The DOM port (hexagonal boundary). The patcher only talks to this trait;
//! `fake_dom::FakeDom` implements it in memory, `web::WebDom` over web-sys.

use super::diff::PatchOp;
use super::Path;
use crate::simple_vnode::{VAttr, VNode};

pub trait DomPort {
    type Node: Clone + PartialEq;

    /// Container the app tree is mounted in; the tree root is its only child.
    fn container(&self) -> Self::Node;
    fn create_element(&mut self, tag: &str) -> Self::Node;
    fn create_text(&mut self, text: &str) -> Self::Node;
    /// One wrapper node whose content is trusted markup.
    fn create_raw(&mut self, html: &str) -> Self::Node;
    fn set_attr(&mut self, node: &Self::Node, name: &str, value: &str);
    fn remove_attr(&mut self, node: &Self::Node, name: &str);
    fn set_text(&mut self, node: &Self::Node, text: &str);
    fn child_at(&self, parent: &Self::Node, index: usize) -> Option<Self::Node>;
    fn parent(&self, node: &Self::Node) -> Option<Self::Node>;
    fn index_in_parent(&self, node: &Self::Node) -> Option<usize>;
    /// Insert at `index`; if `child` is already attached it is moved (DOM `insertBefore`):
    /// it is detached first and `index` addresses the remaining siblings.
    fn insert_child(&mut self, parent: &Self::Node, index: usize, child: &Self::Node);
    fn remove(&mut self, node: &Self::Node);
    fn replace(&mut self, old: &Self::Node, new: &Self::Node);
    /// Start delivering `event` (capturing, at the container) to the app.
    fn listen(&mut self, event: &str);
}

/// Build the DOM subtree for a sanitized VNode.
pub fn create_tree<P: DomPort>(port: &mut P, vnode: &VNode) -> P::Node {
    match vnode {
        VNode::Text(t) => port.create_text(t),
        VNode::RawHtml(h) => port.create_raw(h),
        VNode::Element { tag, attrs, children } => {
            let el = port.create_element(tag);
            for (name, value) in super::diff::plain_attrs(attrs) {
                port.set_attr(&el, name, value);
            }
            for (i, child) in children.iter().enumerate() {
                let c = create_tree(port, child);
                port.insert_child(&el, i, &c);
            }
            el
        }
    }
}

fn resolve<P: DomPort>(port: &P, path: &[usize]) -> P::Node {
    let mut cur = port.child_at(&port.container(), 0).expect("mounted root");
    for &i in path {
        cur = port.child_at(&cur, i).expect("path resolves");
    }
    cur
}

/// Path of `node` from the mounted root, or `None` if it is outside the tree.
pub fn path_of<P: DomPort>(port: &P, node: &P::Node) -> Option<Path> {
    let container = port.container();
    let mut path = Vec::new();
    let mut cur = node.clone();
    while cur != container {
        path.push(port.index_in_parent(&cur)?);
        cur = port.parent(&cur)?;
    }
    path.pop()?; // drop the root's index inside the container
    path.reverse();
    Some(path)
}

/// Apply ops to the live DOM in order.
pub fn apply_ops<P: DomPort>(port: &mut P, ops: &[PatchOp]) {
    for op in ops {
        apply_op(port, op);
    }
}

fn apply_op<P: DomPort>(port: &mut P, op: &PatchOp) {
    match op {
        PatchOp::Replace { path, node } => {
            let old = resolve(port, path);
            let new = create_tree(port, node);
            port.replace(&old, &new);
        }
        PatchOp::Insert { parent, index, node } => {
            let p = resolve(port, parent);
            let new = create_tree(port, node);
            port.insert_child(&p, *index, &new);
        }
        PatchOp::Remove { path } => {
            let n = resolve(port, path);
            port.remove(&n);
        }
        PatchOp::Move { parent, from, to } => {
            let p = resolve(port, parent);
            let n = port.child_at(&p, *from).expect("move source");
            // insert_child detaches first, so `to` indexes the list without `n`.
            port.insert_child(&p, *to, &n);
        }
        PatchOp::SetAttr { path, name, value } => {
            let n = resolve(port, path);
            port.set_attr(&n, name, value);
        }
        PatchOp::RemoveAttr { path, name } => {
            let n = resolve(port, path);
            port.remove_attr(&n, name);
        }
        PatchOp::SetText { path, text } => {
            let n = resolve(port, path);
            port.set_text(&n, text);
        }
    }
}

/// Event names (`on_click` -> `click`) of all event attributes in the tree.
pub fn collect_event_names(vnode: &VNode, out: &mut std::collections::BTreeSet<String>) {
    if let VNode::Element { attrs, children, .. } = vnode {
        for (k, v) in attrs {
            if let (VAttr::Event(_), Some(ev)) = (v, k.strip_prefix("on_")) {
                out.insert(ev.to_string());
            }
        }
        children.iter().for_each(|c| collect_event_names(c, out));
    }
}
