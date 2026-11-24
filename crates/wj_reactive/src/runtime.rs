//! The per-thread arena and the propagation algorithm (clean / check / dirty colouring).
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::handles::{Memo, Signal};
use crate::scope::ScopeArena;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Color {
    Clean,
    Check,
    Dirty,
}

pub(crate) type Value = Rc<dyn Any>;
/// Recompute function: receives the current value slot, returns whether the value changed.
type Compute = Box<dyn FnMut(&mut Option<Value>) -> bool>;

pub(crate) struct Node {
    pub is_effect: bool,
    pub value: Option<Value>,
    pub f: Option<Compute>,
    pub subs: Vec<u32>,
    pub srcs: Vec<u32>,
    pub color: Color,
}

pub(crate) struct Slot {
    pub gen: u32,
    pub node: Option<Node>,
}

pub(crate) struct Rt {
    pub slots: Vec<Slot>,
    pub free: Vec<u32>,
    pub observer: Option<u32>,
    pub queue: Vec<u32>,
    pub batch_depth: u32,
    pub flushing: bool,
    pub scopes: ScopeArena,
}

impl Rt {
    fn new() -> Self {
        Rt {
            slots: Vec::new(),
            free: Vec::new(),
            observer: None,
            queue: Vec::new(),
            batch_depth: 0,
            flushing: false,
            scopes: ScopeArena::new(),
        }
    }
    pub(crate) fn node(&mut self, id: u32) -> &mut Node {
        self.slots[id as usize].node.as_mut().expect("reactive node used after its scope was disposed")
    }
    pub(crate) fn alive(&self, id: u32, gen: u32) -> bool {
        self.slots.get(id as usize).map_or(false, |s| s.gen == gen && s.node.is_some())
    }
}

thread_local! {
    pub(crate) static RT: RefCell<Rt> = RefCell::new(Rt::new());
}

pub(crate) fn with_rt<R>(f: impl FnOnce(&mut Rt) -> R) -> R {
    RT.with(|r| f(&mut r.borrow_mut()))
}

/// Drop every node and scope on this thread. Tests call this to start from a clean arena.
pub fn reset() {
    RT.with(|r| *r.borrow_mut() = Rt::new());
}

/// Arena statistics, used by the memory gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stats {
    /// Nodes currently alive.
    pub live_nodes: usize,
    /// Total slots ever allocated (live plus free list). Bounded under create/dispose churn.
    pub slots: usize,
    /// Live scopes, including the root.
    pub live_scopes: usize,
}

pub fn stats() -> Stats {
    with_rt(|r| Stats {
        live_nodes: r.slots.iter().filter(|s| s.node.is_some()).count(),
        slots: r.slots.len(),
        live_scopes: r.scopes.live(),
    })
}

pub(crate) fn alloc(is_effect: bool, value: Option<Value>, f: Option<Compute>, color: Color) -> (u32, u32) {
    with_rt(|r| {
        let scope = r.scopes.current();
        let node = Node { is_effect, value, f, subs: Vec::new(), srcs: Vec::new(), color };
        let (idx, gen) = if let Some(i) = r.free.pop() {
            let s = &mut r.slots[i as usize];
            s.node = Some(node);
            (i, s.gen)
        } else {
            r.slots.push(Slot { gen: 0, node: Some(node) });
            ((r.slots.len() - 1) as u32, 0)
        };
        r.scopes.add_node(scope, idx, gen);
        (idx, gen)
    })
}

/// Free a node: unlink it from the graph and recycle its slot under a new generation.
pub(crate) fn free_node(r: &mut Rt, id: u32) {
    let Some(node) = r.slots[id as usize].node.take() else { return };
    for s in &node.srcs {
        if let Some(n) = r.slots[*s as usize].node.as_mut() {
            n.subs.retain(|&x| x != id);
        }
    }
    for s in &node.subs {
        if let Some(n) = r.slots[*s as usize].node.as_mut() {
            n.srcs.retain(|&x| x != id);
        }
    }
    let slot = &mut r.slots[id as usize];
    slot.gen = slot.gen.wrapping_add(1);
    r.free.push(id);
}

fn track(id: u32) {
    with_rt(|r| {
        let Some(o) = r.observer else { return };
        let on = r.node(o);
        if on.srcs.contains(&id) {
            return;
        }
        on.srcs.push(id);
        r.node(id).subs.push(o);
    });
}

fn mark(r: &mut Rt, id: u32, c: Color) {
    let subs = std::mem::take(&mut r.node(id).subs);
    for &s in &subs {
        let n = r.node(s);
        if n.color == Color::Clean {
            n.color = c;
            if n.is_effect {
                r.queue.push(s);
            } else {
                mark(r, s, Color::Check);
            }
        } else if n.color == Color::Check && c == Color::Dirty {
            n.color = Color::Dirty;
        }
    }
    r.node(id).subs = subs;
}

fn color_of(id: u32) -> Option<Color> {
    with_rt(|r| r.slots[id as usize].node.as_ref().map(|n| n.color))
}

fn update(id: u32) {
    match color_of(id) {
        None | Some(Color::Clean) => return,
        Some(Color::Check) => {
            let srcs = with_rt(|r| r.node(id).srcs.clone());
            for s in srcs {
                update(s);
                if color_of(id) != Some(Color::Check) {
                    break;
                }
            }
        }
        Some(Color::Dirty) => {}
    }
    if color_of(id) != Some(Color::Dirty) {
        with_rt(|r| {
            if let Some(n) = r.slots[id as usize].node.as_mut() {
                n.color = Color::Clean;
            }
        });
        return;
    }
    let gen = with_rt(|r| r.slots[id as usize].gen);
    let (mut f, mut val, old_srcs) = with_rt(|r| {
        let n = r.node(id);
        (n.f.take().expect("reactive cycle: node recomputed while already computing"), n.value.take(), std::mem::take(&mut n.srcs))
    });
    // Unsubscribe from the previous sources; re-tracking re-adds the ones still read.
    with_rt(|r| {
        for s in &old_srcs {
            if let Some(n) = r.slots[*s as usize].node.as_mut() {
                n.subs.retain(|&x| x != id);
            }
        }
    });
    let prev = with_rt(|r| r.observer.replace(id));
    let changed = f(&mut val);
    with_rt(|r| {
        r.observer = prev;
        if r.slots[id as usize].gen != gen || r.slots[id as usize].node.is_none() {
            return; // disposed while running
        }
        let n = r.node(id);
        n.f = Some(f);
        n.value = val;
        n.color = Color::Clean;
        if changed {
            let subs = n.subs.clone();
            for s in subs {
                let sn = r.node(s);
                if sn.color == Color::Check {
                    sn.color = Color::Dirty;
                }
            }
        }
    });
}

pub(crate) fn read_value(idx: u32, gen: u32, tracked: bool) -> Value {
    if !with_rt(|r| r.alive(idx, gen)) {
        panic!("reactive handle used after its scope was disposed");
    }
    update(idx);
    if tracked {
        track(idx);
    }
    with_rt(|r| r.node(idx).value.clone().expect("reactive value missing (cycle?)"))
}

pub(crate) fn write_notify(idx: u32) {
    with_rt(|r| mark(r, idx, Color::Dirty));
    flush_if_idle();
}

pub(crate) fn flush_if_idle() {
    let idle = with_rt(|r| r.batch_depth == 0 && !r.flushing);
    if idle {
        flush();
    }
}

fn flush() {
    with_rt(|r| r.flushing = true);
    let mut rounds = 0u32;
    loop {
        let mut q = with_rt(|r| std::mem::take(&mut r.queue));
        if q.is_empty() {
            break;
        }
        rounds += 1;
        if rounds > 100_000 {
            with_rt(|r| r.flushing = false);
            panic!("effects keep re-triggering each other (reactive cycle)");
        }
        q.sort_unstable();
        q.dedup();
        for id in q {
            run_effect(id);
        }
    }
    with_rt(|r| r.flushing = false);
}

fn run_effect(id: u32) {
    if color_of(id).is_none() {
        return;
    }
    update(id); // handles Check (verifies sources) and Dirty (runs)
}

/// A state cell. `set` drops writes equal to the current value (needs `PartialEq`); `update` /
/// `set_always` work for any type.
pub fn state<T: 'static>(v: T) -> Signal<T> {
    let (idx, gen) = alloc(false, Some(Rc::new(RefCell::new(v)) as Value), None, Color::Clean);
    Signal::from_raw(idx, gen)
}

/// A cached computation over other cells. Re-runs lazily; stops propagation if its value is unchanged.
pub fn derived<T: PartialEq + 'static>(mut f: impl FnMut() -> T + 'static) -> Memo<T> {
    let compute: Compute = Box::new(move |slot: &mut Option<Value>| {
        let nv = f();
        match slot {
            Some(v) => {
                let cell = v.downcast_ref::<RefCell<T>>().expect("memo type");
                let mut cur = cell.borrow_mut();
                if *cur == nv {
                    false
                } else {
                    *cur = nv;
                    true
                }
            }
            None => {
                *slot = Some(Rc::new(RefCell::new(nv)));
                true
            }
        }
    });
    let (idx, gen) = alloc(false, None, Some(compute), Color::Dirty);
    Memo::from_raw(idx, gen)
}

/// A side effect: runs once now, then again after any cell it read changes (once per batch).
pub fn effect(mut f: impl FnMut() + 'static) {
    let compute: Compute = Box::new(move |_s: &mut Option<Value>| {
        f();
        false
    });
    let (idx, _) = alloc(true, None, Some(compute), Color::Dirty);
    update(idx);
}

/// Group writes: effects run once, after the outermost batch ends.
pub fn batch<R>(f: impl FnOnce() -> R) -> R {
    with_rt(|r| r.batch_depth += 1);
    let out = f();
    with_rt(|r| r.batch_depth -= 1);
    flush_if_idle();
    out
}

/// Run `f` without tracking reads.
pub fn untrack<R>(f: impl FnOnce() -> R) -> R {
    let prev = with_rt(|r| r.observer.take());
    let out = f();
    with_rt(|r| r.observer = prev);
    out
}
