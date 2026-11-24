# Web and WebAssembly

Windjammer now runs in the browser: the interpreter compiles to `wasm32-unknown-unknown`, and the Windjammer Book
playground (compiler ADR-0034) runs single-file and multi-file programs in the reader's tab, with the same capability
model as `wj run --sandbox`. This note places windjammer-ui next to that work and lays out the steps that join them.

## Two wasm stories

| | windjammer-ui (this repo) | Windjammer playground (compiler repo) |
|---|---|---|
| What runs in the browser | the Rust reactive core and keyed DOM patcher, compiled by `wasm-pack` (feature `web`, the default) | the Windjammer interpreter, compiled to wasm; programs are interpreted |
| What drives the page | `wasm-bindgen` + `web-sys`: signals, VNode tree, `dom_patch` against the real DOM | a Worker that calls `run` / `run_project` and prints output |
| Components | `.wj` builders render HTML strings; the generated Rust under `src/components/generated/` ships in the wasm | the program's own `.wj` files, in memory |
| Tested by | `cargo test --test dom_patch_test` (fake DOM), embedded JS runtimes under node, `wasm_node_test` (patcher compiled to wasm32, run in node) | headless Chrome tests and a native-versus-wasm parity pass over the book samples |

They meet at one point: both want the component library to be ordinary Windjammer that renders a tree.

## What works today

- The patcher, the escape-by-default renderers and the keyed diff have host tests, and the patcher also runs as wasm32
  inside node (`docs/TESTING_GUIDE.md`).
- Every `.wj` in `src/components_wj/` passes `wj check` and transpiles with the current compiler, and every example in
  `examples/` and `examples_wj/` transpiles. `tests/wj_sources_check_test.rs` enforces both, so a compiler change that
  rejects the sources fails here first. (Set `WJ=/path/to/wj`; `WJ_UI_REQUIRE_WJ=1` turns a missing compiler into a failure.)
- `cargo check -p windjammer-ui --target wasm32-unknown-unknown --no-default-features --features web` is the link
  contract for the web feature (`tests/wasm_web_link_test.rs`).

## Next steps

- Behaviour against a real browser DOM (`moveBefore`, focus restore, delegated listeners) is covered by
  `tests/wasm_browser_test.rs` (headless Chrome through `wasm-bindgen-test-runner`; command in `docs/TESTING_GUIDE.md`). A CI job with a pinned Chrome and chromedriver is the remaining piece.
- Components that call into the crate's own Rust modules by module path (`vnode_ffi::f(..)`) need the module imported and the
  function called by bare name: the compiler has no boundary signature for module-qualified calls into hand-written
  modules of the same crate . A literal passed to an owned `String`
  formal of a method that forwards it to such a module is not converted ;
  `VNode::hr()` exists because of it.
- The size budget of ADR-0005 (counter under 400 KB brotli) is a target; a measurement will be committed with it.
- `crates/wj_samples` pins `wj/rust/wj_reactive.wji` with the `i32` instantiation of `state`: a surface cannot declare generic
  functions or opaque crate types yet (ADR-0026), so other element types wait for the compiler.
- Desktop (egui) shares no code with the web components; iOS and Android are stubs.

## Roadmap

Each step has a measurable exit, and a fallback where one applies.

1. **Regenerate for real.** Done: `src/components/generated/` is the compiler's output byte for byte, and
   `tests/generated_fresh_test.rs` regenerates it and fails on any difference.
2. **Browser DOM tests.** Landed (5 tests, green in Chrome 157): one headless-Chrome suite for `dom_patch::web`: focus survives a keyed
   move, delegated clicks fire once, `stop_propagation` holds. Exit: green in CI on a pinned Chrome.
3. **Render in the playground.** Prerequisite: `components_wj/vnode.wj` builds its tree through the Rust
   `vnode_ffi` registry, which the interpreter cannot call, so no component runs interpreted yet (`wj run --interpret`
   over `src/components_wj` stops at the first `VNode` method). The step starts with a pure-Windjammer `VNode` (a tree value
   with tag, attributes and children, rendered to HTML by a Windjammer function) that the compiled build can still map
   onto the Rust registry. Then the `examples_wj/` programs run under the playground's `run_project` (in-memory source tree). Publish a "UI gallery" page that loads
   `src/components_wj` plus an example into the playground and shows the produced HTML in an iframe. Exit: three
   examples render identically in the playground and in native `wj run`, checked by the playground parity script.
4. **Events in the playground.** Add a small host call that lets an interpreted program register handlers and receive
   DOM events from the page (the patcher's event delegation already works in terms of handler names). Exit: the
   counter example increments in the playground without the Rust core.
5. **One tree, two engines.** Run the same component program interpreted in the browser (instant edit-run) and
   compiled into the wasm bundle (small, fast), selected by a flag. Exit: the counter at under 400 KB brotli compiled,
   and a hot edit in under a second interpreted. Fallback: if the interpreted path needs a fork of the
   interpreter rather than host calls, stop and keep the compiled path only.
6. **Ship it.** SSR plus hydration adoption (ADR-0005), accessibility checks per component, and published size numbers.

Steps 1 and 2 are independent and can start today; step 3 depends only on the playground's multi-file API, which exists.
