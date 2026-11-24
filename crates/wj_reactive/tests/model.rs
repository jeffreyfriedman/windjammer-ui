//! Model-based property test: random reactive graphs checked against a naive
//! "recompute everything" oracle (ADR-0011 / VISION 6.4 item 1).
//!
//! The oracle evaluates every node from scratch after each operation. An effect must re-run
//! exactly when a value it read on its previous run now differs, at most once per batch, and the
//! values it observed must equal the oracle's (no glitches).
use std::cell::RefCell;
use std::rc::Rc;
use wj_reactive::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

#[derive(Clone, Debug)]
enum Spec {
    Sum(usize, usize),
    Pick(usize, usize, usize), // if node[c] even { node[a] } else { node[b] }
    Parity(usize),
    Min(usize, usize),
}

#[derive(Clone, Debug)]
struct EffectSpec {
    first: usize,
    gate: Option<usize>, // if node[first] even, also read node[gate]
}

#[derive(Clone, Copy)]
enum Cell {
    S(Signal<i32>),
    M(Memo<i32>),
}
impl Cell {
    fn get(&self) -> i32 {
        match self {
            Cell::S(s) => s.get(),
            Cell::M(m) => m.get(),
        }
    }
}

struct Oracle {
    n_src: usize,
    specs: Vec<Spec>,
}
impl Oracle {
    fn eval(&self, src: &[i32]) -> Vec<i32> {
        let mut v: Vec<i32> = src.to_vec();
        for s in &self.specs {
            let x = match *s {
                Spec::Sum(a, b) => (v[a] + v[b]) % 5,
                Spec::Pick(c, a, b) => if v[c] % 2 == 0 { v[a] } else { v[b] },
                Spec::Parity(a) => v[a] % 2,
                Spec::Min(a, b) => v[a].min(v[b]),
            };
            v.push(x);
        }
        let _ = self.n_src;
        v
    }
}

fn effect_reads(e: &EffectSpec, vals: &[i32]) -> Vec<(usize, i32)> {
    let mut r = vec![(e.first, vals[e.first])];
    if let Some(g) = e.gate {
        if vals[e.first] % 2 == 0 {
            r.push((g, vals[g]));
        }
    }
    r
}

fn run_case(seed: u64) {
    reset();
    let mut rng = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    let n_src = 1 + rng.below(5);
    let n_memo = rng.below(12);
    let mut specs = vec![];
    for i in 0..n_memo {
        let hi = n_src + i;
        let kind = rng.next() % 4;
        let mut p = || rng.below(hi);
        let s = match kind {
            0 => Spec::Sum(p(), p()),
            1 => Spec::Pick(p(), p(), p()),
            2 => Spec::Parity(p()),
            _ => Spec::Min(p(), p()),
        };
        specs.push(s);
    }
    let total = n_src + n_memo;
    let oracle = Oracle { n_src, specs: specs.clone() };
    let mut src: Vec<i32> = (0..n_src).map(|_| (rng.next() % 4) as i32).collect();

    let mut cells: Vec<Cell> = src.iter().map(|&v| Cell::S(state(v))).collect();
    for (i, s) in specs.iter().cloned().enumerate() {
        let c = cells.clone();
        let f: Box<dyn Fn() -> i32> = match s {
            Spec::Sum(a, b) => Box::new(move || (c[a].get() + c[b].get()) % 5),
            Spec::Pick(cc, a, b) => Box::new(move || if c[cc].get() % 2 == 0 { c[a].get() } else { c[b].get() }),
            Spec::Parity(a) => Box::new(move || c[a].get() % 2),
            Spec::Min(a, b) => Box::new(move || c[a].get().min(c[b].get())),
        };
        let _ = i;
        cells.push(Cell::M(derived(move || f())));
    }

    let n_eff = 1 + rng.below(4);
    let especs: Vec<EffectSpec> = (0..n_eff)
        .map(|_| EffectSpec { first: rng.below(total), gate: if rng.below(2) == 0 { Some(rng.below(total)) } else { None } })
        .collect();
    let logs: Vec<Rc<RefCell<Vec<Vec<(usize, i32)>>>>> = especs.iter().map(|_| Rc::new(RefCell::new(vec![]))).collect();
    for (e, log) in especs.iter().zip(&logs) {
        let (e, log, c) = (e.clone(), log.clone(), cells.clone());
        effect(move || {
            let mut r = vec![(e.first, c[e.first].get())];
            if let Some(g) = e.gate {
                if r[0].1 % 2 == 0 {
                    r.push((g, c[g].get()));
                }
            }
            log.borrow_mut().push(r);
        });
    }

    let vals = oracle.eval(&src);
    let mut last: Vec<Vec<(usize, i32)>> = especs.iter().map(|e| effect_reads(e, &vals)).collect();
    let mut runs: Vec<usize> = vec![1; especs.len()];
    for (i, l) in logs.iter().enumerate() {
        assert_eq!(l.borrow().len(), 1, "seed {seed}");
        assert_eq!(l.borrow()[0], last[i], "seed {seed}: initial run");
    }

    for step in 0..(5 + rng.below(20)) {
        let k = 1 + rng.below(3);
        // Distinct sources per step: a batch that writes a signal and then restores it re-runs
        // dependents (dirty marking cannot know the value came back), which the value-diff oracle
        // would not predict. That behaviour is pinned separately in tests/semantics.rs.
        let mut writes: Vec<(usize, i32)> = vec![];
        for _ in 0..k {
            let i = rng.below(n_src);
            if !writes.iter().any(|w| w.0 == i) {
                writes.push((i, (rng.next() % 4) as i32));
            }
        }
        // A unit is one propagation: the whole step if batched, otherwise each write alone.
        let units: Vec<Vec<(usize, i32)>> = if rng.below(2) == 0 { vec![writes] } else { writes.into_iter().map(|w| vec![w]).collect() };
        for unit in units {
            let apply = |cells: &[Cell]| {
                for &(i, v) in &unit {
                    if let Cell::S(s) = cells[i] {
                        s.set(v);
                    }
                }
            };
            batch(|| apply(&cells));
            for &(i, v) in &unit {
                src[i] = v;
            }
            let new_vals = oracle.eval(&src);
            for (ei, e) in especs.iter().enumerate() {
                if last[ei].iter().any(|&(n, v)| new_vals[n] != v) {
                    runs[ei] += 1;
                    last[ei] = effect_reads(e, &new_vals);
                }
                assert_eq!(logs[ei].borrow().len(), runs[ei], "seed {seed} step {step} effect {ei}: run count");
                assert_eq!(logs[ei].borrow().last().unwrap(), &last[ei], "seed {seed} step {step} effect {ei}: observed values");
            }
            for (i, c) in cells.iter().enumerate() {
                if let Cell::M(m) = c {
                    assert_eq!(m.get_untracked(), new_vals[i], "seed {seed} step {step}: memo {i}");
                }
            }
        }
    }
}

#[test]
fn random_graphs_match_the_recompute_everything_oracle_10k_cases() {
    for seed in 0..10_000u64 {
        run_case(seed);
    }
}
