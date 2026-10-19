//! Sample views written in Windjammer (`wj/src/*.wj`), compiled by `build.rs`.
//! Behaviour tests in `tests/` run them through `wj-shell`'s `FakeShell`.
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
