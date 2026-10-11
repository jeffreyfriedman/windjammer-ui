mod wjsig;
use std::time::Instant;
use std::hint::black_box;

fn time<R>(label: &str, reps: u32, mut f: impl FnMut() -> R) {
    let mut v = vec![];
    for _ in 0..reps { let t = Instant::now(); black_box(f()); v.push(t.elapsed().as_secs_f64() * 1e3); }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{label:<48} median {:>9.3} ms  min {:>9.3} ms", v[v.len() / 2], v[0]);
}

const N: usize = 10_000;

fn wj_create() { wjsig::reset(); let mut s = vec![]; for i in 0..N { s.push(wjsig::signal(i as u64)); } for i in 0..N { let a = s[i]; let m = wjsig::memo(move || a.get() * 2); black_box(m.get()); } }
fn wj_fanout() { // 1 source -> N memos -> 1 effect-less read of all
    wjsig::reset(); let src = wjsig::signal(0u64); let ms: Vec<_> = (0..N).map(|i| wjsig::memo(move || src.get() + i as u64)).collect();
    for m in &ms { black_box(m.get()); }
    for k in 1..=10 { src.set(k); let mut t = 0; for m in &ms { t += m.get(); } black_box(t); }
}
fn wj_chain() { wjsig::reset(); let src = wjsig::signal(0u64); let mut prev = src; for _ in 0..1000 { let p = prev; prev = wjsig::memo(move || p.get() + 1); }
    for k in 1..=100 { src.set(k); black_box(prev.get()); } }
fn wj_effects() { // 10k effects each reading its own signal; update 1 -> only 1 runs
    wjsig::reset(); let s: Vec<_> = (0..N).map(|i| wjsig::signal(i as u64)).collect();
    let cnt = std::rc::Rc::new(std::cell::Cell::new(0u64));
    for &x in &s { let c = cnt.clone(); wjsig::effect(move || { c.set(c.get() + x.get()); }); }
    for k in 0..1000 { s[(k * 7) % N].set(1_000_000 + k as u64); } black_box(cnt.get());
}
fn wj_diamond() { wjsig::reset(); let a = wjsig::signal(0u64); let b = wjsig::memo(move || a.get() + 1); let c = wjsig::memo(move || a.get() * 2);
    let d = wjsig::memo(move || b.get() + c.get()); for k in 1..=100_000u64 { a.set(k); black_box(d.get()); } }

mod rg {
    use reactive_graph::prelude::*;
    use reactive_graph::signal::RwSignal;
    use reactive_graph::computed::Memo;
    use reactive_graph::owner::Owner;
    use super::N;
    use std::hint::black_box;
    pub fn create() { let o = Owner::new(); o.set(); let s: Vec<_> = (0..N).map(|i| RwSignal::new(i as u64)).collect();
        for i in 0..N { let a = s[i]; let m = Memo::new(move |_| a.get() * 2); black_box(m.get()); } drop(o); }
    pub fn fanout() { let o = Owner::new(); o.set(); let src = RwSignal::new(0u64); let ms: Vec<_> = (0..N).map(|i| Memo::new(move |_| src.get() + i as u64)).collect();
        for m in &ms { black_box(m.get()); }
        for k in 1..=10 { src.set(k); let mut t = 0; for m in &ms { t += m.get(); } black_box(t); } drop(o); }
    pub fn chain() { let o = Owner::new(); o.set(); let src = RwSignal::new(0u64); let mut prev: Memo<u64> = Memo::new(move |_| src.get());
        for _ in 0..1000 { let p = prev; prev = Memo::new(move |_| p.get() + 1); }
        for k in 1..=100 { src.set(k); black_box(prev.get()); } drop(o); }
    pub fn diamond() { let o = Owner::new(); o.set(); let a = RwSignal::new(0u64); let b = Memo::new(move |_| a.get() + 1); let c = Memo::new(move |_| a.get() * 2);
        let d = Memo::new(move |_| b.get() + c.get()); for k in 1..=100_000u64 { a.set(k); black_box(d.get()); } drop(o); }
}

fn main() {
    let _ = any_spawner::Executor::init_futures_executor();
    println!("N = {N}; release, LTO. wjsig = candidate core; reactive_graph 0.2.15 = Leptos 0.8 core");
    time("wjsig  create 10k sig + 10k memo + read", 15, wj_create);
    time("rg     create 10k sig + 10k memo + read", 15, rg::create);
    time("wjsig  fanout 1->10k memos x10 updates+reads", 15, wj_fanout);
    time("rg     fanout 1->10k memos x10 updates+reads", 15, rg::fanout);
    time("wjsig  chain depth 1000 x100 updates", 15, wj_chain);
    time("rg     chain depth 1000 x100 updates", 15, rg::chain);
    time("wjsig  diamond x100k updates", 15, wj_diamond);
    time("rg     diamond x100k updates", 15, rg::diamond);
    time("wjsig  10k effects, 1000 single updates", 15, wj_effects);
}
