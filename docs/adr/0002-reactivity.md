# ADR-0002: Reactivity: arena signal graph with Copy handles, auto-tracking, compiler lowering as optimization

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
The current crate has three overlapping reactive modules and a render loop that rebuilds the whole tree; signals are only a dirty flag (`ASSESSMENT_2026.md` 2.2). The language infers ownership, so handles that are Copy remove closure-capture ceremony. Spike `spikes/signal-bench` implements an arena graph that passes glitch-freedom, equality-cutoff and dynamic-dependency tests.

## Options
A. Adopt `reactive_graph` (Leptos): mature, Rc/Arc handles, async effects, but handle semantics fight `@handle`.
B. VDOM with component-level re-render (Dioxus/React/Xilem): simple, O(subtree).
C. Own arena graph, runtime auto-tracking first, compile-time dependency lowering later.

## Decision (recommended)
**C.** `state`, `derived`, `effect`, `resource` over an arena; handles are `Copy` (`@handle`, requirement R1). Reads inside view/derived/effect track; reads in handlers do not. Scene nodes receive binding effects that write props and set dirty flags (layout/paint/semantics). Lists/conditionals are keyed primitives.

## Consequences
Own the core: more code to maintain, async/threading/scope disposal still to build (spike lacks them). The spike's relative numbers vs `reactive_graph` are caveated and must not be quoted as a Leptos comparison.

## Decision needed from: project owner
Approve building the core; approve R1/R2/R3 as P0 compiler requirements.
