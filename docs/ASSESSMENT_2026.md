# windjammer-ui assessment, October 2026

Scope: where windjammer-ui stands against the current state of the art, what in the code is
worth keeping, and what that implies. Decision and roadmap: `DESIGN_DIRECTION.md`.

Evidence labels: **[V]** verified by a web source this session (listed in section 8),
**[K]** from prior knowledge, not re-verified, **[U]** uncertain or sources disagree.
Nothing here is a benchmark we ran ourselves.

## 1. State of the art (what matters for this project)

### 1.1 Reactivity and rendering models

| Model | Examples | What it buys | What it costs |
|---|---|---|---|
| Compile-time fine-grained signals, no VDOM | Svelte 5 (runes), Solid, Leptos, Sycamore, Dioxus signals (partial) | Updates touch only the DOM nodes that depend on a changed value; no tree diff; smallest per-update work | Needs a compiler (Svelte, Solid JSX) or a macro/typed-builder convention (Leptos `view!`); control flow (lists, conditionals) needs dedicated primitives (`<For>`, `{#each}`) |
| VDOM + diff, with a compiler to cut re-renders | React 19 + React Compiler 1.0 (stable 7 Oct 2025 [V]), Dioxus, Yew | Simple mental model ("UI = f(state)"); easy SSR; renderer-agnostic (native, terminal, mobile) | Per-update cost proportional to rendered subtree; needs memoisation discipline or a compiler |
| Resumability / zero-hydration | Qwik [K] | No hydration replay cost; lazy-loaded handlers | Different programming model; serialization constraints on closures; small ecosystem |
| Own renderer (canvas/GPU) | Flutter (Skia/Impeller), Compose Multiplatform, Slint, egui, iced, Dioxus Native/Blitz | Pixel-identical cross-platform; no DOM boundary; one rendering stack for desktop+mobile | Re-implements text, IME, selection, a11y, scrolling; heavy web payload; web a11y and SEO are weak points |
| Webview shell + native core | Tauri 2, Electron, Capacitor | Reuses the web platform for UI; small native glue | Webview differences per OS; IPC boundary; mobile story newer |
| Native widget bridge | React Native/Expo, SwiftUI, Jetpack Compose, .NET MAUI | Native look, a11y and input for free | One bridge per platform; each platform's UI toolkit leaks through |

Relevant verified facts:

- Svelte 5 and Solid sit within a few percent of hand-written vanilla JS on the
  js-framework-benchmark geometric mean in third-party summaries: roughly 1.04-1.10x vanilla,
  versus roughly 1.2-1.7x for React 19 with compiler (the two blogs disagree widely) **[U]**.
  The official results page (krausest.github.io/js-framework-benchmark) is a dynamic page that
  could not be scraped, so no number here should be quoted as authoritative. Leptos is said (README,
  possibly dated) to be the fastest Rust/Wasm framework and slightly slower than Solid **[U]**.
  Direction of the result is not in doubt: fine-grained signals beat VDOM on update-heavy
  benchmarks, and Rust/Wasm frameworks do not beat JS signal frameworks on DOM-bound work.
- Wasm to DOM: every DOM call crosses the Wasm/JS boundary through wasm-bindgen glue. No current
  measured number was found **[U]**; sources only say the advantage of Rust is "partly lost at the
  boundary" for typical UIs. The practical rule used by Leptos, Dioxus and Sycamore is the same:
  minimise the number of crossings (batch ops, cache JS-side handles, set text content rather than
  rebuild), which favours fine-grained updates and patch-op batching over whole-tree rebuilds.
- Wasm size: the Leptos book recommends `opt-level='z'`, LTO, 1 codegen unit, and notes compressed Wasm
  is typically under half its raw size **[V]**. A Rust UI app is realistically hundreds of KB compressed,
  against a few KB-tens of KB for Svelte/Solid output **[K]**. Flutter's own docs list CanvasKit +1.5 MB and
  skwasm +1.1 MB of download **[V]**; a third-party measurement of the Wonderous demo (Aug 2026) is ~4.2 MB compressed
  (CanvasKit) and ~2.9 MB (Wasm) **[U]**.
- `Element.moveBefore()` (state-preserving DOM move: focus, iframe, animations survive) is Chrome/Edge 133+
  and Opera 118+ only; Firefox and Safari: not supported (MDN) **[V]**. So the portable fix for keyed reorders is
  restore-after-move, with `moveBefore` as a progressive enhancement. That is what `dom_patch` now does.
- Tauri 2.0 stable shipped 2 Oct 2024 with iOS and Android; the 2.11 line was current in July 2026 **[V, one
  third-party blog]**. The Tauri team itself said at the RC that "mobile as first class citizen" was
  overpromised and that not every desktop plugin exists on mobile **[V]**. No 2026 official maturity statement was
  found, so treat mobile as usable but with gaps you will hit **[U]**.
- Compose Multiplatform: Android, iOS (stable since 1.8.0) and Desktop are stable; Web (Kotlin/Wasm) is Beta
  as of the 1.9.0 line (Sep 2025), no stable announcement found **[V]**.
- Dioxus 0.7 (late 2025): Subsecond Rust hot-patching (experimental flag), "Dioxus Native" (WGPU HTML/CSS renderer,
  via Blitz; experimental), Axum server functions **[V]**. This is the closest existing analogue to what windjammer-ui
  wants to be (Rust, RSX macro, multi-renderer).
- Slint moved to AccessKit in 1.1 (2023) and got accessibility actions in 1.6 (2024) **[V]**. egui and iced also
  integrate AccessKit to varying degrees **[K, unverified this session]**. Own-renderer toolkits can have an a11y tree,
  but only because they build and maintain one.
- Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`) compile to signal-style fine-grained updates; Svelte
  output has a very small runtime **[K]**. Solid never re-runs components, only effects **[K]**. React Server
  Components move data fetching and rendering to the server and ship less JS **[K]**.

### 1.2 What the good frameworks have that is not rendering

Every framework people actually adopt ships these; they are where adoption is won or lost:

1. Accessibility by construction (semantic elements, ARIA, focus management, keyboard) - free on the web, hard
   on own-renderer targets.
2. A component authoring syntax that is shorter than the output it produces (Svelte templates, JSX, SwiftUI/Compose
   builders, Leptos `view!`).
3. Dev loop: hot reload/hot patch with state preserved, an error overlay that points at source.
4. Router, forms/validation, async data (suspense/resources), SSR + hydration or an explicit SPA story.
5. Test tooling that does not need a browser for most tests, plus a browser path for the rest.
6. Escape-by-default output, CSP-friendliness, URL sanitising.

## 2. windjammer-ui as it exists

Numbers from the repo at commit f6b7048: about 120 `.wj` component sources (~13k lines), ~9k lines of Rust in `src/*.rs`,
~24k lines in `src/components/generated/` (of which a large part are stale copies of core modules, see 2.9).

### 2.1 Component model

Two unrelated models coexist:

- **A. String-builder components** (`src/components_wj/*.wj`, the bulk): `Button::new("x").variant(..).render()`
  returns an HTML `String`. No tree, no events, no state; interactivity is JS embedded in Rust strings
  (17 hand-maintained runtimes). Values were interpolated unescaped until the previous pass; compiler-generated
  builders still are (`docs/known-issues/escaping.md`).
- **B. VNode components** (`simple_vnode::VNode`, `app_reactive`, `dom_patch`, `vnode_ffi`): trees with `VAttr::Event`
  closure handlers, rendered by a keyed patcher. This is the only model that supports interactivity in Windjammer.
  `vnode_ffi.rs` (520 lines, thread-local handle tables, `vnode_element`/`vnode_child`/...) is the authoring surface
  from `.wj`, i.e. an imperative builder through integer handles.
- **C. Legacy `vdom.rs`** (`Component` trait, `renderer.rs`, `runtime.rs`, `ssr.rs`, `platform/*`): attribute map without
  events, path-based positional diff that never removes attributes and mis-indexes after removals. Nothing in the app
  loop uses it. A bridge `From<vdom::VNode> for simple_vnode::VNode` now exists, so legacy trees can be mounted.
- **D. `#[component]`/`Props` proc macros** (`windjammer-ui-macro`, 125 lines): a Rust-side convenience; cannot be used
  from `.wj`, which has no macros.

The compiler repo's own design doc (`windjammer/docs/design/windjammer-ui.md`, v0.34.0) specifies a JSX-like `@component`
with `state`, `props`, `computed` and an `Html` literal. None of that exists; the implemented design diverged to
string builders and a handle-based FFI.

### 2.2 Reactivity and rendering

- `signal.rs`: `Signal/Computed/Effect` with dependency tracking via thread-local current effect (reasonable, Solid-style).
  Subscribers are stored per signal in `HashSet<EffectId>`; `notify` runs effects synchronously, with no batching,
  no glitch-free ordering, no ownership/disposal scopes beyond `on_cleanup`. There are three overlapping reactivity
  modules (`signal.rs`, `reactivity.rs`, `reactivity_optimized.rs`) plus `signal_ffi.rs`.
- The render loop is not fine-grained. `reactivity::Signal::set` calls a global `trigger_rerender()` (a `static mut`
  callback) that sets a flag; on the next animation frame the app closure rebuilds the **entire** VNode tree and
  `Mounted::update` diffs it against the previous one. So the cost model is React-without-memoisation: O(tree) allocation
  and diff per frame in which anything changed, even though signals exist. Signals and the renderer are not connected;
  signals are merely a "dirty" flag source.
- What was done well in the last pass: the patcher is correct and well-tested: keyed diff, ops applied sequentially,
  property test `apply(diff(a,b),a) == b` (1500 cases), nodes reused so focus, caret and input values survive, delegated
  events with a `(path,event)` handler table, escape-by-default `sanitized()`. All behind a `DomPort` trait with a
  `FakeDom`. This is a sound, small piece and its port boundary is the most reusable asset in the crate.
- Boundary cost: ops are produced in Rust and applied one DOM call each through web-sys (`child_nodes().item(i)` walks,
  `index_in_parent` walks siblings: O(n) per lookup, and every op resolves its path from the root). For large lists this
  is quadratic-ish. Fixable by caching node handles per VNode (see roadmap), but fine-grained bindings would avoid it.

### 2.3 Wasm size and boundary

No wasm artifact is committed and none was measured here (the machine is oversubscribed; a size number would be fiction).
Structural observations: default features pull `web-sys`, `js-sys`, `uuid(js)`, `serde_json`, `serde`; the lib is built
as `cdylib+rlib` with ~120 string-builder components compiled in; `counter_wasm.rs` uses the legacy `vdom` types with
`wasm_events`, not the patcher. Expect a counter app in the several-hundred-KB range before `wasm-opt`, which is two
orders of magnitude over Svelte/Solid **[K, unmeasured]**. Measuring this is a roadmap exit criterion, not an assumption.

### 2.4 SSR and hydration

`ssr.rs` renders legacy `vdom` to strings (now with tag validation, sanitised URLs, and a script-safe state JSON; before
this pass `</script>` in state broke out, tag names were unchecked, and streaming chunks split UTF-8). Hydration is a
script that stamps `data-hydrated`; there is no node-adoption: the client rebuilds from scratch. Streaming splits a
string. No suspense, no resource/async data. SSR of the canonical `VNode` goes through `simple_renderer::render_to_html`,
a second implementation. Two string renderers, neither hydrating.

### 2.5 Desktop

`desktop` feature = eframe 0.29 (egui, wgpu) + egui_dock; `desktop_renderer*.rs` walk `VNode` and emit immediate-mode
egui widgets. That is a different widget model from the HTML/CSS one the 120 components target: string-builder
components output HTML and cannot render on egui at all. `tauri` is an optional dependency, "legacy, not exercised". So
there is no coherent desktop story: web components for web, a parallel VNode-to-egui mapping for desktop (used by the
engine editor panels), nothing shared at the component level. egui's a11y is whatever AccessKit gives it **[K]**.

### 2.6 Mobile

`platform/ios.rs`, `android.rs`: types, no renderer. `mobile-ios` pulls `objc`/`cocoa`, `mobile-android` pulls `jni`.
There is no mobile story. The shortest honest path is a webview shell (Tauri 2 mobile or Capacitor) around the web build.

### 2.7 Accessibility

Only 13 of ~120 component sources mention `aria-`/`role=`; none manage focus (apart from `propertyeditor`,
`shortcutchord`). No focus trap in `modal`/`dialog`/`drawer`/`commandpalette`, no roving tabindex in `tabs`/`menu`, no
live regions for `toast`/`alert` beyond what individual files do. On the web, correct semantics are mostly a matter of
emitting the right elements and attributes, so this is cheap to fix per component and impossible to fix globally by
renderer choice. On egui the tree is whatever egui exposes. A custom canvas renderer would need to build the entire tree.

### 2.8 Theming, styling, routing, state, forms, data

- Styling: one `styles/components.css`; components use class names and some inline style strings. No design tokens at
  the type level, no scoped styles, no dark mode mechanism beyond CSS (`DESIGN_SYSTEM.md`, `STYLE_GUIDE.md` are guides).
- Routing: `routing.rs` is a table with `Router::navigate`, `FileBasedRouter` (filesystem scan, not wired to a browser
  history API in this crate), `AuthGuard`. The hash router actually used is embedded JS (`hashrouter`).
- State: signals, plus `undo_redo.rs` (command pattern; good, editor-oriented). No store/context/DI.
- Forms: `form`/`input` string builders; validation is per example. No form state model.
- Data fetching: JS `authfetch`/`jsonpost` runtimes (app-specific bearer-token code). Nothing in Windjammer.

### 2.9 Testing, DX

- Testing: strong in the places that were just built. `FakeDom` models browser blur-on-reparent; the patcher is
  property-tested; embedded JS runs under node against a stub DOM (`tests/js/dom_stub.js`, which caught a real
  shadowed-`load` bug that made memorized-list delete/save always throw); patcher logic runs as wasm32 under node via
  `wasm-bindgen-test`. Missing: any real-browser test, visual regression, a11y checks (axe), and tests of the `.wj`
  components beyond string assertions.
- DX: no hot reload, no error overlay, no dev server beyond `serve_wasm`. `build.rs` can regenerate via `wj` only with
  `WJ_REGEN=1` because regeneration corrupts the hand-maintained files. `src/components/generated/` contains 27 stale
  copies of core modules that exist in `src/` (signal, reactivity, renderer, ssr, routing, vdom, ...); tests run
  twice (`components::generated::ssr::tests` and `ssr::tests`) and the copies drift (the generated `ssr.rs` still has the
  old hydration-state bug). That is pure liability.

### 2.10 Security posture (after the last two passes)

Good: escape-by-default in `VNode` paths, SSR and patcher; `RawHtml` opt-in (now also at field level for the `*_html`
component fields); embedded runtimes no longer assign `innerHTML` directly except one audited sink. Remaining: every
compiler-generated string builder still concatenates unescaped (`escaping.md`), and `lkRender` output is trusted markup
passed through a sanitiser whose allow/deny list is ad hoc.

## 3. How the Windjammer language should shape UI authoring

Language facts that matter (compiler ADRs 0002, 0014, 0015, `pipe-operator.md`): inferred ownership/borrowing, explicit
`let mut`, `${}` string interpolation, pipe operator, closures as first-class values, no macros, `int`/`float` default
numerics, `T?`, `List/Map`, intrinsics instead of macros, no `unsafe`.

Consequences:

1. **No `view!`/`rsx!`/JSX macro, and the compiler design doc's JSX literal is a syntax-extension decision for the
   compiler team**, not something this crate can do. Leptos, Dioxus and Yew all lean on proc macros; that route is
   closed. Svelte and SwiftUI/Compose show the two macro-free alternatives: **(a)** a template language compiled by the
   toolchain (`.wjx`/component blocks, Svelte style), **(b)** a typed builder DSL with trailing closures (SwiftUI,
   Compose). (b) needs only language features (closures, method chaining, inferred ownership) and is available today;
   (a) needs compiler work but gives fine-grained reactivity almost for free (the compiler sees which expressions read
   which signals).
2. Inferred ownership removes the biggest Rust UI pain (cloning `Rc`/`Signal` handles into every closure: the current
   example `.wj` files contain `let todos_add = todos; let todos_toggle = todos` aliases to work around exactly this).
   A Windjammer UI should make signals `Copy`-like handles (id into an arena) so closures capture them freely, as Leptos
   does with arena-backed signals. That is a runtime decision here, plus a compiler guarantee that handle types are
   treated as `Copy` (request C3 in `DESIGN_DIRECTION.md`).
3. String interpolation `"${count} items"` is the natural template-expression syntax; in an interpolation-based text
   binding the compiler can see the dependency on `count`.
4. The pipe operator suits data shaping (`items |> filter(..) |> map(..)`), less so layout nesting.
5. Rust-isms in examples (`.to_string()`, `println!`) were what failed `test_todo_app_example_compiles` and
   `test_contact_form_example_compiles` (the compiler's W0006 "Rust leakage" error is correct; the example sources were at
   fault, fixed in this pass). The examples must also move to `println(...)`/interpolation per ADR-0014 as the compiler
   deprecates the `!` forms.

### 3.1 Proposed `.wj` component shape (builder DSL, usable today in spirit; template form later)

State, props, events, conditionals and keyed lists in a macro-free builder, in Windjammer syntax:

```windjammer
use windjammer_ui.*

struct TodoItem { id: int, text: string, done: bool }

component TodoApp(title: string = "Todos") {          // props with defaults
    let items = state([TodoItem { id: 1, text: "Learn Windjammer", done: false }])
    let draft = state("")
    let remaining = derived(|| items.get() |> count(|t| !t.done))   // memo

    fn add() {
        if draft.get().is_empty() { return }
        items.update(|list| list.push(TodoItem { id: next_id(), text: draft.get(), done: false }))
        draft.set("")
    }

    view {
        column(gap: 12) {
            heading(title)
            row {
                text_input(bind: draft, placeholder: "What needs doing?", on_enter: add)
                button("Add", on_click: add)
            }
            for item in items.get() key item.id {            // keyed list
                row {
                    checkbox(checked: item.done, on_change: |v| toggle(item.id, v))
                    text("${item.text}", strike: item.done)
                }
            }
            if remaining.get() == 0 { text("All done") } else { text("${remaining.get()} left") }
        }
    }
}
```

This is aspirational syntax for `component`/`view`/`state`/`derived` and trailing-block children; it requires compiler
features (section 5). Expressed with only today's language (closures + method chaining), the same tree is:

```windjammer
fn todo_view(items: Signal<List<TodoItem>>, draft: Signal<string>) -> VNode {
    column().gap(12).children([
        heading("Todos"),
        row().children([
            text_input().bind(draft).placeholder("What needs doing?"),
            button("Add").on_click(|| add(items, draft)),
        ]),
        each(items, |t| t.id, |t| row().children([ checkbox().checked(t.done), text("${t.text}") ])),
        show(|| remaining(items) == 0, || text("All done"), || text("${remaining(items)} left")),
    ])
}
```

`each` and `show` are the dynamic-region primitives (Solid's `<For>`/`<Show>`): they own a sub-scope, re-run only
their closure when their dependency changes, and key rows by id. Comparison: Svelte `{#each items as t (t.id)}`,
SwiftUI `ForEach(items, id: \.id)`, Compose `LazyColumn { items(items, key = { it.id }) }`. The builder form needs no
macros and keeps a plain function as the unit of composition; the `component ... view { }` form is the target once the
compiler can analyse it.

## 4. Strengths to keep

- `dom_patch` (port/fake/web split, property test) and `escape.rs`.
- `RawHtml` discipline and the escape-by-default policy.
- The test setups added now: FakeDom focus model, node DOM-stub for embedded JS, wasm-in-node.
- Editor-oriented pieces (`undo_redo`, `ecs_inspector_*`, profiler panels) as the actual consumer-facing value for the engine.
- `Signal/Computed/Effect` semantics as a base (needs batching, scopes, arena handles).

## 5. Weaknesses, ranked by cost of leaving them

1. Whole-tree rebuild + diff per frame; signals not wired to the render.
2. Two parallel component models (string builders vs VNode), plus a third legacy tree and proc macros.
3. Authoring from `.wj` is an integer-handle FFI; no template or builder DSL; example aliasing workarounds.
4. No coherent desktop (egui mapping) or mobile story; Tauri unexercised.
5. Accessibility is incidental.
6. Behaviour lives in JS strings; 17 hand-maintained generated files; 27 stale duplicated core modules.
7. No DX loop (hot reload, error overlay), no real-browser test, no measured size/perf numbers.
8. Domain (app-specific) components living in a general library.

## 6. Honest tradeoffs

- **Fine-grained vs VDOM.** Fine-grained wins on update cost and on boundary crossings, loses on simplicity of
  non-DOM renderers (desktop/native need either a retained scene graph with per-node setters, or a VDOM). A single
  tree IR (`VNode`) plus a binding layer that updates individual nodes covers both: Leptos/Sycamore do DOM-direct;
  Dioxus uses signals to schedule VDOM sub-tree re-renders and a mutation stream for renderers. Dioxus's design is the
  better fit when the renderer set includes non-DOM targets, Solid/Leptos's when the web dominates.
- **Rust/Wasm vs compiling `.wj` to JS.** The Wasm path pays size and boundary costs for DOM-bound UI. The compiler's
  JavaScript backend (Tier 2 per ADR-0007 proposed) would give Svelte-like output for the web target, but only
  if it passes conformance for the UI subset. Until that is real, Wasm is the only credible web target; plan for both.
- **Own renderer vs webview.** An own renderer (egui/Slint/wgpu) is attractive for game-editor panels that live next to a
  GPU viewport and is what the engine consumer actually uses today. It is a poor fit for web (size, a11y, text) and an
  open-ended cost for mobile (IME, a11y, platform conventions). A webview shell shares the web component set and its a11y.
  There is no single answer; there are two real consumers (web business apps, and the engine editor).
- **Scope.** The honest competitor set for web is Leptos/Dioxus/Sycamore, who have years of head start and teams. A
  differentiator has to be the language (inferred ownership, no macros, multi-backend), not a bigger widget library.

## 7. Verification gaps (do before relying on any number)

- Run js-framework-benchmark or an equivalent harness on a windjammer-ui keyed-list app (Chrome version pinned).
- Measure counter and todo wasm size: raw, after `wasm-opt -Oz`, gzip and brotli.
- Measure DOM-op throughput of the patcher on 1k/10k rows, with and without cached node handles.
- Check Tauri 2 mobile plugin coverage for what a downstream business app and the engine editor need.
- Re-verify Svelte 5, Solid, Qwik, egui/iced claims marked **[K]**.

## 8. Sources (retrieved 2026-10-11)

- React Compiler v1.0: https://react.dev/blog/2025/10/07/react-compiler-1 (stable 7 Oct 2025)
- MDN Element.moveBefore: https://developer.mozilla.org/en-US/docs/Web/API/Element/moveBefore ; Chrome blog https://developer.chrome.com/blog/movebefore-api
- Tauri 2.0 RC post: https://v2.tauri.app/blog/tauri-2-0-0-release-candidate/ ; 2026 third-party post https://www.mayhemcode.com/2026/07/build-mobile-and-desktop-apps-from-one.html (weak)
- Kotlin Multiplatform stability table: https://kotlinlang.org/docs/multiplatform/supported-platforms.html ; Compose Multiplatform 1.9.0 web beta https://blog.jetbrains.com/kotlin/2025/10/compose-multiplatform-1-9-0-compose-for-web-beta/
- Dioxus 0.7: https://dioxuslabs.com/learn/0.7/essentials/ui/hotreload ; https://alternativeto.net/news/2025/11/dioxus-0-7-brings-hot-patching-wgpu-rendering-and-axum-integration
- Flutter web renderers (size): https://docs.flutter.dev/platform-integration/web/renderers ; third-party measurement https://www.issoh.co.jp/tech/details/16727/ (uncertain)
- Leptos binary size guidance: https://book.leptos.dev/deployment/binary_size.html
- Slint AccessKit: https://slint.dev/blog/slint-1.1-released , https://slint.dev/blog/slint-1.6-released
- Benchmark summaries (third-party, inconsistent): https://www.pkgpulse.com/guides/solidjs-vs-svelte-5-vs-react-reactivity-2026 ; official data: https://krausest.github.io/js-framework-benchmark/ (not retrievable)
