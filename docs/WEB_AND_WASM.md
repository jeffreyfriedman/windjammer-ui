# Web and WebAssembly: where windjammer-ui stands, and the road ahead

Windjammer now runs in the browser: the interpreter compiles to `wasm32-unknown-unknown`, and the Windjammer Book
playground (compiler ADR-0034) runs single-file and multi-file programs in the reader's tab, with the same capability
model as `wj run --sandbox`. This note places windjammer-ui next to that work and lays out the steps that join them.

## Two wasm stories, today

| | windjammer-ui (this repo) | Windjammer playground (compiler repo) |
|---|---|---|
| What runs in the browser | the Rust reactive core and keyed DOM patcher, compiled by `wasm-pack` (feature `web`, the default) | the Windjammer interpreter, compiled to wasm; programs are interpreted |
| What drives the page | `wasm-bindgen` + `web-sys`: signals, VNode tree, `dom_patch` against the real DOM | a Worker that calls `run` / `run_project` and prints output |
| Components | `.wj` builders render HTML strings; the generated Rust under `src/components/generated/` ships in the wasm | the program's own `.wj` files, in memory |
| Tested by | `cargo test --test dom_patch_test` (fake DOM), embedded JS runtimes under node, `wasm_node_test` (patcher compiled to wasm32, run in node) | headless Chrome tests and a native-versus-wasm parity pass over the book samples |

They meet at one point: both want the component library to be ordinary Windjammer that renders a tree.

## What is solid now

- The patcher, the escape-by-default renderers and the keyed diff have host tests, and the patcher also runs as wasm32
  inside node (`docs/TESTING_GUIDE.md`).
- Every `.wj` in `src/components_wj/` passes `wj check` and transpiles with the current compiler, and every example in
  `examples/` and `examples_wj/` transpiles. `tests/wj_sources_check_test.rs` enforces both, so a compiler change that
  rejects the sources fails here first. (Set `WJ=/path/to/wj`; `WJ_UI_REQUIRE_WJ=1` turns a missing compiler into a failure.)
- `cargo check -p windjammer-ui --target wasm32-unknown-unknown --no-default-features --features web` is the link
  contract for the web feature (`tests/wasm_web_link_test.rs`).

## What is not done yet

- Behaviour against a real browser DOM (`moveBefore`, focus restore, delegated listeners) has no automated test;
  `wasm-pack test --headless --chrome` is the planned home.
- The checked-in `src/components/generated/` is not regenerated from `src/components_wj/` (a full regeneration at the
  current compiler differs from the hand-maintained files); closing that gap is the main authoring task.
- The size budgets of ADR-0005 (counter under 400 KB brotli) are targets; no measurement is committed.
- `crates/wj_samples` (the new-core samples) stops at the current compiler with WJ0050: `wj_reactive::state` and the
  other generic functions of a Rust crate are not in its generated signature surface, and the opaque `Signal` type is not
  expressible yet (ADR-0026). It needs a pinned `rust/wj_reactive.wji` once the compiler supports generic and opaque types.
- Desktop (egui) shares no code with the web components; iOS and Android are stubs.

## Roadmap

Each step has a measurable exit, and a kill criterion where one applies.

1. **Regenerate for real.** Make `wj build src/components_wj --library` produce the files in `src/components/generated/`
   (byte for byte, or with the differences moved into `.wj`), then delete the hand-maintained copies. Exit: a CI job
   that regenerates and fails on any diff. Blocked on nothing; the sources already build.
2. **Browser DOM tests.** One `wasm-pack test --headless --chrome` suite for `dom_patch::web`: focus survives a keyed
   move, delegated clicks fire once, `stop_propagation` holds. Exit: green in CI on a pinned Chrome.
3. **Render in the playground.** The `examples_wj/` programs build VNode trees with plain string work, so they run
   under the playground's `run_project` (in-memory source tree). Publish a "UI gallery" page that loads
   `src/components_wj` plus an example into the playground and shows the produced HTML in an iframe. Exit: three
   examples render identically in the playground and in native `wj run`, checked by the playground parity script.
4. **Events in the playground.** Add a small host call that lets an interpreted program register handlers and receive
   DOM events from the page (the patcher's event delegation already works in terms of handler names). Exit: the
   counter example increments in the playground without the Rust core.
5. **One tree, two engines.** Run the same component program interpreted in the browser (instant edit-run) and
   compiled into the wasm bundle (small, fast), selected by a flag. Exit: the counter at under 400 KB brotli compiled,
   and a hot edit in under a second interpreted. Kill criterion: if the interpreted path needs a fork of the
   interpreter rather than host calls, stop and keep the compiled path only.
6. **Ship it.** SSR plus hydration adoption (ADR-0005), accessibility checks per component, and published size numbers.

Steps 1 and 2 are independent and can start today; step 3 depends only on the playground's multi-file API, which exists.
