//! Shared helpers for the kit DOM tests: mount a model into a fresh container, drive real DOM events, and read back
//! ARIA attributes, focus and tab order the way assistive technology and the keyboard would see them.
#![cfg(all(target_arch = "wasm32", feature = "web"))]
#![allow(dead_code)]

use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};
use windjammer_ui::components::generated::a11y::Interactive;
use windjammer_ui::kit_mount::{mount, KitMount};

pub fn document() -> Document {
    web_sys::window().expect("window").document().expect("document")
}

pub fn fresh_container() -> Element {
    let doc = document();
    let c = doc.create_element("div").unwrap();
    doc.body().unwrap().append_child(&c).unwrap();
    c
}

/// Mount `model` and return the handle plus its container.
pub fn mount_kit<M: Interactive + 'static>(model: M) -> (Rc<KitMount<M>>, Element) {
    let container = fresh_container();
    (mount(document(), container.clone(), model), container)
}

pub fn by_id(id: &str) -> Element {
    document().get_element_by_id(id).unwrap_or_else(|| panic!("no element with id `{id}`"))
}

pub fn attr(id: &str, name: &str) -> Option<String> {
    by_id(id).get_attribute(name)
}

pub fn exists(id: &str) -> bool {
    document().get_element_by_id(id).is_some()
}

pub fn active_id() -> String {
    document().active_element().map(|e| e.id()).unwrap_or_default()
}

pub fn focus(id: &str) {
    by_id(id).dyn_into::<HtmlElement>().unwrap().focus().unwrap();
}

/// Dispatch a `keydown` for `key` on `id`; returns true when a handler cancelled the default action.
pub fn press(id: &str, key: &str) -> bool {
    let (key, shift) = match key.strip_prefix("Shift+") {
        Some(k) => (k, true),
        None => (key, false),
    };
    let init = web_sys::KeyboardEventInit::new();
    init.set_key(key);
    init.set_shift_key(shift);
    init.set_bubbles(true);
    init.set_cancelable(true);
    let ev = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    !by_id(id).dispatch_event(&ev).unwrap()
}

pub fn click(id: &str) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    let ev = web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    by_id(id).dispatch_event(&ev).unwrap();
}

/// Fire a bubbling event of `name` (`input`, `change`, `focusin`, `focusout`, `scroll`).
pub fn fire(id: &str, name: &str) {
    let init = web_sys::EventInit::new();
    init.set_bubbles(true);
    let ev = web_sys::Event::new_with_event_init_dict(name, &init).unwrap();
    by_id(id).dispatch_event(&ev).unwrap();
}

/// Type into a text-like control the way the browser does: set the value, then fire `input`.
pub fn type_into(id: &str, text: &str) {
    js_sys::Reflect::set(&by_id(id), &"value".into(), &text.into()).unwrap();
    fire(id, "input");
}

/// Choose `value` in a select (or radio value): set it, fire `input` and `change`.
pub fn choose(id: &str, value: &str) {
    js_sys::Reflect::set(&by_id(id), &"value".into(), &value.into()).unwrap();
    fire(id, "input");
    fire(id, "change");
}

/// Ids of the elements a Tab key press could land on, in document order: natively focusable or with a
/// non-negative `tabindex`, not disabled, not inside a `hidden` subtree.
pub fn tab_order(container: &Element) -> Vec<String> {
    let list = container.query_selector_all("button, input, select, textarea, a[href], [tabindex]").unwrap();
    let mut out = Vec::new();
    for i in 0..list.length() {
        let el: Element = list.item(i).unwrap().dyn_into().unwrap();
        if el.has_attribute("disabled") || el.closest("[hidden]").unwrap().is_some() {
            continue;
        }
        if let Some(t) = el.get_attribute("tabindex") {
            if t.parse::<i32>().map(|n| n < 0).unwrap_or(false) {
                continue;
            }
        }
        out.push(el.id());
    }
    out
}

pub fn text_of(id: &str) -> String {
    by_id(id).text_content().unwrap_or_default()
}

/// The accessible name an `aria-labelledby` reference resolves to.
pub fn labelled_by_text(id: &str) -> String {
    let refs = attr(id, "aria-labelledby").unwrap_or_else(|| panic!("`{id}` has no aria-labelledby"));
    refs.split_whitespace().map(text_of).collect::<Vec<_>>().join(" ")
}

pub fn described_by_text(id: &str) -> String {
    let refs = attr(id, "aria-describedby").unwrap_or_else(|| panic!("`{id}` has no aria-describedby"));
    refs.split_whitespace().map(text_of).collect::<Vec<_>>().join(" ")
}
