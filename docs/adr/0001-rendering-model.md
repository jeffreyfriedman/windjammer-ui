# ADR-0001: Rendering model: own GPU scene graph with a semantic projection (not DOM, not native widgets)

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
The previous direction (signal regions over HTML + a WebView shell) was rejected by the owner. The mandate is one UI codebase and one rendering model across web, desktop, mobile and inside a game engine, with no WebView/JS-runtime wrapper. `VISION_2026.md` section 2 compares DOM-everywhere, native widgets, own scene, and own scene plus semantic projection.

## Options
A. DOM/HTML everywhere (Dioxus/Leptos/Tauri style): free web a11y/SEO; needs a browser or CSS engine on every platform; cannot be hosted by a game engine; Blitz (CSS-on-wgpu) is pre-alpha with AccessKit pending.
B. Native widgets per platform: native feel/a11y; N implementations; no engine embedding.
C. Own GPU scene (Flutter/Slint/GPUI): one renderer; web a11y/SEO/selection weak.
D. C plus a semantic tree produced every frame and projected to AccessKit (native) and a DOM mirror/SSR skeleton (web).

## Decision (recommended)
**D.** Retained scene graph rendered by our own painter on wgpu (Vello-class; WebGPU, WebGL2 and CPU fallbacks). DOM is a projection for a11y, SEO, find-in-page and IME hosts, never the primary render target. A DOM render backend (document mode) is deferred and gated by kill criteria.

## Consequences
Highest engineering cost of the viable options; web a11y/SEO must be built, not inherited. Everything above the Painter trait is platform-independent and testable headless. Revisit if M3 kill criteria trip (counter app > 800 KB brotli, axe critical violations unfixable).

## Decision needed from: project owner
Approve D, and approve deferring a DOM document-mode backend.
