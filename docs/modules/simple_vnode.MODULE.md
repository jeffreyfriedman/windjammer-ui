# simple_vnode (`src/simple_vnode.rs`)

**Purpose.** The one canonical VNode (`Element | Text | RawHtml`, attrs `Static | Dynamic |
Event`). Legacy `vdom::VNode` remains only for the Component/Renderer/SSR/platform layer.

**Inputs.** Built by components, `to_vnode`, `vnode_ffi`.
**Outputs.** Trees for `render_to_html` and `dom_patch`.

**Invariants.**
- `sanitized()` is idempotent and is applied before every serialize/patch: invalid tags
  become empty text (keeps child indices stable), invalid names and inline `on*` dropped,
  URL attributes sanitized, duplicate attributes collapse (last wins).
- Static `on_*` attributes are dropped; handlers must be `VAttr::Event`.
- `key()` reads the `key` attribute for keyed reconciliation.
- `RawHtml` is passed through untouched; construct via `VNode::raw(RawHtml)`.

**Owns.** Node shape and sanitization entry point. **Never decides.** Escaping rules
(`escape`), diffing (`dom_patch`). **Allowed imports.** std, `escape`; wasm32 `render`
uses wasm-bindgen/web-sys (legacy direct-DOM path, unused by the app loop).
**Entry points.** `VNode::{element,text,raw,raw_html,key,sanitized}`.
**Tests.** `src/simple_vnode.rs`, `src/simple_renderer.rs`, `tests/dom_patch_test.rs`.
