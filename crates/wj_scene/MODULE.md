# wj-scene (L1)

The retained node tree. Depends only on `wj-reactive`. No platform, layout-engine or painter imports.

## Contract
- `Scene` is a `Copy` handle to a thread-local arena; `NodeId` is a generational `Copy` handle.
- A node has layout props (`Style`), paint props (`Paint`, `TextProps`), `Semantics`, event handlers and a layout
  result (`rect`, written by `wj-layout`). Setters compare and raise dirty flags (`LAYOUT`, `PAINT`, `SEMANTICS`) only
  when the value changed. `drain_dirty()` returns the per-node log, so downstream stages touch only what changed.
- Binding sites are effects owned by the node (`bind`, `bind_text`, `bind_background`, `keyed_children`,
  `virtual_list`). Removing a node disposes its bindings. Keys must be unique.
- `dispatch(Event)` hit-tests, bubbles, synthesizes `Click`, and runs handlers in one batch outside tracking.
- `semantic_tree()` is derived on demand; roleless nodes are transparent; text nodes become `Role::Text`.
  `audit_semantics()` is the lint that interactive nodes declare role and name (ADR-0004).

## Files
`tree.rs` arena and props, `bind.rs` binding sites and lists, `interact.rs` hit test/events/semantic tree,
`style.rs`, `geom.rs`, `dirty.rs`, `event.rs`, `semantics.rs`, `virtual_list.rs`.

## Tests
- `tests/scene.rs`: binding granularity, equal-write suppression, disposal, keyed lists, virtual window, events,
  hit-testing with scroll and clip, semantic tree, audit, slot recycling.
- `tests/perf_gate.rs`: the M1 10k-row benchmark. Asserts (release builds) median update-one-row <= 0.1 ms and
  create-10k <= 150 ms; prints the full table with `-- --nocapture`.
