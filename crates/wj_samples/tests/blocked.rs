//! Tripwires for compiler defects that block natural-style samples (repros in
//! docs/compiler-requests/repros). Each test pins the defective output; when the compiler is fixed
//! the test fails, and the sample moves from `wj/blocked` to `wj/src` (add a behaviour test there).
fn generated(name: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("WJ_BLOCKED_DIR")).join(format!("{name}.rs"))).unwrap()
}

#[test]
fn counter_sample_blocked_by_closure_capture_without_move() {
    let g = generated("counter");
    assert!(g.contains("button_fn(|| format!"), "first closure is still emitted without `move` (repro: closure-capturing-copy-value-not-move). Fixed? Promote wj/blocked/counter.wj.\n{g}");
    assert!(g.contains("on_click(move ||"), "second closure gets `move`:\n{g}");
}

#[test]
fn todo_sample_blocked_by_empty_literal_and_literal_suffixes() {
    let g = generated("todo");
    assert!(g.contains("text(String::new())"), "empty string literal still becomes String::new() (repro: empty-string-literal-to-str-param)\n{g}");
    assert!(g.contains("state(1_i32)") && g.contains("id: i64"), "integer literal suffixed i32 while the field is i64 (repro: eager-integer-literal-suffix)\n{g}");
}
