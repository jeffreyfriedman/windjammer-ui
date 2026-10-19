# windjammer-ui ADRs

All ADRs here are **Proposed**; none is accepted until the owner edits the status. They implement `docs/VISION_2026.md`.
Template: see the compiler repo's `docs/adr/README.md`.

| ADR | Title |
|---|---|
| [0001](0001-rendering-model.md) | Rendering model: own GPU scene graph with a semantic projection (not DOM, not native widgets) |
| [0002](0002-reactivity.md) | Reactivity: arena signal graph with Copy handles, auto-tracking, compiler lowering as optimization |
| [0003](0003-text-stack.md) | Text stack: adopt Parley (shaping, bidi, segmentation), own editing/IME glue |
| [0004](0004-accessibility.md) | Accessibility: required semantics, AccessKit natively, own DOM projection on web |
| [0005](0005-web.md) | Web: WebGPU primary, WebGL2/CPU fallback, SSR semantic skeleton with hydration adoption |
| [0006](0006-authoring.md) | Authoring: `component` / view DSL lowering to typed builders; no macros |
| [0007](0007-game-engine-integration.md) | Game-engine integration: UI renders into the engine's wgpu pass; editor on the same framework |
| [0008](0008-platform-shells-and-build-vs-adopt.md) | Platform shells and build-vs-adopt |
| [0009](0009-dev-loop.md) | Dev loop: interpreter for hot reload, compiled Rust/wasm for release |
| [0010](0010-fate-of-existing-windjammer-ui-code.md) | Fate of existing windjammer-ui code |
| [0011](0011-verification-strategy.md) | Verification strategy: independent oracles, goldens, a11y snapshots, determinism, differential tests |
- [0012](0012-m0-m1-implementation-decisions.md): M0/M1 implementation decisions and measured results.
