//! Hit testing, event dispatch, and the semantic tree. All derived from the node tree.
use wj_reactive::batch;

use crate::event::{Event, EventKind};
use crate::geom::{Point, Rect};
use crate::semantics::{Action, Role, SemanticNode, SemanticTree};
use crate::tree::{NodeId, Scene};

impl Scene {
    /// Rect of `id` in scene coordinates (parent origins and scroll offsets applied).
    pub fn abs_rect(&self, id: NodeId) -> Rect {
        let r = self.rect(id);
        let (mut x, mut y) = (r.x, r.y);
        let mut cur = self.parent(id);
        while let Some(p) = cur {
            let pr = self.rect(p);
            let (sx, sy) = self.scroll(p);
            x += pr.x - sx;
            y += pr.y - sy;
            cur = self.parent(p);
        }
        Rect { x, y, ..r }
    }

    /// Deepest node under the point; later siblings are on top. Clipping nodes hide children outside them.
    pub fn hit_test(&self, x: f32, y: f32) -> Option<NodeId> {
        self.hit(self.root(), Point { x, y }, 0.0, 0.0)
    }

    fn hit(&self, id: NodeId, p: Point, ox: f32, oy: f32) -> Option<NodeId> {
        let r = self.rect(id);
        let abs = Rect { x: r.x + ox, y: r.y + oy, ..r };
        let clip = self.paint(id).clip || self.style(id).scroll;
        if clip && !abs.contains(p) {
            return None;
        }
        let (sx, sy) = self.scroll(id);
        for c in self.children(id).into_iter().rev() {
            if let Some(h) = self.hit(c, p, abs.x - sx, abs.y - sy) {
                return Some(h);
            }
        }
        if abs.contains(p) {
            Some(id)
        } else {
            None
        }
    }

    /// Dispatch an input event. Pointer events go to the hit node and bubble to the root; keys go
    /// to the focused node (or root). Handlers run in one batch. Returns whether any handler ran.
    pub fn dispatch(&self, event: Event) -> bool {
        let target = match &event {
            Event::PointerDown { x, y } | Event::PointerUp { x, y } | Event::PointerMove { x, y } | Event::Click { x, y } | Event::Wheel { x, y, .. } => self.hit_test(*x, *y),
            Event::Key { .. } => Some(self.focus().filter(|f| self.is_alive(*f)).unwrap_or(self.root())),
        };
        let mut ran = false;
        if let Some(t) = target {
            ran |= self.bubble(t, &event);
        }
        match &event {
            Event::PointerDown { .. } => self.with(|d| d.pressed = target),
            Event::PointerUp { x, y } => {
                let pressed = self.with(|d| d.pressed.take());
                if target.is_some() && pressed == target {
                    ran |= self.dispatch(Event::Click { x: *x, y: *y });
                }
            }
            _ => {}
        }
        ran
    }

    /// Run the handlers registered directly on `node` for `event` (no hit test, no bubbling): how
    /// assistive technology triggers a control. Returns whether a handler ran.
    pub fn invoke(&self, node: NodeId, event: Event) -> bool {
        if !self.is_alive(node) {
            return false;
        }
        let hs = self.handlers_of(node, event.kind());
        let ran = !hs.is_empty();
        batch(|| {
            for h in hs {
                h(&event);
            }
        });
        ran
    }

    fn bubble(&self, from: NodeId, event: &Event) -> bool {
        let kind = event.kind();
        let mut hs = vec![];
        let mut cur = Some(from);
        while let Some(n) = cur {
            hs.extend(self.handlers_of(n, kind));
            cur = self.parent(n);
        }
        let ran = !hs.is_empty();
        batch(|| {
            for h in hs {
                h(event);
            }
        });
        ran
    }

    /// Accessible name: explicit name, else own text, else concatenated descendant text.
    pub fn accessible_name(&self, id: NodeId) -> Option<String> {
        if let Some(n) = self.semantics(id).name {
            return Some(n);
        }
        let mut parts = vec![];
        self.collect_text(id, &mut parts);
        if parts.is_empty() { None } else { Some(parts.join(" ")) }
    }

    fn collect_text(&self, id: NodeId, out: &mut Vec<String>) {
        if let Some(t) = self.text(id) {
            if !t.content.is_empty() {
                out.push(t.content);
            }
        }
        for c in self.children(id) {
            self.collect_text(c, out);
        }
    }

    /// Build the semantic tree. Text nodes without explicit semantics become `Role::Text`.
    /// Nodes with no role are transparent (children are hoisted).
    pub fn semantic_tree(&self) -> SemanticTree {
        SemanticTree { roots: self.sem_nodes(self.root()) }
    }

    fn sem_nodes(&self, id: NodeId) -> Vec<SemanticNode> {
        let sem = self.semantics(id);
        let has_text = self.text(id).is_some();
        let role = if sem.role == Role::None && has_text { Role::Text } else { sem.role };
        let kids: Vec<SemanticNode> = self.children(id).into_iter().flat_map(|c| self.sem_nodes(c)).collect();
        if role == Role::None {
            return kids;
        }
        let mut actions = vec![];
        if self.has_handler(id, EventKind::Click) {
            actions.push(Action::Click);
        }
        if sem.focusable {
            actions.push(Action::Focus);
        }
        if role == Role::TextField {
            actions.push(Action::SetValue);
        }
        // Leaf roles that take their name from text must not repeat the text as a child.
        let (name, children) = match role {
            Role::Text => (sem.name.or_else(|| self.text(id).map(|t| t.content)), vec![]),
            Role::Button | Role::Checkbox | Role::Heading => (self.accessible_name(id), kids.into_iter().filter(|k| k.role != Role::Text).collect()),
            _ => (sem.name, kids),
        };
        vec![SemanticNode {
            id,
            role,
            name,
            value: sem.value,
            checked: sem.checked,
            disabled: sem.disabled,
            focusable: sem.focusable,
            actions,
            bounds: self.abs_rect(id),
            children,
        }]
    }

    /// Lint: interactive nodes must declare semantics (ADR-0004). Returns one message per violation.
    pub fn audit_semantics(&self) -> Vec<String> {
        let mut out = vec![];
        self.audit(self.root(), &mut out);
        out
    }

    fn audit(&self, id: NodeId, out: &mut Vec<String>) {
        let sem = self.semantics(id);
        let interactive = self.has_handler(id, EventKind::Click) || sem.focusable;
        if interactive && sem.role == Role::None {
            out.push(format!("{id:?}: interactive node has no role"));
        }
        let needs_name = matches!(sem.role, Role::Button | Role::Checkbox | Role::TextField | Role::Image);
        if needs_name && self.accessible_name(id).map_or(true, |n| n.trim().is_empty()) {
            out.push(format!("{id:?}: {:?} has no accessible name", sem.role));
        }
        for c in self.children(id) {
            self.audit(c, out);
        }
    }
}
