//! Diff/patch behavior: pure oracle (property test), keyed reorder, and the fake DOM.

use std::cell::RefCell;
use std::rc::Rc;
use windjammer_ui::dom_patch::apply::{apply, canon};
use windjammer_ui::dom_patch::fake_dom::FakeDom;
use windjammer_ui::dom_patch::{diff, prevent_default, stop_propagation, DomPort, Mounted, PatchOp};
use windjammer_ui::simple_renderer::render_to_html;
use windjammer_ui::simple_vnode::{VAttr, VNode};

fn st(v: &str) -> VAttr {
    VAttr::Static(v.into())
}
fn el(tag: &str, attrs: Vec<(&str, VAttr)>, kids: Vec<VNode>) -> VNode {
    VNode::element(tag, attrs, kids)
}
fn keyed(tag: &str, key: &str, kids: Vec<VNode>) -> VNode {
    el(tag, vec![("key", st(key))], kids)
}
fn li(key: &str) -> VNode {
    keyed("li", key, vec![VNode::text(key)])
}
fn ul(keys: &[&str]) -> VNode {
    el("ul", vec![], keys.iter().map(|k| li(k)).collect())
}

fn roundtrip(a: &VNode, b: &VNode) -> Vec<PatchOp> {
    let (a, b) = (a.sanitized(), b.sanitized());
    let ops = diff(&a, &b);
    let mut t = a.clone();
    apply(&mut t, &ops);
    assert_eq!(canon(&t), canon(&b));
    ops
}

#[test]
fn identical_trees_produce_no_ops() {
    assert!(roundtrip(&ul(&["a", "b"]), &ul(&["a", "b"])).is_empty());
}

#[test]
fn text_change_is_a_single_set_text() {
    let ops = roundtrip(&el("p", vec![], vec![VNode::text("a")]), &el("p", vec![], vec![VNode::text("b")]));
    assert!(matches!(ops.as_slice(), [PatchOp::SetText { .. }]));
}

#[test]
fn attr_add_change_remove_are_minimal() {
    let a = el("div", vec![("class", st("x")), ("id", st("1"))], vec![]);
    let b = el("div", vec![("class", st("y")), ("title", st("t"))], vec![]);
    let ops = roundtrip(&a, &b);
    assert_eq!(ops.len(), 3);
    assert!(ops.iter().any(|o| matches!(o, PatchOp::RemoveAttr { name, .. } if name == "id")));
    assert!(ops.iter().any(|o| matches!(o, PatchOp::SetAttr { name, value, .. } if name == "class" && value == "y")));
}

#[test]
fn tag_change_replaces_node() {
    let ops = roundtrip(&el("div", vec![], vec![]), &el("span", vec![], vec![]));
    assert!(matches!(ops.as_slice(), [PatchOp::Replace { .. }]));
}

#[test]
fn keyed_reorder_uses_moves_not_rebuilds() {
    let ops = roundtrip(&ul(&["a", "b", "c", "d"]), &ul(&["d", "a", "b", "c"]));
    assert!(ops.iter().all(|o| matches!(o, PatchOp::Move { .. })), "only moves expected");
    assert_eq!(ops.len(), 1);
}

#[test]
fn keyed_insert_and_remove_in_middle() {
    let ops = roundtrip(&ul(&["a", "b", "c"]), &ul(&["a", "x", "c"]));
    assert_eq!(ops.iter().filter(|o| matches!(o, PatchOp::Insert { .. })).count(), 1);
    assert_eq!(ops.iter().filter(|o| matches!(o, PatchOp::Remove { .. })).count(), 1);
}

#[test]
fn unkeyed_lists_patch_positionally() {
    let mk = |n: usize| el("div", vec![], (0..n).map(|i| el("p", vec![], vec![VNode::text(&i.to_string())])).collect());
    roundtrip(&mk(5), &mk(2));
    roundtrip(&mk(2), &mk(5));
}

#[test]
fn raw_html_change_replaces_wrapper() {
    let ops = roundtrip(
        &el("div", vec![], vec![VNode::raw_html("<b>1</b>")]),
        &el("div", vec![], vec![VNode::raw_html("<b>2</b>")]),
    );
    assert!(matches!(ops.as_slice(), [PatchOp::Replace { .. }]));
}

// ---- property test (deterministic xorshift, no external crates) ----

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

fn gen_node(r: &mut Rng, depth: usize) -> VNode {
    match r.below(if depth == 0 { 3 } else { 6 }) {
        0 => VNode::text(["a", "b", "<x>", "日本"][r.below(4)]),
        1 => VNode::raw_html(["<i>1</i>", "<i>2</i>"][r.below(2)]),
        2 | 3 | 4 | 5 => gen_element(r, depth),
        _ => unreachable!(),
    }
}

fn gen_element(r: &mut Rng, depth: usize) -> VNode {
    let tag = ["div", "span", "ul", "script", "br"][r.below(5)];
    let mut attrs = Vec::new();
    for name in ["class", "id", "href", "onclick", "key", "title"] {
        if r.below(3) == 0 {
            let v = ["x", "y", "javascript:alert(1)", "k1", "k2", "k3", "\"q"][r.below(7)];
            attrs.push((name, st(v)));
        }
    }
    let kids = (0..r.below(6)).map(|_| gen_node(r, depth.saturating_sub(1))).collect();
    el(tag, attrs, kids)
}

/// Mutate a tree lightly so old/new share structure (the realistic case).
fn mutate(r: &mut Rng, v: &VNode) -> VNode {
    match v {
        VNode::Element { tag, attrs, children } if r.below(4) != 0 => {
            let mut kids: Vec<VNode> = children.iter().map(|c| mutate(r, c)).collect();
            match r.below(4) {
                0 if kids.len() > 1 => {
                    let last = kids.len() - 1;
                    kids.swap(0, last);
                }
                1 => {
                    let at = r.below(kids.len() + 1);
                    kids.insert(at, gen_node(r, 1));
                }
                2 if !kids.is_empty() => {
                    let at = r.below(kids.len());
                    kids.remove(at);
                }
                3 if kids.len() > 2 => kids.reverse(),
                _ => {}
            }
            let mut attrs = attrs.clone();
            if r.below(3) == 0 {
                attrs.push(("class".to_string(), st(["p", "q"][r.below(2)])));
            }
            VNode::Element { tag: tag.clone(), attrs, children: kids }
        }
        VNode::Text(_) if r.below(2) == 0 => VNode::text("changed"),
        other => if r.below(8) == 0 { gen_node(r, 2) } else { other.clone() },
    }
}

#[test]
fn property_apply_diff_equals_target_pure_and_on_fake_dom() {
    let mut r = Rng(0x9E3779B97F4A7C15);
    for case in 0..1500 {
        let a = gen_element(&mut r, 3);
        let b = if case % 3 == 0 { gen_element(&mut r, 3) } else { mutate(&mut r, &a) };
        roundtrip(&a, &b);

        let mut m = Mounted::new(FakeDom::new());
        m.update(&a);
        m.update(&b);
        assert_eq!(m.port.snapshot(), Some(canon(&b.sanitized())), "case {case}");
    }
}

// ---- fake DOM behavior ----

#[test]
fn update_reuses_existing_nodes() {
    let mut m = Mounted::new(FakeDom::new());
    m.update(&ul(&["a", "b", "c"]));
    let created = m.port.created;
    m.update(&ul(&["c", "a", "b"]));
    assert_eq!(m.port.created, created, "reorder must not create nodes");
    assert_eq!(m.port.snapshot(), Some(canon(&ul(&["c", "a", "b"]))));
}

fn input_page(value: &str, label: &str) -> VNode {
    el("div", vec![], vec![
        el("h1", vec![], vec![VNode::text(label)]),
        el("input", vec![("id", st("name")), ("value", st(value))], vec![]),
    ])
}

#[test]
fn focus_caret_and_typed_value_survive_rerender() {
    let mut m = Mounted::new(FakeDom::new());
    m.update(&input_page("", "Hi"));
    let input = m.port.find_by_attr("id", "name").unwrap();
    m.port.focus(input);
    m.port.type_value(input, "hello", 3);
    // Unrelated change elsewhere in the tree (value attr still stale "").
    m.update(&input_page("", "Hi again"));
    assert_eq!(m.port.focused(), Some(input));
    assert_eq!(m.port.find_by_attr("id", "name"), Some(input), "same node");
    assert_eq!(m.port.value(input).as_deref(), Some("hello"));
    assert_eq!(m.port.caret(input), 3);
    // Controlled input echoing what was typed: no caret jump.
    m.update(&input_page("hello", "Hi again"));
    assert_eq!(m.port.caret(input), 3);
}

#[test]
fn focused_node_survives_keyed_move() {
    let page = |keys: &[&str]| el("div", vec![], keys.iter().map(|k| keyed("input", k, vec![])).collect());
    let mut m = Mounted::new(FakeDom::new());
    m.update(&page(&["a", "b", "c"]));
    let b = m.port.find_by_attr("key", "b").unwrap();
    m.port.focus(b);
    m.update(&page(&["c", "b", "a"]));
    assert_eq!(m.port.focused(), Some(b));
    assert_eq!(m.port.find_by_attr("key", "b"), Some(b));
}

fn counter(n: Rc<RefCell<i32>>, label: &str) -> VNode {
    let h: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(move || *n.borrow_mut() += 1));
    el("div", vec![], vec![el("button", vec![("on_click", VAttr::Event(h))], vec![VNode::text(label)])])
}

#[test]
fn delegated_handlers_survive_patches_and_use_latest_closure() {
    let (n1, n2) = (Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)));
    let mut m = Mounted::new(FakeDom::new());
    m.update(&counter(n1.clone(), "one"));
    assert!(m.port.listening.contains("click"));
    let btn = m.port.child_at(&m.port.child_at(&m.port.container(), 0).unwrap(), 0).unwrap();
    assert_eq!(m.dispatch(&btn, "click"), 1);
    m.update(&counter(n2.clone(), "two"));
    let btn2 = m.port.child_at(&m.port.child_at(&m.port.container(), 0).unwrap(), 0).unwrap();
    assert_eq!(btn, btn2, "button node reused");
    m.dispatch(&btn2, "click");
    assert_eq!((*n1.borrow(), *n2.borrow()), (1, 1));
}

#[test]
fn events_bubble_to_ancestor_handlers_innermost_first() {
    let log = Rc::new(RefCell::new(Vec::<&'static str>::new()));
    let mk = |tag: &'static str| -> VAttr {
        let l = log.clone();
        VAttr::Event(Rc::new(RefCell::new(move || l.borrow_mut().push(tag))))
    };
    let tree = el("div", vec![("on_click", mk("outer"))], vec![el("span", vec![("on_click", mk("inner"))], vec![])]);
    let mut m = Mounted::new(FakeDom::new());
    m.update(&tree);
    let span = m.port.child_at(&m.port.child_at(&m.port.container(), 0).unwrap(), 0).unwrap();
    m.dispatch(&span, "click");
    assert_eq!(*log.borrow(), vec!["inner", "outer"]);
}

#[test]
fn handlers_follow_their_nodes_through_keyed_moves() {
    let hits = Rc::new(RefCell::new(Vec::<String>::new()));
    let page = |keys: &[&str]| {
        el("div", vec![], keys.iter().map(|k| {
            let (h, k2) = (hits.clone(), k.to_string());
            let cb: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(move || h.borrow_mut().push(k2.clone())));
            el("button", vec![("key", st(k)), ("on_click", VAttr::Event(cb))], vec![])
        }).collect())
    };
    let mut m = Mounted::new(FakeDom::new());
    m.update(&page(&["a", "b"]));
    m.update(&page(&["b", "a"]));
    let a = m.port.find_by_attr("key", "a").unwrap();
    m.dispatch(&a, "click");
    assert_eq!(*hits.borrow(), vec!["a".to_string()]);
}

#[test]
fn xss_payloads_never_reach_the_dom_through_patching() {
    let page = |href: &str, text: &str| {
        el("div", vec![], vec![
            el("a", vec![("href", st(href)), ("onclick", st("alert(1)"))], vec![VNode::text(text)]),
            el("script", vec![], vec![VNode::text("alert(1)")]),
        ])
    };
    let mut m = Mounted::new(FakeDom::new());
    m.update(&page("https://ok.test", "x"));
    m.update(&page("javascript:alert(1)", "<img src=x onerror=alert(1)>"));
    let snap = format!("{:?}", m.port.snapshot().unwrap());
    assert!(!snap.contains("javascript:"), "{snap}");
    assert!(!snap.contains("onclick") && !snap.contains("script"), "{snap}");
    // Text stays literal text (the DOM never parses it), and the string renderer escapes it.
    assert!(render_to_html(&page("x", "<img src=x onerror=alert(1)>")).contains("&lt;img"));
}

// --- focus across moves (browser reparenting blurs; the patcher must restore) ---

fn input_list(keys: &[&str]) -> VNode {
    el("div", vec![], keys.iter().map(|k| keyed("input", k, vec![])).collect())
}

#[test]
fn reparenting_blurs_in_the_fake_dom_like_a_browser() {
    let mut m = Mounted::new(FakeDom::new());
    m.update(&input_list(&["a", "b"]));
    let a = m.port.find_by_attr("key", "a").unwrap();
    m.port.focus(a);
    let root = m.port.child_at(&m.port.container(), 0).unwrap();
    m.port.insert_child(&root, 1, &a); // raw reparent, no patcher
    assert_eq!(m.port.focused(), None, "model must reproduce the browser's blur-on-move");
}

#[test]
fn keyed_move_restores_focus_and_selection_without_atomic_move() {
    let mut m = Mounted::new(FakeDom::new());
    m.update(&input_list(&["a", "b", "c"]));
    let b = m.port.find_by_attr("key", "b").unwrap();
    m.port.focus(b);
    m.port.type_value(b, "hello", 2);
    m.update(&input_list(&["c", "b", "a"]));
    m.update(&input_list(&["b", "a", "c"]));
    assert_eq!(m.port.focused(), Some(b));
    assert_eq!(m.port.caret(b), 2);
    assert_eq!(m.port.value(b).as_deref(), Some("hello"));
    assert_eq!(m.port.atomic_moves, 0);
}

#[test]
fn keyed_move_prefers_atomic_move_before_when_the_host_has_it() {
    let mut dom = FakeDom::new();
    dom.supports_move_before = true;
    let mut m = Mounted::new(dom);
    m.update(&input_list(&["a", "b", "c"]));
    let a = m.port.find_by_attr("key", "a").unwrap();
    m.port.focus(a);
    m.update(&input_list(&["b", "c", "a"]));
    assert_eq!(m.port.focused(), Some(a));
    assert!(m.port.atomic_moves > 0, "should use moveBefore semantics");
}

#[test]
fn moving_an_ancestor_of_the_focused_node_keeps_focus() {
    let page = |keys: &[&str]| {
        el("div", vec![], keys.iter().map(|k| keyed("div", k, vec![el("input", vec![("id", st(&format!("in-{k}")))], vec![])])).collect())
    };
    let mut m = Mounted::new(FakeDom::new());
    m.update(&page(&["a", "b"]));
    let inp = m.port.find_by_attr("id", "in-a").unwrap();
    m.port.focus(inp);
    m.update(&page(&["b", "a"]));
    assert_eq!(m.port.focused(), Some(inp));
}

#[test]
fn removing_the_focused_node_still_blurs() {
    let mut m = Mounted::new(FakeDom::new());
    m.update(&input_list(&["a", "b"]));
    let b = m.port.find_by_attr("key", "b").unwrap();
    m.port.focus(b);
    m.update(&input_list(&["a"]));
    assert_eq!(m.port.focused(), None);
}

// --- stop_propagation / prevent_default ---

fn nested_click_tree(log: &Rc<RefCell<Vec<&'static str>>>, inner_action: fn()) -> VNode {
    let mk = |tag: &'static str, act: Option<fn()>| -> VAttr {
        let l = log.clone();
        VAttr::Event(Rc::new(RefCell::new(move || {
            l.borrow_mut().push(tag);
            if let Some(f) = act { f() }
        })))
    };
    el("div", vec![("on_click", mk("outer", None))], vec![
        el("button", vec![("on_click", mk("inner", Some(inner_action)))], vec![]),
    ])
}

#[test]
fn stop_propagation_in_a_handler_skips_ancestor_handlers() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut m = Mounted::new(FakeDom::new());
    m.update(&nested_click_tree(&log, stop_propagation));
    let btn = m.port.child_at(&m.port.child_at(&m.port.container(), 0).unwrap(), 0).unwrap();
    let out = m.dispatch_event(&btn, "click");
    assert_eq!(*log.borrow(), vec!["inner"]);
    assert!(out.propagation_stopped && !out.default_prevented);
    assert_eq!(out.handled, 1);
}

#[test]
fn prevent_default_is_reported_and_does_not_stop_bubbling() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut m = Mounted::new(FakeDom::new());
    m.update(&nested_click_tree(&log, prevent_default));
    let btn = m.port.child_at(&m.port.child_at(&m.port.container(), 0).unwrap(), 0).unwrap();
    let out = m.dispatch_event(&btn, "click");
    assert_eq!(*log.borrow(), vec!["inner", "outer"]);
    assert!(out.default_prevented && !out.propagation_stopped);
}

#[test]
fn propagation_flags_do_not_leak_between_dispatches() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut m = Mounted::new(FakeDom::new());
    m.update(&nested_click_tree(&log, stop_propagation));
    let btn = m.port.child_at(&m.port.child_at(&m.port.container(), 0).unwrap(), 0).unwrap();
    m.dispatch_event(&btn, "click");
    let div = m.port.child_at(&m.port.container(), 0).unwrap();
    let out = m.dispatch_event(&div, "click"); // outer only, handler does not stop
    assert!(!out.propagation_stopped);
    assert_eq!(*log.borrow(), vec!["inner", "outer"]);
}
