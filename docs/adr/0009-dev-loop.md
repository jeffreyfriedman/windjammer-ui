# ADR-0009: Dev loop: interpreter for hot reload, compiled Rust/wasm for release

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
`wj run --interpret` starts in ~16 ms versus ~14 s for transpile+cargo (compiler docs, measured under load). The interpreter is a tree walker (~100x slower than compiled), has no extern/FFI, async, Rc/RefCell or UI modules, and captures closures by reference where compiled Rust copies on `move`.

## Options
A. Rust binary patching (Dioxus Subsecond). B. Interpreter with prebuilt Rust host. C. Both.

## Decision (recommended)
**B first** (host runtime in Rust; user components interpreted via a host-call interface, requirement R9); state kept by (component path, name, type hash); error overlay with source spans (R6); differential tests interpreter vs compiled; unify closure capture semantics (R12). Binary patching is a later optional path.

## Consequences
Interpreter UI performance unmeasured; interpreter on wasm unverified.

## Decision needed from: project owner
Approve R9/R12 as P0.
