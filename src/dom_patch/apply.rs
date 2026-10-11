//! Apply patch ops to a pure VNode tree (reference semantics; the oracle for DOM adapters).

use super::diff::PatchOp;
use crate::simple_vnode::{VAttr, VNode};

/// Apply `ops` in order. Panics on an op that does not address an existing node
/// (a `diff` bug), which tests rely on.
pub fn apply(tree: &mut VNode, ops: &[PatchOp]) {
    for op in ops {
        apply_one(tree, op);
    }
}

fn node_at<'a>(tree: &'a mut VNode, path: &[usize]) -> &'a mut VNode {
    let mut cur = tree;
    for &i in path {
        match cur {
            VNode::Element { children, .. } => cur = &mut children[i],
            _ => panic!("path descends into a leaf"),
        }
    }
    cur
}

fn children_of<'a>(tree: &'a mut VNode, path: &[usize]) -> &'a mut Vec<VNode> {
    match node_at(tree, path) {
        VNode::Element { children, .. } => children,
        _ => panic!("parent is not an element"),
    }
}

fn apply_one(tree: &mut VNode, op: &PatchOp) {
    match op {
        PatchOp::Replace { path, node } => *node_at(tree, path) = node.clone(),
        PatchOp::Insert { parent, index, node } => children_of(tree, parent).insert(*index, node.clone()),
        PatchOp::Remove { path } => {
            let (last, parent) = path.split_last().expect("cannot remove root");
            children_of(tree, parent).remove(*last);
        }
        PatchOp::Move { parent, from, to } => {
            let kids = children_of(tree, parent);
            let n = kids.remove(*from);
            kids.insert(*to, n);
        }
        PatchOp::SetAttr { path, name, value } => {
            if let VNode::Element { attrs, .. } = node_at(tree, path) {
                attrs.retain(|(k, v)| !(k == name && !matches!(v, VAttr::Event(_))));
                attrs.push((name.clone(), VAttr::Static(value.clone())));
            }
        }
        PatchOp::RemoveAttr { path, name } => {
            if let VNode::Element { attrs, .. } = node_at(tree, path) {
                attrs.retain(|(k, v)| !(k == name && !matches!(v, VAttr::Event(_))));
            }
        }
        PatchOp::SetText { path, text } => *node_at(tree, path) = VNode::Text(text.clone()),
    }
}

/// Structural form for comparisons: attrs sorted, events dropped.
#[derive(Debug, PartialEq, Clone)]
pub enum Canon {
    El { tag: String, attrs: Vec<(String, String)>, kids: Vec<Canon> },
    Text(String),
    Raw(String),
}

pub fn canon(v: &VNode) -> Canon {
    match v {
        VNode::Text(t) => Canon::Text(t.clone()),
        VNode::RawHtml(h) => Canon::Raw(h.clone()),
        VNode::Element { tag, attrs, children } => {
            let mut a: Vec<(String, String)> = super::diff::plain_attrs(attrs)
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            a.sort();
            Canon::El { tag: tag.clone(), attrs: a, kids: children.iter().map(canon).collect() }
        }
    }
}
