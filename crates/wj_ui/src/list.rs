use std::hash::Hash;

use crate::ctx::with_scene;
use crate::node::{column, Node};

/// A keyed list: one child per key from `keys` (tracked), built once by `build` and kept across
/// reorders and unrelated changes. Keys must be unique.
pub fn each<K: Hash + Eq + Clone + 'static>(keys: impl FnMut() -> Vec<K> + 'static, build: impl Fn(K) -> Node + 'static) -> Node {
    let container = column();
    with_scene(|scene| scene.keyed_children(container.id(), keys, move |k| build(k).id()));
    container
}
