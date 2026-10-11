# ADR-0004: Accessibility: required semantics, AccessKit natively, own DOM projection on web

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
A custom renderer has no platform a11y tree. Flutter web exposes semantics only after a hidden placeholder is activated. AccessKit provides platform adapters (UIA, NSAccessibility, AT-SPI, Android); no web adapter and no iOS adapter were found in the searches. 13 of ~120 current components mention aria.

## Options
A. Opt-in a11y (Flutter web). B. Always-on semantic tree, AccessKit native + own DOM projection. C. Only use real DOM on web.

## Decision (recommended)
**B.** Every widget must implement `semantics()` (role, name, value, state, actions, bounds); the scene emits a semantic tree each frame; AccessKit on native; on web an incremental DOM mirror (migrated from `dom_patch`) populated lazily for visible subtrees; iOS via own UIAccessibility mapping. Gate: axe-core clean, manual VoiceOver/NVDA/TalkBack/Orca scripts per release.

## Consequences
Per-frame tree diffing cost (budgeted in the 8.33 ms frame); iOS and web adapters are own work.

## Decision needed from: project owner
Approve making a11y a compile-time requirement for widgets.
