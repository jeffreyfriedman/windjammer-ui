# wj-paint (L3, backend-neutral)
`DisplayList` (absolute-coordinate commands: clip, opacity layer, rounded fill, inside stroke, glyph run) is the contract between
the scene and every renderer; `Painter::render(list, w, h, clear) -> Image` is what backends implement. `build_display_list`
walks the scene with scroll offsets, clips, opacity, and culls subtrees outside the viewport. `FakePainter` records lists.
`TextLayouter` is the seam to shaping (`wj-text`). Depends only on `wj-scene`.
