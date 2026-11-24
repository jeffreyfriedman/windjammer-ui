//! Scopes own the nodes created while they are current and dispose them together.
use std::collections::HashSet;

use crate::runtime::{free_node, with_rt};

pub(crate) struct ScopeSlot {
    gen: u32,
    alive: bool,
    parent: u32,
    nodes: Vec<(u32, u32)>,
    children: HashSet<u32>,
}

pub(crate) struct ScopeArena {
    slots: Vec<ScopeSlot>,
    free: Vec<u32>,
    stack: Vec<u32>,
}

impl ScopeArena {
    pub(crate) fn new() -> Self {
        let root = ScopeSlot { gen: 0, alive: true, parent: 0, nodes: Vec::new(), children: HashSet::new() };
        ScopeArena { slots: vec![root], free: Vec::new(), stack: vec![0] }
    }
    pub(crate) fn current(&self) -> u32 {
        *self.stack.last().unwrap()
    }
    pub(crate) fn add_node(&mut self, scope: u32, node: u32, gen: u32) {
        self.slots[scope as usize].nodes.push((node, gen));
    }
    pub(crate) fn live(&self) -> usize {
        self.slots.iter().filter(|s| s.alive).count()
    }
    fn create(&mut self, parent: u32) -> (u32, u32) {
        let fresh = ScopeSlot { gen: 0, alive: true, parent, nodes: Vec::new(), children: HashSet::new() };
        let (idx, gen) = if let Some(i) = self.free.pop() {
            let s = &mut self.slots[i as usize];
            let gen = s.gen;
            *s = ScopeSlot { gen, ..fresh };
            (i, gen)
        } else {
            self.slots.push(fresh);
            ((self.slots.len() - 1) as u32, 0)
        };
        self.slots[parent as usize].children.insert(idx);
        (idx, gen)
    }
}

/// A disposal scope. `Copy`, like every handle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Scope {
    idx: u32,
    gen: u32,
}

/// Run `f` with a fresh child scope current; everything created inside is owned by it.
pub fn create_scope<R>(f: impl FnOnce() -> R) -> (R, Scope) {
    let (idx, gen) = with_rt(|r| {
        let parent = r.scopes.current();
        let (i, g) = r.scopes.create(parent);
        r.scopes.stack.push(i);
        (i, g)
    });
    let out = f();
    with_rt(|r| {
        r.scopes.stack.pop();
    });
    (out, Scope { idx, gen })
}

impl Scope {
    pub fn is_alive(&self) -> bool {
        with_rt(|r| r.scopes.slots.get(self.idx as usize).map_or(false, |s| s.alive && s.gen == self.gen))
    }

    /// Re-enter this scope: nodes created inside `f` are owned by it. Panics if disposed.
    pub fn run<R>(self, f: impl FnOnce() -> R) -> R {
        assert!(self.is_alive(), "Scope::run on a disposed scope");
        with_rt(|r| r.scopes.stack.push(self.idx));
        let out = f();
        with_rt(|r| {
            r.scopes.stack.pop();
        });
        out
    }

    /// Dispose every node and child scope created inside this scope. Idempotent.
    pub fn dispose(self) {
        if !self.is_alive() || self.idx == 0 {
            return;
        }
        with_rt(|r| dispose_scope(r, self.idx));
    }
}

fn dispose_scope(r: &mut crate::runtime::Rt, idx: u32) {
    let (nodes, children, parent) = {
        let s = &mut r.scopes.slots[idx as usize];
        (std::mem::take(&mut s.nodes), std::mem::take(&mut s.children), s.parent)
    };
    for c in children {
        if r.scopes.slots[c as usize].alive {
            dispose_scope(r, c);
        }
    }
    for (n, gen) in nodes {
        if r.alive(n, gen) {
            free_node(r, n);
        }
    }
    let s = &mut r.scopes.slots[idx as usize];
    s.alive = false;
    s.gen = s.gen.wrapping_add(1);
    r.scopes.free.push(idx);
    if r.scopes.slots[parent as usize].alive {
        r.scopes.slots[parent as usize].children.remove(&idx);
    }
}
