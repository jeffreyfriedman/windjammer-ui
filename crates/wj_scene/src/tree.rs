//! The node arena. Every access goes through short `RefCell` borrows; user closures (handlers,
//! binding bodies) are never run while a borrow is held.
use std::cell::RefCell;

use wj_reactive::Scope;

use crate::dirty::{Dirty, DirtyEntry};
use crate::event::{EventKind, Handler};
use crate::geom::Rect;
use crate::semantics::Semantics;
use crate::style::{Paint, Style, TextProps};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId {
    pub(crate) idx: u32,
    pub(crate) gen: u32,
}

impl NodeId {
    /// `(index, generation)`, for adapters that need a stable numeric identity.
    pub fn parts(&self) -> (u32, u32) {
        (self.idx, self.gen)
    }
}

pub(crate) struct NodeData {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub style: Style,
    pub paint: Paint,
    pub text: Option<TextProps>,
    pub sem: Semantics,
    pub handlers: Vec<(EventKind, Handler)>,
    /// Layout result, relative to the parent's content origin.
    pub rect: Rect,
    pub scroll: (f32, f32),
    pub dirty: Dirty,
    pub scope: Scope,
}

struct Slot {
    gen: u32,
    node: Option<NodeData>,
}

pub(crate) struct SceneData {
    slots: Vec<Slot>,
    free: Vec<u32>,
    pub root: NodeId,
    pub log: Vec<DirtyEntry>,
    pub pressed: Option<NodeId>,
    pub focus: Option<NodeId>,
    live: usize,
}

thread_local! {
    static SCENES: RefCell<Vec<Option<SceneData>>> = const { RefCell::new(Vec::new()) };
}

/// A `Copy` handle to a scene arena owned by the current thread.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Scene(pub(crate) u32);

impl Scene {
    pub fn new() -> Scene {
        let (_, scope) = wj_reactive::create_scope(|| {});
        let id = SCENES.with(|s| {
            let mut s = s.borrow_mut();
            s.push(Some(SceneData {
                slots: Vec::new(),
                free: Vec::new(),
                root: NodeId { idx: 0, gen: 0 },
                log: Vec::new(),
                pressed: None,
                focus: None,
                live: 0,
            }));
            (s.len() - 1) as u32
        });
        let scene = Scene(id);
        let root = scene.insert(None, scope);
        scene.with(|d| d.root = root);
        scene
    }

    pub fn root(&self) -> NodeId {
        self.with(|d| d.root)
    }

    pub(crate) fn with<R>(&self, f: impl FnOnce(&mut SceneData) -> R) -> R {
        SCENES.with(|s| f(s.borrow_mut()[self.0 as usize].as_mut().expect("scene was dropped")))
    }

    pub(crate) fn node<R>(&self, id: NodeId, f: impl FnOnce(&mut NodeData) -> R) -> R {
        self.with(|d| {
            let slot = &mut d.slots[id.idx as usize];
            assert!(slot.gen == id.gen && slot.node.is_some(), "scene node used after removal");
            f(slot.node.as_mut().unwrap())
        })
    }

    pub fn is_alive(&self, id: NodeId) -> bool {
        self.with(|d| d.slots.get(id.idx as usize).map_or(false, |s| s.gen == id.gen && s.node.is_some()))
    }

    pub fn node_count(&self) -> usize {
        self.with(|d| d.live)
    }

    /// Slots ever allocated (for memory gates).
    pub fn slot_count(&self) -> usize {
        self.with(|d| d.slots.len())
    }

    fn insert(&self, parent: Option<NodeId>, scope: Scope) -> NodeId {
        let data = NodeData {
            parent,
            children: Vec::new(),
            style: Style::default(),
            paint: Paint::default(),
            text: None,
            sem: Semantics::default(),
            handlers: Vec::new(),
            rect: Rect::default(),
            scroll: (0.0, 0.0),
            dirty: Dirty::NONE,
            scope,
        };
        self.with(|d| {
            d.live += 1;
            if let Some(i) = d.free.pop() {
                let s = &mut d.slots[i as usize];
                s.node = Some(data);
                NodeId { idx: i, gen: s.gen }
            } else {
                d.slots.push(Slot { gen: 0, node: Some(data) });
                NodeId { idx: (d.slots.len() - 1) as u32, gen: 0 }
            }
        })
    }

    /// Create a node that is not yet in the tree (attach it with `append` or `set_children`).
    pub fn create_detached(&self) -> NodeId {
        let (_, scope) = wj_reactive::create_scope(|| {});
        let id = self.insert(None, scope);
        self.raise(id, Dirty::ALL);
        id
    }

    /// Create a node as the last child of `parent`.
    pub fn create(&self, parent: NodeId) -> NodeId {
        let id = self.create_detached();
        self.append(parent, id);
        id
    }

    pub fn append(&self, parent: NodeId, child: NodeId) {
        self.node(child, |n| n.parent = Some(parent));
        self.node(parent, |n| n.children.push(child));
        self.raise(parent, Dirty::LAYOUT | Dirty::PAINT | Dirty::SEMANTICS);
    }

    /// Replace the child list (reorders, adds and detaches in one step). Nodes that leave the list
    /// are detached, not removed; call `remove` for those.
    pub fn set_children(&self, parent: NodeId, kids: Vec<NodeId>) {
        let same = self.node(parent, |n| n.children == kids);
        if same {
            return;
        }
        let old = self.node(parent, |n| std::mem::replace(&mut n.children, kids.clone()));
        let keep: std::collections::HashSet<NodeId> = kids.iter().copied().collect();
        for o in old {
            if self.is_alive(o) && !keep.contains(&o) {
                self.node(o, |n| n.parent = None);
            }
        }
        for k in &kids {
            self.node(*k, |n| n.parent = Some(parent));
        }
        self.raise(parent, Dirty::LAYOUT | Dirty::PAINT | Dirty::SEMANTICS);
    }

    /// Remove a subtree: unlink from its parent, dispose every node scope (stops all bindings), free slots.
    pub fn remove(&self, id: NodeId) {
        if !self.is_alive(id) {
            return;
        }
        if let Some(p) = self.node(id, |n| n.parent) {
            if self.is_alive(p) {
                self.node(p, |n| n.children.retain(|c| *c != id));
                self.raise(p, Dirty::LAYOUT | Dirty::PAINT | Dirty::SEMANTICS);
            }
        }
        self.free_subtree(id);
    }

    fn free_subtree(&self, id: NodeId) {
        let (kids, scope) = self.node(id, |n| (std::mem::take(&mut n.children), n.scope));
        for k in kids {
            if self.is_alive(k) {
                self.free_subtree(k);
            }
        }
        scope.dispose();
        self.with(|d| {
            let s = &mut d.slots[id.idx as usize];
            s.node = None;
            s.gen = s.gen.wrapping_add(1);
            d.free.push(id.idx);
            d.live -= 1;
            if d.focus == Some(id) {
                d.focus = None;
            }
            if d.pressed == Some(id) {
                d.pressed = None;
            }
        });
    }

    pub(crate) fn scope_of(&self, id: NodeId) -> Scope {
        self.node(id, |n| n.scope)
    }

    // ---- dirty tracking ----

    pub(crate) fn raise(&self, id: NodeId, flags: Dirty) {
        self.with(|d| {
            let n = d.slots[id.idx as usize].node.as_mut().expect("node");
            let new = Dirty::from_bits(flags.bits() & !n.dirty.bits());
            if !new.is_empty() {
                n.dirty |= new;
                d.log.push(DirtyEntry { node: id, flags: new });
            }
        });
    }

    /// Take the dirty log. Nodes removed since being logged are filtered out. Clears node flags.
    pub fn drain_dirty(&self) -> Vec<DirtyEntry> {
        let log = self.with(|d| std::mem::take(&mut d.log));
        let mut out: Vec<DirtyEntry> = Vec::with_capacity(log.len());
        for e in log {
            if self.is_alive(e.node) {
                self.node(e.node, |n| n.dirty = Dirty::NONE);
                out.push(e);
            }
        }
        out
    }

    /// Union of all flags currently raised (without draining).
    pub fn pending(&self) -> Dirty {
        self.with(|d| d.log.iter().fold(Dirty::NONE, |a, e| a | e.flags))
    }

    // ---- prop access ----

    pub fn children(&self, id: NodeId) -> Vec<NodeId> {
        self.node(id, |n| n.children.clone())
    }
    pub fn child_count(&self, id: NodeId) -> usize {
        self.node(id, |n| n.children.len())
    }
    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.node(id, |n| n.parent)
    }
    pub fn style(&self, id: NodeId) -> Style {
        self.node(id, |n| n.style.clone())
    }
    pub fn paint(&self, id: NodeId) -> Paint {
        self.node(id, |n| n.paint.clone())
    }
    pub fn text(&self, id: NodeId) -> Option<TextProps> {
        self.node(id, |n| n.text.clone())
    }
    pub fn semantics(&self, id: NodeId) -> Semantics {
        self.node(id, |n| n.sem.clone())
    }
    pub fn rect(&self, id: NodeId) -> Rect {
        self.node(id, |n| n.rect)
    }
    pub fn scroll(&self, id: NodeId) -> (f32, f32) {
        self.node(id, |n| n.scroll)
    }

    pub fn set_style(&self, id: NodeId, style: Style) {
        if self.node(id, |n| if n.style == style { false } else { n.style = style; true }) {
            self.raise(id, Dirty::LAYOUT | Dirty::PAINT);
        }
    }
    pub fn set_paint(&self, id: NodeId, paint: Paint) {
        if self.node(id, |n| if n.paint == paint { false } else { n.paint = paint; true }) {
            self.raise(id, Dirty::PAINT);
        }
    }
    pub fn set_text(&self, id: NodeId, text: TextProps) {
        let (changed, content_changed) = self.node(id, |n| {
            if n.text.as_ref() == Some(&text) {
                return (false, false);
            }
            let cc = n.text.as_ref().map(|t| (&t.content, t.font_size)) != Some((&text.content, text.font_size));
            n.text = Some(text);
            (true, cc)
        });
        if changed {
            // Colour-only changes repaint; content or size changes also re-layout and re-announce.
            let flags = if content_changed { Dirty::ALL } else { Dirty::PAINT };
            self.raise(id, flags);
        }
    }
    pub fn set_semantics(&self, id: NodeId, sem: Semantics) {
        if self.node(id, |n| if n.sem == sem { false } else { n.sem = sem; true }) {
            self.raise(id, Dirty::SEMANTICS);
        }
    }
    pub fn set_scroll(&self, id: NodeId, x: f32, y: f32) {
        if self.node(id, |n| if n.scroll == (x, y) { false } else { n.scroll = (x, y); true }) {
            self.raise(id, Dirty::PAINT | Dirty::SEMANTICS);
        }
    }
    /// Written by the layout engine; repaints on change.
    pub fn set_rect(&self, id: NodeId, rect: Rect) {
        if self.node(id, |n| if n.rect == rect { false } else { n.rect = rect; true }) {
            self.raise(id, Dirty::PAINT | Dirty::SEMANTICS);
        }
    }

    pub fn on(&self, id: NodeId, kind: EventKind, handler: impl Fn(&crate::event::Event) + 'static) {
        self.node(id, |n| n.handlers.push((kind, std::rc::Rc::new(handler))));
        self.raise(id, Dirty::SEMANTICS);
    }
    pub(crate) fn handlers_of(&self, id: NodeId, kind: EventKind) -> Vec<Handler> {
        self.node(id, |n| n.handlers.iter().filter(|(k, _)| *k == kind).map(|(_, h)| h.clone()).collect())
    }
    pub fn has_handler(&self, id: NodeId, kind: EventKind) -> bool {
        self.node(id, |n| n.handlers.iter().any(|(k, _)| *k == kind))
    }

    pub fn set_focus(&self, id: Option<NodeId>) {
        self.with(|d| d.focus = id);
    }
    pub fn focus(&self) -> Option<NodeId> {
        self.with(|d| d.focus)
    }
}

impl Dirty {
    pub(crate) fn from_bits(b: u8) -> Dirty {
        let mut d = Dirty::NONE;
        if b & 1 != 0 {
            d |= Dirty::LAYOUT;
        }
        if b & 2 != 0 {
            d |= Dirty::PAINT;
        }
        if b & 4 != 0 {
            d |= Dirty::SEMANTICS;
        }
        d
    }
}
