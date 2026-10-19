# Requests for the compiler team

Source: `docs/DESIGN_DIRECTION.md` (staged roadmap). Nothing here is implemented in this repo. IDs match the table there.

Closed investigations (no compiler change needed):

- `test_todo_app_example_compiles` / `test_contact_form_example_compiles` failed because the example `.wj` files used
  `"...".to_string()`, which `wj 0.50.0` rejects with W0006 "string literal `.to_string()` is forbidden Rust leakage"
  (ADR-0014). The compiler was correct; fixed in the examples (commit e70ac81). Both now transpile. Follow-up for the
  compiler team only: the examples still use `println!` (deprecated form, ADR-0014) and the old `Signal::new(...)`
  alias-per-closure pattern, which C1 would remove.

Open requests (each with a minimal repro or acceptance sketch):

## C1. Copy-like handle types in closures
Repro (today needs aliases, see `examples_wj/todo_app.wj`):
```windjammer
let count = Signal::new(0)
let a = count          // alias needed so the closure does not move `count`
button("+").on_click(move || a.set(a.get() + 1))
text("${count.get()}")  // error today if `count` was moved
```
Want: a type attribute (`@handle` / `@copy_handle`) making a type implicitly copyable for ownership inference
regardless of its Rust representation (`Signal<T>` becomes `{id, generation}`).

## C2. Trailing-closure builders and named arguments
```windjammer
column(gap: 12) {
    heading("Hi")
    button("Add", on_click: add)
}
```
Lowers to `column(gap: 12, children: [heading("Hi"), button("Add", on_click: add)])`; children are a list expression in
a block, `for`/`if` allowed as expressions producing lists.

## C3. Structured interpolation
`"${a} items of ${b}"` should be visible to a lowering pass as `Template(["", " items of ", ""], [a, b])`, so a UI pass
can emit per-expression bindings. Today it lowers directly to a format call.

## C4. Two-way binding lowering
`text_input(bind: draft)` => `value: draft.get()` + `on_input: |v| draft.set(v)`.

## C5. Backend-neutral browser capabilities
`extern` capability traits (fetch, storage, history, timers) implemented per backend (Wasm via wasm-bindgen, JS natively),
so components stop embedding JS in Rust strings (ADR-0012).

## C6. Component syntax with dependency analysis
`component Name(props) { let x = state(..); let y = derived(|| ..); view { ... } }` with `for item in list key item.id`
and `if` inside `view`. Output: creation code plus a list of (expression, handles read) pairs.

## C7. JS backend Tier 2 for the UI subset
Per ADR-0007: closures, structs/enums, interpolation, `List/Map`, with conformance cases covering the builder DSL.

## C8. Source maps (.wj spans in generated output) for an error overlay.

## C9. Watch/incremental build mode with stable output paths for hot reload.

## C10. Hand-maintained file marker
Honour a header marker (for example `// wj:hand-maintained`) in `src/components/generated/*.rs` so a full regen never
overwrites them; retire `SKIP_WJ_REGEN`/`WJ_REGEN` env toggles in `build.rs`.

---
**Update 2026-10-11:** refined for the unified-renderer direction in `docs/VISION_2026.md` section 7 (requirements R1-R19, with priorities). C1->R1, C2->R2, C3->R4, C4->R5, C5/C7->R7, C6->R3, C8->R6, C9->R13/R9, C10 dropped.

## Minimal repros found while building the M1 API (2026-10)
Each directory in `repros/` is a neutral project (tiny external crate + `.wj` + `check.sh`) against compiler `next/v0.50.0` @ 8f70969e:
- `empty-string-literal-to-str-param/`: `f("")` emitted as `f(String::new())` for a `&str` parameter.
- `closure-capturing-copy-value-not-move/`: of two closures capturing one `Copy` value, the first lacks `move` (E0373 with `'static` callbacks). Needs R1.
- `eager-integer-literal-suffix/`: literals suffixed `_i32` / `_i64` inconsistently around dependency method calls (E0277/E0308).
Also known (compiler repo, `examples/verified/iter_map_filter.wj`): `xs.map(..)` / `xs.filter(..)` on `Vec` emit no iterator call, while `.iter()` is rejected by lint W0003, so there is no compiling spelling for collection pipelines over a `Vec` today.
The natural-style samples these block live in `crates/wj_samples/wj/blocked/` with tripwire tests.
