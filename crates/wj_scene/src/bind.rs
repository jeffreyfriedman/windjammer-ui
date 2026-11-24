//! Reactive binding sites. A binding is an effect owned by its node's scope: it re-runs when a
//! signal it read changes and writes props through the setters, which raise dirty flags only when
//! the value actually changed. Removing the node disposes the binding.
use std::collections::HashMap;
use std::hash::Hash;

use wj_reactive::{create_scope, effect, untrack, Scope};

use crate::geom::Color;
use crate::style::TextProps;
use crate::tree::{NodeId, Scene};

impl Scene {
    /// Run `f` as an effect owned by `node`. Reads inside `f` track; call prop setters inside it.
    pub fn bind(&self, node: NodeId, f: impl FnMut() + 'static) {
        let scope = self.scope_of(node);
        scope.run(|| effect(f));
    }

    /// Bind a node's text content to a reactive string (keeps size and colour).
    pub fn bind_text(&self, node: NodeId, mut f: impl FnMut() -> String + 'static) {
        let scene = *self;
        self.bind(node, move || {
            let content = f();
            let mut props = scene.text(node).unwrap_or_else(|| TextProps::new(""));
            props.content = content;
            scene.set_text(node, props);
        });
    }

    /// Bind the background colour.
    pub fn bind_background(&self, node: NodeId, mut f: impl FnMut() -> Option<Color> + 'static) {
        let scene = *self;
        self.bind(node, move || {
            let c = f();
            let mut p = scene.paint(node);
            p.background = c;
            scene.set_paint(node, p);
        });
    }

    /// Keyed child list. `keys` is tracked; when it changes, rows for new keys are built with
    /// `build` (untracked, in their own scope), rows for vanished keys are removed with all their
    /// bindings, survivors keep their nodes, and the parent's child order is set to match `keys`.
    /// Keys must be unique.
    pub fn keyed_children<K>(&self, parent: NodeId, mut keys: impl FnMut() -> Vec<K> + 'static, build: impl Fn(K) -> NodeId + 'static)
    where
        K: Hash + Eq + Clone + 'static,
    {
        let scene = *self;
        let mut rows: HashMap<K, (NodeId, Scope)> = HashMap::new();
        self.bind(parent, move || {
            let new_keys = keys();
            untrack(|| {
                let mut next: HashMap<K, (NodeId, Scope)> = HashMap::with_capacity(new_keys.len());
                let mut order = Vec::with_capacity(new_keys.len());
                for k in &new_keys {
                    let entry = match rows.remove(k) {
                        Some(e) => e,
                        None => {
                            let (node, scope) = create_scope(|| build(k.clone()));
                            (node, scope)
                        }
                    };
                    assert!(next.insert(k.clone(), entry).is_none(), "keyed_children: duplicate key");
                    order.push(entry.0);
                }
                // Detach first so removing each row does not scan the parent's child list.
                scene.set_children(parent, order);
                for (_, (node, scope)) in rows.drain() {
                    scene.remove(node);
                    scope.dispose();
                }
                rows = next;
            });
        });
    }

    /// Fixed-row-height virtual list. Only rows in the visible window (plus `overscan`) exist as
    /// nodes. `count`, `scroll` and `viewport` are tracked; `row` builds the content of row `i` and
    /// is positioned absolutely at `i * row_height` inside `parent`, which gets height
    /// `count * row_height` so scroll extents are right.
    pub fn virtual_list(
        &self,
        parent: NodeId,
        row_height: f32,
        overscan: usize,
        mut count: impl FnMut() -> usize + 'static,
        mut scroll: impl FnMut() -> f32 + 'static,
        mut viewport: impl FnMut() -> f32 + 'static,
        row: impl Fn(usize) -> NodeId + 'static,
    ) {
        let scene = *self;
        let mut window = (0usize, 0usize);
        let mut total = usize::MAX;
        let mut live: HashMap<usize, (NodeId, Scope)> = HashMap::new();
        self.bind(parent, move || {
            let n = count();
            let (s, e) = crate::visible_range(scroll(), viewport(), row_height, n, overscan);
            untrack(|| {
                if (s, e) == window && n == total {
                    return;
                }
                window = (s, e);
                total = n;
                let gone: Vec<usize> = live.keys().copied().filter(|i| *i < s || *i >= e).collect();
                for i in gone {
                    let (node, scope) = live.remove(&i).unwrap();
                    scene.remove(node);
                    scope.dispose();
                }
                for i in s..e {
                    if !live.contains_key(&i) {
                        let (node, scope) = create_scope(|| row(i));
                        let mut st = scene.style(node);
                        st.absolute = Some((0.0, i as f32 * row_height));
                        st.height = crate::Dim::Px(row_height);
                        scene.set_style(node, st);
                        live.insert(i, (node, scope));
                    }
                }
                let mut idx: Vec<usize> = live.keys().copied().collect();
                idx.sort_unstable();
                scene.set_children(parent, idx.iter().map(|i| live[i].0).collect());
                let mut st = scene.style(parent);
                st.height = crate::Dim::Px(n as f32 * row_height);
                st.flex_shrink = 0.0; // the extent is exact, never squeezed by the viewport
                scene.set_style(parent, st);
            });
        });
    }
}
