#!/bin/bash
# Build Windjammer UI WASM packages for the web, bundler and Node.js targets.
set -e

if ! command -v wasm-pack &> /dev/null; then
    echo "error: wasm-pack is not installed (install it with: cargo install wasm-pack)" >&2
    exit 1
fi

wasm-pack build --target web --out-dir pkg/web
wasm-pack build --target bundler --out-dir pkg/bundler
wasm-pack build --target nodejs --out-dir pkg/nodejs

echo "Built:"
echo "  pkg/web/      (vanilla HTML/JS)"
echo "  pkg/bundler/  (webpack/rollup/vite)"
echo "  pkg/nodejs/   (Node.js)"
