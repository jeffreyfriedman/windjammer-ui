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
That is what makes the testing strategy in section 6 possible, and it mirrors the `gpu_port` hexagonal boundary already used in
the GPU-port pattern of the 3D game engine host (a trait, typed generational handles, a recording fake).

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

