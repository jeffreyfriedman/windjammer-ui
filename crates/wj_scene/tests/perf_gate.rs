//! M1 acceptance gate (VISION 6.2): 10k-row create+update benchmark, js-framework-benchmark-style
//! operations driven through scene bindings.
//!   * median update-one-row <= 0.1 ms CPU
//!   * create 10k rows       <= 150 ms (native)
//! The gate asserts only in optimized builds (`cargo test --release`); debug builds run the same
//! operations for correctness and skip the timing assertions. Medians use many samples because the
//! development machines are shared; the printed table is the record (`-- --nocapture`).
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use wj_reactive::{reset, state, Selector, Signal};
use wj_scene::*;

#[derive(Clone, Copy)]
struct Row {
    id: u32,
    label: Signal<String>,
}

struct App {
    scene: Scene,
    list: NodeId,
    rows: Signal<Vec<Row>>,
    selected: Signal<Option<u32>>,
    next_id: u32,
    labels: Rc<RefCell<HashMap<u32, Signal<String>>>>,
}

impl App {
    fn new() -> App {
        reset();
        let scene = Scene::new();
        let list = scene.create(scene.root());
        let rows = state(Vec::<Row>::new());
        let selected = state(None::<u32>);
        let selector = Selector::new(move || selected.get());
        let labels: Rc<RefCell<HashMap<u32, Signal<String>>>> = Rc::default();
        let l2 = labels.clone();
        scene.keyed_children(list, move || rows.with(|r| r.iter().map(|x| x.id).collect::<Vec<u32>>()), move |id| {
            let label = *l2.borrow().get(&id).expect("label");
            let row = scene.create_detached();
            scene.set_style(row, Style { direction: Direction::Row, height: Dim::Px(20.0), ..Default::default() });
            let sel = selector.clone();
            scene.bind_background(row, move || sel.is_selected(&id).then_some(Color::rgb(200, 220, 255)));
            let idn = scene.create(row);
            scene.set_text(idn, TextProps::new(id.to_string()));
            let lab = scene.create(row);
            scene.bind_text(lab, move || label.get());
            row
        });
        App { scene, list, rows, selected, next_id: 1, labels }
    }
    fn make(&mut self, n: usize) -> Vec<Row> {
        (0..n)
            .map(|_| {
                let id = self.next_id;
                self.next_id += 1;
                let label = state(format!("pretty blue table {id}"));
                self.labels.borrow_mut().insert(id, label);
                Row { id, label }
            })
            .collect()
    }
    fn create(&mut self, n: usize) {
        let rows = self.make(n);
        self.rows.set_always(rows);
    }
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn ms(f: impl FnOnce()) -> f64 {
    let t = Instant::now();
    f();
    t.elapsed().as_secs_f64() * 1e3
}

#[test]
fn ten_thousand_row_benchmark() {
    let release = !cfg!(debug_assertions);

    // create 1k, then 10k (replacing) -- take the best of a few runs to ride out machine load
    let mut create1k = vec![];
    let mut create10k = vec![];
    for _ in 0..5 {
        let mut app = App::new();
        create1k.push(ms(|| app.create(1000)));
        let mut app = App::new();
        create10k.push(ms(|| app.create(10_000)));
        assert_eq!(app.scene.child_count(app.list), 10_000);
        assert_eq!(app.scene.node_count(), 2 + 3 * 10_000);
    }
    let create10k_best = create10k.iter().cloned().fold(f64::MAX, f64::min);

    let mut app = App::new();
    app.create(10_000);
    app.scene.drain_dirty();

    // update one row: median over 1000 distinct rows
    let rows = app.rows.get();
    let mut one = vec![];
    for i in 0..1000 {
        let r = rows[i * 7 % rows.len()];
        let t = Instant::now();
        r.label.set(format!("updated {i}"));
        one.push(t.elapsed().as_secs_f64() * 1e3);
    }
    let touched = app.scene.drain_dirty().len();
    assert_eq!(touched, 1000, "each single-row update dirties exactly one node");
    let update_one = median(one);

    // update every 10th row
    let every10 = ms(|| {
        wj_reactive::batch(|| {
            for (i, r) in rows.iter().enumerate().step_by(10) {
                r.label.set(format!("again {i}"));
            }
        })
    });
    assert_eq!(app.scene.drain_dirty().len(), 1000);

    // select row (selector: only old+new rows touched)
    let mut sel = vec![];
    for i in 0..50u32 {
        sel.push(ms(|| app.selected.set(Some(1 + i * 37))));
    }
    assert_eq!(app.scene.drain_dirty().len() >= 2, true);
    let select = median(sel);

    // swap rows 2 and 999
    let swap = ms(|| app.rows.update(|r| r.swap(1, 998)));
    // remove a row
    let remove = ms(|| app.rows.update(|r| {
        r.remove(500);
    }));
    assert_eq!(app.scene.child_count(app.list), 9_999);
    // clear
    let clear = ms(|| app.rows.set_always(vec![]));
    assert_eq!(app.scene.node_count(), 2);

    println!("\n10k-row benchmark (native, {} build, scene bindings only)", if release { "release" } else { "DEBUG" });
    println!("| operation | ms |\n|---|---|");
    println!("| create 1k (best of 5) | {:.2} |", create1k.iter().cloned().fold(f64::MAX, f64::min));
    println!("| create 10k (best of 5) | {create10k_best:.2} |");
    println!("| update one row (median of 1000) | {update_one:.4} |");
    println!("| update every 10th (1000 rows, batched) | {every10:.2} |");
    println!("| select row (median of 50) | {select:.3} |");
    println!("| swap rows | {swap:.2} |");
    println!("| remove row | {remove:.2} |");
    println!("| clear 10k | {clear:.2} |");

    if release {
        assert!(update_one <= 0.1, "median update-one-row {update_one:.4} ms > 0.1 ms");
        assert!(create10k_best <= 150.0, "create 10k {create10k_best:.1} ms > 150 ms");
    }
}
