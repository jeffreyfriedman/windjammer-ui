use std::cell::Cell;
use std::rc::Rc;
use wj_reactive::{reset, state, batch};
use wj_scene::*;

fn setup() -> Scene {
    reset();
    Scene::new()
}

#[test]
fn text_binding_updates_only_the_bound_node() {
    let scene = setup();
    let a = scene.create(scene.root());
    let b = scene.create(scene.root());
    let n = state(1);
    scene.bind_text(a, move || format!("count {}", n.get()));
    scene.set_text(b, TextProps::new("static"));
    scene.drain_dirty();
    n.set(2);
    let log = scene.drain_dirty();
    assert_eq!(log.len(), 1, "exactly one node is touched: {log:?}");
    assert_eq!(log[0].node, a);
    assert!(log[0].flags.contains(Dirty::LAYOUT) && log[0].flags.contains(Dirty::PAINT) && log[0].flags.contains(Dirty::SEMANTICS));
    assert_eq!(scene.text(a).unwrap().content, "count 2");
}

#[test]
fn colour_binding_only_dirties_paint() {
    let scene = setup();
    let a = scene.create(scene.root());
    let sel = state(false);
    scene.bind_background(a, move || Some(if sel.get() { Color::rgb(255, 0, 0) } else { Color::rgb(0, 0, 255) }));
    scene.drain_dirty();
    sel.set(true);
    let log = scene.drain_dirty();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].flags, Dirty::PAINT);
}

#[test]
fn setters_with_equal_values_do_not_dirty() {
    let scene = setup();
    let a = scene.create(scene.root());
    scene.set_text(a, TextProps::new("x"));
    scene.drain_dirty();
    scene.set_text(a, TextProps::new("x"));
    scene.set_style(a, Style::default());
    assert!(scene.drain_dirty().is_empty());
}

#[test]
fn removing_a_node_stops_its_bindings_and_frees_the_subtree() {
    let scene = setup();
    let parent = scene.create(scene.root());
    let child = scene.create(parent);
    let n = state(0);
    let runs = Rc::new(Cell::new(0));
    let r = runs.clone();
    scene.bind(child, move || {
        r.set(r.get() + 1);
        let _ = n.get();
    });
    n.set(1);
    assert_eq!(runs.get(), 2);
    let before = scene.node_count();
    scene.remove(parent);
    assert_eq!(scene.node_count(), before - 2);
    n.set(2);
    assert_eq!(runs.get(), 2, "binding disposed with its node");
    assert!(!scene.is_alive(child));
}

fn row_label(scene: Scene, k: u32, label: wj_reactive::Signal<u32>) -> NodeId {
    let node = scene.create_detached();
    scene.bind_text(node, move || format!("row {k} v{}", label.get()));
    node
}

#[test]
fn keyed_children_reuse_reorder_and_remove() {
    let scene = setup();
    let list = scene.create(scene.root());
    let keys = state(vec![1u32, 2, 3]);
    let tick = state(0u32);
    let built = Rc::new(Cell::new(0));
    let b = built.clone();
    scene.keyed_children(list, move || keys.get(), move |k| {
        b.set(b.get() + 1);
        row_label(scene, k, tick)
    });
    assert_eq!(built.get(), 3);
    let before = scene.children(list);
    keys.set(vec![3, 2, 1]); // reverse
    assert_eq!(built.get(), 3, "no rows rebuilt on reorder");
    let after = scene.children(list);
    assert_eq!(after, before.iter().rev().copied().collect::<Vec<_>>());
    keys.set(vec![3, 1]); // remove 2
    assert_eq!(scene.child_count(list), 2);
    assert!(!scene.is_alive(before[1]));
    let n = scene.node_count();
    tick.set(9);
    assert_eq!(scene.text(scene.children(list)[0]).unwrap().content, "row 3 v9");
    keys.set(vec![]);
    assert_eq!(scene.node_count(), n - 2);
}

#[test]
#[should_panic(expected = "duplicate key")]
fn duplicate_keys_are_rejected() {
    let scene = setup();
    let list = scene.create(scene.root());
    scene.keyed_children(list, || vec![1u32, 1], move |_| scene.create_detached());
}

#[test]
fn virtual_list_materializes_only_the_window() {
    let scene = setup();
    let list = scene.create(scene.root());
    let scroll = state(0.0f32);
    scene.virtual_list(list, 20.0, 2, || 10_000, move || scroll.get(), || 400.0, move |i| {
        let n = scene.create_detached();
        scene.set_text(n, TextProps::new(format!("item {i}")));
        n
    });
    // 400/20 = 20 visible + 2 overscan below
    assert_eq!(scene.child_count(list), 22);
    assert!(scene.node_count() < 30);
    assert_eq!(scene.style(list).height, Dim::Px(200_000.0));
    scroll.set(20_000.0);
    let kids = scene.children(list);
    assert_eq!(kids.len(), 24); // 2 above + 20 + 2 below
    assert_eq!(scene.text(kids[0]).unwrap().content, "item 998");
    assert_eq!(scene.style(kids[0]).absolute, Some((0.0, 998.0 * 20.0)));
    assert!(scene.node_count() < 40, "old window removed, nodes bounded: {}", scene.node_count());
}

#[test]
fn visible_range_edges() {
    assert_eq!(visible_range(0.0, 100.0, 10.0, 0, 0), (0, 0));
    assert_eq!(visible_range(0.0, 100.0, 10.0, 5, 0), (0, 5));
    assert_eq!(visible_range(95.0, 100.0, 10.0, 1000, 0), (9, 20));
    assert_eq!(visible_range(-50.0, 100.0, 10.0, 1000, 1), (0, 11));
    assert_eq!(visible_range(1e9, 100.0, 10.0, 100, 3), (100, 100));
}

fn button(scene: Scene, parent: NodeId, label: &str, rect: Rect) -> NodeId {
    let b = scene.create(parent);
    scene.set_rect(b, rect);
    scene.set_semantics(b, Semantics { role: Role::Button, focusable: true, ..Default::default() });
    let t = scene.create(b);
    scene.set_text(t, TextProps::new(label));
    b
}

#[test]
fn click_is_synthesized_and_runs_handlers_in_one_batch() {
    let scene = setup();
    scene.set_rect(scene.root(), Rect::new(0.0, 0.0, 200.0, 200.0));
    let b = button(scene, scene.root(), "Add", Rect::new(10.0, 10.0, 50.0, 20.0));
    let count = state(0);
    let effect_runs = Rc::new(Cell::new(0));
    let er = effect_runs.clone();
    wj_reactive::effect(move || {
        er.set(er.get() + 1);
        let _ = count.get();
    });
    scene.on(b, EventKind::Click, move |_| {
        count.set(count.get_untracked() + 1);
        count.set(count.get_untracked() + 1);
    });
    assert!(!scene.dispatch(Event::PointerDown { x: 300.0, y: 300.0 }));
    assert!(!scene.dispatch(Event::PointerDown { x: 15.0, y: 15.0 }));
    assert!(scene.dispatch(Event::PointerUp { x: 15.0, y: 15.0 }));
    assert_eq!(count.get(), 2);
    assert_eq!(effect_runs.get(), 2, "two writes in a handler flush once");
    // press on the button, release elsewhere: no click
    scene.dispatch(Event::PointerDown { x: 15.0, y: 15.0 });
    scene.dispatch(Event::PointerUp { x: 150.0, y: 150.0 });
    assert_eq!(count.get(), 2);
}

#[test]
fn handler_reads_do_not_subscribe() {
    let scene = setup();
    scene.set_rect(scene.root(), Rect::new(0.0, 0.0, 10.0, 10.0));
    let n = state(0);
    let seen = Rc::new(Cell::new(0));
    let s = seen.clone();
    scene.on(scene.root(), EventKind::PointerDown, move |_| s.set(s.get() + n.get()));
    scene.dispatch(Event::PointerDown { x: 1.0, y: 1.0 });
    n.set(5);
    assert_eq!(seen.get(), 0, "no effect re-ran the handler");
}

#[test]
fn hit_testing_respects_order_scroll_and_clip() {
    let scene = setup();
    let root = scene.root();
    scene.set_rect(root, Rect::new(0.0, 0.0, 100.0, 100.0));
    let under = scene.create(root);
    scene.set_rect(under, Rect::new(0.0, 0.0, 50.0, 50.0));
    let over = scene.create(root);
    scene.set_rect(over, Rect::new(25.0, 25.0, 50.0, 50.0));
    assert_eq!(scene.hit_test(30.0, 30.0), Some(over), "later sibling is on top");
    assert_eq!(scene.hit_test(10.0, 10.0), Some(under));
    assert_eq!(scene.hit_test(90.0, 90.0), Some(root));
    assert_eq!(scene.hit_test(500.0, 500.0), None);

    let sc = scene.create(root);
    scene.set_rect(sc, Rect::new(0.0, 60.0, 100.0, 40.0));
    scene.set_style(sc, Style { scroll: true, ..Default::default() });
    let item = scene.create(sc);
    scene.set_rect(item, Rect::new(0.0, 100.0, 100.0, 20.0)); // below the viewport until scrolled
    assert_ne!(scene.hit_test(5.0, 70.0), Some(item));
    scene.set_scroll(sc, 0.0, 100.0);
    assert_eq!(scene.hit_test(5.0, 70.0), Some(item));
    assert_eq!(scene.abs_rect(item).y, 60.0);
    // content scrolled past the clip edge is not hittable outside the scroll box
    scene.set_scroll(sc, 0.0, 0.0);
    scene.set_rect(item, Rect::new(0.0, 0.0, 100.0, 200.0));
    assert_eq!(scene.hit_test(5.0, 99.0), Some(item));
}

#[test]
fn semantic_tree_roles_names_and_actions() {
    let scene = setup();
    let root = scene.root();
    scene.set_rect(root, Rect::new(0.0, 0.0, 100.0, 100.0));
    let group = scene.create(root); // no role: transparent
    let b = button(scene, group, "Save", Rect::new(5.0, 6.0, 40.0, 20.0));
    scene.on(b, EventKind::Click, |_| {});
    let t = scene.create(root);
    scene.set_text(t, TextProps::new("Hello"));
    let tree = scene.semantic_tree();
    assert_eq!(tree.roots.len(), 2, "group hoisted");
    let btn = &tree.roots[0];
    assert_eq!((btn.role, btn.name.as_deref()), (Role::Button, Some("Save")));
    assert!(btn.actions.contains(&Action::Click) && btn.actions.contains(&Action::Focus));
    assert!(btn.children.is_empty(), "button text folded into its name");
    assert_eq!(btn.bounds, Rect::new(5.0, 6.0, 40.0, 20.0));
    assert_eq!((tree.roots[1].role, tree.roots[1].name.as_deref()), (Role::Text, Some("Hello")));
}

#[test]
fn audit_flags_unlabeled_and_role_less_interactive_nodes() {
    let scene = setup();
    let root = scene.root();
    let ok = button(scene, root, "OK", Rect::default());
    scene.on(ok, EventKind::Click, |_| {});
    assert!(scene.audit_semantics().is_empty());
    let bad = scene.create(root);
    scene.on(bad, EventKind::Click, |_| {});
    let nameless = scene.create(root);
    scene.set_semantics(nameless, Semantics { role: Role::Button, ..Default::default() });
    let report = scene.audit_semantics();
    assert_eq!(report.len(), 2, "{report:?}");
    assert!(report.iter().any(|m| m.contains("no role")) && report.iter().any(|m| m.contains("no accessible name")));
}

#[test]
fn batching_state_writes_dirties_each_node_once() {
    let scene = setup();
    let a = scene.create(scene.root());
    let n = state(0);
    scene.bind_text(a, move || format!("{}", n.get()));
    scene.drain_dirty();
    batch(|| {
        n.set(1);
        n.set(2);
        n.set(3);
    });
    assert_eq!(scene.drain_dirty().len(), 1);
}

#[test]
fn node_slots_are_recycled_under_churn() {
    let scene = setup();
    for _ in 0..5_000 {
        let n = scene.create(scene.root());
        let c = scene.create(n);
        scene.bind_text(c, || "x".to_string());
        scene.remove(n);
    }
    assert_eq!(scene.node_count(), 1);
    assert!(scene.slot_count() <= 4, "slots {}", scene.slot_count());
    assert!(wj_reactive::stats().slots < 20, "reactive arena bounded too: {:?}", wj_reactive::stats());
}
