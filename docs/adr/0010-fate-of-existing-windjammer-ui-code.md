# ADR-0010: Fate of existing windjammer-ui code

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
~120 string-builder components emitting HTML, four component models, three reactive modules, stale generated copies of core modules, an egui desktop path, a correct and well-tested DOM patcher.

## Options
A. Incremental evolution of the HTML approach. B. Rewrite from scratch. C. Keep the tested kernels, migrate or delete the rest on a schedule.

## Decision (recommended)
**C.** Table in `VISION_2026.md` 6.1: keep `dom_patch` (as web projection), `escape`, routing, undo/redo; replace signals; rewrite components incrementally in the new DSL (editor panels first); delete stale generated copies, legacy vdom/renderer/ssr/platform, the proc-macro crate and CSS after replacement; keep egui until M6.

## Consequences
Two systems coexist for a long period; discipline needed to delete.

## Decision needed from: project owner
Approve the deletion schedule; decide where business-domain components live.
