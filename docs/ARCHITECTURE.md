# Architecture (current state)

Windjammer UI is a Rust crate plus a set of Windjammer (`.wj`) component sources.
This document describes what exists today, not what is planned.

## Layers

1. **Components (`src/components_wj/*.wj`)** - about 120 files. Each one is a builder
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

## Which targets are real

| Target | State |
|--------|-------|
| Web / WASM (`web`, default) | Real. HTML strings plus the reactive loop in `app_reactive.rs`. |
| Desktop (`desktop`, eframe/egui) | Works for the editor-style UI; less covered by tests. |
| SSR | Basic string rendering; hydration is minimal. |
| iOS (`mobile-ios`) | Stub. `platform/ios.rs` has types but no working renderer. |
| Android (`mobile-android`) | Stub. `platform/android.rs`, same status. |
| Tauri (`desktop-tauri`) | Legacy optional dependency; not exercised. |
| Terminal | Not implemented (`simple_renderer.rs` only prints nodes to console). |

`platform/web.rs` and `platform/desktop.rs` are placeholders that re-export other modules.

## Generated vs hand-maintained code

`src/components/generated/` should only contain `wj` output. Seventeen files there are
currently hand-maintained (marked `SKIP_WJ_REGEN=1`) because the compiler cannot yet
produce them. They are listed in `docs/EXTRACTION_PLAN.md`. This diverges from the
repo rule and is treated as debt, not a feature.

## Known gaps

- Escaping: the VNode path, SSR and the hand-maintained generated files escape by
  default. Compiler-generated HTML-string builders (`.wj` sources) still interpolate raw
  values; see `docs/known-issues/escaping.md`.
- `vdom.rs` is still used by `component.rs`, `renderer.rs`, `ssr.rs`, `platform/*`.
- The web patcher has no in-browser test yet (host fake-DOM tests only).
- Interactive behaviour in several components is JS embedded in Rust strings; the rule
  is to write it in Windjammer signals/closures.
- Domain (ledger) components still live here; see `docs/EXTRACTION_PLAN.md`.
- `.disabled` tests were resolved (see `docs/EXTRACTION_PLAN.md` section C).

See `docs/MODULE_MAP.md` for per-module responsibilities.

## New core (crates/, separate Cargo workspace)

The vision's scene renderer is being built beside the legacy crate in `crates/`: `wj-reactive` (L0), `wj-scene` (L1),
`wj-layout`/`wj-text` (L2), `wj-paint`, `wj-paint-cpu`, `wj-paint-vello`, `wj-a11y` (L3), `wj-shell`, `wj-shell-desktop` (L4),
`wj-ui` (Windjammer-facing API), `wj-samples`, `wj-golden`. Each has a `MODULE.md`. See ADR-0012 for decisions and measurements.
Run a crate's tests with `cd crates && CARGO_TARGET_DIR=<cache> cargo test -p wj-scene --release`.
