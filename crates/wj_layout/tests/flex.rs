//! Layout fixtures with expected rectangles worked out by hand from the CSS flexbox spec
//! (independent of Taffy), plus the text-measure seam and absolute/scroll behaviour.
use wj_layout::*;
use wj_scene::*;

fn run(scene: Scene, vp: (f32, f32)) -> LayoutEngine {
    let mut e = LayoutEngine::new();
    let d = scene.drain_dirty();
    e.update(scene, &d, vp, &FixedAdvanceMeasure);
    e
}

fn boxed(scene: Scene, parent: NodeId, w: Dim, h: Dim) -> NodeId {
    let n = scene.create(parent);
    scene.set_style(n, Style { width: w, height: h, ..Default::default() });
    n
}

fn r(scene: Scene, n: NodeId) -> (f32, f32, f32, f32) {
    let x = scene.rect(n);
    (x.x, x.y, x.w, x.h)
}

#[test]
fn row_with_gap_and_padding() {
    let s = Scene::new();
    let root = s.root();
    s.set_style(root, Style { direction: Direction::Row, padding: Edges::all(10.0), gap: 5.0, align_items: Align::Start, ..Default::default() });
    let a = boxed(s, root, Dim::Px(30.0), Dim::Px(20.0));
    let b = boxed(s, root, Dim::Px(40.0), Dim::Px(10.0));
    run(s, (200.0, 100.0));
    assert_eq!(r(s, root), (0.0, 0.0, 200.0, 100.0));
    assert_eq!(r(s, a), (10.0, 10.0, 30.0, 20.0));
    assert_eq!(r(s, b), (45.0, 10.0, 40.0, 10.0)); // 10 + 30 + 5
}

#[test]
fn flex_grow_distributes_free_space_in_proportion() {
    let s = Scene::new();
    let root = s.root();
    s.set_style(root, Style { direction: Direction::Row, align_items: Align::Start, ..Default::default() });
    let a = s.create(root);
    s.set_style(a, Style { width: Dim::Px(50.0), height: Dim::Px(10.0), flex_grow: 1.0, ..Default::default() });
    let b = s.create(root);
    s.set_style(b, Style { width: Dim::Px(50.0), height: Dim::Px(10.0), flex_grow: 3.0, ..Default::default() });
    run(s, (300.0, 50.0));
    // free = 300 - 100 = 200 -> a += 50, b += 150
    assert_eq!(r(s, a), (0.0, 0.0, 100.0, 10.0));
    assert_eq!(r(s, b), (100.0, 0.0, 200.0, 10.0));
}

#[test]
fn flex_shrink_when_content_overflows() {
    let s = Scene::new();
    let root = s.root();
    s.set_style(root, Style { direction: Direction::Row, align_items: Align::Start, ..Default::default() });
    let a = boxed(s, root, Dim::Px(150.0), Dim::Px(10.0));
    let b = boxed(s, root, Dim::Px(150.0), Dim::Px(10.0));
    run(s, (200.0, 50.0));
    assert_eq!(r(s, a).2, 100.0);
    assert_eq!(r(s, b), (100.0, 0.0, 100.0, 10.0));
}

#[test]
fn justify_and_align_variants() {
    let s = Scene::new();
    let root = s.root();
    s.set_style(root, Style { direction: Direction::Row, justify: Justify::SpaceBetween, align_items: Align::Center, ..Default::default() });
    let a = boxed(s, root, Dim::Px(20.0), Dim::Px(20.0));
    let b = boxed(s, root, Dim::Px(20.0), Dim::Px(40.0));
    run(s, (100.0, 100.0));
    assert_eq!(r(s, a), (0.0, 40.0, 20.0, 20.0));
    assert_eq!(r(s, b), (80.0, 30.0, 20.0, 40.0));
    s.set_style(root, Style { direction: Direction::Row, justify: Justify::Center, align_items: Align::End, ..Default::default() });
    let mut e = LayoutEngine::new();
    let d = s.drain_dirty();
    e.update(s, &d, (100.0, 100.0), &FixedAdvanceMeasure);
    assert_eq!(r(s, a), (30.0, 80.0, 20.0, 20.0));
    assert_eq!(r(s, b), (50.0, 60.0, 20.0, 40.0));
}

#[test]
fn percent_margin_and_column_stretch() {
    let s = Scene::new();
    let root = s.root();
    let a = s.create(root);
    s.set_style(a, Style { height: Dim::Px(10.0), margin: Edges { left: 5.0, right: 15.0, top: 2.0, bottom: 3.0 }, ..Default::default() });
    let b = boxed(s, root, Dim::Pct(50.0), Dim::Px(10.0));
    run(s, (200.0, 100.0));
    assert_eq!(r(s, a), (5.0, 2.0, 180.0, 10.0)); // stretched across 200 minus margins
    assert_eq!(r(s, b), (0.0, 15.0, 100.0, 10.0)); // 2 + 10 + 3
}

#[test]
fn text_measures_through_the_seam_and_wraps() {
    let s = Scene::new();
    let root = s.root();
    s.set_style(root, Style { align_items: Align::Start, ..Default::default() });
    let t = s.create(root);
    let mut tp = TextProps::new("hello"); // 5 chars * 0.5 * 10
    tp.font_size = 10.0;
    s.set_text(t, tp);
    let w = s.create(root);
    let mut tp = TextProps::new("aaaa bbbb cccc"); // 14 chars = 70 wide unwrapped
    tp.font_size = 10.0;
    s.set_text(w, tp);
    s.set_style(w, Style { width: Dim::Px(40.0), ..Default::default() });
    run(s, (200.0, 200.0));
    assert_eq!(r(s, t), (0.0, 0.0, 25.0, 12.0));
    // width fixed at 40: "aaaa" (20), "bbbb" fits? 20+5+20 = 45 > 40, so one word per line: 3 lines
    assert_eq!((r(s, w).2, r(s, w).3), (40.0, 36.0));
}

#[test]
fn absolute_children_do_not_take_flow_space() {
    let s = Scene::new();
    let root = s.root();
    let flow = boxed(s, root, Dim::Px(50.0), Dim::Px(10.0));
    let abs = s.create(root);
    s.set_style(abs, Style { width: Dim::Px(20.0), height: Dim::Px(20.0), absolute: Some((7.0, 30.0)), ..Default::default() });
    let after = boxed(s, root, Dim::Px(50.0), Dim::Px(10.0));
    run(s, (100.0, 100.0));
    assert_eq!(r(s, flow).1, 0.0);
    assert_eq!(r(s, abs), (7.0, 30.0, 20.0, 20.0));
    assert_eq!(r(s, after).1, 10.0, "absolute sibling is out of flow");
}

#[test]
fn scroll_container_keeps_its_size_and_lets_content_overflow() {
    let s = Scene::new();
    let root = s.root();
    let sc = s.create(root);
    s.set_style(sc, Style { height: Dim::Px(50.0), scroll: true, ..Default::default() });
    for _ in 0..10 {
        let row = s.create(sc);
        // CSS semantics: flex items shrink unless told not to; scroll content opts out.
        s.set_style(row, Style { height: Dim::Px(20.0), flex_shrink: 0.0, ..Default::default() });
    }
    run(s, (100.0, 200.0));
    assert_eq!(r(s, sc).3, 50.0);
    let kids = s.children(sc);
    assert_eq!(r(s, kids[9]), (0.0, 180.0, 100.0, 20.0), "children keep their size and overflow the box");
}

#[test]
fn virtual_list_rows_land_at_row_height_multiples() {
    let s = Scene::new();
    let root = s.root();
    let list = s.create(root);
    let scroll = wj_reactive::state(0.0f32);
    s.virtual_list(list, 20.0, 0, || 1000, move || scroll.get(), || 100.0, move |i| {
        let n = s.create_detached();
        let mut t = TextProps::new(format!("row {i}"));
        t.font_size = 10.0;
        s.set_text(n, t);
        n
    });
    let mut e = LayoutEngine::new();
    let d = s.drain_dirty();
    e.update(s, &d, (100.0, 100.0), &FixedAdvanceMeasure);
    let kids = s.children(list);
    assert_eq!(kids.len(), 5);
    for (i, k) in kids.iter().enumerate() {
        assert_eq!(r(s, *k), (0.0, 20.0 * i as f32, 100.0, 20.0));
    }
    assert_eq!(r(s, list).3, 20_000.0);
    scroll.set(2000.0);
    let d = s.drain_dirty();
    e.update(s, &d, (100.0, 100.0), &FixedAdvanceMeasure);
    let kids = s.children(list);
    assert_eq!(r(s, kids[0]).1, 2000.0);
    assert_eq!(e.mirrored(), s.node_count(), "taffy mirror has no leaked nodes");
}

#[test]
fn nothing_dirty_means_nothing_recomputed() {
    let s = Scene::new();
    let root = s.root();
    boxed(s, root, Dim::Px(10.0), Dim::Px(10.0));
    let mut e = LayoutEngine::new();
    let d = s.drain_dirty();
    e.update(s, &d, (100.0, 100.0), &FixedAdvanceMeasure);
    assert_eq!(e.update(s, &[], (100.0, 100.0), &FixedAdvanceMeasure), 0);
}
