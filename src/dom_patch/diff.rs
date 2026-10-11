//! Diff two sanitized VNode trees into a sequence of patch operations.
//!
//! Ops are *sequential*: each op's paths are valid against the tree as left by the
//! previous ops. Event attributes are not diffed (handlers live in `Mounted`'s table).

use super::Path;
use crate::simple_vnode::{VAttr, VNode};
use std::collections::HashMap;

#[derive(Clone)]
pub enum PatchOp {
    /// Replace the node at `path` (a root replace uses `[]`).
    Replace { path: Path, node: VNode },
    /// Insert `node` as child `index` of `parent`.
    Insert { parent: Path, index: usize, node: VNode },
    /// Remove the node at `path`.
    Remove { path: Path },
    /// Move child `from` of `parent` so it ends up at index `to`.
    Move { parent: Path, from: usize, to: usize },
    SetAttr { path: Path, name: String, value: String },
    RemoveAttr { path: Path, name: String },
    SetText { path: Path, text: String },
}

/// Diff `old` into `new`. Both should already be `sanitized()`.
pub fn diff(old: &VNode, new: &VNode) -> Vec<PatchOp> {
    let mut ops = Vec::new();
    diff_node(old, new, &mut Vec::new(), &mut ops);
    ops
}

fn diff_node(old: &VNode, new: &VNode, path: &mut Path, ops: &mut Vec<PatchOp>) {
    match (old, new) {
        (VNode::Text(a), VNode::Text(b)) => {
            if a != b {
                ops.push(PatchOp::SetText { path: path.clone(), text: b.clone() });
            }
        }
        (VNode::RawHtml(a), VNode::RawHtml(b)) => {
            if a != b {
                ops.push(PatchOp::Replace { path: path.clone(), node: new.clone() });
            }
        }
        (
            VNode::Element { tag: ta, attrs: aa, children: ca },
            VNode::Element { tag: tb, attrs: ab, children: cb },
        ) if ta == tb => {
            diff_attrs(aa, ab, path, ops);
            diff_children(ca, cb, path, ops);
        }
        _ => ops.push(PatchOp::Replace { path: path.clone(), node: new.clone() }),
    }
}

/// Plain (non-event) attributes as a name -> value map, last write wins.
pub(crate) fn plain_attrs(attrs: &[(String, VAttr)]) -> HashMap<&str, &str> {
    attrs
        .iter()
        .filter_map(|(k, v)| match v {
            VAttr::Static(s) | VAttr::Dynamic(s) => Some((k.as_str(), s.as_str())),
            VAttr::Event(_) => None,
        })
        .collect()
}

fn diff_attrs(old: &[(String, VAttr)], new: &[(String, VAttr)], path: &Path, ops: &mut Vec<PatchOp>) {
    let (old, new) = (plain_attrs(old), plain_attrs(new));
    let mut removed: Vec<&&str> = old.keys().filter(|k| !new.contains_key(*k)).collect();
    removed.sort();
    for name in removed {
        ops.push(PatchOp::RemoveAttr { path: path.clone(), name: (*name).to_string() });
    }
    let mut changed: Vec<(&&str, &&str)> = new.iter().filter(|(k, v)| old.get(*k) != Some(*v)).collect();
    changed.sort();
    for (name, value) in changed {
        ops.push(PatchOp::SetAttr { path: path.clone(), name: (*name).to_string(), value: (*value).to_string() });
    }
}

/// Identity of a child for matching: its key, or its ordinal among unkeyed siblings.
#[derive(Hash, PartialEq, Eq, Clone)]
enum Ident {
    Key(String),
    Pos(usize),
}

fn identities(children: &[VNode]) -> Vec<Ident> {
    let mut unkeyed = 0;
    children
        .iter()
        .map(|c| match c.key() {
            Some(k) => Ident::Key(k.to_string()),
            None => {
                unkeyed += 1;
                Ident::Pos(unkeyed - 1)
            }
        })
        .collect()
}

/// For each new child, the index of the matching old child (if any, first claim wins).
fn match_children(old: &[VNode], new: &[VNode]) -> Vec<Option<usize>> {
    let mut by_ident: HashMap<Ident, usize> = HashMap::new();
    for (i, id) in identities(old).into_iter().enumerate() {
        by_ident.entry(id).or_insert(i);
    }
    let mut claimed = vec![false; old.len()];
    identities(new)
        .into_iter()
        .map(|id| {
            let m = by_ident.get(&id).copied().filter(|&i| !claimed[i])?;
            claimed[m] = true;
            Some(m)
        })
        .collect()
}

fn diff_children(old: &[VNode], new: &[VNode], path: &mut Path, ops: &mut Vec<PatchOp>) {
    let matches = match_children(old, new);
    // `order[p]` = old index of the node currently at position p.
    let mut order = remove_unmatched(old.len(), &matches, path, ops);
    for (i, m) in matches.iter().enumerate() {
        match m {
            None => ops.push(PatchOp::Insert { parent: path.clone(), index: i, node: new[i].clone() }),
            Some(old_idx) => {
                let from = order.iter().position(|o| o == old_idx).expect("matched node present");
                if from != i {
                    ops.push(PatchOp::Move { parent: path.clone(), from, to: i });
                    let moved = order.remove(from);
                    order.insert(i, moved);
                }
                path.push(i);
                diff_node(&old[*old_idx], &new[i], path, ops);
                path.pop();
            }
        }
        if m.is_none() {
            order.insert(i, usize::MAX);
        }
    }
}

/// Emit removals (highest index first) for old children nobody matched; return survivors.
fn remove_unmatched(old_len: usize, matches: &[Option<usize>], path: &Path, ops: &mut Vec<PatchOp>) -> Vec<usize> {
    let kept: std::collections::HashSet<usize> = matches.iter().flatten().copied().collect();
    for i in (0..old_len).rev().filter(|i| !kept.contains(i)) {
        let mut p = path.clone();
        p.push(i);
        ops.push(PatchOp::Remove { path: p });
    }
    (0..old_len).filter(|i| kept.contains(i)).collect()
}
