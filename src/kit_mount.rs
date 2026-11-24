//! Browser mount for kit components (`src/components_wj/a11y.wj`): wires a model that implements `Interactive`
//! to a container, delivering `keydown`, `click`, `input`, `change` and `focusout` as `(event, act, key, value)`.
//!
//! ```ignore
//! let kit = kit_mount::mount(document, container, TabStrip::new("docs").tab("One", "First").tab("Two", "Second"));
//! kit.with_model(|m| m.selected_index());
//! ```
//!
//! After every event the model's `view()` is patched into the DOM (focus, caret and typed text survive), then focus
//! moves to the model's `focus_id()` when the event was handled. Handled events have their native default cancelled,
//! so arrow keys and Space do not scroll the page.
#![cfg(all(target_arch = "wasm32", feature = "web"))]

use crate::components::generated::a11y::Interactive;
use crate::dom_patch::web::{new_mounted, WebDom};
use crate::dom_patch::{current_event, prevent_default, Mounted};
use crate::simple_vnode::{VAttr, VNode};
use crate::vnode_ffi::vnode_take;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

/// Events a kit listens to. `focusout` lets fields validate when the user leaves them.
pub const KIT_EVENTS: [&str; 7] = [
    "keydown", "click", "input", "change", "focusin", "focusout", "scroll",
];

pub struct KitMount<M: Interactive> {
    model: RefCell<M>,
    mounted: Rc<RefCell<Mounted<WebDom>>>,
    document: Document,
}

impl<M: Interactive + 'static> KitMount<M> {
    /// Read or inspect the model (for example to ask a form for its values).
    pub fn with_model<R>(&self, f: impl FnOnce(&mut M) -> R) -> R {
        f(&mut self.model.borrow_mut())
    }

    /// Patch the DOM from the model's current view.
    pub fn render(self: &Rc<Self>) {
        let tree = self.tree();
        self.mounted.borrow_mut().update(&tree);
    }

    fn tree(self: &Rc<Self>) -> VNode {
        let inner =
            vnode_take(self.model.borrow().view().raw_handle()).expect("view produced a node");
        let mut attrs: Vec<(String, VAttr)> =
            vec![("style".into(), VAttr::Static("display:contents".into()))];
        for ev in KIT_EVENTS {
            let weak: Weak<Self> = Rc::downgrade(self);
            let handler: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(move || {
                if let Some(kit) = weak.upgrade() {
                    kit.deliver(ev);
                }
            }));
            attrs.push((format!("on_{ev}"), VAttr::Event(handler)));
        }
        VNode::Element {
            tag: "div".into(),
            attrs,
            children: vec![inner],
        }
    }

    fn deliver(self: &Rc<Self>, event: &str) {
        let info = current_event();
        let (handled, focus) = {
            let mut model = self.model.borrow_mut();
            let handled = model.on_event(event.to_string(), info.act, info.key, info.value);
            (handled, model.focus_id())
        };
        if handled {
            prevent_default();
        }
        self.render();
        if handled && !focus.is_empty() {
            if let Some(el) = self.document.get_element_by_id(&focus) {
                let already = self
                    .document
                    .active_element()
                    .map(|a| a == el)
                    .unwrap_or(false);
                if !already {
                    if let Ok(h) = el.dyn_into::<HtmlElement>() {
                        let _ = h.focus();
                    }
                }
            }
        }
    }
}

/// Mount `model` into `container` and render it.
pub fn mount<M: Interactive + 'static>(
    document: Document,
    container: Element,
    model: M,
) -> Rc<KitMount<M>> {
    let mounted = new_mounted(document.clone(), container);
    let kit = Rc::new(KitMount {
        model: RefCell::new(model),
        mounted,
        document,
    });
    kit.render();
    kit
}
