//! Property tests: random scenes and random edit sequences.
//!  * the incremental engine always agrees with a fresh full layout of the same scene (oracle),
//!  * every rect is finite and non-negative; unpadded auto-width column children fit their parent.
use wj_layout::*;
use wj_scene::*;

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
    fn f(&mut self, max: u64) -> f32 {
        (self.next() % max) as f32
    }
}

fn rand_style(r: &mut Rng) -> Style {
    let dim = |r: &mut Rng| match r.below(4) {
        0 => Dim::Auto,
        1 => Dim::Px(r.f(60)),
        2 => Dim::Pct(r.f(100)),
        _ => Dim::Auto,
    };
    Style {
        direction: if r.below(2) == 0 { Direction::Row } else { Direction::Column },
        width: dim(r),
        height: dim(r),
        padding: Edges::all(r.f(6)),
        margin: Edges { left: r.f(4), top: r.f(4), right: r.f(4), bottom: r.f(4) },
        gap: r.f(5),
        flex_grow: r.f(3),
        flex_shrink: (r.below(2)) as f32,
        align_items: [Align::Start, Align::Center, Align::End, Align::Stretch][r.below(4)],
        justify: [Justify::Start, Justify::Center, Justify::End, Justify::SpaceBetween, Justify::SpaceAround][r.below(5)],
        wrap: r.below(4) == 0,
        absolute: if r.below(8) == 0 { Some((r.f(30), r.f(30))) } else { None },
        scroll: r.below(10) == 0,
    }
}

fn all_nodes(s: Scene, n: NodeId, out: &mut Vec<NodeId>) {
    out.push(n);
    for c in s.children(n) {
        all_nodes(s, c, out);
    }
}

fn snapshot(s: Scene) -> Vec<(NodeId, Rect)> {
    let mut v = vec![];
    all_nodes(s, s.root(), &mut v);
    v.into_iter().map(|n| (n, s.rect(n))).collect()
}

fn check_case(seed: u64) {
    let mut r = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    wj_reactive::reset();
    let s = Scene::new();
    let m = FixedAdvanceMeasure;
    let mut nodes = vec![s.root()];
    for _ in 0..(3 + r.below(25)) {
        let parent = nodes[r.below(nodes.len())];
        let n = s.create(parent);
        s.set_style(n, rand_style(&mut r));
        if r.below(4) == 0 {
            let mut t = TextProps::new("lorem ipsum dolor sit amet".chars().take(1 + r.below(26)).collect::<String>());
            t.font_size = 8.0 + r.f(12);
            s.set_text(n, t);
        }
        nodes.push(n);
    }
    let vp = (50.0 + r.f(300), 50.0 + r.f(300));
    let mut inc = LayoutEngine::new();
    let d = s.drain_dirty();
    inc.update(s, &d, vp, &m);
    for step in 0..(1 + r.below(12)) {
        // random edit
        let live: Vec<NodeId> = {
            let mut v = vec![];
            all_nodes(s, s.root(), &mut v);
            v
        };
        let mut edit = String::new();
        match r.below(5) {
            0 | 1 => {
                let n = live[r.below(live.len())];
                edit = format!("style {n:?}");
                s.set_style(n, rand_style(&mut r))
            }
            2 => {
                let n = live[r.below(live.len())];
                edit = format!("text {n:?}");
                let mut t = TextProps::new("x".repeat(1 + r.below(30)));
                t.font_size = 8.0 + r.f(10);
                s.set_text(n, t);
            }
            3 => {
                let p = live[r.below(live.len())];
                let c = s.create(p);
                edit = format!("add {c:?} under {p:?}");
                s.set_style(c, rand_style(&mut r));
            }
            _ => {
                if live.len() > 2 {
                    let n = live[1 + r.below(live.len() - 1)];
                    edit = format!("remove {n:?}");
                    s.remove(n);
                }
            }
        }
        let d = s.drain_dirty();
        inc.update(s, &d, vp, &m);
        let a = snapshot(s);
        let mut fresh = LayoutEngine::new();
        fresh.update(s, &[], vp, &m);
        let b = snapshot(s);
        assert_eq!(a, b, "seed {seed} step {step} edit {edit}: incremental layout diverged from a fresh layout");
        assert_eq!(inc.mirrored(), s.node_count(), "seed {seed} step {step}: mirror leak");
        s.drain_dirty();
        for (n, rc) in &b {
            assert!(rc.x.is_finite() && rc.y.is_finite() && rc.w.is_finite() && rc.h.is_finite(), "seed {seed}: non-finite {n:?} {rc:?}");
            assert!(rc.w >= 0.0 && rc.h >= 0.0, "seed {seed}: negative size {rc:?} style {:?} text {:?} parent {:?}", s.style(*n), s.text(*n), s.parent(*n).map(|p| (s.style(p), s.rect(p))));
        }
    }
}

#[test]
fn incremental_layout_matches_fresh_layout_and_stays_finite_3000_cases() {
    for seed in 1..=3000u64 {
        check_case(seed);
    }
}

#[test]
fn plain_column_children_fit_inside_the_parent() {
    for seed in 1..=500u64 {
        let mut r = Rng(seed * 7919 + 1);
        wj_reactive::reset();
        let s = Scene::new();
        let w = 80.0 + r.f(200);
        for _ in 0..(1 + r.below(8)) {
            let c = s.create(s.root());
            s.set_style(c, Style { height: Dim::Px(5.0 + r.f(20)), margin: Edges { left: r.f(5), right: r.f(5), ..Default::default() }, ..Default::default() });
        }
        let mut e = LayoutEngine::new();
        let d = s.drain_dirty();
        e.update(s, &d, (w, 1000.0), &FixedAdvanceMeasure);
        for c in s.children(s.root()) {
            let rc = s.rect(c);
            assert!(rc.x >= 0.0 && rc.x + rc.w <= w + 1e-3, "seed {seed}: {rc:?} outside width {w}");
        }
    }
}
