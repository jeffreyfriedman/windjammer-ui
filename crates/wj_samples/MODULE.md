# wj-samples
Windjammer sources in `wj/src` are transpiled by `build.rs` with the pinned compiler (`wj-pin.txt`, `WJ_BIN`) and tested through
`FakeShell`. `wj/blocked/` holds the natural-style samples that the compiler cannot compile yet; `tests/blocked.rs` pins the
generated-code defect for each, so fixing the compiler turns the tripwire red and the sample moves to `wj/src`.
