# simple_renderer (`src/simple_renderer.rs`)

**Purpose.** `render_to_html(&simple_vnode::VNode) -> String` (first paint, SSR-ish, tests)
and a legacy console renderer for `vdom::VNode`.

**Invariants.** Output is escaped by default: text and attribute values escaped, URL
attributes neutralized, hostile tags/attribute names dropped, event attrs not serialized,
void elements have no closing tag. Only `VNode::RawHtml` is emitted verbatim.
**Owns.** Serialization. **Never decides.** What counts as safe (`escape`).
**Allowed imports.** `simple_vnode`, `escape`, legacy `vdom` (console renderer only).
**Entry points.** `render_to_html`, `is_void_tag`.
**Tests.** `render_to_html_tests`, `xss_tests` in the same file.
