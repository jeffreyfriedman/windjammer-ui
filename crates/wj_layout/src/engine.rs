use std::collections::{HashMap, HashSet};

use taffy::prelude::*;
use taffy::compute_leaf_layout;
use wj_scene::{Dirty, DirtyEntry, NodeId, Rect as SRect, Scene};

use crate::convert::to_taffy;
use crate::measure::TextMeasure;

#[derive(Clone)]
struct TextCtx {
    content: String,
    font_size: f32,
}

/// Owns a Taffy tree mirroring the scene and updates it incrementally from the dirty log.
pub struct LayoutEngine {
    tree: TaffyTree<TextCtx>,
    to_taffy: HashMap<NodeId, taffy::NodeId>,
    to_scene: HashMap<taffy::NodeId, NodeId>,
    synced_children: HashMap<NodeId, Vec<NodeId>>,
    primed: bool,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        let mut tree = TaffyTree::new();
        tree.disable_rounding();
        LayoutEngine { tree, to_taffy: HashMap::new(), to_scene: HashMap::new(), synced_children: HashMap::new(), primed: false }
    }

    /// Drop every cached layout result so the next `update` recomputes from scratch.
    pub fn invalidate_all(&mut self) {
        let ids: Vec<taffy::NodeId> = self.to_scene.keys().copied().collect();
        for t in ids {
            let _ = self.tree.mark_dirty(t);
        }
    }

    /// Number of mirrored nodes (for leak checks).
    pub fn mirrored(&self) -> usize {
        self.to_taffy.len()
    }

    /// Update layout for the scene. `dirty` is the entries from `Scene::drain_dirty()` for this frame
    /// (entries without the `LAYOUT` flag are ignored); the first call syncs the whole tree. Writes
    /// changed rectangles back with `Scene::set_rect`. Returns the number of nodes whose rect changed.
    pub fn update(&mut self, scene: Scene, dirty: &[DirtyEntry], viewport: (f32, f32), measure: &dyn TextMeasure) -> usize {
        let root = scene.root();
        let mut touched: HashSet<NodeId> = HashSet::new();
        if !self.primed {
            self.sync_subtree(scene, root, &mut touched);
            self.primed = true;
        } else {
            let mut any = false;
            for e in dirty.iter().filter(|e| e.flags.contains(Dirty::LAYOUT)) {
                any = true;
                if scene.is_alive(e.node) {
                    self.sync_node(scene, e.node, &mut touched);
                }
            }
            let viewport_changed = {
                let tr = self.to_taffy[&root];
                let st = self.tree.style(tr).unwrap();
                st.size.width != length(viewport.0) || st.size.height != length(viewport.1)
            };
            if !any && !viewport_changed {
                return 0;
            }
        }
        // The root is the viewport.
        let tr = self.to_taffy[&root];
        let mut rs = self.tree.style(tr).unwrap().clone();
        let want = Size { width: length(viewport.0), height: length(viewport.1) };
        if rs.size != want {
            rs.size = want;
            self.tree.set_style(tr, rs).unwrap();
            touched.insert(root);
        }
        self.tree
            .compute_layout_with_measure(tr, Size { width: AvailableSpace::Definite(viewport.0), height: AvailableSpace::Definite(viewport.1) }, |input, _id, ctx, style| {
                compute_leaf_layout(input, style, |_, _| 0.0, |known, avail| {
                    let Some(ctx) = ctx else { return Size { width: known.width.unwrap_or(0.0), height: known.height.unwrap_or(0.0) } };
                    let max_w = known.width.or(match avail.width {
                        AvailableSpace::Definite(w) => Some(w),
                        AvailableSpace::MaxContent => None,
                        AvailableSpace::MinContent => Some(0.0),
                    });
                    let (w, h) = measure.measure(&ctx.content, ctx.font_size, max_w);
                    Size { width: known.width.unwrap_or(w), height: known.height.unwrap_or(h) }
                })
            })
            .unwrap();
        // Taffy exposes no per-node "was recomputed" signal, and a node can lay out its children
        // differently without its own size changing (percentages, flex resolution), so every node
        // is compared. `Scene::set_rect` only raises dirty flags where the rect really changed.
        let mut changed = 0;
        self.write_back(scene, root, &mut changed);
        changed
    }

    fn write_back(&self, scene: Scene, node: NodeId, changed: &mut usize) {
        let l = self.tree.layout(self.to_taffy[&node]).unwrap();
        // A stretched child whose margins exceed the container's content box can come back slightly
        // negative from Taffy; CSS clamps used sizes at zero, so the adapter does too.
        let r = SRect::new(l.location.x, l.location.y, l.size.width.max(0.0), l.size.height.max(0.0));
        if r != scene.rect(node) {
            scene.set_rect(node, r);
            *changed += 1;
        }
        for c in scene.children(node) {
            self.write_back(scene, c, changed);
        }
    }

    fn sync_subtree(&mut self, scene: Scene, node: NodeId, touched: &mut HashSet<NodeId>) {
        self.sync_node(scene, node, touched);
        for c in scene.children(node) {
            self.sync_subtree(scene, c, touched);
        }
    }

    fn ensure(&mut self, scene: Scene, node: NodeId) -> taffy::NodeId {
        if let Some(t) = self.to_taffy.get(&node) {
            return *t;
        }
        let style = to_taffy(&scene.style(node));
        let t = match scene.text(node) {
            Some(tp) => self.tree.new_leaf_with_context(style, TextCtx { content: tp.content, font_size: tp.font_size }).unwrap(),
            None => self.tree.new_leaf(style).unwrap(),
        };
        self.to_taffy.insert(node, t);
        self.to_scene.insert(t, node);
        t
    }

    fn sync_node(&mut self, scene: Scene, node: NodeId, touched: &mut HashSet<NodeId>) {
        let t = self.ensure(scene, node);
        touched.insert(node);
        let style = to_taffy(&scene.style(node));
        if self.tree.style(t).unwrap() != &style {
            self.tree.set_style(t, style).unwrap();
        }
        match scene.text(node) {
            Some(tp) => {
                let ctx = TextCtx { content: tp.content, font_size: tp.font_size };
                let same = self.tree.get_node_context(t).map_or(false, |c| c.content == ctx.content && c.font_size == ctx.font_size);
                if !same {
                    self.tree.set_node_context(t, Some(ctx)).unwrap();
                    self.tree.mark_dirty(t).unwrap();
                }
            }
            None => {
                if self.tree.get_node_context(t).is_some() {
                    self.tree.set_node_context(t, None).unwrap();
                    self.tree.mark_dirty(t).unwrap();
                }
            }
        }
        let kids = scene.children(node);
        if self.synced_children.get(&node) != Some(&kids) {
            let old = self.synced_children.get(&node).cloned().unwrap_or_default();
            let keep: HashSet<NodeId> = kids.iter().copied().collect();
            let tk: Vec<taffy::NodeId> = kids.iter().map(|k| self.ensure(scene, *k)).collect();
            self.tree.set_children(t, &tk).unwrap();
            for o in old {
                if !keep.contains(&o) {
                    self.drop_subtree(o);
                }
            }
            self.synced_children.insert(node, kids);
        }
    }

    fn drop_subtree(&mut self, node: NodeId) {
        let Some(t) = self.to_taffy.remove(&node) else { return };
        self.to_scene.remove(&t);
        for c in self.synced_children.remove(&node).unwrap_or_default() {
            self.drop_subtree(c);
        }
        let _ = self.tree.remove(t);
    }
}
