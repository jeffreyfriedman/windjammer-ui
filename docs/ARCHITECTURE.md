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
3. **Virtual node types** - `vdom.rs` (VElement/VNode/VText with diff/patch) and
   `simple_vnode.rs` (the lighter VNode used by the app loops). `to_vnode.rs` converts
   components to `simple_vnode::VNode`; `vnode_ffi.rs` lets `.wj` code build nodes.
4. **App loops** - `app_reactive.rs` (WASM/browser loop, the primary supported path),
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

- Escaping: builders build strings; escaping-by-default with a `RawHtml` type is a
  rule in AGENTS.md but must be audited per component.
- Interactive behaviour in several components is JS embedded in Rust strings; the rule
  is to write it in Windjammer signals/closures.
- Domain (ledger) components still live here; see `docs/EXTRACTION_PLAN.md`.
- Six integration tests are parked as `.disabled` (listed in the plan).

See `docs/MODULE_MAP.md` for per-module responsibilities.
