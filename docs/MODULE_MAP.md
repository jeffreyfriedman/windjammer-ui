# Module map

Each entry: single responsibility -> allowed dependencies. Lower layers never import upper ones.

## Rust core (`src/`)
| Module | Responsibility | May depend on |
|--------|----------------|---------------|
| `signal.rs` | Signal/Computed/Effect primitives | std |
| `reactivity.rs` | Public reactive API over signals | `signal` |
| `reactivity_optimized.rs` | Experimental batched/SmallVec reactivity | std |
| `signal_ffi.rs` | Signal access for generated code | `signal` |
| `vdom.rs` | VNode tree, diff, patch | std |
| `simple_vnode.rs` | Lightweight VNode used by app loops | std |
| `to_vnode.rs` | Component to VNode trait | `simple_vnode` |
| `vnode_ffi.rs` | Build VNodes from `.wj` code | `simple_vnode`, `vdom` |
| `component.rs`, `component_runtime.rs`, `runtime.rs` | Component trait, lifecycle, DOM runtime | `vdom`, `reactivity` |
| `reactive_mount.rs` | Mount-target selection | `simple_vnode` |
| `app_reactive.rs` | WASM app loop | `reactive_mount`, `simple_vnode`, `reactivity` |
| `app_reactive_eframe.rs`, `desktop_renderer*.rs`, `desktop_app_context.rs`, `app.rs`, `app_docking.rs` | Desktop app via eframe/egui (feature `desktop`) | `simple_vnode`, egui |
| `renderer.rs`, `simple_renderer.rs` | Cross-platform renderer entry; console renderer | `component`, `vdom`, `platform` |
| `ssr.rs` | Server-side string rendering | `component`, `vdom` |
| `routing.rs` | Route table and matching | std |
| `platform/` | Platform capability traits; ios/android are stubs | `component`, `vdom` |
| `undo_redo.rs` | Command-pattern undo/redo | std |
| `wasm_events.rs`, `event_handler.rs`, `events*` | DOM event plumbing | `reactivity` |
| `frame_trace_ffi.rs` | Engine frame-trace read bridge for profiler panels | std |
| `examples_wasm.rs` | Demo code compiled for wasm | everything above |

## Components (`src/components_wj/`)
- Layout and primitives (`row`, `column`, `stack`, `grid`, `flex`, `container`, `text`, `button`, `input`...): depend only on `traits`, `style`, `html_elements`.
- Composite widgets (`datatable`, `tabs`, `modal`, `commandpalette`...): may use primitives, never each other cyclically.
- Editor/tooling panels (`ecs_inspector_*`, `profiler_*`, `frame_debugger_panel`, `node_graph`, `curve_editor`, `bt_visual_*`): may use primitives; consumed by the engine editor.
- Ledger/finance components: to be moved out (see `EXTRACTION_PLAN.md`); they currently depend on primitives and, for runtimes, on hand-maintained JS in `components/generated/`.

## Support
- `src/components/generated/`: transpiler output (plus the 17 hand-maintained files). Never edit generated files by hand.
- `tests/`: Rust integration tests, one per feature area.
