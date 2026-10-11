# Design direction (decision record, proposed)

- **Status:** Proposed. Needs owner approval before any stage past 0 starts.
- **Date:** 2026-10-11
- **Evidence:** `ASSESSMENT_2026.md` (facts, sources and their reliability labels).
- **Related:** compiler `docs/adr/0007` (target tiers), `0012` (Rust interop), `0014`/`0015` (surface syntax),
  `docs/design/windjammer-ui.md` (older JSX-style proposal), `docs/compiler-requests/`.

## Context

windjammer-ui has a solid, tested patcher and escaping core, but three component models, a render loop that rebuilds
and diffs the whole tree per frame, no connection between signals and DOM nodes, no mobile story, an egui desktop path
that shares nothing with the web components, and accessibility that depends on whoever wrote each component. Two real
consumers exist: web business apps (a downstream finance app) and the engine's editor panels (egui desktop).

## Options considered

### 1. Rendering and reactivity

- **A. Keep whole-tree VNode rebuild + keyed diff (status quo).** Cheapest; fine for small trees and the editor; the
  per-frame cost is O(tree). Does not use signals for what they are for.
- **B. Compile-time fine-grained signals, DOM-direct (Solid/Svelte/Leptos).** Best update cost and fewest Wasm/JS
  crossings; needs the compiler (or a typed convention) to turn a view into creation code plus per-binding effects; non-DOM
  renderers need an equivalent retained node API.
- **C. Signals schedule component-scoped VDOM re-renders; renderers consume a mutation stream (Dioxus).** Keeps one
  tree IR for web, egui and any future native target; update cost O(changed subtree); the existing `dom_patch` and
  `DomPort` are exactly the web half of this.

### 2. Targets

- **Web-first, Tauri shell for desktop+mobile.** One component set, real DOM a11y, small shell.
- **Native renderer (egui/wgpu/Slint-like) everywhere.** Pixel control; reimplements text, IME, a11y; poor web payload.
- **Both, with the editor staying on egui.** Reality today.

## Decision (recommended)

### D1. Architecture: C now, B where the compiler can do it

1. **One canonical tree IR: `simple_vnode::VNode`.** Everything else (string-builder components, legacy `vdom`) is
   either converted to it or deleted. The `vdom`-to-`VNode` bridge already exists for the transition.
2. **Make signals drive rendering at region granularity.** Introduce scoped reactive *regions*: `each(list, key, row)`,
   `show(cond, then, else)`, and a `memo`/`derived`-backed `view` closure. A region owns an effect; a signal change re-runs
   only the regions that read it and feeds that subtree through the existing keyed diff. The root closure stops being
   re-run on every change. This removes the global `static mut` rerender callback and the "any signal change rebuilds
   everything" behaviour with an incremental change to `signal.rs` + `app_reactive.rs`, not a rewrite.
3. **Direct bindings as an optimisation, not the architecture.** Text and attribute expressions that read signals
   (`"${count}"`) become per-node setters (`SetText`/`SetAttr` ops addressed by cached node handle) skipping the diff,
   once the compiler can identify them (stage 3). Until then regions are the unit.
4. **Signals become arena handles that are `Copy`** (index + generation), batched writes (`batch`, microtask flush),
   ownership scopes with disposal. This is what makes closure-heavy authoring ergonomic without Rust's clone ceremony.
5. **Cache node handles in the patcher** (path-to-node memo per update) to remove the O(depth * siblings) child lookups,
   and batch ops per frame. Measure before and after (stage 1 exit criterion).

Why not B all the way: the numbers say B is fastest on the DOM **[U, third-party benchmark summaries]**, but B only pays
off with compiler analysis we do not have, and it cuts off the egui path that the engine editor uses today. C gets most of
the update-cost win (subtree-scoped diffs with keyed lists) without a compiler dependency and keeps both renderers on
one IR. Why not A: signals that don't drive anything are dead weight and the whole-tree cost is a cliff for tables and
editors.

### D2. Targets and priority

1. **Web (Wasm) is the product target.** Priority 1. Ship a real size budget and measured benchmark.
2. **Desktop = Tauri 2 webview shell around the web build** for business apps. Priority 2. Uses the same components and
   gets DOM a11y. Low code in this repo (a capability trait and a template), mostly packaging.
3. **Engine editor keeps the egui renderer** (`desktop` feature) fed by the same `VNode` IR, as a separate, explicitly
   scoped renderer for tool panels, not a general desktop target. Stop promising "Desktop" as a cross-platform app target
   on that basis; call it "editor panels".
4. **Mobile = Tauri 2 mobile (webview) as an experiment after web is measured.** The evidence is that Tauri 2 mobile
   exists and works but its own team said plugin coverage and DX lag **[V]**. No `objc`/`jni` renderers; delete those stubs.
5. **Compile `.wj` to JS for web** is a compiler-team decision (ADR-0007). If the JS backend reaches Tier 2 for the UI
   subset, it becomes the size-optimal web path (Svelte-class output) and the Wasm path remains for logic-heavy apps.
   windjammer-ui must not depend on web-sys-only APIs in components, so either backend can host it (see C6).
6. Not pursued: own GPU renderer for web, React Native-style native widget bridges, terminal target.

### D3. Authoring

- Primary surface: **typed builder DSL** in plain Windjammer (closures, method chaining, `each`/`show` regions;
  assessment section 3.1). It needs only language features that exist and can be taught and tested now.
- Target surface: **`component` / `state` / `derived` / `view` blocks with `for ... key` and `if`** compiled by the
  toolchain, giving the compiler the dependency information for direct bindings. Requires compiler work (stage 3).
- No macro-based surface, in line with the language. No JSX-in-strings. Any HTML-ish literal is a compiler decision.

### D4. Keep, delete, merge

Keep: `dom_patch/`, `escape.rs`, `simple_vnode.rs`, `signal.rs` semantics, `undo_redo.rs`, editor panel components,
`wasm-bindgen-test` and node test setups, `vdom -> VNode` bridge (temporary).

Delete (after the stated precondition):
- `src/components/generated/` copies of core modules (27 files that duplicate `src/*.rs`; no `.wj` sources): immediately
  after confirming nothing depends on the `components::generated::<core module>` paths; drop their `mod`/`pub use` lines.
- `reactivity_optimized.rs` and the second/third reactivity modules: after merging one set of semantics into `signal.rs`
  (stage 1).
- `vdom.rs`, `renderer.rs` (777 lines, per-platform `Renderer` trait with mostly stubs), `runtime.rs`, `component_runtime.rs`,
  `platform/{ios,android,mobile,desktop,web}.rs`, `mobile-ios`/`mobile-android` features and `objc`/`cocoa`/`jni`
  deps: when `Component`/`mount` callers are moved to `VNode` (stage 1; the bridge covers the gap).
- `windjammer-ui-macro` `#[component]`/`Props`: when `.wj` builder DSL covers the use; Rust consumers can use plain functions.
- `vnode_ffi.rs` integer-handle surface: replaced by the builder DSL; keep only as the compiler-facing lowering target if
  the compiler still wants it.
- JS runtimes in Rust strings and ledger components: move out per `EXTRACTION_PLAN.md`, rewriting behaviour in
  Windjammer closures over `VNode`/signals, using `fetch` capability + resource primitive (stage 2).

Merge: string-builder components become `VNode`-returning functions (same builder names, `.render()` kept as a
thin `render_to_html(&node)` shim for SSR consumers during migration). Generated builders then inherit escape-by-default.

## Staged roadmap

Each stage has exit criteria that can be checked by a test or a measurement. Compiler requests are listed per stage and
detailed in `docs/compiler-requests/README.md`.

### Stage 0 (done in this pass)
Escape-by-default, keyed patcher with focus restore and stop_propagation, runtime innerHTML removed, `RawHtml` on `*_html`
fields, SSR hardening, examples compile, browserless test setups. Exit: `cargo test --lib --tests` green, wasm-in-node green.

### Stage 1: one tree, signal-driven regions (target: 4-6 weeks)
- Merge reactivity modules into one `signal.rs` with arena `Copy` handles, `batch`, scopes/disposal; remove global
  `static mut` rerender callback.
- `each`/`show`/`derived` regions; `app_reactive` mounts a root region; patcher caches node handles and batches ops.
- Convert `Component`/`renderer`/`runtime`/`ssr` users to `VNode`; delete `vdom.rs`, duplicate generated copies, mobile stubs.
- Single SSR renderer over `VNode` with hydration that adopts server nodes (matching on structure and `key`), not rebuild.
- Exit: (1) counter+todo+1k-row table benchmark committed with numbers for wasm size (raw, `-Oz`, gzip/brotli) and 1k/10k row
  create/update/swap/remove in a pinned Chrome; (2) updating one signal in a 1k-row table re-runs only the affected region
  (asserted by a test counting builder calls); (3) `cargo test` shows no duplicated module tests; (4) hydration test
  with FakeDom shows zero created nodes on first client render.
- Compiler asks: C1, C2, C3.

### Stage 2: authoring, a11y, web-platform pieces (6-8 weeks)
- Builder DSL for all remaining components as `VNode` functions; escape-by-default everywhere; delete string-builder runtime
  path and the 17 hand-maintained generated files (behaviour reimplemented over capabilities: `fetch`, storage, history).
- A11y contract per component: role/ARIA/keyboard/focus table in each `MODULE.md`; focus trap and restore in
  modal/dialog/drawer/command palette; roving tabindex in tabs/menu; live regions for toast/alert; axe run in a real browser
  test (`wasm-pack test --headless --chrome`) in CI.
- Router on History API with typed params, guards; form state + validation primitives; `resource` (async data with
  loading/error states) built on signals.
- Exit: every component has a keyboard test (FakeDom event simulation) and passes axe in the browser suite; a downstream finance app
  builds against the new API with domain components extracted.
- Compiler asks: C4, C5.

### Stage 3: compiler-assisted reactivity and DX (needs compiler team)
- `component`/`state`/`derived`/`view` block syntax; the compiler emits direct bindings for signal-reading text/attrs and
  regions for `for`/`if`; props type-checked at call sites.
- Hot reload with state preservation (dev server re-instantiates view, keeps signal arena); error overlay mapping panics
  to `.wj` spans (needs source maps).
- Exit: counter update performs zero diff work (asserted); edit-save-see under 1 second in the dev server on the sample app.
- Compiler asks: C6, C7, C8, C9.

### Stage 4: shells (after stage 1 numbers exist)
- Tauri 2 desktop template, then mobile experiment; capability trait implementations (files, notifications, deep links).
- Exit: signed sample app on macOS plus one mobile OS, with a documented plugin-gap list.

## Requests for the compiler team (do not implement here)

Details and minimal repros in `docs/compiler-requests/README.md`.

| ID | Request | Needed by |
|----|---------|-----------|
| C1 | Treat designated handle types (`Signal<T>`, `Computed<T>`, `StateHandle`) as `Copy`-like so inferred ownership never moves them into closures; `@copy_handle` or equivalent attribute | Stage 1 |
| C2 | Trailing-closure call syntax for builder DSLs (`column(gap: 12) { ... }`) and named arguments/defaults in calls | Stage 1 (ergonomics) |
| C3 | `${...}` interpolation lowered to a structured template (static parts + expression list) available to a UI lowering pass, not only a `format` call | Stage 1/3 |
| C4 | A `bind:` convention or two-way binding lowering for form controls (`bind: draft` expands to value + on_input) | Stage 2 |
| C5 | Capability/extern-trait story for browser APIs from `.wj` (fetch, storage, history, timers) that is backend-neutral (Wasm and JS backends), per ADR-0012 | Stage 2 |
| C6 | Component syntax (`component`, `state`, `derived`, `view`, `for ... key`, `if` in view) with dependency analysis exposing which expressions read which handles | Stage 3 |
| C7 | JS backend reaching Tier 2 for the UI subset (closures, structs, string interpolation, no-GC friendly) and conformance cases for it | Stage 3 |
| C8 | Source maps from generated Rust/JS back to `.wj` spans for the error overlay | Stage 3 |
| C9 | Incremental `wj build` / watch mode with stable output paths, so hot reload can re-link quickly | Stage 3 |
| C10 | Make `build.rs` regeneration safe: a marker that makes `wj` skip hand-maintained files (so `SKIP_WJ_REGEN` goes away) | Stage 2 |

## Consequences

- Short term: no rewrite. Stage 1 is a refactor of reactivity plus deletions, protected by the existing property tests.
- Library surface shrinks (string-builder path, vdom, mobile stubs, macros removed) and breaks consumers that use them;
  downstream apps must migrate (also true for the `RawHtml` field change landed now).
- The Dioxus-style IR plus mutation stream decision means desktop-egui and web stay on one tree; the cost is that
  direct-DOM bindings (stage 3) need a second code path for non-DOM renderers (fall back to regions).
- We accept that Wasm size will not match Svelte/Solid until the JS backend exists; the budget and measurement are in stage 1.

## Decision needed from: project owner

1. Approve C (regions on one VNode IR) over A (status quo) and B (DOM-direct only).
2. Approve "web first, Tauri shell for desktop/mobile, egui kept for editor panels only".
3. Approve deleting vdom/renderer/mobile stubs/proc macros/duplicated generated modules after the stage 1 migration.
4. Approve sending C1-C10 to the compiler team.
