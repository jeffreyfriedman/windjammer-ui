# Architecture

Windjammer UI is a Rust crate plus a set of Windjammer (`.wj`) component sources.
This document describes the crate as it is today; the scene-renderer direction is in `docs/VISION_2026.md`.

## Layers

1. **Components (`src/components_wj/*.wj`)** - 100 files. Each one is a builder
   (`Button::new("x").variant(..).render()`) that returns an **HTML string**. They are
   transpiled by `wj` into `src/components/generated/*.rs`. There is no component
   tree at this layer; output is a string that the host inserts into the page.
2. **Reactive core (Rust)** - `signal.rs` (Signal/Computed/Effect), `reactivity.rs`
   (the public reactive API), `reactivity_optimized.rs` (experimental variant, not the
   default), `signal_ffi.rs` (exposes signals to generated code).
3. **Virtual node types** - `simple_vnode.rs` is the canonical VNode (all new code,
   app loops, `to_vnode.rs`, `vnode_ffi.rs`). `vdom.rs` is legacy: it is still the type of
   the `Component` trait, `renderer.rs`, `ssr.rs` and `platform/*`, and has its own
   path-based diff that nothing in the app loop uses any more. It is not deleted because
   those callers have not been migrated (see gaps).
3a. **Escaping (`escape.rs`)** - escape-by-default policy: `escape_text`/`escape_attr`,
   `sanitize_url`, tag/attribute-name validation, and the explicit `RawHtml` opt-out.
   `VNode::sanitized()` applies it to a tree; `simple_renderer::render_to_html` and the
   patcher both consume sanitized trees.
3b. **DOM patcher (`dom_patch/`)** - pure keyed diff -> `PatchOp`s, applied through the
   `DomPort` trait (`FakeDom` for host tests, `WebDom` over web-sys on wasm32).
   `Mounted` owns the current tree and a `(path, event)` handler table; events are
   delegated from the mount container. See `src/dom_patch/MODULE.md`.
4. **App loops** - `app_reactive.rs` (WASM/browser loop, the primary supported path;
   patches the DOM in place instead of replacing `innerHTML`),
   `app_reactive_eframe.rs` + `desktop_renderer.rs` (desktop via eframe/egui, feature
   `desktop`), `reactive_mount.rs` (mount-target selection).
5. **Other runtime pieces** - `ssr.rs` (string SSR), `routing.rs`, `runtime.rs`,
   `component_runtime.rs`, `undo_redo.rs` (editor command pattern), `wasm_events.rs`.

## Targets

| Target | State |
|--------|-------|
| Web / WASM (`web`, default) | Supported. HTML strings plus the reactive loop in `app_reactive.rs`. |
| Desktop (`desktop`, eframe/egui) | Works for the editor-style UI; less covered by tests. |
| SSR | Basic string rendering; hydration is minimal. |
| iOS (`mobile-ios`) | Stub: `platform/ios.rs` has types but no renderer yet. |
| Android (`mobile-android`) | Stub: `platform/android.rs`, same status. |
| Tauri (`desktop-tauri`) | Optional dependency. |
| Terminal | Not implemented (`simple_renderer.rs` only prints nodes to console). |

`platform/web.rs` and `platform/desktop.rs` re-export the web and desktop modules.

## Generated code

`src/components/generated/` holds only `wj` output of `src/components_wj/`, byte for byte; `tests/generated_fresh_test.rs`
regenerates it and fails on any difference. Hand-written Rust (reactivity, vdom, renderers, the VNode and signal FFI) lives
at the crate root. Refresh the tree with the command in that test's header.

## Current limitations

- Escaping: the VNode path and SSR escape by
  default. Compiler-generated HTML-string builders (`.wj` sources) still interpolate raw
  values; see `docs/known-issues/escaping.md`.
- `vdom.rs` is still used by `component.rs`, `renderer.rs`, `ssr.rs`, `platform/*`.
- The web patcher is covered by host fake-DOM tests, wasm32-in-node tests and a headless-Chrome suite (`docs/TESTING_GUIDE.md`).
- Interactive behaviour in several components is JS embedded in Rust strings; the rule
  is to write it in Windjammer signals/closures.
- Components are generic; domain-specific components belong in the consuming application.

See `docs/MODULE_MAP.md` for per-module responsibilities.

## New core (crates/, separate Cargo workspace)

The vision's scene renderer is built beside the main crate in `crates/`: `wj_reactive` (L0), `wj_scene` (L1),
`wj_layout`/`wj_text` (L2), `wj_paint`, `wj_paint_cpu`, `wj_paint_vello`, `wj_a11y` (L3), `wj_shell`, `wj_shell_desktop` (L4),
`wj_ui` (Windjammer-facing API), `wj_samples`, `wj_golden`. Each has a `MODULE.md`. See ADR-0012 for decisions and measurements.
Run a crate's tests with `cd crates && cargo test -p wj_scene --release`.
