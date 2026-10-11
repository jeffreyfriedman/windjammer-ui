//! Candidate Windjammer-UI reactive core: arena + Copy generational handles + lazy memos with
//! three-colour (clean/check/dirty) invalidation. No Rc/RefCell per node, no per-node allocation
//! beyond the value box. Copy handles are what the compiler needs to make closure capture free (C1).
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq)]
enum Color { Clean, Check, Dirty }

struct Node {
    value: Option<Box<dyn Any>>,
    f: Option<Box<dyn FnMut(&mut Option<Box<dyn Any>>) -> bool>>, // recompute; returns changed
    subs: Vec<u32>,
    srcs: Vec<u32>,
    prev: Vec<u32>,
    color: Color,
    is_effect: bool,
}

#[derive(Default)]
struct Rt { nodes: Vec<Node>, observer: Option<u32>, queue: Vec<u32> }
thread_local! { static RT: RefCell<Rt> = RefCell::new(Rt::default()); }

pub struct Sig<T> { id: u32, _p: PhantomData<fn() -> T> }
impl<T> Clone for Sig<T> { fn clone(&self) -> Self { *self } }
impl<T> Copy for Sig<T> {}

fn alloc(n: Node) -> u32 { RT.with(|r| { let mut r = r.borrow_mut(); r.nodes.push(n); (r.nodes.len() - 1) as u32 }) }
pub fn reset() { RT.with(|r| *r.borrow_mut() = Rt::default()); }

pub fn signal<T: 'static>(v: T) -> Sig<T> {
    Sig { id: alloc(Node { value: Some(Box::new(v)), f: None, subs: vec![], srcs: vec![], prev: vec![], color: Color::Clean, is_effect: false }), _p: PhantomData }
}
fn track(id: u32) {
    RT.with(|r| { let mut r = r.borrow_mut();
        if let Some(o) = r.observer { let on = &mut r.nodes[o as usize];
            if on.srcs.contains(&id) { return; }
            on.srcs.push(id);
            let was = on.prev.contains(&id);
            if !was { r.nodes[id as usize].subs.push(o); } } });
}
fn mark(r: &mut Rt, id: u32, c: Color) {
    let subs = std::mem::take(&mut r.nodes[id as usize].subs);
    for &s in &subs {
        let n = &mut r.nodes[s as usize];
        if n.color == Color::Clean { n.color = c; if n.is_effect { r.queue.push(s); } else { mark(r, s, Color::Check); } }
        else if n.color == Color::Check && c == Color::Dirty { n.color = Color::Dirty; }
    }
    r.nodes[id as usize].subs = subs;
}
fn color_of(id: u32) -> Color { RT.with(|r| r.borrow().nodes[id as usize].color) }
fn update(id: u32) {
    if color_of(id) == Color::Clean { return; }
    if color_of(id) == Color::Check {
        let srcs = RT.with(|r| r.borrow().nodes[id as usize].srcs.clone());
        for s in srcs { update(s); if color_of(id) == Color::Dirty { break; } }
    }
    if color_of(id) == Color::Dirty {
        let (mut f, mut val, old_srcs) = RT.with(|r| { let mut r = r.borrow_mut(); let n = &mut r.nodes[id as usize];
            (n.f.take().unwrap(), n.value.take(), std::mem::take(&mut n.srcs)) });
        RT.with(|r| r.borrow_mut().nodes[id as usize].prev = old_srcs);
        let prev = RT.with(|r| r.borrow_mut().observer.replace(id));
        let changed = f(&mut val);
        RT.with(|r| { let mut r = r.borrow_mut(); r.observer = prev; let old = std::mem::take(&mut r.nodes[id as usize].prev); let cur = r.nodes[id as usize].srcs.clone();
            for s in old { if !cur.contains(&s) { r.nodes[s as usize].subs.retain(|&x| x != id); } }
            let n = &mut r.nodes[id as usize]; n.f = Some(f); n.value = val; n.color = Color::Clean;
            if changed { let subs = n.subs.clone(); for s in subs { let sn = &mut r.nodes[s as usize]; if sn.color == Color::Check { sn.color = Color::Dirty; } } } });
    } else { RT.with(|r| r.borrow_mut().nodes[id as usize].color = Color::Clean); }
}
impl<T: Clone + PartialEq + 'static> Sig<T> {
    pub fn get(&self) -> T { update(self.id); track(self.id); RT.with(|r| r.borrow().nodes[self.id as usize].value.as_ref().unwrap().downcast_ref::<T>().unwrap().clone()) }
    pub fn set(&self, v: T) {
        RT.with(|r| { let mut r = r.borrow_mut();
            let slot = r.nodes[self.id as usize].value.as_mut().unwrap().downcast_mut::<T>().unwrap();
            if *slot == v { return; } *slot = v; mark(&mut r, self.id, Color::Dirty); });
        flush();
    }
}
pub fn memo<T: Clone + PartialEq + 'static>(mut f: impl FnMut() -> T + 'static) -> Sig<T> {
    let id = alloc(Node { value: None, f: Some(Box::new(move |slot: &mut Option<Box<dyn Any>>| {
        let nv = f();
        match slot { Some(b) => { let old = b.downcast_mut::<T>().unwrap(); if *old == nv { false } else { *old = nv; true } }
                     None => { *slot = Some(Box::new(nv)); true } } })),
        subs: vec![], srcs: vec![], prev: vec![], color: Color::Dirty, is_effect: false });
    Sig { id, _p: PhantomData }
}
/// Effect: runs eagerly once, then re-runs after any source changes (on flush).
pub fn effect(mut f: impl FnMut() + 'static) {
    let id = alloc(Node { value: None, f: Some(Box::new(move |_s: &mut Option<Box<dyn Any>>| { f(); false })),
        subs: vec![], srcs: vec![], prev: vec![], color: Color::Dirty, is_effect: true });
    update(id);
}
pub fn flush() {
    loop {
        let next = RT.with(|r| r.borrow_mut().queue.pop());
        let Some(id) = next else { break };
        update_effect(id);
    }
}
fn update_effect(id: u32) {
    let color = RT.with(|r| r.borrow().nodes[id as usize].color);
    if color == Color::Check {
        let srcs = RT.with(|r| r.borrow().nodes[id as usize].srcs.clone());
        for s in srcs { update(s); if RT.with(|r| r.borrow().nodes[id as usize].color) == Color::Dirty { break; } }
    }
    if color_of(id) == Color::Dirty { update(id); } else { RT.with(|r| r.borrow_mut().nodes[id as usize].color = Color::Clean); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};
    #[test] fn diamond_is_glitch_free_and_runs_once() {
        reset(); let a = signal(1i32); let b = memo(move || a.get() + 1); let c = memo(move || a.get() * 2);
        let runs = Rc::new(Cell::new(0)); let seen = Rc::new(Cell::new((0, 0)));
        let (r, s) = (runs.clone(), seen.clone());
        effect(move || { r.set(r.get() + 1); s.set((b.get(), c.get())); });
        assert_eq!(runs.get(), 1); a.set(5);
        assert_eq!(runs.get(), 2, "effect ran exactly once per update"); assert_eq!(seen.get(), (6, 10));
    }
    #[test] fn equality_cutoff_stops_propagation() {
        reset(); let a = signal(3i32); let parity = memo(move || a.get() % 2);
        let runs = Rc::new(Cell::new(0)); let r = runs.clone();
        effect(move || { r.set(r.get() + 1); let _ = parity.get(); });
        a.set(5); assert_eq!(runs.get(), 1, "parity unchanged => effect not re-run");
        a.set(6); assert_eq!(runs.get(), 2);
    }
    #[test] fn dynamic_dependencies() {
        reset(); let flag = signal(true); let x = signal(1i32); let y = signal(100i32);
        let m = memo(move || if flag.get() { x.get() } else { y.get() });
        let runs = Rc::new(Cell::new(0)); let r = runs.clone();
        effect(move || { r.set(r.get() + 1); let _ = m.get(); });
        y.set(101); assert_eq!(runs.get(), 1, "y not yet a dependency");
        flag.set(false); assert_eq!(runs.get(), 2); x.set(2); assert_eq!(runs.get(), 2, "x dropped");
        y.set(102); assert_eq!(runs.get(), 3);
    }
}
