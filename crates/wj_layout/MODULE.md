# wj-layout (L2)
Scene `Style` -> rectangles. Flexbox from Taffy 0.14 (adopted); `LayoutEngine` owns a mirrored Taffy tree and syncs it
from the scene's dirty log (`update(scene, dirty_entries, viewport, &dyn TextMeasure)`). Text sizes come through the
`TextMeasure` trait (`wj-text` implements it; `FixedAdvanceMeasure` is the deterministic test double).

## Behaviour worth knowing
- Taffy's cache makes recomputation incremental; write-back compares every node's rect (Taffy has no per-node "changed"
  signal and a node can lay out children differently without its own size changing). `Scene::set_rect` raises dirty only on change.
- Sizes are clamped at zero (Taffy can return slightly negative for stretched children whose margins exceed the box).
- Flex items shrink by default, as in CSS; scroll content and virtual-list extents set `flex_shrink: 0`.

## Tests
- `tests/flex.rs`: hand-computed fixtures from the flexbox spec.
- `tests/chrome.rs`: 200 random trees recorded from headless Chrome (`tools/gen_chrome_fixtures.py`), tolerance 0.05 px. Zero mismatches.
- `tests/model.rs`: 3,000 random scenes with random edit sequences; incremental layout must equal a fresh layout (this caught a real
  pruning bug in write-back), no leaked mirror nodes, finite non-negative rects.
