# signal-bench

Candidate reactive core (`src/wjsig.rs`: arena, lazy memos, clean/check/dirty colouring, effects) vs `reactive_graph` 0.2.15.
Run: `CARGO_TARGET_DIR=<dir> cargo test --release && cargo run --release`. Results and caveats: `docs/VISION_2026.md` section 9.1
(shared, noisy machine; reactive_graph driven synchronously in an unusual pattern; do not quote as a Leptos comparison).
Missing from the spike: scopes/disposal, batching, async, Send.
