# wj-text (L2)
Parley 0.12 shaping and line breaking behind `TextMeasure` (layout) and `TextLayouter` (paint). Embedded Inter Regular (SIL OFL,
`fonts/Inter-LICENSE.txt`) means no system font scan at startup (first layout ~3 ms vs 165-735 ms with discovery) and identical metrics
everywhere; `with_system_fonts()` opts into fallback for other scripts. Layouts are cached per (text, size, width).
Editing, IME and selection are not here yet (M4). Glyph-id oracle: font cmap (ttf-parser).
