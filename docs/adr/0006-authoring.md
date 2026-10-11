# ADR-0006: Authoring: `component` / view DSL lowering to typed builders; no macros

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
The language has no macros; the compiler design doc proposes JSX-like literals that were never built; windjammer-ui has four coexisting component models. Pipe operator precedent: one desugaring site in the parser.

## Options
A. Builder methods only (today). B. Trailing-block builders with named args plus `component` sugar. C. JSX-like literal.

## Decision (recommended)
**B.** `component Name(props) { state/derived/effect/resource; view { ... } }`, `Column(gap: 8) { ... }`, keyed `for`, inline `if/match`, structured interpolation, `bind:`, `form`, `@preview`. Lowering is ordinary calls so interpreter, Rust and wasm share it. Compiler requirements R1-R5, R11, R14.

## Consequences
Entire thesis depends on compiler work outside this repo; syntax in docs is proposed, not implemented. Prototype as a library convention in M1 to judge ergonomics before language changes.

## Decision needed from: project owner
Approve the P0 compiler program and name its owner (kill criterion at M2).
