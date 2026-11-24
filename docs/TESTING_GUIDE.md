# Testing guide

## Commands

Use a private target dir on shared machines (`CARGO_TARGET_DIR=...`, `CARGO_BUILD_JOBS=3`).

| What | Command |
|------|---------|
| Unit + integration (host) | `cargo test --lib --tests` |
| Keyed patcher, focus, delegation, stop_propagation | `cargo test --test dom_patch_test` (uses `FakeDom`, which blurs on reparent like a browser) |
| Embedded JS runtimes (no browser) | `cargo test --test embedded_js_test` (needs `node`; `WJ_UI_REQUIRE_NODE=1` makes a missing node an error). Runs runtimes against the stub in `tests/js/dom_stub.js` |
| Patcher compiled to wasm32, run in node | `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --target wasm32-unknown-unknown --test wasm_node_test` |
| `dom_patch::web` against a real DOM (headless Chrome) | `CHROMEDRIVER=/path/to/chromedriver CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --target wasm32-unknown-unknown --no-default-features --features web --test wasm_browser_test` |
| Kit accessibility (ARIA, tab order, keys, focus) in headless Chrome | same command with `--test wasm_kit_a11y_test` |
| Kit forms, table, virtual list, theme tokens in headless Chrome | same command with `--test wasm_kit_data_test` |
| Kit logic, token contrast, docs coverage (host) | `cargo test --test kit_logic_test --test kit_docs_test` |
| Generated tree equals a fresh compiler run | `WJ=/path/to/wj cargo test --test generated_fresh_test` |
| Nested build checks (separate target dir, timeouts) | `cargo test --test integration_ui_examples -- --ignored` (`WJ_UI_NESTED_TARGET_DIR`, `WJ_UI_NESTED_TIMEOUT_SECS`) |
| Example `.wj` transpile checks | `cargo test --test examples_integration_test` (skipped when `wj` is not installed) |

Edge: it is Chromium, so the same tests apply. The runner drives `msedgedriver` when it is configured instead of chromedriver: install the
driver that matches the installed Edge, unset `CHROMEDRIVER`, put `msedgedriver` on `PATH` and run the same cargo command.
To set browser capabilities (headless flags) use a `webdriver.json` with `ms:edgeOptions`. 

Wasm-in-node notes: the installed `wasm-bindgen` CLI must match the lockfile exactly
(`wasm-bindgen 0.2.108` <-> `wasm-bindgen-test =0.3.58`, pinned in `Cargo.toml`). The wasm32
dev-dependency set excludes `windjammer-runtime`/tokio (they do not build for wasm32).
The node setup tests pure logic. `tests/wasm_browser_test.rs` drives `dom_patch::web` in headless Chrome (focus across a keyed move,
caret and typed value across a re-render, one delegated click per click, `stop_propagation`, text never becoming markup). It needs a
`chromedriver` whose version matches the installed Chrome (Chrome for Testing publishes both); pass it as `CHROMEDRIVER`. Use the cargo
command above rather than `wasm-pack test`: wasm-pack builds every test target, and the crate's own lib tests do not build for wasm32
(`windjammer-runtime` is not a wasm32 dependency).
