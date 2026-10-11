# ADR-0011: Verification strategy: independent oracles, goldens, a11y snapshots, determinism, differential tests

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
A custom-rendered UI cannot be trusted by inspection. The compiler repo's interpreter harness shows the value of independent oracles and mutant-kill thresholds.

## Options
A. Manual QA. B. Snapshot tests only. C. Layered oracles (reactive model-checking, layout vs Chrome/Taffy, text vs HarfBuzz/Unicode conformance data, CPU-vs-GPU goldens plus unrelated-rasterizer references, a11y snapshots and axe, FakeClock event replay, interpreter-vs-compiled-vs-wasm differential, CI perf gates).

## Decision (recommended)
**C**, as enumerated in `VISION_2026.md` 6.4. Goldens change only through an explicit reviewed command.

## Consequences
Test infrastructure is a milestone cost (M1), not an afterthought; recorded Chrome/macOS fixtures need maintenance.

## Decision needed from: project owner
Approve funding test infrastructure as part of M1.
