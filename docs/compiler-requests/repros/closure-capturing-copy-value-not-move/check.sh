#!/bin/sh
# Transpile with the pinned compiler, show the generated Rust, then type-check it.
# Usage: WJ_BIN=/path/to/wj ./check.sh
set -e
WJ="${WJ_BIN:-wj}"
here="$(cd "$(dirname "$0")" && pwd)"
out="$(mktemp -d)"
( cd "$here/app" && "$WJ" build src --library --no-cargo --no-lint -o "$out" >/dev/null )
echo "--- generated Rust ---"; cat "$out/main.rs"
cat > "$out/Cargo.toml" <<TOML
[package]
name = "repro"
version = "0.1.0"
edition = "2021"
[lib]
path = "lib.rs"
[dependencies]
handle_lib = { path = "$here/lib" }
[workspace]
TOML
printf 'pub mod main;\n' > "$out/lib.rs"
echo "--- cargo check ---"
( cd "$out" && cargo check 2>&1 | grep -E '^(error|warning: unused)' -A6 | head -40 ) || true
