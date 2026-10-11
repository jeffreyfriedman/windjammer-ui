//! web-sys adapter for [`DomPort`] (wasm32 + `web` feature only).

use super::mounted::Mounted;
use super::port::DomPort;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Node};

pub struct WebDom {
    document: Document,
    container: Element,
    /// Set by `listen`; the owner wires closures to `Mounted::dispatch`.
    pub on_event: Option<std::rc::Rc<dyn Fn(Node, String)>>,
    closures: Vec<Closure<dyn FnMut(web_sys::Event)>>,
}

impl WebDom {
    pub fn new(document: Document, container: Element) -> Self {
        WebDom { document, container, on_event: None, closures: Vec::new() }
    }
}

fn is_value_prop(name: &str) -> bool {
    matches!(name, "value" | "checked" | "selected")
}

fn sync_property(el: &Element, name: &str, value: &str) {
    let key = JsValue::from_str(name);
    let current = js_sys::Reflect::get(el, &key).ok();
    if name == "value" {
        // Only write when different: rewriting an equal value moves the caret.
        if current.and_then(|v| v.as_string()).as_deref() != Some(value) {
            let _ = js_sys::Reflect::set(el, &key, &JsValue::from_str(value));
        }
    } else {
        let _ = js_sys::Reflect::set(el, &key, &JsValue::from_bool(true));
    }
}

impl DomPort for WebDom {
    type Node = Node;

    fn container(&self) -> Node {
        self.container.clone().into()
    }
    fn create_element(&mut self, tag: &str) -> Node {
        self.document.create_element(tag).expect("valid tag").into()
    }
    fn create_text(&mut self, text: &str) -> Node {
        self.document.create_text_node(text).into()
    }
    fn create_raw(&mut self, html: &str) -> Node {
        let wrap = self.document.create_element("div").expect("div");
        let _ = wrap.set_attribute("style", "display:contents");
        wrap.set_inner_html(html);
        wrap.into()
    }
    fn set_attr(&mut self, node: &Node, name: &str, value: &str) {
        if let Some(el) = node.dyn_ref::<Element>() {
            let _ = el.set_attribute(name, value);
            if is_value_prop(name) {
                sync_property(el, name, value);
            }
        }
    }
    fn remove_attr(&mut self, node: &Node, name: &str) {
        if let Some(el) = node.dyn_ref::<Element>() {
            let _ = el.remove_attribute(name);
            if name == "checked" {
                let _ = js_sys::Reflect::set(el, &JsValue::from_str("checked"), &JsValue::FALSE);
            }
        }
    }
    fn set_text(&mut self, node: &Node, text: &str) {
        node.set_text_content(Some(text));
    }
    fn child_at(&self, parent: &Node, index: usize) -> Option<Node> {
        parent.child_nodes().item(index as u32)
    }
    fn parent(&self, node: &Node) -> Option<Node> {
        node.parent_node()
    }
    fn index_in_parent(&self, node: &Node) -> Option<usize> {
        let mut i = 0;
        let mut cur = node.previous_sibling();
        while let Some(s) = cur {
            i += 1;
            cur = s.previous_sibling();
        }
        node.parent_node().map(|_| i)
    }
    fn insert_child(&mut self, parent: &Node, index: usize, child: &Node) {
        if child.parent_node().is_some() {
            let _ = child.parent_node().unwrap().remove_child(child);
        }
        let reference = parent.child_nodes().item(index as u32);
        let _ = parent.insert_before(child, reference.as_ref());
    }
    fn remove(&mut self, node: &Node) {
        if let Some(p) = node.parent_node() {
            let _ = p.remove_child(node);
        }
    }
    fn replace(&mut self, old: &Node, new: &Node) {
        if let Some(p) = old.parent_node() {
            let _ = p.replace_child(new, old);
        }
    }
    fn listen(&mut self, event: &str) {
        let Some(cb) = self.on_event.clone() else { return };
        let name = event.to_string();
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Some(t) = e.target().and_then(|t| t.dyn_into::<Node>().ok()) {
                cb(t, name.clone());
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        let opts = web_sys::AddEventListenerOptions::new();
        opts.set_capture(true);
        let _ = self.container.add_event_listener_with_callback_and_add_event_listener_options(
            event,
            closure.as_ref().unchecked_ref(),
            &opts,
        );
        self.closures.push(closure);
    }
}

/// Mount helper: wires delegated events to the shared `Mounted`.
pub fn new_mounted(document: Document, container: Element) -> std::rc::Rc<std::cell::RefCell<Mounted<WebDom>>> {
    use std::cell::RefCell;
    use std::rc::Rc;
    container.set_inner_html(""); // the patcher owns the container's only child
    let mounted = Rc::new(RefCell::new(Mounted::new(WebDom::new(document, container))));
    let weak = Rc::downgrade(&mounted);
    mounted.borrow_mut().port.on_event = Some(Rc::new(move |target: Node, event: String| {
        let Some(m) = weak.upgrade() else { return };
        // Clone handlers out so no borrow is held while user code runs.
        let handlers = m.borrow().handlers_for(&target, &event);
        for h in handlers {
            (h.borrow_mut())();
        }
    }));
    mounted
}
