//! Runs the pure patcher (diff/apply/FakeDom) compiled to wasm32, under node, no browser.
//!
//!   CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
//!     cargo test --target wasm32-unknown-unknown --test wasm_node_test
//!
//! (wasm-bindgen CLI version must match the lockfile; see docs/TESTING_GUIDE.md.)
//! Real-DOM behaviour of `dom_patch::web` needs a browser: `wasm-pack test --headless --chrome`
//! with `wasm_bindgen_test_configure!(run_in_browser)`.
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use windjammer_ui::dom_patch::fake_dom::FakeDom;
use windjammer_ui::dom_patch::Mounted;
use windjammer_ui::simple_vnode::VNode;

fn list(keys: &[&str]) -> VNode {
    VNode::element(
        "ul",
        vec![],
        keys.iter()
            .map(|k| VNode::element("li", vec![("key", windjammer_ui::simple_vnode::VAttr::Static(k.to_string()))], vec![VNode::text(k)]))
            .collect(),
    )
}

#[wasm_bindgen_test]
fn keyed_reorder_reuses_nodes_under_wasm() {
    let mut m = Mounted::new(FakeDom::new());
    m.update(&list(&["a", "b", "c"]));
    let created = m.port.created;
    m.update(&list(&["c", "a", "b"]));
    assert_eq!(m.port.created, created);
}

#[wasm_bindgen_test]
fn escaping_holds_under_wasm() {
    let html = windjammer_ui::simple_renderer::render_to_html(&VNode::text("<script>"));
    assert_eq!(html, "&lt;script&gt;");
}
