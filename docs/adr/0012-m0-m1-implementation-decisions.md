# ADR-0012: M0/M1 implementation decisions and measured results

- **Status:** Accepted with the M1 implementation
- **Date:** 2026-10-19
- **Related:** `VISION_2026.md` 6.2, ADR-0001/0002/0003/0004/0008/0011, `crates/*/MODULE.md`

## Decisions
1. **Layout: adopt Taffy 0.14** behind `wj-layout`. Evidence: 200 random flexbox trees recorded from headless Chrome match to 0.05 px (0 mismatches);
   3,000 random edit sequences show incremental layout equal to fresh layout. Stack/scroll/virtual-list stay own code in the scene.
2. **Windowing: winit 0.30** (stable) with AccessKit's winit adapter 0.34; revisit 0.31 when it leaves beta.
3. **Reactive core:** arena, `Copy` generational handles, memo/effect/batch/scope, plus `Selector` (O(1) selection) and `Readable`.
   Verified against a recompute-everything oracle on 10,000 random graphs; removing equality cutoff or dependency GC makes the test fail.
4. **Text:** Parley with an embedded Inter Regular (OFL). No system font scan at startup; `with_system_fonts()` for fallback.
5. **Painter contract is the display list.** CPU painter = `vello_cpu`, GPU = Vello on wgpu 30, both behind `Painter`.
6. **Crates live in `crates/` as a separate Cargo workspace** so the legacy crate's dependency graph and lockfile are untouched.

## Measured (Apple M1 Pro, shared and heavily loaded machine; release builds)
- 10k-row benchmark through scene bindings: create 10k rows ~10-13 ms, update one row ~0.0007 ms median, select row ~0.001 ms,
  swap ~1 ms, clear 10k ~29 ms. Gate: median update <= 0.1 ms, create <= 150 ms (asserted in `wj-scene/tests/perf_gate.rs`).
  These are native, scene-only numbers; they are not compared with Leptos/Dioxus here (browser harness not built yet).
- Goldens: 240 scenes. GPU vs CPU: interior pixels within 2/255 (alpha-layer rounding), mean abs error < 1/255, edge-dense small scenes
  up to ~9% of pixels differ by > 3/255 (glyph and curve anti-aliasing). The vision wording (<= 0.5% over 3/255) is met only on larger scenes;
  tolerances in the test are the honest measured bounds. CPU vs tiny-skia: interior within 2/255.
- Desktop counter example, first frame ~175-270 ms after launch (window + device + pipelines), under load average 20-40.

## Not done in M0/M1 (see report)
Browser WebGPU run, Android measurement, Safari, pipeline cache, VoiceOver run, Parley editing/IME on web, Leptos/Dioxus comparison tables.
