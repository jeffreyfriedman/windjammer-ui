# Empty string literal passed to a `&str` parameter becomes `String::new()`

`set_label("hello")` is emitted as `set_label("hello")`, but `set_label("")` is emitted as `set_label(String::new())`,
which does not type-check against `fn set_label(text: &str)` from the dependency.

Expected: `set_label("")`.

Run: `WJ_BIN=/path/to/wj ./check.sh` (shows generated Rust, then `cargo check` output: E0308 expected `&str`, found `String`).
Compiler: `next/v0.50.0` at 8f70969e.
