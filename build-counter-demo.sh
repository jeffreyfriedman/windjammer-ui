#!/bin/bash
# Build the browser counter demo (wasm-pack, web target).
set -e

if ! command -v wasm-pack &> /dev/null; then
    echo "error: wasm-pack not found (install it with: cargo install wasm-pack)" >&2
    exit 1
fi

wasm-pack build --target web --dev

echo "Built pkg/windjammer_ui.js"
echo "Serve the repo root (python3 -m http.server 8000) and open http://localhost:8000/examples/counter_demo.html"
