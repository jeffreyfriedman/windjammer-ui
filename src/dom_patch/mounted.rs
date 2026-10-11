//! A mounted tree: owns the current VNode, patches the DOM on update, and routes
//! delegated events to handlers. Handlers are looked up by (path, event) in a table
//! rebuilt from the current tree after every update, so patches never lose them.

use super::diff::diff;
use super::port::{apply_ops, collect_event_names, create_tree, path_of, DomPort};
use super::Path;
use crate::simple_vnode::{VAttr, VNode};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

pub type Handler = Rc<RefCell<dyn FnMut()>>;

pub struct Mounted<P: DomPort> {
    pub port: P,
    current: Option<VNode>,
    handlers: HashMap<(Path, String), Handler>,
    listened: BTreeSet<String>,
}

impl<P: DomPort> Mounted<P> {
    pub fn new(port: P) -> Self {
        Mounted { port, current: None, handlers: HashMap::new(), listened: BTreeSet::new() }
    }

    /// First render builds the tree; later renders diff and patch in place.
    pub fn update(&mut self, vnode: &VNode) {
        let next = vnode.sanitized();
        match self.current.take() {
            None => {
                let root = create_tree(&mut self.port, &next);
                let container = self.port.container();
                self.port.insert_child(&container, 0, &root);
            }
            Some(prev) => apply_ops(&mut self.port, &diff(&prev, &next)),
        }
        self.refresh_handlers(&next);
        self.current = Some(next);
    }

    fn refresh_handlers(&mut self, tree: &VNode) {
        self.handlers.clear();
        collect_handlers(tree, &mut Vec::new(), &mut self.handlers);
        let mut names = BTreeSet::new();
        collect_event_names(tree, &mut names);
        for name in names.difference(&self.listened.clone()) {
            self.port.listen(name);
        }
        self.listened.extend(names);
    }

    /// Handlers for `event` on `target` and its ancestors, innermost first.
    pub fn handlers_for(&self, target: &P::Node, event: &str) -> Vec<Handler> {
        let Some(mut path) = path_of(&self.port, target) else { return vec![] };
        let mut found = Vec::new();
        loop {
            if let Some(h) = self.handlers.get(&(path.clone(), event.to_string())) {
                found.push(h.clone());
            }
            if path.pop().is_none() {
                return found;
            }
        }
    }

    /// Deliver a DOM event as the container's capture listener would.
    pub fn dispatch(&self, target: &P::Node, event: &str) -> usize {
        let hs = self.handlers_for(target, event);
        for h in &hs {
            (h.borrow_mut())();
        }
        hs.len()
    }
}

fn collect_handlers(v: &VNode, path: &mut Path, out: &mut HashMap<(Path, String), Handler>) {
    if let VNode::Element { attrs, children, .. } = v {
        for (k, a) in attrs {
            if let (VAttr::Event(h), Some(ev)) = (a, k.strip_prefix("on_")) {
                out.insert((path.clone(), ev.to_string()), h.clone());
            }
        }
        for (i, c) in children.iter().enumerate() {
            path.push(i);
            collect_handlers(c, path, out);
            path.pop();
        }
    }
}
