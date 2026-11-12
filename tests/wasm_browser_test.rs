//! Behaviour of the real-DOM adapter (`dom_patch::web::WebDom`) in a browser.
//!
//!   wasm-pack test --headless --chrome -- --test wasm_browser_test
//!
//! (needs wasm-pack and Chrome; wasm-pack fetches a matching chromedriver on first use.) Each test mounts a tree into
//! a fresh container in `document.body`, drives real DOM events and focus, and asserts on what the browser did.
#![cfg(all(target_arch = "wasm32", feature = "web"))]

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Document, Element, HtmlElement};
use windjammer_ui::dom_patch::web::new_mounted;
use windjammer_ui::dom_patch::stop_propagation;
use windjammer_ui::simple_vnode::{VAttr, VNode};

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> Document {
    web_sys::window().expect("window").document().expect("document")
}

fn fresh_container() -> Element {
    let doc = document();
    let c = doc.create_element("div").unwrap();
    doc.body().unwrap().append_child(&c).unwrap();
    c
}

fn el(tag: &str, attrs: Vec<(&str, VAttr)>, kids: Vec<VNode>) -> VNode {
    VNode::element(tag, attrs, kids)
}

fn st(v: &str) -> VAttr {
    VAttr::Static(v.into())
}

fn handler(f: impl FnMut() + 'static) -> VAttr {
    VAttr::Event(Rc::new(RefCell::new(f)))
}

fn click(target: &Element) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    let ev = web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    target.dispatch_event(&ev).unwrap();
}

fn inputs(keys: &[&str]) -> VNode {
    el(
        "div",
        vec![],
        keys.iter().map(|k| el("input", vec![("key", st(k)), ("id", st(k))], vec![])).collect(),
    )
}

#[wasm_bindgen_test]
fn focus_survives_a_keyed_move() {
    let container = fresh_container();
    let m = new_mounted(document(), container.clone());
    m.borrow_mut().update(&inputs(&["a", "b", "c"]));
    let b: HtmlElement = document().get_element_by_id("b").unwrap().dyn_into().unwrap();
    b.focus().unwrap();
    assert_eq!(document().active_element().unwrap().id(), "b");

    m.borrow_mut().update(&inputs(&["c", "b", "a"]));

    let b_after = document().get_element_by_id("b").unwrap();
    assert!(b_after == *b.as_ref(), "the keyed node is moved, not rebuilt");
    assert_eq!(document().active_element().unwrap().id(), "b", "focus stays on the moved input");
    let ids: Vec<String> = (0..3).map(|i| container.first_element_child().unwrap().children().item(i).unwrap().id()).collect();
    assert_eq!(ids, ["c", "b", "a"]);
}

#[wasm_bindgen_test]
fn typed_value_and_caret_survive_a_rerender() {
    let container = fresh_container();
    let m = new_mounted(document(), container);
    let page = |label: &str| {
        el("div", vec![], vec![el("h1", vec![], vec![VNode::text(label)]), el("input", vec![("id", st("name")), ("value", st(""))], vec![])])
    };
    m.borrow_mut().update(&page("one"));
    let input: web_sys::HtmlInputElement = document().get_element_by_id("name").unwrap().dyn_into().unwrap();
    input.focus().unwrap();
    input.set_value("hello");
    input.set_selection_range(3, 3).unwrap();

    m.borrow_mut().update(&page("two"));

    assert_eq!(input.value(), "hello", "an unrelated patch keeps what the user typed");
    assert_eq!(input.selection_start().unwrap(), Some(3));
    assert_eq!(document().active_element().unwrap().id(), "name");
}

#[wasm_bindgen_test]
fn a_delegated_click_fires_its_handler_once() {
    let container = fresh_container();
    let m = new_mounted(document(), container.clone());
    let count = Rc::new(RefCell::new(0));
    let c = count.clone();
    let tree = el("div", vec![], vec![el("button", vec![("id", st("go")), ("on_click", handler(move || *c.borrow_mut() += 1))], vec![VNode::text("go")])]);
    m.borrow_mut().update(&tree);
    // Re-rendering must not stack a second listener.
    let c2 = count.clone();
    let tree2 = el("div", vec![], vec![el("button", vec![("id", st("go")), ("on_click", handler(move || *c2.borrow_mut() += 1))], vec![VNode::text("go again")])]);
    m.borrow_mut().update(&tree2);

    click(&document().get_element_by_id("go").unwrap());
    assert_eq!(*count.borrow(), 1);
    click(&document().get_element_by_id("go").unwrap());
    assert_eq!(*count.borrow(), 2);
}

#[wasm_bindgen_test]
fn stop_propagation_skips_ancestor_handlers_in_the_browser() {
    let container = fresh_container();
    let m = new_mounted(document(), container);
    let log = Rc::new(RefCell::new(Vec::<&'static str>::new()));
    let (outer, inner) = (log.clone(), log.clone());
    let tree = el(
        "div",
        vec![("on_click", handler(move || outer.borrow_mut().push("outer")))],
        vec![el(
            "button",
            vec![
                ("id", st("inner")),
                ("on_click", handler(move || {
                    inner.borrow_mut().push("inner");
                    stop_propagation();
                })),
            ],
            vec![],
        )],
    );
    m.borrow_mut().update(&tree);
    click(&document().get_element_by_id("inner").unwrap());
    assert_eq!(*log.borrow(), vec!["inner"]);
}

#[wasm_bindgen_test]
fn untrusted_text_never_becomes_markup() {
    let container = fresh_container();
    let m = new_mounted(document(), container.clone());
    m.borrow_mut().update(&el("p", vec![], vec![VNode::text("<img src=x onerror=alert(1)>")]));
    assert_eq!(container.query_selector("img").unwrap().is_none(), true);
    assert_eq!(container.text_content().unwrap(), "<img src=x onerror=alert(1)>");
}
