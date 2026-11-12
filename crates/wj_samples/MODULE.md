# wj-samples
Windjammer sources in `wj/src` are transpiled by `build.rs` with the pinned compiler (`wj-pin.txt`, `WJ_BIN`) and tested through
`FakeShell`. Every sample in `wj/src` has a behaviour test in `tests/samples.rs`. When a natural-style sample hits a compiler defect, park it in
`wj/blocked` with a test that pins the defective output, so a compiler fix turns that test red and the sample moves to `wj/src`
(`clicks.wj` went that way once the closure-capture defect was fixed).

`wj/rust/wj_reactive.wji` pins the signature surface of `wj_reactive` (ADR-0026): `state` is declared for `i32` because a
surface cannot declare generic functions or opaque crate types yet. `wj/blocked_generic/` holds samples that need them
(`todo.wj` uses `state(Vec::new())` and `derived`); `wj build` reports WJ0050 for them today, so `build.rs` does not
build that directory. When the surface language gains generics, move them to `wj/src`.
