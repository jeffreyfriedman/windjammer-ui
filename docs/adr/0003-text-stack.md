# ADR-0003: Text stack: adopt Parley (shaping, bidi, segmentation), own editing/IME glue

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
Text shaping, bidi, line breaking, IME and selection are what sink custom renderers. Parley 0.12 (HarfRust, ICU4X segmentation, fontique) and its `editing` module exist; the spike shaped a Hebrew/Arabic/CJK/emoji paragraph and 10,000 rows (80-490 ms cold, load-dependent) and showed first font-context init of 165-735 ms.

## Options
A. cosmic-text. B. Parley. C. Own shaping on HarfRust. D. Platform text APIs per OS (CoreText/DirectWrite/Android) for shaping.

## Decision (recommended)
**B.** Parley for shaping/layout/segmentation; own editing model, selection rendering and per-platform text-input bridges (winit IME, hidden DOM input on web, UITextInput, InputConnection). Layouts are cached per text node and incremental. Lazy/async system font loading; embedded default font.

## Consequences
Cross-platform identical metrics (a benefit); platform text-service parity (spellcheck, autofill, reconversion) is a long tail. Startup cost of font enumeration needs mitigation (M0).

## Decision needed from: project owner
Approve Parley; approve funding the IME/selection work as the highest-risk item (M0/M4).
