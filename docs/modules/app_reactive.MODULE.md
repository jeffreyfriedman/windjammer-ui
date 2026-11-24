# app_reactive (`src/app_reactive.rs`, wasm32 only)

**Purpose.** Browser app loop: re-render on signal change, patch the DOM.

**Behavior.** `run()` mounts a `dom_patch::Mounted<WebDom>` on the mount selector; a
`requestAnimationFrame` loop calls `render_fn` when `trigger_rerender()` set the dirty flag
and passes the result to `Mounted::update` (diff, minimal patches). No `innerHTML` after
mount; focus, caret, scroll and typed input values survive; `VAttr::Event` handlers work
via container-level delegation. `paint_once()` still does a one-shot escaped `innerHTML`.

**Invariants.** The patcher owns the mount element's only child (it is cleared on `run`).
**Owns.** Scheduling/dirty flag. **Never decides.** Diffing, escaping.
**Allowed imports.** `reactive_mount`, `simple_vnode`, `dom_patch`, `simple_renderer`.
**Entry points.** `ReactiveApp::{new, mount_target, run, paint_once, paint_html}`,
`trigger_rerender`.
**Tests.** Logic covered host-side by `tests/dom_patch_test.rs`; wasm path is
compile-checked (`cargo check --target wasm32-unknown-unknown --lib`). No browser test yet.
