# ADR-0007: Game-engine integration: UI renders into the engine's wgpu pass; editor on the same framework

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
An engine host owns a wgpu device and uses a hexagonal GPU port with a recording fake and a host/shader layout contract. The current editor reaches desktop via an egui walker that shares no components with the web UI.

## Options
A. Keep egui for engine/editor. B. Separate overlay process/window. C. Shared widget tree rendered by `wj_engine_bridge` into the engine's view; quad painter for HUDs, Vello for editor.

## Decision (recommended)
**C.** Engine calls `ui.frame(dt, input, target)`; input routing with `consumed`; engine state exposed as read-only signals via double-buffered snapshots; UI cost budget <= 1.0 ms for a HUD at 1080p measured inside a real 3D frame (M6). Keep egui until editor parity, then delete.

## Consequences
UI compute cost competes with the 3D frame (unmeasured); two painters.

## Decision needed from: project owner
Approve the M6 budget and the plan to retire egui.
