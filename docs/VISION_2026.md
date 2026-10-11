# windjammer-ui vision, October 2026: one framework, one rendering model, every surface

- **Status:** Proposed. Replaces the direction in `DESIGN_DIRECTION.md` (signal regions over HTML + Tauri shell), which the
  owner rejected. Nothing here is accepted until the owner edits the ADR statuses (`docs/adr/`).
- **Date:** 2026-10-11
- **Scope of this change:** documents plus three small spikes (`spikes/`). No existing code was changed, no compiler or
  game repo was touched.
- **Evidence labels:** **[M]** measured by us on this machine in this change (numbers and caveats in section 9);
  **[V]** read in a web source this session (URL in section 10); **[K]** prior knowledge, not re-verified;
  **[U]** sources disagree or are weak. Vendor claims are marked as such. Nothing marked [K] or [U] should be quoted as fact.
- **Companion files:** `docs/adr/0001..0011`, `docs/vision/SAMPLES.md` (side-by-side code), `spikes/*/README.md`.

## 0. The mandate, restated, and the bar

One UI codebase, written in Windjammer, with one rendering model, running on web, desktop, iOS, Android, and inside the
game engine (HUD, menus, the editor). No WebView or JS runtime wrapper. It must beat the alternatives on ergonomics and on
performance, otherwise nobody has a reason to use it.

"Beat" has to be made concrete or it is a slogan. The bar this document commits to:

| Axis | Bar to clear (each is a milestone acceptance test in section 6) |
|---|---|
| Ergonomics | The counter, todo, async-fetch and form samples are shorter than Flutter/Compose/Leptos/SwiftUI **and** need zero `clone`, `move`, `Rc`, `key`, `useMemo` or `setState` ceremony. Hot reload under 1 s from save to pixels on desktop and web. |
| Frame time | 120 Hz budget (8.33 ms). A 10k-row virtualized list scrolls with p99 frame time under 8.33 ms on an M1-class laptop and under 16.6 ms on a mid-range Android phone. |
| Update cost | Changing one signal that feeds one text node costs O(1) in tree size (no tree diff, no re-layout of unaffected subtrees). Measured against Leptos and Dioxus on the same benchmark. |
| Startup | Desktop: window with first interactive frame in under 300 ms warm. Web: first paint of the app shell under 1.0 s on a 4G profile with cached shader/font assets. |
| Size | Web: a counter app under 400 KB brotli (text-less "lite" profile), a text-capable app under 2.0 MB brotli. Desktop: a text app binary under 12 MB. |
| Accessibility | Every widget in the stock set exposes a role, name, state and actions in an AccessKit-style tree; on web a DOM mirror passes automated axe checks; manual VoiceOver / NVDA / TalkBack passes are release gates. |

If the spikes and milestones show we cannot meet the size and a11y rows, section 8 lists the kill criteria.

## 1. State of the art survey (what people actually choose these for)

Honest caveat first: this survey is a desk survey. I fetched or searched primary pages where I could and say so; where
only third-party blogs were available the claim is labelled [U]. No competitor benchmark was run by us except the Leptos
signal-core comparison in section 9.

### 1.1 Per-framework summary

| Framework | Rendering model | Reactivity | Layout | Text | A11y | Size / startup | Weaknesses (that matter to us) |
|---|---|---|---|---|---|---|---|
| **Flutter** (Impeller, Skia, CanvasKit, Skwasm) | Own retained render-object tree, painted by own GPU renderer. Impeller (precompiled shaders, Metal/Vulkan) is the only renderer on iOS and, per the 2026 roadmap, is replacing Skia on Android 10+ [V roadmap; Skia-removal version numbers only from blogs, **U**]. Web: CanvasKit (Skia compiled to wasm, +1.5 MB) or Skwasm (+1.1 MB), per Flutter docs [V via ASSESSMENT_2026]; roadmap says Wasm is meant to become the default for web [V]. | Coarse: `setState`/InheritedWidget rebuild widget subtrees; rebuild is cheap because widgets are immutable descriptions diffed against Elements. Not fine-grained; Riverpod/Bloc layer on top. | Constraints go down, sizes go up, parent sets position (single-pass, O(n)). Predictable, no CSS. | Own text stack (HarfBuzz/ICU via Skia/libtxt); IME integrated per platform; web uses a hidden DOM input for IME. | Semantics tree mapped to platform a11y; on web it is a parallel DOM that is **off until a hidden "Enable accessibility" placeholder is activated** [V docs.flutter.dev]. | Web: multi-MB (a Wonderous demo measured ~4.2 MB compressed on CanvasKit, ~2.9 MB Wasm, third-party, **U**). Mobile apps tens of MB. | Web SEO/selection/find-in-page/a11y are structurally weak (single canvas); Firefox and Safari fell back to JS+CanvasKit rather than Skwasm in one July 2026 test [U]; Dart-only. |
| **Compose Multiplatform** | Skia (Skiko) drawing on Android/desktop/iOS canvas; Kotlin/Wasm for web (Beta since 1.9.0, Sep 2025 [V]); iOS stable since 1.8.0 (May 2025) [V]. | Snapshot state + compiler plugin: reads in composition record dependencies, recomposition skips unchanged groups (positional memoization). Fine-grained at composable-function granularity [K]. | Single-pass measure/place, constraints, intrinsics on demand [K]. | Skia paragraph + platform IME bridges [K]. | Maps to platform a11y on Android/iOS/desktop; web a11y basics added in 1.9 [V]. | iOS adds a few MB (Skiko) [K]; Kotlin/Wasm bundles ~MBs [K]. | Web is Beta; Kotlin ecosystem lock; recomposition model has its own pitfalls (stability, lambdas). |
| **SwiftUI** | Declarative value-type view tree diffed by identity; renders through Core Animation/Metal on Apple platforms only. | `@State/@Observable` (Observation macro, iOS 17+) tracks property reads per view body; invalidates views, not nodes [K]. | Propose-size / choose-size / place. | Native CoreText, native IME, free. | Best-in-class, free from system controls. | Tiny (system framework). | Apple only. The ergonomic gold standard for declarative UI; our syntax target. |
| **Slint** | Own retained scene; renderers: FemtoVG(OpenGL), Skia, software; `.slint` DSL compiled to Rust/C++/JS. | Property bindings with automatic dependency tracking: fine-grained, no VDOM [K]. | Own layout (constraints/stack/grid) in DSL. | Own, basic; RTL/complex scripts improving [K]. | AccessKit since 1.1, custom actions since 1.6 (May 2024) [V]. | Designed for embedded: runtime < 300 KiB RAM claim (vendor) [V search]; Android supported since 1.5 [V search]; iOS: not found [U]. | DSL separate from host language; smaller ecosystem; web via wasm with canvas, a11y weak [K]. **Closest philosophical match to what we want; its DSL is the thing to beat.** |
| **Iced** | Elm architecture; retained widget tree rebuilt from `view()` each update; wgpu or tiny-skia renderers. | Message passing (`update`/`view`); no signals. | Flex-like, own. | cosmic-text [K]. | Minimal [K]. | Moderate. | Whole-view rebuild; boilerplate messages; a11y/IME lag. |
| **egui** | Immediate mode, tessellated meshes via wgpu/glow. | None: UI is a function re-run every frame. | Immediate cursor-based; single-frame lag on auto-sizing. | Own glyph atlas, epaint; weak shaping/bidi/IME historically [K]. | AccessKit integration exists, quality bounded by immediate-mode ids [K, V partial]. | Small, instant to start. | Not good for rich text, large documents, complex a11y; great for tools. This is what the engine editor uses today (`desktop_renderer*.rs`). |
| **Xilem / Masonry / Vello / Parley / AccessKit (Linebender)** | Masonry: retained widget tree with AccessKit integration; Vello: GPU compute-based 2D renderer (wgpu); Parley: shaping/layout (HarfRust, ICU segmentation, bidi) with an `editing` module; `vello_cpu` and `vello_hybrid` (WebGL-capable, **not production-ready per its README** [V]) in `sparse_strips/`. Current crates: vello 0.11, parley 0.12, masonry 0.4, xilem 0.4, accesskit 0.25 [M: `cargo info`, 2026-10-11]. | Xilem: typed view tree + diff of view states (React/Elm-like, "View" trait), plus lenses/`Memoize`; not fine-grained signals [K]. | Masonry layout (flex-like) with Taffy-style pieces [K]. | **Best Rust text stack available** (Parley). | Masonry builds an AccessKit tree natively: the design target. | Not measured except our spike [M]. | All pre-1.0 (0.x). Mobile/web support lagging [K]. Our adoption candidates for paint+text. |
| **Dioxus** (+ Blitz / Dioxus Native) | Default: WebView (desktop/mobile) and DOM (web) via VDOM mutation stream. **Dioxus Native**: Blitz (Stylo CSS + Taffy + Vello + Parley) paints HTML/CSS on wgpu without WebView, shipped as experimental in 0.7 (Sep 2025); Blitz is "pre-alpha" with AccessKit "coming soon" and, by the team's own words, no performance focus [V]. Subsecond hot-patching of Rust across web/desktop/iOS/Android (state preserved; struct field changes not migrated) [V]. | Signals (0.5+) with component re-render scheduling; Stores for nested state (0.7) [V]. | CSS (flexbox/grid) via Taffy in Blitz; browser CSS elsewhere. | Browser/WebView text, or Parley in Blitz. | Browser's on web/WebView; AccessKit planned in Blitz [V]. | Wasm web bundles hundreds of KB to MBs [K]. | The closest Rust analogue. Its answer to "unified" is "use HTML/CSS as the universal UI language" (renderer swap). Hot-patching is the dev-loop benchmark to match. |
| **Leptos / Solid / Svelte 5 runes** | DOM-direct: the view is compiled to node-creation code; reactive effects mutate text/attrs of specific nodes. No VDOM. | Fine-grained signals (Solid, Leptos), compile-time runes → signals (Svelte 5). Updates only touch dependents. SSR + hydration (Leptos islands, SolidStart, SvelteKit). | Browser CSS. | Browser. | Browser (free, best). | Svelte/Solid: KBs; Leptos: hundreds of KB wasm; tune with opt-level z/LTO [V Leptos book]. | Web only. They set the **ergonomics-per-update-cost bar** and SSR/SEO bar for web. |
| **GPUI (Zed)** | Retained-ish: elements built each frame from views, GPU-rasterized with Metal / Blade(Vulkan) / DirectX; glyph atlas text. Pre-1.0, breaking changes; Windows/Linux support lagged macOS [V search, Register 2025]. Zed itself reported 1.0 in April 2026 (third-party, **U**). | Entities + `cx.notify()` re-render of views; explicit. | Tailwind-like flexbox styling in Rust builders. | Platform text systems (CoreText etc.) for shaping on each OS + own atlas [K]. | Weak historically [K]. | Fast, native. | No web target found [V search]; no mobile; tied to Zed's needs. Proof that GPU-rendered UI can feel faster than Electron; also proof of how long Windows/Linux/a11y/IME take. |
| **Makepad** | Own GPU shader-based widgets (SDF drawing), live-editable DSL, runs on desktop/mobile/web (wasm) [K]. A "2.0" line with a script-based DSL replaced `live_design!` per third-party skill docs [U]. | Event-driven, widget-local state; not signals [K]. | Own turtle layout. | Own text rendering with shaping; IME/a11y gaps [K]. | Weak [K]. | Small wasm, very fast startup [K]. | Idiosyncratic; a11y/IME; ergonomics. Its live design (edit shaders and layout while running) is the dev-loop inspiration. |
| **Rive** | Own vector/state-machine runtime and GPU renderer (C++); native FFI, wasm on web; Flutter runtime rewritten onto the C++ runtime in 0.14 with data binding and layout [V]. | State machines + data binding to view models. | Rive layout (flex-like) [V]. | Text runs. | Limited. | Small runtime. | Not a UI toolkit; the reference for **animation as declarative state machines authored in a tool**. We should be able to host Rive artboards, not compete. |
| **Servo / Blitz** | Servo: full web engine (WebRender). Blitz: modular subset (Stylo+Taffy+Vello+Parley), embeddable [V]. | n/a | CSS. | Parley. | Servo has a11y design docs in progress [V search]. | Large (Servo), unknown (Blitz). | Rendering "the web" is a different (much bigger) problem than a UI toolkit. We do **not** implement CSS. |
| **React Native new architecture** | JS/Hermes drives native views through JSI/Fabric (synchronous, no JSON bridge). | React (VDOM, React Compiler). | Yoga flexbox. | Native text views. | Native, free. | App size modest; JS engine adds ~MB. | Not a Rust/Wasm option; the model "native widgets" gives native feel and a11y free at the cost of per-platform divergence. Why people choose it: JS talent, native look. |
| **Lynx (ByteDance)** | Native views driven by a dual-thread JS architecture (UI thread + background logic thread), "instant first frame rendering", open-sourced March 2025, custom PrimJS engine; vendor claims 2-4x faster launches than web surfaces [U, vendor]. | React-like (ReactLynx) | CSS-ish subset | Native | Native | - | Small community; needs a host app [U]. Interesting idea: **split UI-thread / logic-thread** so input never waits behind app code. |

### 1.2 What users actually choose these for (and what that means for us)

1. **Flutter: pixel-identical UI and animation on every platform from one codebase, with one language and great tooling (hot reload,
   DevTools).** Weakness users accept on web: size and a11y/SEO. Flutter proves a custom renderer is *commercially viable* and
   shows exactly where it hurts. The gap we can exploit is a better web story than canvas-only (section 2.3) and fine-grained updates.
2. **SwiftUI / Compose: terse declarative syntax and native feel/a11y/IME for free.** Their syntax is the ergonomic bar. They
   win on "feels native" because they *are* native; a custom renderer must buy that back item by item (section 2.4).
3. **Leptos/Solid/Svelte: the cheapest possible update and the smallest output on web, plus SSR/SEO.** Fine-grained
   reactivity is now the consensus winning model for update cost [K; V partial in `ASSESSMENT_2026.md`, whose numbers are labelled uncertain].
4. **Slint / egui / Iced: Rust-native GUIs with small footprint, embedded targets, instant startup.** Chosen when the
   alternative is a browser engine. Weak on text, a11y, ergonomics.
5. **GPUI / Makepad: raw speed and visual control.** Chosen by people building one flagship app.
6. **Dioxus: "React in Rust" with hot-patching and everything on every platform, by leaning on HTML/CSS.**
7. **Rive: designers own motion.**
8. **Everyone's recurring pain**, from the survey and the assessment: (a) web accessibility/SEO/text selection for canvas
   renderers; (b) IME and international text; (c) startup and bundle size on web; (d) the dev loop; (e) per-platform polish
   (scroll physics, text fields, context menus) that takes years; (f) pre-1.0 churn.

### 1.3 The gap

Nobody ships all of: Rust-class performance, SwiftUI-class syntax, Solid-class fine-grained updates, Flutter-class
single-renderer consistency, **and** a credible web a11y/SEO story. Flutter has the renderer but coarse reactivity and weak web
text; Leptos has reactivity but only the DOM; Dioxus bets on HTML/CSS everywhere; Linebender has the best renderer/text parts
but Xilem is VDOM-style and pre-1.0. Windjammer's differentiator is the **language**: inferred ownership means closures
capture reactive handles without ceremony, which is the single biggest ergonomic tax in every Rust UI framework
(`clone()`/`move`/`Rc<RefCell<..>>`), and the interpreter gives a sub-second dev loop that no Rust framework can match without
dynamic patching tricks (compare Dioxus Subsecond).

## 2. Architecture decision

### 2.1 Options

| | A. DOM/HTML everywhere (Dioxus, Leptos, Tauri) | B. Native widgets per platform (RN, SwiftUI, MAUI) | C. Own GPU scene, one renderer (Flutter, Slint, GPUI) | D. C plus a semantic projection (recommended) |
|---|---|---|---|---|
| One rendering model | Only if desktop/mobile also use a browser engine (WebView, rejected) or a CSS engine (Blitz, pre-alpha) | No: N toolkits | Yes | Yes |
| Engine/HUD embedding | Poor (needs CSS engine in the game) | Impossible | Natural (draw into the engine's wgpu pass) | Natural |
| Web a11y / SEO / selection | Free | n/a | Weak (single canvas) | Good if we build the projection (section 2.3) |
| Pixel/animation control, 120 fps | Limited by browser | Limited by platform | Full | Full |
| Native feel | Web feel | Best | Must be rebuilt | Must be rebuilt (section 2.4) |
| Cost to build | Lowest | Very high (per platform) | High | Highest of the viable options, but each hard part has an existing-crate starting point |

**Decision (ADR-0001): D.** A single retained scene graph, rendered by our own GPU painter on wgpu (Vello-class vector renderer,
WebGPU first, WebGL2 and CPU fallbacks), with a **semantic tree** as a first-class output of every frame. The semantic tree
feeds AccessKit on native platforms and a DOM projection on web (a11y, SEO, find-in-page, IME and selection hosts).
DOM is a *projection*, never the primary render target. A true DOM render backend ("document mode" for content sites) is
explicitly deferred and gated by the kill criteria (section 8), because it forces a second layout engine to agree with the
first (CSS vs ours) and doubles the golden-image surface.

Why not A: the mandate rules out WebView wrappers, Blitz is pre-alpha and its own README lists AccessKit as "coming soon" [V],
and the game engine cannot host a browser engine. Why not B: it is N frameworks. Why not plain C: Flutter-on-web is the
cautionary tale (a11y off until a hidden placeholder is activated [V], multi-MB payloads [U]); solving that is the main
product differentiator on web, so it is designed in from day one rather than retrofitted.

### 2.2 Layer diagram and module contracts

```
  .wj source (component / view DSL)  --compile-->  Rust (release)  |  interpreter (dev, hot reload)
          |
   L0  wj_reactive     arena signal graph, Copy generational handles, memo/effect/resource, batch, scopes
   L1  wj_scene        retained node tree: layout props, paint props, semantics props, event handlers;
                       dirty flags {layout, paint, semantics}; fine-grained binding sites write props directly
   L2  wj_layout       flex + grid (Taffy-class; adopt) + stack/scroll/virtual-list (own)
   L2  wj_text         shaping/bidi/segmentation (Parley: adopt) + text editing model + IME state (own glue)
   L3  wj_paint        DisplayList -> Painter trait.  Backends: vello (GPU, wgpu), hybrid/WebGL2, vello_cpu (fallback + test oracle)
   L3  wj_a11y         SemanticTree -> AccessKit (native) | DOM projection (web)
   L4  wj_shell        window/surface/input/clipboard/dialogs per platform (winit; web canvas; iOS/Android host glue)
   L5  wj_widgets      stock widget set + design tokens + theming (written in Windjammer, on L1-L3)
   L5  wj_engine_bridge   renders the scene into a caller-provided wgpu pass; routes engine input (section 4)
```

Rules: lower layers never import upper ones (same discipline as `MODULE_MAP.md`); L0-L3 have no platform imports, so
they run headless in tests and in the interpreter; every layer has a fake (FakePainter, FakeClock, FakeShell, RecordingA11y).
That is what makes the testing strategy in section 6 possible, and it mirrors the hexagonal GPU-port boundary already used in
a 3D game engine host (a trait, typed generational handles, a recording fake).

### 2.3 The hard parts that sink custom-rendered frameworks, one plan each

| Hard part | Why it sinks frameworks | Plan | Build vs adopt | Evidence status |
|---|---|---|---|---|
| **Text shaping, bidi, line breaking, fonts** | Latin-only demos hide that complex scripts, bidi, emoji, fallback fonts and vertical metrics take years. | Parley (HarfRust shaping, ICU4X segmentation, fontique font matching). Smoke test passed for Hebrew/Arabic/CJK/emoji in one paragraph (9 glyph runs) [M]. Glyph rendering by Vello `draw_glyphs`. Embed one text subset font + lazy system-font fallback on web (via a font-access shim or bundled Noto subsets by unicode-range). | Adopt (Parley). Own: font cache policy, web font loading, text decoration/rich spans API. | Shaping 10k rows (~40 chars) = 8-49 us/row depending on machine load, i.e. 80-490 ms cold [M, noisy]: **must be incremental and cached per text node**. First `FontContext` + layout = 165-735 ms [M]: system-font enumeration is a startup problem; mitigate with lazy/async fontique init and an embedded default font. |
| **Text input, IME, selection, clipboard** | Custom text fields are where "Flutter-class" frameworks and games UIs break: composition, dead keys, CJK candidates, mobile keyboards, spellcheck, autofill, password managers. | Own editing model (caret, selection, undo) on Parley's `editing` module (present in parley 0.12 [M: source dir]). Platform IME is **never reimplemented**: per platform a thin "text input client" bridges the OS (winit IME events on desktop; hidden `<input>/<textarea>` host element on web, as Flutter does [K]; UIKeyInput/UITextInput on iOS; InputConnection on Android). Selection rendering is ours; selection *semantics* come from the focused text node's text+range, mirrored to a11y. | Own glue, adopt Parley editing core. | Highest risk item; first spike in M1 (section 6). |
| **Accessibility** | Pixel renderers have no native tree; retrofitting is painful (Flutter web: opt-in placeholder [V]). | Semantic tree is produced by the scene itself every frame (role, name, value, state, actions, bounds, focus, live-region) - widgets cannot be built without declaring it (a required trait method, section 3.7). Native: **AccessKit** (Windows UIA, macOS NSAccessibility, Linux AT-SPI, Android; winit adapter exists [V search: adapters listed]). Web: AccessKit has no web adapter in the results I found [U], so we write a **DOM projection** (`<div role=..>` mirror, incremental, off-screen but real elements; the approach used by Ply, a third-party project [V search]). Always-on, not behind a placeholder button, but lazily populated for subtrees in view to protect frame time. | Adopt AccessKit core+adapters; own web projection and iOS adapter glue (AccessKit has no iOS adapter listed [U]; map to UIAccessibility ourselves). | Slint reached AccessKit in 1.1 and custom actions in 1.6 (about 1 year) [V]: realistic effort sizing. |
| **SEO / SSR / hydration (web)** | Canvas apps are invisible to crawlers; Flutter prerender tools click the a11y placeholder in a headless browser [V search]. | **Semantic tree -> HTML on the server** (the same Rust code runs on the server, no layout needed for the skeleton): real headings, links, text, landmark roles; the client **adopts** these DOM nodes as its projection (hydration = attach scene node ids to existing elements, no rebuild). Result: crawlers, no-JS readers, link previews and find-in-page get real content; first contentful paint is HTML before wasm loads. Router uses real URLs. | Own. Shares code with the DOM projection (one implementation, two uses). | Design only; verify in M3. SEO ranking of such pages is not something we can measure here. |
| **Native feel** (scroll physics, text field behavior, context menus, haptics, system fonts, dark mode, dynamic type, reduced motion, platform key bindings) | Users notice within seconds; the long tail is thousands of details. | A `Platform` profile object chosen at startup supplies physics curves (iOS rubber-banding vs Android overscroll), key bindings (Cmd vs Ctrl), text-field behaviors, metrics, and system preference signals (as **signals**, so the UI reacts). Stock widgets read the profile via theme tokens. Native menus, file dialogs, share sheets, notifications, pickers: *real native* via the shell trait, not painted. Accept "consistent first, native where it is cheap and visible" as a stated policy. | Own. | Never "done"; budgeted per release (section 6). |
| **Input and gestures** | Hit testing with transforms/clips, gesture arena disambiguation (tap vs drag vs scroll), multi-touch, pointer capture, hover, stylus, keyboard focus order. | Scene-owned hit testing (reverse paint order with clip/transform stack), a Flutter-style gesture arena as a library on top of raw pointer events, focus tree separate from hit tree, all deterministic given an event log (replayable in tests). | Own (small, well-understood). | - |
| **Animation** | Janky if tied to rebuilds; hard to test; interacts badly with layout. | Animations are **values**: `Animated<T>`/springs/keyframes are reactive sources driven by a frame clock (an injected `Clock`), applied to *paint/transform properties on the compositor path* (no re-layout, no re-measure). Layout animations use FLIP-style snapshot transitions. Tests advance a `FakeClock`. | Own. | - |
| **Scrolling physics and virtualization** | 10k rows: naive implementations re-lay-out and re-paint everything. Our spike: submitting all 10,000 rows (no culling) took 113-343 ms/frame (median 150-185) on M1 Pro vs 2.5-4 ms for a 40-row window [M]. | Virtual list is a core primitive (`List(items, key, height_hint)`), not a widget library afterthought; offscreen children are not instantiated, scroll position is a signal, cached text layouts and cached encoded sub-scenes per row (Vello scenes can be appended [K]). Scroll runs on the compositor thread where the platform allows (see below). | Own. | The measured 10k number is why culling is mandatory, not optional. |
| **i18n / RTL / locale** | Often bolted on late: mirrored layout, number/date/plural formats, locale-aware sorting, text direction in widgets. | Layout uses logical `start/end` (never left/right) from the first widget; ICU4X (already a Parley dependency [M: `icu_segmenter` fetched]) for plural/number/date; message catalogs as typed Windjammer values with compile-time key checking (section 3.8). | Adopt ICU4X; own catalog tooling. | - |
| **Bundle size (web)** | Flutter-class payloads (multi-MB) lose to HTML on first load. | Budgets: counter (no text) < 400 KB brotli; text-capable < 2.0 MB brotli (**measured baseline for wgpu+vello+parley is in section 9**); feature-split `lite` profile; lazy-load shader/pipeline set, fonts by unicode-range, route-level code splitting; `opt-level=z`, LTO, `wasm-opt -Oz`; CI size gate. | - | **[M]** see section 9. |
| **Startup time** | Shader compilation and font enumeration dominate, not our code. | Spike [M]: first run Renderer::new (compiles Vello compute pipelines) took 1.0-1.4 s cold, 20-120 ms warm (OS shader cache); `Request device` 26-1670 ms; first `FontContext` 165-735 ms. Mitigations: `wgpu::PipelineCache` (Vello exposes `RendererOptions.pipeline_cache` [M]), precompiled-pipeline cache shipped per platform, async init while the semantic skeleton (SSR HTML / splash scene) is already visible, embedded default font, lazy system fonts. Budget: first interactive frame < 300 ms warm desktop. | - | The cold 1+ s is a real risk; it must be solved before M2 exits. |
| **Memory** | Glyph atlases, scene buffers, per-node overhead. | Arena nodes (no per-node `Rc<RefCell>`), budget 200 bytes/node target, 10k-row list under 60 MB RSS steady-state (to be measured, M2). | - | Not measured. |
| **120 fps budgets** | Missing frames = jank; JS/wasm GC pauses, GPU stalls, layout storms. | Frame budget 8.33 ms split: input+reactive 1.0, layout 1.5, scene encode 1.5, GPU 3.0 (headroom 1.3). Spike [M]: virtualized-list scene encode 0.05-0.06 ms; render+GPU 1.4-4 ms median 2.5-4 (noisy machine; Metal, M1 Pro). Dirty-flag discipline: only changed subtrees re-layout; paint property changes (opacity/transform) skip layout; retained per-layer encoded scenes. Adaptive: drop to 60 fps when the platform's `ProMotion` is off; always frame-pace to the display link. A frame-time overlay + trace is a stock dev feature (the engine already has frame-trace bridges, `src/frame_trace_ffi.rs`). | - | Needs a real-display, in-window test (the spike was headless offscreen, no vsync, no compositor). |
| **Testing** | Visual UIs are hard to test; "it looks right" does not scale. | Section 6.4: golden images from an *independent* software rasterizer path; a11y-tree snapshots; deterministic `FakeClock` + event-log replay; differential tests GPU vs CPU painter and native vs wasm; layout oracle vs Taffy/Chrome flexbox for a flexbox subset. | - | - |

### 2.4 Native feel, concretely

We do not pretend a custom renderer is native. The policy (ADR-0008): **paint ours, delegate system services**. Native
code (or the platform's system UI) is used for: menus and context menus where the OS has a canonical one, file/colour
pickers, share sheets, notifications, text *input service* (IME/autofill/spellcheck), accessibility, clipboard, drag
and drop, window chrome. Everything inside the window is ours and themeable with platform presets ("iOS-like",
"Material-like", "Fluent-like", "Plain") selected from the same widget code via tokens.

### 2.5 Thread model (new vs most frameworks)

Two logical threads (Lynx's idea [U, vendor]; Flutter has UI/raster split [K]): the **UI thread** runs app logic, reactive
graph and scene mutation; the **render thread** owns the wgpu device and consumes immutable display-list frames. Scroll
and animation-of-paint-properties are executable by the render thread alone on a snapshot. On wasm, this needs
SharedArrayBuffer + cross-origin isolation (an adoption cost, section 8), so web v1 is single-threaded with the same code
structure, and threading is an optimization later. The interpreter runs on the UI thread.

### 2.6 Web rendering strategy

WebGPU is shipped in Chrome/Edge 113+ (desktop), Android 121+ on supported GPUs, Safari 26 (Sep 2025), and Firefox on
Windows (141) and Apple Silicon macOS (147+); Intel Mac and Linux Firefox not yet [V search; secondary sources, global coverage
number not verified, one site claims ~76% [U]]. So: WebGPU + Vello primary; **WebGL2 fallback via `vello_hybrid`, which its own
README says is not production-ready [V]** -> plan for our own small fallback painter if it is not ready at M3 (rects, rounded rects,
glyph atlas quads, clips, gradients cover >95% of UI). Last resort: Canvas2D/`vello_cpu` painter (also the
test oracle). Capability probing selects the painter at startup; the semantic projection is painter-independent.

## 3. Language-level ergonomics: why Windjammer can be the better choice

All syntax below is **proposed**; none of it parses today. Every claim in this section is conditional on the compiler work in section 7
(R1-R4 are P0). Side-by-side code for six scenarios is in `docs/vision/SAMPLES.md`; the verdict table is in 3.10.

### 3.1 Declarative syntax: a `component` block with a view DSL (no macros)

The language has no macros (ADR-0014/0015), so Leptos/Dioxus-style `view!`/`rsx!` is closed. The two macro-free
options are (a) a compiler-recognised `component` form, and (b) a typed builder with trailing closures. We choose (b) as
the always-available lowering target and (a) as sugar that lowers to it, so the lowering is inspectable and the interpreter,
the Rust backend and the wasm backend all see ordinary calls (the pipe operator is the precedent: it desugars in one place in
the parser, `docs/design/pipe-operator.md`).

```windjammer
component Counter(step: int = 1) {
    state count = 0                       // reactive cell; reads/writes need no .get()/.set()
    derived label = "Count: ${count}"     // memo; dependencies inferred

    view {
        Column(gap: 8) {
            Text(label)
            Button("Add ${step}", on_click: || count += step)
        }
    }
}
```

Lowering (conceptual): `Column(gap: 8) { ... }` is `Column(gap: 8, children: [...])`; named arguments and a trailing
block of children are the language feature R2. `"Count: ${count}"` inside `view`/`derived` is a *structured* interpolation (R4)
so a binding can target just the text node.

### 3.2 Automatic fine-grained reactivity inferred by the compiler

Two layers, deliberately separable so we can ship value early:

1. **Runtime auto-tracking** (Solid/Leptos/MobX style, needs no compiler analysis): reads of `state`/`derived` inside a
   `view` expression, `derived` or `effect` register the enclosing binding site as a subscriber. Reads in event handlers are
   untracked. The scene node for `Text(label)` gets one effect that writes its string prop; nothing else re-runs. This
   works as soon as R1 (Copy handles in closures) and R2 (trailing blocks) exist.
2. **Compile-time lowering** (Svelte-5-runes style, an optimization): the compiler sees which `state` a binding expression
   reads, emits static subscription lists, hoists constants out of bindings, and removes dynamic tracking for fixed-shape
   expressions. Needs R3/R4 plus structured lowering. It is an optimization, not a requirement for correctness, which
   keeps the risk low.

Control flow is first-class so lists and conditionals are also fine-grained: `for item in items key item.id { ... }` and
`if`/`match` inside `view` lower to keyed list and switch primitives (like Solid's `<For>`/`<Show>`) instead of rebuilding
the parent. My spike (section 9) implemented the reactive core as an arena with Copy handles: it passes three semantic tests
(glitch-free diamond, equality cutoff, dynamic dependencies) and is 5x-1000x faster than Leptos's `reactive_graph` on four
microbenchmarks **in our harness** (caveats there; not a statement that Leptos is slow in real apps).

### 3.3 Inferred ownership: no `clone`/`move`/`Rc` in view code

The recurring tax in every Rust UI framework is capturing state in `'static` closures. Here:

- `state` handles are `Copy` at the type level (R1: `@handle` attribute for ownership inference): `{ index: u32, generation: u32 }` into the arena.
  Closures capture them freely; reading the *value* borrows from the arena inside a tracking scope.
- Non-handle captures in handlers (a `String`, a `List`) are cloned or moved by the existing ownership inference (ADR-0001: the IR
  solver is the single writer of ownership truth), and the compiler picks `move`. The author never writes it.
- Props are passed by inferred borrow/move; large props are shared via reference counting chosen by the compiler only when
  the closure must outlive the call (the analysis already decides `&`/`&mut`/owned for parameters).

Honest caveat: the interpreter currently captures closures *by reference* while compiled Rust copies on `move`
(`docs/INTERPRETER.md`, findings list: `let mut c = 0; let mut f = || { c += 1 }` prints 0 compiled, 2 interpreted). A UI framework lives
and dies on closure semantics, so unifying this is P0 (R12) before hot reload can be trusted.

### 3.4 Compile-time layout and style checking

What can be checked statically, and what cannot:

| Check | How | Feasibility |
|---|---|---|
| Typed units and tokens: `gap: 8` (logical px), `Pct(50)`, `color: theme.accent` (a token enum, not a string) | ordinary types; no CSS strings | Easy, typed props |
| Child-slot constraints: `Expanded` only inside `Row/Column/Flex`; `Tab` only in `Tabs`; `Positioned` only in `Stack` | typed child lists / marker traits | Moderate (needs trait objects + generics, R10) |
| Unbounded constraints: a `Column` inside an unbounded-height `Scroll` with an `Expanded` child (Flutter's "unbounded height" error) | a conservative lint over the static view tree, falling back to a debug-mode runtime check with a source-mapped message | Partial; the lint is approximate; the runtime assert is exact |
| Accessibility: every `Image` has `label` or `decorative`; every `Button` has an accessible name | required props / lint | Easy; makes a11y a compile error |
| Exhaustive state: `match` on UI state enums | already in the language | Free |
| i18n: missing translation keys, wrong placeholder types | typed message catalogs | Moderate |

### 3.5 Hot reload via the interpreter, Rust/wasm for release

Measured fact from the compiler docs: `wj run hello.wj` (transpile + cargo + run) took 13.9 s under load; `wj run --interpret` 0.016 s
(`docs/INTERPRETER.md`). That is the dev loop advantage, and also the constraint: the interpreter is a tree walker about two orders of
magnitude slower than compiled code and does **not** support extern/FFI, async, `Rc/RefCell`, UI modules or generic static dispatch
(same document). So the design is:

- Dev profile: **host runtime in Rust** (scene, layout, text, painter, reactive core, widgets compiled once, as a prebuilt
  `wj-ui-host` binary / wasm module); **user code interpreted**. User components call host functions through a registered
  host-function table (R9: interpreter host-call interface, replacing "extern unsupported"). Component bodies run at interpreter speed
  but they only run on state change and create/bind nodes; layout, painting and text stay native. This is why the 2-orders-of-magnitude
  penalty is tolerable (and is measurable, M2 criterion: 10k-row list creation under interpretation < 1 s, scroll unaffected).
- State preservation: `state` cells are keyed by (component path, declaration name, type hash); on reload, matching keys keep values,
  changed types reset with a visible toast. Same trade-off as Dioxus Subsecond (struct field changes are not migrated automatically [V]).
- Web dev: same host as a wasm module, user code interpreted in-page via a wasm-compiled interpreter (the interpreter is Rust; build to wasm
  is an assumption to verify, not tested here) or streamed from the dev server.
- Release: `wj build --target rust` for native/mobile and `--target wasm` for web, optimized with LTO; differential tests prove
  interpreter and compiled behavior agree on the UI corpus (the compiler team's differential harness is the model).
- Error overlay with source spans (R6 source maps).

### 3.6 Typed props, state management, routing, forms, theming, animation, previews

- **Typed props**: `component Card(title: string, body: View, on_close: fn())`; defaults, required props, children slots as `View` parameters; no stringly props.
- **State**: three scopes without libraries: `state` (component), `provide`/`inject` typed context (compile-time checked key), and `store`
  (a `@derive(Store)` struct whose fields are individually reactive, like Dioxus Stores [V] and Solid stores). `undo_redo.rs` (command pattern,
  exists today) becomes the editor undo service.
- **Routing**: routes are an enum (`enum Route { Home, User(id: int), NotFound }`) with a derived URL parser/printer; `navigate(Route::User(7))` is type checked,
  deep links on mobile and the browser URL use the same table; route-level code splitting on web; SSR renders by route.
- **Forms**: `form Signup { field email: string = validate(required, email); field age: int? = validate(range(13, 120)) }` lowers to a form-state struct
  with `value/error/touched/dirty` signals per field and a `submit()` that is only callable when valid (type-level `Valid<Signup>`).
- **Theming**: design tokens are a typed struct tree (`Theme { color: Palette, space: Scale, radius, type_ramp, motion }`), provided by context,
  switchable at runtime (light/dark/high-contrast/platform preset) as a signal, so a theme switch is a fine-grained update, not a rebuild.
- **Animations as declarative values**: `opacity: if open { 1.0 } else { 0.0 } |> animate(spring(stiffness: 300, damping: 30))`; `animate` returns a reactive value
  driven by the frame clock, targeting compositor-only properties. Lists: `for item in items key item.id enter: fade_in() exit: slide_out()`.
  The pipe operator (`|>`) makes animation modifiers read naturally and is already in the language (`docs/design/pipe-operator.md`).
- **Previews**: `@preview fn card_loading() -> View { Card(...) }` registers a named story rendered in a gallery window (hot reloaded) and automatically becomes
  (a) a golden-image test and (b) an a11y-tree snapshot test (section 6.4). One declaration, three uses.

### 3.7 Widgets must declare semantics

The widget trait has a required `semantics(&self) -> Semantics` (role, name, value, actions). The stock widgets implement it; a
custom widget that does not is a compile error. This is the structural answer to the finding in `ASSESSMENT_2026.md` section 2.7
(13 of 120 components mention aria; no focus management): accessibility cannot be forgotten, only deliberately marked `decorative`.

### 3.8 Typed i18n

`messages Strings { greeting(name: string) = "Hello, ${name}" }` per locale file, checked at compile time for key and placeholder
parity; plural/gender via ICU4X; direction-aware layout (`start`/`end`).

### 3.9 Async data

`resource user = fetch_user(id)` (id is a tracked read; changing it cancels and re-fetches) exposes `user.loading`, `user.error`, `user.value`; `Suspense`
and `ErrorBoundary` are ordinary components. Needs async in the compiled targets (ADR-0004 accepted a surface) and, for dev, an interpreter
story (async is unsupported in the interpreter today): R8.

### 3.10 Where Windjammer wins and loses (honest scorecard vs the samples)

| Scenario (SAMPLES.md) | Wins | Loses / risks |
|---|---|---|
| Counter | Fewest tokens of all five; no `setState`, no `.get()`, no `useState`, no `move` | Needs R1-R4 to exist; Compose/SwiftUI are nearly as short and real today |
| Todo list | Keyed `for` with fine-grained row updates; no `clone` ceremony vs Leptos; no `Key`/`setState(() {})` vs Flutter | Mutating a list element through reactive state needs a defined rule (store fields vs whole-list set); this needs a design spike |
| Async fetch | `resource` has loading/error/value in one line; auto-cancel | Needs async in interpreter and wasm; Leptos `Resource`/Compose `produceState` are real today |
| Form with validation | Declarative `form` + type-level validity; a11y labels required | The most speculative feature; a form DSL is language-level work (R5) |
| Animated list | Enter/exit/reorder as declarative modifiers, compositor-driven | SwiftUI and Flutter `AnimatedList`/implicit animations are mature; ours needs FLIP layout transitions built |
| Game HUD | Same code as the app UI, bound to engine signals, rendering in the engine pass | egui/Bevy UI exist today and are simple; the engine needs the bridge (section 4) |
| Tooling/ecosystem/IDE/docs/hiring | - | Flutter, Compose, SwiftUI win outright for years; a pre-1.0 language has no IDE completion for DSL blocks, no package ecosystem |

Summary: the ergonomic edge is real but narrow. It exists *only* if the compiler delivers inferred closure capture and trailing-block syntax; if those slip, we are a
worse Leptos with a worse ecosystem. That is why they are the first milestone and a kill criterion.

## 4. Game-engine integration

Goal: the HUD, menus, and the editor are the same framework as the app UI, drawn inside the engine's frame, not in a separate window or overlay process.

### 4.1 One tree, the engine's wgpu pass

- The engine host already owns a wgpu device, queue and surface, and exposes GPU functionality through a trait-based port (`GpuPort`: typed generational handles, a
  device-less recording fake, an adapter over the real device) and a typed host/shader struct layout contract (std140/std430 validation of host structs against WGSL).
  Both are private-repo internals in the engine host; the pattern is what matters. `wj_engine_bridge` consumes the engine's device and queue **by handle** and records into a
  caller-supplied `wgpu::TextureView` plus encoder, i.e. the UI is composited as a last pass over the 3D frame, or into an offscreen texture that a 3D material can sample
  (in-world screens, holographic UI).
- Vello renders via compute into a storage texture, then a blit pass composites with the scene. Because the renderer is Vello-class compute, UI cost is GPU compute time added to the
  frame; measured standalone: 2.5-4 ms median for a 40-row virtualized list at 1280x720 on an M1 Pro, headless [M]. A typical HUD is far smaller, but the compute-shader cost is *shared
  with the 3D frame*, which this spike did not measure. M6 must measure UI cost inside a real 3D frame and set a hard budget (target <= 1.0 ms for a HUD at 1080p on the reference GPU).
- A second, cheaper path for HUDs: a **quad/glyph-atlas painter** (same `Painter` trait) that draws UI as instanced triangles in the engine's own render graph (no compute, no storage
  texture). Needed anyway for the WebGL2 fallback (section 2.6), so it is built once and serves both.
- Immediate vs retained: the UI is retained (scene graph, dirty flags); the *engine* calls `ui.frame(dt, input, target)` once per frame, which is an immediate-style integration point. A frame with no
  state change and no animation re-encodes nothing and reuses the last display list. Editor panels that need immediate-mode ergonomics (debug inspectors) can still be written as ordinary components; no egui-style per-frame widget calls are needed.
- Input routing: the engine owns raw input and forwards events with a `consumed` result (`UiInput::feed(event) -> Consumed`). The UI reports `wants_pointer`/`wants_keyboard` (derived from hit-test and focus) so
  gameplay input is suppressed only when a widget captures it. Gamepad/focus navigation uses the same focus tree as keyboard accessibility (directional focus).
- Determinism: the engine may drive a `FakeClock`/replayed input log, enabling replay and automated playtests of menus (the same mechanism as UI tests, section 6.4).
- Signals as the engine bridge: engine-side state (health, ammo, quest log) is exposed as read-only reactive sources updated at a defined tick; the UI never calls back into gameplay except through typed commands.
  Cross-thread: the engine mutates a double-buffered snapshot; the UI thread applies it at frame start (one batch = one reactive flush).

### 4.2 The editor on the same framework

The existing editor panels (inspectors, profilers, node graphs, curve editors, behaviour-tree views in `src/components_wj/`) currently reach desktop through an egui walker (`desktop_renderer*.rs`)
that shares nothing with the web components (`ASSESSMENT_2026.md` 2.5). Under this vision they are ordinary components. The editor therefore runs as a native desktop app, in the browser (wasm + WebGPU),
and docked *inside* the running game window, with identical code. This is also the best dogfooding case: the editor is a large, text-heavy, a11y- and IME-sensitive application and will find the problems the demo apps hide.

### 4.3 What to reuse and what not

Reuse: the port/fake/recording pattern, the layout-contract idea (generate Rust-side `#[repr(C)]` mirrors of display-list structs from the WGSL source and fail loudly on mismatch: the UI painter has the same
host/shader struct hazard), the frame-trace bridge (`frame_trace_ffi.rs`) for a UI frame-time overlay. Do not reuse: egui as a dependency of the UI path (keep only until M6 for parity of existing panels).

## 5. Platform plan

### 5.1 The thin shell trait

```
trait Shell {   // one implementation per platform; everything above this line is shared
    surface() -> wgpu::Surface/Texture target        window_size/scale/safe_area as signals
    events() -> pointer, touch, key, text-input (IME composition), focus, lifecycle, system prefs (theme, locale, text scale, reduced motion)
    clipboard, drag_drop, cursor, haptics, share, open_url, file_dialog, notifications
    a11y_adapter(tree_update)       // AccessKit update sink, or DOM projection on web
    run_frame_loop(callback)        // display-link / requestAnimationFrame / engine tick
}
```

### 5.2 Per target

| Target | Window/surface | Text input & IME | A11y | Clipboard/dialogs | Build and adopt |
|---|---|---|---|---|---|
| **Web (wasm)** | `<canvas>` + WebGPU (primary), WebGL2 via hybrid/own quad painter, canvas2D/CPU last | hidden `<textarea>`/`contenteditable` host focused per text field; composition events mapped to our editor; mobile keyboards via the same host | DOM projection (section 2.3), SSR skeleton, landmarks | async Clipboard API, `<input type=file>`/File System Access | wgpu (webgpu/webgl backends), a tiny own JS shim (< 20 KB target) written **without wasm-bindgen** if R7 delivers wasm imports; until then wasm-bindgen is acceptable [spike used it]. Fonts: bundled subset + `queryLocalFonts` optional. |
| **macOS** | winit window + wgpu Metal; later CAMetalLayer-in-NSView for embedding | winit IME (`Ime` events) now; NSTextInputClient direct if winit's coverage is insufficient (candidate window positioning, reconversion) | AccessKit macOS adapter (via accesskit_winit) | native NSOpenPanel, NSPasteboard through a small own native layer (objc2) or `rfd`/`arboard` crates | winit, wgpu, accesskit, rfd, arboard (adopt). |
| **Windows** | winit + wgpu (DX12/Vulkan) | winit IME; TSF behaviors verified early (known tricky) | AccessKit UIA adapter | same crates | same. |
| **Linux** | winit + wgpu (Vulkan), X11 and Wayland | winit IME (ibus/fcitx via text-input-v3 on Wayland) | AccessKit AT-SPI adapter | same crates; xdg-portal dialogs | same. Wayland decorations and fractional scaling are known rough edges **[K]**. |
| **iOS** | UIKit host view with CAMetalLayer; shell in Swift (or Rust via objc2) as a static library linked by a generated Xcode project | a hidden UITextInput-conforming view (UIKeyInput/UITextInput: marked text, selection rects) | UIAccessibility elements generated from the semantic tree (no AccessKit iOS adapter found [U]; own work) | UIPasteboard, UIDocumentPicker, share sheet | winit's iOS backend is an option for windowing, but text input/a11y need our own UIKit glue; decide at the M7 spike. wgpu Metal. |
| **Android** | `NativeActivity`/GameActivity or a `SurfaceView` in a thin Kotlin host; Vulkan via wgpu (GLES fallback) | `InputConnection` via a hidden `View` (Kotlin shim + JNI) | AccessKit Android adapter (exists in the adapter list [V search, verify]) via `AccessibilityNodeProvider` | ClipboardManager, SAF file picker | android-activity + winit Android backend; Kotlin shim; cargo-ndk. Vello on mid-range Mali/Adreno GPUs is the **largest performance unknown** (compute-heavy renderer; no measurement here). |
| **Game engine** | caller-provided wgpu view (section 4) | engine forwards text events, or in-engine IME not supported in v1 for HUD | in-game a11y optional; editor yes (OS tree) | n/a / host | `wj_engine_bridge`. |

### 5.3 Build vs adopt (decisions with reasons)

| Component | Decision | Reason |
|---|---|---|
| GPU API | **Adopt wgpu** | One API for Metal/Vulkan/DX12/WebGPU/GLES; the engine already uses it. |
| 2D vector renderer | **Adopt Vello** (GPU), `vello_cpu` (fallback/oracle); **write own quad painter** for WebGL2 and engine HUD | Vello is the best-fit modern Rust renderer, but `vello_hybrid` is not production-ready [V] and compute shaders are heavy on mobile/WebGL; own simple painter covers the 95% case and gives a controlled fallback. |
| Shaping/bidi/segmentation | **Adopt Parley** (HarfRust + ICU4X + fontique) | Years of work we should not repeat; actively developed (0.12 now) [M]. |
| Text editing model | Adopt Parley editing, **own** selection/IME/undo glue | Platform specifics. |
| Layout | **Adopt Taffy** (flex/grid) for the common case; **own** stack/scroll/virtual list/constraints-based boxes | Taffy 0.14 has an MSRV of 1.71 and is widely used [M: `cargo info`]; but we want Flutter-style constraints for custom widgets, so own both a "box constraints" protocol and a Taffy bridge. Decision pending ADR-0002 layout spike (see M1). |
| Windowing | **Adopt winit** (0.31 beta now [M]) for desktop and Android; own iOS/web shells | Ubiquitous; text input and a11y glue on top. |
| A11y | **Adopt AccessKit** core + adapters; **own** web projection and iOS mapping | See 2.3. |
| Reactive core | **Own** (arena, Copy handles) | Spike shows the design works and it is what makes `@handle` and the compiler lowering possible; adopting `reactive_graph` would tie the ownership story to Rc/Arc handles. |
| Scene graph | **Own** | Core IP. |
| Animation/gesture | **Own** | Small, must fit the scene and clock. |
| Clipboard/file dialogs | **Adopt** `arboard`/`rfd` where adequate | Commodity. |
| Images/SVG/Lottie/Rive | **Adopt** `image`, `resvg`/Vello SVG, optionally Rive runtime | Not our differentiator. |
| Networking/async | Windjammer stdlib (ADR-0008 portable core) with capability externs | Keeps UI code portable. |

## 6. Existing code, roadmap, acceptance criteria and verification

### 6.1 Existing windjammer-ui code: keep, migrate, delete

| Path | Fate | Reason |
|---|---|---|
| `src/dom_patch/` (keyed diff, `DomPort`, `FakeDom`, delegated events, property tests) | **Migrate** into the web DOM *projection* (semantic mirror); keep the tests | Correct, well-tested patcher with a clean port; the projection needs exactly keyed incremental DOM updates. |
| `src/escape.rs`, `safe_dom_js.rs` | **Keep** | The projection and SSR skeleton still emit HTML; escape-by-default is mandatory. |
| `src/simple_vnode.rs` | **Migrate** into the SSR/semantic-HTML node type | Small canonical tree for HTML output. |
| `src/signal.rs`, `reactivity*.rs`, `signal_ffi.rs` | **Replace** with `wj_reactive` (spike core) | Three overlapping modules; no batching; handles not Copy. Keep tests as behavior oracles. |
| `src/routing.rs`, `undo_redo.rs` | **Migrate** (typed routes; undo service for the editor) | Reusable logic. |
| `src/components_wj/*.wj` (~120 string-builder components) | **Rewrite incrementally** in the new DSL as widgets once M2 exists; **delete** each string-builder version when its replacement passes parity tests. Domain-specific business/finance components move out to the apps that use them (`EXTRACTION_PLAN.md`). Editor panels are rewritten first (they are the dogfood). | They emit HTML strings: wrong output model for a scene renderer. |
| `src/components/generated/` (stale copies of core modules, 17 hand-maintained JS runtimes) | **Delete** (stale copies immediately; JS runtimes with their components) | Pure liability (`ASSESSMENT_2026.md` 2.9). C10 becomes moot. |
| `vdom.rs`, `renderer.rs`, `runtime.rs`, `ssr.rs`, `platform/*`, `vnode_ffi.rs`, `component*.rs` | **Delete** after M2 | Legacy models; SSR re-implemented over the semantic tree. |
| `desktop_renderer*.rs`, `app_reactive_eframe.rs`, `app*.rs` (egui/eframe) | **Keep until M6**, then delete | Existing editor depends on it; do not break it before parity. |
| `windjammer-ui-macro` | **Delete** | Macros unusable from `.wj`; replaced by compiler support. |
| `tests/js/`, wasm-bindgen-test harness | **Keep** the harness idea for the projection | Real DOM behaviors. |
| `styles/*.css` | **Delete** (tokens replace) after web projection ships | Scene renderer does not use CSS. |

### 6.2 Staged roadmap with measurable acceptance criteria

Each milestone is also a go/no-go gate (section 8). "Compare" means run the same app in the named frameworks on the same machine, pinned versions, published scripts.

| M | Name | Contents | Acceptance criteria (all must pass) |
|---|---|---|---|
| **M0** (2-3 weeks) | De-risk | Extend the spikes: wasm in a real browser (WebGPU), mid-range Android device Vello measurement, startup with pipeline cache, Parley editing + IME on web hidden-input, Taffy-vs-own-constraints layout choice, AccessKit tree on one native + DOM mirror on web for a 5-widget demo. | (1) wgpu+vello+parley wasm runs in Chrome and Safari 26 with WebGPU and renders text; measured brotli size reported (baseline 0.58 MB without app code or fonts, section 9). (2) Vello on a mid-range Android phone: virtualized 40-row list p95 GPU+encode < 8.3 ms at 1080p, else adopt the quad painter as primary on mobile. (3) Warm desktop first frame < 300 ms with pipeline cache. (4) VoiceOver reads the demo on web and macOS. |
| **M1** (6-8 weeks) | Core: reactive + scene + layout + paint, no DSL | `wj_reactive` (productionized), `wj_scene`, layout, `Painter` trait + Vello and CPU painters, FakeClock/FakeShell, desktop shell (macOS first). Rust API only. | 10k-row create+update benchmark (js-framework-benchmark-style operations: create 1k/10k, update every 10th, select, swap, remove, clear): **median update-one-row <= 0.1 ms** CPU; create 10k rows <= 150 ms native; each operation is also run in Leptos and Dioxus (web, wasm in a browser) and in Dioxus Native where it runs, and the table reports each framework in its own habitat; a native-vs-wasm ratio is never presented as a like-for-like win (the like-for-like browser comparison is the M3 criterion). Golden-image suite >= 200 scenes, CPU-vs-GPU painter diff <= 1/255 per channel on all but anti-aliased edge pixels (<= 0.5% pixels differ by > 3/255). Reactive core passes the port of the signal-bench tests plus property tests (random graph vs naive recomputation oracle, 10k cases). |
| **M2** (6-8 weeks) | Language: `component`/view DSL, interpreter dev loop | Compiler R1-R4, R9, R12; stock widget set v0 (Text, Button, TextField, Checkbox, List, Scroll, Row/Column/Stack); `wj dev` hot reload | The six samples compile and run (SAMPLES.md); counter sample <= 6 lines, no `clone/move`; hot reload save-to-pixels **p95 < 300 ms** on desktop (compared with Dioxus Subsecond on the same edit: report both); interpreter vs compiled differential tests identical on the UI corpus; 10k-row list creation under interpretation < 1 s. |
| **M3** (8-10 weeks) | Web: wasm, WebGPU/WebGL2, DOM projection, SSR | Web shell, hidden-input text, DOM projection (migrated `dom_patch`), SSR skeleton + hydration adoption, fallbacks | Counter app **<= 400 KB brotli** (lite) and text-capable demo **<= 2.0 MB brotli**; first contentful paint (SSR HTML) < 0.6 s and interactive < 1.0 s on throttled 4G, cached assets; axe-core: zero critical violations on the 6 samples; Lighthouse accessibility >= 95; crawler-fetched HTML contains the page text; browser 10k-row benchmark vs Leptos and Dioxus-web: update-one-row within 3x of Leptos (the DOM wins small updates; we win big-surface animation), scroll a 10k virtual list at p99 frame < 8.3 ms on Chrome WebGPU. |
| **M4** (8 weeks) | Text and input depth | IME (CJK, dead keys, Android/iOS keyboards deferred to M7), selection, clipboard, bidi, rich text, i18n | A conformance suite of 100 text-editing scenarios (independent oracle: ICU/browser contenteditable behaviors recorded from Chrome and macOS TextEdit for the same event sequences); Hebrew/Arabic/CJK/emoji paragraphs render equal to a reference shaping (HarfBuzz via a separate tool) at glyph-id level; CJK IME composition passes a manual script on macOS, Windows, Linux (fcitx5/ibus) and web. |
| **M5** (8 weeks) | Widget set, theming, routing, forms, animation, a11y complete | Full stock set; typed routing; `form`; animation primitives; previews as tests | Every stock widget has role/name/state/actions test; NVDA, VoiceOver, Orca manual script passes; 60 previews run as golden + a11y tests; animation tests use FakeClock and match analytic curves within 1e-3. |
| **M6** (6-8 weeks) | Engine integration and editor port | `wj_engine_bridge`, quad painter, editor panels ported, egui path deleted | HUD cost inside a real 3D frame <= 1.0 ms @1080p on reference GPU; editor panels reach feature parity with the egui path (checklist); input capture tests (UI consumes click, gameplay does not); editor runs in the browser and natively from the same source. |
| **M7** (10-12 weeks) | iOS and Android | Shells, IME/a11y glue, packaging, `wj build --target ios/android` | Sample apps pass TalkBack and VoiceOver scripts; 120 Hz ProMotion device holds p95 < 8.3 ms on the list sample, mid-range Android p95 < 16.6 ms; app size: counter < 12 MB (iOS) / < 8 MB (Android AAB download); cold start < 500 ms mid-range Android. |
| **M8** | 1.0 hardening | docs, migration, perf regressions gated in CI | All benchmark gates in CI, no P0 a11y bugs. |

Calendar numbers are engineer-effort estimates by one person-equivalent team of 3-4; they are guesses ([U]) and exist to size the plan, not to promise dates. Headline: roughly 14-18 months to a credible 1.0 on desktop+web+engine,
mobile after. If the owner expects "Flutter-class everywhere" sooner, that is a different (much larger) staffing decision.

### 6.3 Benchmark protocol

- Reactive/update: js-framework-benchmark operations (it is the public baseline; its official results page is dynamic and could not be scraped this session, so we run our own harness against pinned Leptos, Dioxus, Solid versions).
- List scroll: 10k rows, 60 s scripted scroll, frame times from a frame-callback recorder; report p50/p95/p99 and dropped-frame %.
- Startup: process start to first interactive frame (native), navigation start to first input response (web), cold and warm.
- Size: brotli of wasm+js shim+shaders+fonts; native stripped binary.
- Flutter comparison: same screens built in Flutter 3.44-line (current in 2026 per sources [U]) on web (Wasm) and mobile; report size, startup and frame times side by side.
- Never publish a ratio without the script and the machine description (the spike READMEs state the load level of the machine).

### 6.4 TDD and verification strategy (independent oracles)

1. **Reactive core**: model-based tests, a naive "recompute everything" interpreter of the same graph as oracle; random graphs with 10k cases; invariants: no glitches, effects run at most once per batch, equality cutoff, dependency GC. (The spike has three hand-written cases; the oracle harness is M1.)
2. **Layout**: differential against Taffy and against Chrome's flexbox for the supported subset (recorded as fixtures from a headless browser: an independent implementation); property tests for constraints invariants (child size within constraints, no NaN).
3. **Text**: shaping output compared with HarfBuzz CLI (`hb-shape`) glyph ids/advances for a corpus; bidi against the Unicode BidiTest.txt conformance file (an authoritative external oracle); line breaking against UAX#14 test data.
4. **Painting**: golden images rendered by the **CPU painter** (`vello_cpu`) and compared against the GPU painters with tolerance; plus a handful of reference images produced by an unrelated rasterizer (resvg for SVG-expressible scenes) so the CPU painter is not its own oracle. Goldens are updated only by an explicit command with a reviewed diff image.
5. **Accessibility**: tree snapshot tests per preview; role/name/state completeness lints; web: axe-core in headless Chrome; scripted screen-reader passes before each release (manual, recorded).
6. **Determinism**: `FakeClock`, event-log record/replay, seeded randomness; no sleeps in tests.
7. **Differential across backends**: interpreter vs compiled Rust vs wasm for component logic (compiler team's harness pattern); native vs web scene output equality on the display list (not pixels): the display list is the contract, pixels are a second check.
8. **Mutation testing** of the test suite on the reactive core and layout (the compiler repo already applies mutant-kill thresholds in its interpreter harness; reuse the practice).
9. **Performance regression**: benchmarks in CI with stored baselines and a 10% gate; frame-time traces attached to failures.
10. **Fuzzing**: event streams and tree mutations against the scene for panics/invariant violations; the DOM projection against `FakeDom` (exists).

## 7. Compiler requirements (refines `docs/compiler-requests/README.md` C1-C10)

Priority: **P0** blocks M2 (the language milestone); **P1** needed by M3-M5; **P2** nice to have; **drop** no longer needed. "Touches" names the compiler ADR
(`windjammer/docs/adr/`) or work package. None are implemented; the compiler repo was not modified.

| ID | Requirement | Replaces | Pri | Touches |
|---|---|---|---|---|
| R1 | **`@handle` (Copy-like handle types)**: attribute making a type implicitly copyable for ownership inference regardless of representation; closures capture it by copy, never move; reading a handle's value in a tracked scope borrows from the runtime. | C1 | **P0** | ADR-0001 (IR solver as single ownership writer), ADR-0003 (no `owned` keyword) |
| R2 | **Trailing-block children and named arguments**: `Column(gap: 8) { ... }` lowers to a call with a `children` list; `for`/`if`/`match` as expressions producing child lists (with `key` clause). | C2 | **P0** | ADR-0014/0015 (surface syntax) |
| R3 | **`component` / `state` / `derived` / `effect` / `resource` / `view` forms**: parsed and lowered by the compiler to builder calls plus dependency metadata; reads/writes of `state` need no `.get()/.set()` (compound assignment desugars to `set`). Runtime tracking suffices for v1. | C6 (extended) | **P0** | ADR-0015; IR (`IrModule`) |
| R4 | **Structured interpolation** visible to lowering (`Template(parts, exprs)`) so a binding targets one text node. | C3 | P1 | ADR-0014 |
| R5 | **Two-way `bind:`** and `form` declarative lowering (`Valid<T>`). | C4 (extended) | P1 (form: P2) | ADR-0015 |
| R6 | **Source maps and error overlay spans** for generated and interpreted code; stable diagnostics in dev server. | C8 | P1 | compiler diagnostics (ADR-0005 error strategy) |
| R7 | **wasm without wasm-bindgen**: the wasm target emits plain wasm imports/exports (`extern "host"` capability functions), linear-memory string/slice ABI, a generated <20 KB JS shim; no JS-glue dependence in user code. This replaces "JS backend Tier 2": the web target is wasm, JS is only a loader. | C5, C7 (reframed) | **P0 for web** (M3) | ADR-0007 (Tier 2 WASM), ADR-0008 (capability-tagged host modules), ADR-0012 (extern boundary) |
| R8 | **Async in compiled and interpreted targets** (`resource`, `await` in handlers), with cancellation tied to scopes. | new | P1 | ADR-0004 (concurrency surface, accepted); interpreter limits (`INTERPRETER.md`) |
| R9 | **Interpreter host-call interface**: register Rust functions/types (scene, text, painter) callable from interpreted code; today `extern` is unsupported there. Handles are opaque values. | C9 (reframed) | **P0** | `INTERPRETER.md` remaining gaps; ADR-0007 (interpreter tier amendment) |
| R10 | **Generics, trait objects (`dyn Widget`), associated types and `impl Fn() -> View` params** working through the Rust and wasm backends; needed for typed child slots and the `Animatable<T>` trait. | new | **P0** (verify) | ADR-0001; IR solver |
| R11 | **Derives for UI**: `@derive(Store)` (per-field reactivity), `@derive(Props)` (builders/defaults), `@derive(Animatable)`, `@derive(Serialize)` for SSR state; no user-visible proc macros, built into the compiler like `@derive(Clone)`. | new (replaces `windjammer-ui-macro`) | P1 | ADR-0015 |
| R12 | **Closure capture semantics identical across interpreter and compiled targets** (the documented divergence: interpreted by-reference vs compiled `move` copy). | new | **P0** | `INTERPRETER.md` findings; ADR-0001 |
| R13 | **Hot-reload support**: incremental/watch mode with stable symbol identity, state keys (component path, declaration name, type hash), reload protocol over a dev-server socket; also a Subsecond-like path for compiled Rust as a later P2. | C9 | P1 | `wj` CLI; interpreter |
| R14 | **Const-evaluable tokens and typed units** (`Px`, `Pct`, `Duration` literals `150ms`), theme token structs checked at compile time; pipe-friendly modifier functions. | new | P1 | ADR-0015 |
| R15 | **C ABI exports and static/cdylib outputs for iOS/Android** (`@export extern "C"`, staticlib), plus `@repr(C)` layouts for GPU/FFI structs. | new | P1 (M7) | ADR-0012 |
| R16 | **Compile-time lints for UI**: required a11y props, slot-constraint violations, unbounded constraints heuristic. | new | P2 | ADR-0009-style lint infrastructure |
| R17 | **Test support**: `@preview` registry, golden-image and a11y snapshot helpers callable from `wj test` and `wj test --interpret`. | new | P1 | `wj test` |
| R18 | ~~Hand-maintained file marker~~ | C10 | **drop** (generated dir is deleted, 6.1) | - |
| R19 | **Backend trait over the IR** so the UI lowering is written once, not per target. | new | P1 | ADR-0007; "WP 11 (Backend trait over IrModule)" |

Early de-risking: R1+R2+R12 can be prototyped as a *library convention* in a spike branch before the compiler work (trailing builders already exist as method chains today), so the DSL's ergonomics can be judged in M1 using the current compiler.

## 8. Risks, kill criteria, owner decisions

### 8.1 Risks (ranked)

1. **Scope**: this is a Flutter-sized product (text, IME, a11y, platforms) plus a language feature program. Mitigation: the staged gates; adopt Parley/AccessKit/wgpu/Vello/winit; only own what differentiates.
2. **Compiler dependency**: the ergonomic edge exists only if R1-R3, R9, R12 land. These are in a repo this project does not control.
3. **Vello on mobile and WebGL**: compute-heavy renderer, `vello_hybrid` not production-ready [V], unmeasured on mid-range Android. Mitigation: own quad painter (also required for the engine).
4. **Startup**: first Vello pipeline compile cost 1.0-1.4 s cold on Metal [M]; browsers have their own shader compile costs; unmeasured on web.
5. **Web a11y/SEO credibility**: the projection must be real; a canvas with a hidden placeholder will be rejected as "Flutter web again".
6. **Text input quality** across IME/OS combinations; years of bug-tail.
7. **Upstream churn**: wgpu 30, vello 0.11, parley 0.12, accesskit 0.25, winit 0.31-beta: all pre-1.0 and moving; pin and budget upgrade work each quarter.
8. **Ecosystem**: no packages, IDE support, docs, community; adoption by outsiders is unlikely before 1.0 regardless of quality.
9. **Cross-origin isolation** for wasm threads limits deployment on some hosts (single-threaded v1 avoids this).
10. **Wasm GC/Component Model** might make the Rust-wasm path obsolete-ish for Dart/Kotlin but not for us; low risk.

### 8.2 Kill / pivot criteria (checked at the named gate; any single one triggers an owner review)

| Gate | Condition | Pivot |
|---|---|---|
| M0 | Vello on the reference mid-range Android misses 16.6 ms p95 on a 40-row list **and** the quad painter prototype cannot reach it | Drop GPU-compute renderer on mobile; make quad painter primary; if still missed, mobile becomes "later" |
| M0 | No viable IME/selection path on web hidden-input for CJK | Web text input uses real DOM `<input>` elements for editable fields only (documented hybrid) |
| M1 | Update-one-row or 10k create misses criteria by > 3x after one optimization sprint | Re-evaluate retained-scene design (consider Xilem/Masonry adoption instead of own scene) |
| M2 | Compiler team cannot commit to R1-R3, R9, R12 within the milestone window | Fall back to a Rust-first library (builder API, own `view` conventions) with Windjammer as optional; the "ergonomics better than Rust frameworks" thesis is then **dead** and the project's reason to exist must be re-argued |
| M3 | Counter app > 800 KB brotli or text app > 3.5 MB brotli (twice the target; Flutter-class), **or** axe-core critical violations that the projection cannot fix | Shrink scope (lite profile only) or reintroduce a DOM document-mode backend for content sites |
| M3 | Browser baseline: update-one-row > 10x Leptos in the same browser | Fix before continuing; no feature work |
| M4 | Text conformance < 95% on the editing scenarios after two sprints | Narrow supported languages for v1 (document it); do not ship partial IME silently |
| Any | Upstream (wgpu/vello/parley) abandonment or a breaking redesign we cannot track with < 0.5 FTE | Vendor and freeze, or switch to the alternative in the same layer (`Painter`/text traits keep this possible) |
| Any | Two milestones in a row slip > 50% | Stop and re-plan with the owner; consider adopting Xilem/Masonry or Slint and contributing instead of building |

### 8.3 Top 5 decisions the owner must make

1. **Commit to a custom-rendered primary target with a DOM *projection* (ADR-0001), accepting the web SEO/a11y engineering cost, vs a pure Flutter-style canvas, vs also building a DOM document-mode backend later.** Recommendation: projection now, document mode deferred.
2. **Authorize the compiler work program (R1-R3, R9, R12 as P0) and name an owner for it**, since the ergonomics thesis depends on it (kill criterion at M2).
3. **Adopt vs build on the Linebender stack** (Vello + Parley + AccessKit; and possibly Masonry/Xilem instead of an own scene): recommendation adopt Vello/Parley/AccessKit/wgpu/winit/Taffy, build own reactive core and scene; revisit Masonry at M1 gate.
4. **Scope and ordering of targets**: web + desktop + engine first (M1-M6), iOS/Android after (M7), or mobile in parallel at higher cost. Includes the headcount assumption (3-4 engineers).
5. **What happens to the existing egui editor path and ~120 string-builder components**: recommendation as 6.1 (keep egui until M6, rewrite components incrementally, delete HTML-string components and stale generated copies now), plus whether business-domain components leave this repo.

## 9. Spikes and measurements (real numbers, with caveats)

Machine: Apple M1 Pro, macOS (Darwin 23.5), Metal, rustc 1.100 nightly (2026-09-01). **The machine was shared with other agents running builds**, so timings are noisy;
ranges across repeated runs are given instead of single numbers. Nothing was run in a browser, on a phone, in a real window with vsync, or inside a 3D frame. All commands are in the spike READMEs.

### 9.1 `spikes/signal-bench`: reactive core vs Leptos's `reactive_graph` 0.2.15

An arena-based graph with Copy generational-style handles (u32 index in this spike), lazy memos with clean/check/dirty colouring, equality cutoff, dynamic dependencies and effects.
Three semantic tests pass (`cargo test --release`: glitch-free diamond runs the effect once, equality cutoff stops propagation, dynamic dependency drop). Benchmarks (release, LTO, 10,000 nodes, median of 15; final run):

| Benchmark | wjsig (ours) | reactive_graph 0.2.15 |
|---|---|---|
| create 10k signals + 10k memos + read | 1.9 ms | 6.8 ms |
| fan-out: 1 source -> 10k memos, 10 updates + full reads | 9.7 ms | 2472 ms |
| chain depth 1000, 100 updates | 12.6 ms | 3312 ms |
| diamond, 100k updates | 25.6 ms | 159 ms |
| 10k effects, 1000 single-source updates | 1.8 ms | not measured (needs async executor) |

Caveats (important): (1) my first fan-out run was 529 ms because of an O(n^2) resubscription bug in my own code, fixed before the numbers above, an example of how easy it is to get this wrong; (2) I drove `reactive_graph`
synchronously without an effect/executor, in an unusual pattern (memo reads from the owner thread), so the 100x+ gaps likely reflect that pattern and not what Leptos does in an app; **do not quote these as "Leptos is slow"**. The valid conclusion
is only that an arena design is *not slower* than the Rc-based one and has no per-node allocation beyond the value box, so it is a safe foundation; (3) my core is incomplete (no scopes/disposal, batching, async, `Send`) and single-threaded.

### 9.2 `spikes/gpu-scene`: wgpu 30 + Vello 0.11 + Parley 0.12, headless on Metal

1280x720 offscreen Rgba8 texture, rows of rounded rect + one text run, text layouts cached per row.

| Measurement | Result (range over 3-4 runs unless noted) |
|---|---|
| wgpu device init | 26-1670 ms (the outlier is cold; typical warm 26-110 ms) |
| `Renderer::new` (Vello pipeline creation) | cold 1049 and 1391 ms; warm 20-120 ms |
| First `FontContext` + one layout (system fonts) | 165-735 ms |
| Shape+layout 10,000 rows of ~40 chars | 80-490 ms (8-49 us/row; load-dependent) |
| Bidi/CJK/emoji paragraph | shaped to 9 glyph runs, one line (smoke test only; not a correctness check) |
| Virtualized 40 visible rows: scene encode | 0.05-0.06 ms median |
| Virtualized 40 rows: render + GPU wait | 2.5-4.0 ms median (p95 4.7-23 ms, min 1.4-2.4) |
| All 10,000 rows, no culling: scene encode | 16-21 ms median |
| All 10,000 rows, no culling: render + GPU wait | 150-185 ms median (p95 285-352) |
| Native release binary | 6.9 MB (unstripped) |
| **Wasm** (`wasm32-unknown-unknown`, opt-level z, LTO, panic abort, stripped; same wgpu+vello+parley code path, one text layout, one rect, one render) | **2.85 MB raw; wasm-opt -Oz 2.47 MB; gzip -9 0.80 MB; brotli -q11 0.58 MB** |

What this shows: (a) culling/virtualization is non-negotiable; (b) per-frame cost for a UI-sized scene is comfortably inside 8.33 ms on this GPU, but the number includes `device.poll(wait)` in a headless loop and is not a vsync'd
display measurement; (c) cold start is the problem (shader pipelines, system fonts), not steady state; (d) the web payload for the *engine* (without app code, fonts or ICU data beyond what linked) is ~0.6 MB brotli, far below Flutter's published
CanvasKit +1.5 MB / Skwasm +1.1 MB [V via assessment], leaving room for app + fonts under the 2.0 MB target.
Not measured: wasm execution (no browser run: the wasm was not passed through `wasm-bindgen` because my installed CLI (0.2.108) did not match the crate version resolved (0.2.129) and the downgrade attempt failed to build, so the byte counts are of the raw module and omit bindgen glue (tens of KB typically) and fonts; whether it runs under WebGPU in Chrome/Safari is **unverified**), Android/iOS, WebGL2 fallback, text editing/IME, accessibility.

### 9.3 Not measured (so no claim is made)

Flutter/Compose/Dioxus/Leptos comparative benchmarks; web frame times; mobile anything; memory; real-window frame pacing; interpreter UI performance; Taffy vs own layout; AccessKit integration.

## 10. Sources (fetched or searched 2026-10-11; reliability as labelled in the text)

- Flutter and Dart 2026 roadmap (Impeller on Android, Wasm intended as web default; fetched): https://flutter.dev/blog/flutter-darts-2026-roadmap
- Flutter web accessibility (placeholder activation; search result summary): https://docs.flutter.dev/ui/accessibility/web-accessibility
- Flutter 3.44 / Skia-removal timing, Skwasm fallback in Firefox/Safari (secondary blogs, **U**): https://www.cmarix.com/blog/latest-flutter-version/ , https://ecorpit.com/impeller-mandatory-flutter-android-ios-2026/ , https://startdebugging.net/pt-br/2026/07/how-to-build-a-flutter-web-app-with-webassembly-using-flutter-build-web-wasm/
- Dioxus 0.7 release (Dioxus Native/Blitz status, Subsecond hot-patching, signals/stores; fetched): https://dioxuslabs.com/blog/release-070
- Blitz pre-alpha and AccessKit "coming soon" (search result): https://lib.rs/crates/blitz-html
- Compose Multiplatform 1.8.0 (iOS stable): https://blog.jetbrains.com/kotlin/2025/05/compose-multiplatform-1-8-0-released-compose-multiplatform-for-ios-is-stable-and-production-ready/ ; 1.9.0 (web Beta): https://blog.jetbrains.com/kotlin/2025/09/compose-multiplatform-1-9-0-compose-for-web-beta/
- Slint 1.6 (accessibility custom actions): https://slint.dev/blog/slint-1.6-released
- vello_hybrid / sparse_strips README (not production-ready): https://skia.googlesource.com/external/github.com/linebender/vello/+/refs/heads/main/sparse_strips
- WebGPU support across browsers (secondary; coverage figure unverified): https://ascii.co.uk/news/article/news-20251126-7cbd345e/webgpu-now-supported-across-chrome-firefox-safari-edge , https://www.khronos.org/developers/linkto/webgl-and-webgpu-sigg26
- AccessKit repository and adapters: https://github.com/AccessKit/accesskit ; Ply web a11y DOM bridge (third party): https://plyx.iz.rs/docs/accessibility/
- GPUI status (Register 2025; gpui-component): https://www.theregister.com/2025/08/22/everything_is_different_on_windows/ , https://github.com/longbridge/gpui-component
- Lynx vs React Native (vendor claims, secondary): https://thenewstack.io/cross-platform-ui-framework-lynx-competes-with-react-native , https://www.pkgpulse.com/guides/lynx-bytedance-framework-vs-react-native-2026
- Rive Flutter runtime migration (C++ runtime, data binding): https://rive.app/docs/runtimes/flutter/migration-guide
- Makepad 2.0 (third-party skill docs, **U**): https://skillselion.com/skills/zhanghandong/makepad-skills/makepad-2.0-migration
- Crate versions: `cargo info` against crates.io, 2026-10-11 [M].
- Internal: `docs/ASSESSMENT_2026.md` (including its own source list), compiler `docs/INTERPRETER.md`, `docs/adr/0001-0015`, `docs/design/pipe-operator.md`.
- Not retrieved (404/blocked): Linebender Xilem roadmap page, Lynx intro page; Xilem/Masonry statements are **[K]** apart from crate versions.
