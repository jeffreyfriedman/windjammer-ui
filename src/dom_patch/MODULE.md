# dom_patch

**Purpose.** Keyed diff/patch of `simple_vnode::VNode` trees onto a DOM, plus delegated
event routing. Replaces the old "set `innerHTML` every frame" loop in `ReactiveApp::run`.

**Inputs.** A VNode tree per render (`Mounted::update`). Trees are `sanitized()` first.
**Outputs.** Minimal `PatchOp`s (`Replace`, `Insert`, `Remove`, `Move`, `SetAttr`,
`RemoveAttr`, `SetText`) applied through a `DomPort`; handler invocations on dispatch.

**Invariants.**
- Ops are sequential: each op's paths are valid after the previous ops.
- `apply(diff(a, b), a)` is structurally equal to `b` (property test, 1500 cases).
- Children match by `key` attribute, else by ordinal among unkeyed siblings; keyed
  reorders are `Move`s, never rebuilds. Matched nodes are reused (focus, caret, typed
  input value, scroll survive).
- Event attributes are never diffed or written to the DOM. Handlers live in a
  `(path, event)` table rebuilt from the current tree after every update; one capturing
  listener per event name sits on the container; dispatch bubbles innermost first.
- `value`/`checked` are written as properties only if they differ from the live value.

**Owns.** Diff algorithm, op semantics, handler table, port contract.
**Never decides.** Escaping/sanitizing rules (`escape.rs`, `VNode::sanitized`), when to
re-render (`app_reactive.rs`), component structure.

**Allowed imports.** `simple_vnode`, `escape` (via `sanitized`), std. `web.rs` only adds
`wasm-bindgen`, `web-sys`, `js-sys` and is `cfg(wasm32 + web)`. `diff`, `apply`,
`port`, `mounted`, `fake_dom` must stay wasm-agnostic.

**Entry points.** `diff::diff`, `apply::apply` (pure oracle), `Mounted::{update, dispatch,
handlers_for}`, `port::DomPort`, `fake_dom::FakeDom`, `web::new_mounted`.

**Tests.** `tests/dom_patch_test.rs` (property test, keyed reorder, focus/caret,
delegation, XSS through patching). Web adapter: compile-checked with
`cargo check --target wasm32-unknown-unknown`; no browser test yet.
