//! A mounted tree: owns the current VNode, patches the DOM on update, and routes
//! delegated events to handlers. Handlers are looked up by (path, event) in a table
//! rebuilt from the current tree after every update, so patches never lose them.

use super::diff::diff;
use super::port::{apply_ops, collect_event_names, create_tree, path_of, DomPort};
use super::Path;
use crate::simple_vnode::{VAttr, VNode};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

pub type Handler = Rc<RefCell<dyn FnMut()>>;

thread_local! {
    /// Flags raised by the handler currently running (handlers take no event argument,
    /// so they signal through these free functions instead).
    static CONTROL: Cell<(bool, bool)> = const { Cell::new((false, false)) };
}

/// Like `Event.stopPropagation()`: ancestor handlers of this event are skipped.
/// Call from inside a handler; no effect elsewhere.
pub fn stop_propagation() {
    CONTROL.with(|c| c.set((true, c.get().1)));
}

/// Like `Event.preventDefault()`: the host (web adapter) cancels the native default.
pub fn prevent_default() {
    CONTROL.with(|c| c.set((c.get().0, true)));
}

/// Result of delivering one event to the handler chain.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DispatchOutcome {
    /// Number of handlers that ran.
    pub handled: usize,
    pub propagation_stopped: bool,
    pub default_prevented: bool,
}

/// Run `handlers` (innermost first) honouring `stop_propagation`. No borrow of the
/// `Mounted` may be held by the caller while this runs user code.
pub fn run_handlers(handlers: &[Handler]) -> DispatchOutcome {
    let saved = CONTROL.with(|c| c.replace((false, false))); // re-entrant dispatch safe
    let mut out = DispatchOutcome::default();
    for h in handlers {
        (h.borrow_mut())();
        out.handled += 1;
        let (stop, prevent) = CONTROL.with(|c| c.get());
        out.propagation_stopped = stop;
        out.default_prevented = prevent;
        if stop {
            break;
        }
    }
    CONTROL.with(|c| c.set(saved));
    out
}

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
    pub fn dispatch_event(&self, target: &P::Node, event: &str) -> DispatchOutcome {
        run_handlers(&self.handlers_for(target, event))
    }

    /// Like [`Self::dispatch_event`], returning only the number of handlers that ran.
    pub fn dispatch(&self, target: &P::Node, event: &str) -> usize {
        self.dispatch_event(target, event).handled
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
