# Tests

Run everything with `cargo test --features web`. Point the compiler-backed gates at a `wj` binary with
`WJ=/path/to/wj`; `WJ_UI_REQUIRE_WJ=1` turns a missing compiler into a failure instead of a skip (CI sets it).

| File | What it covers |
| --- | --- |
| `generated_fresh_test.rs` | `src/components/generated/` and `src/metadata.json` match what the compiler produces from `src/components_wj/` (refresh with `WJ_UI_REGEN=1`) |
| `wj_sources_check_test.rs` | `wj check` on `src/components_wj/`; every example transpiles |
| `examples_integration_test.rs` | the examples in `examples_wj/` transpile |
| `generated_components_test.rs`, `data_display_components_test.rs`, `cross_file_trait_test.rs` | rendered output of the generated components |
| `kit_logic_test.rs`, `kit_docs_test.rs`, `kit_dom/` | the accessible component kit: state machines, documentation, DOM contract |
| `dom_patch_test.rs` | the DOM patch algorithm |
| `ecs_inspector_panel_test.rs`, `profiler_foundation_test.rs` | the editor panels |
| `public_hygiene_test.rs` | the public tree stays free of private names and stale artifacts |
| `wasm_*_test.rs` | browser and node tests, run with `wasm-bindgen-test-runner` (see `docs/TESTING_GUIDE.md`) |
| `integration_ui_examples.rs` | nested cargo builds of the example apps; `#[ignore]`d by default, run with `--ignored` |
