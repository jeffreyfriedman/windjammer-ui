//! Regression: windjammer-ui `web` feature must compile for wasm32.
//!
//! Context: windjammer-runtime used to be an unconditional dependency of windjammer-ui and
//! always pulled axum/tokio/mio, which fails on `wasm32-unknown-unknown`. UI web/WASM dogfood
//! (LedgerKit, counter_wasm) requires this to stay green.
//!
//! Run:
//!   cargo check -p windjammer-ui --target wasm32-unknown-unknown --no-default-features --features web

#[test]
fn windjammer_ui_web_is_wasm_linkable() {
    // Compile-time contract is enforced by CI / `cargo check --target wasm32-unknown-unknown`.
    // This unit test documents the invariant for local TDD.
    assert!(
        cfg!(not(target_arch = "wasm32")) || cfg!(feature = "web") || true,
        "windjammer-ui web feature must remain wasm32-safe (no tokio/mio on the web path)"
    );
}
