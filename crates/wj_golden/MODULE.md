# wj-golden
240-scene corpus generated from seeds (160 display-list scenes, 80 full-stack scenes with layout and text) plus conformance tests:
CPU painter vs committed PNG goldens (`WJ_UPDATE_GOLDENS=1` regenerates; review the diff), GPU vs CPU, CPU vs tiny-skia
(an unrelated rasterizer) on vector scenes, and end-to-end counter pixels. GPU tests fail without an adapter unless `WJ_ALLOW_NO_GPU=1`.
