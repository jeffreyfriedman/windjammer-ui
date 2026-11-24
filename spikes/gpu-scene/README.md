# gpu-scene

wgpu 30 + Vello 0.11 + Parley 0.12, headless. `cargo run --release` prints init, shaping and frame timings (10k-row list, with and without culling).
Wasm size probe: `cargo build --profile wasm --lib --target wasm32-unknown-unknown`, then `wasm-opt -Oz`, gzip/brotli the module.
The wasm was not run in a browser and was not passed through wasm-bindgen (CLI/crate version mismatch on this machine). Results: `docs/VISION_2026.md` section 9.2.
Timings were taken on a shared, loaded machine and vary run to run.
