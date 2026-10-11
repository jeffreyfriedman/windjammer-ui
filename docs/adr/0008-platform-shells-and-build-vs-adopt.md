# ADR-0008: Platform shells and build-vs-adopt

- **Status:** Proposed
- **Date:** 2026-10-11
- **Related:** `docs/VISION_2026.md`, `docs/vision/SAMPLES.md`, `docs/compiler-requests/README.md`

## Context
Per-platform needs (window/surface, text input, a11y, IME, clipboard, dialogs) and available crates (winit 0.31 beta, wgpu 30, AccessKit 0.25, arboard, rfd). Policy: paint ours, delegate system services.

## Options
A. Own shells per platform. B. winit everywhere. C. winit where adequate (desktop, Android) plus own iOS/web shells and native text-input/a11y glue.

## Decision (recommended)
**C.** Thin `Shell` trait (section 5.1 of the vision). Adopt wgpu, Vello, Parley, AccessKit, winit, Taffy, `arboard`, `rfd`; own reactive core, scene, gesture/animation, web projection, quad painter, text-input bridges. Native menus/dialogs/pickers are real native.

## Consequences
Pre-1.0 upstream churn (pin; budget quarterly upgrades); iOS decision between winit's backend and own UIKit host at M7.

## Decision needed from: project owner
Approve adoption list.
