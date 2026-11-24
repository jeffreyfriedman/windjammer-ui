#!/bin/sh
# Runs every M0/M1 gate for the new core. Usage: WJ_BIN=/path/to/wj CARGO_TARGET_DIR=<cache> ./gates.sh
set -e
cd "$(dirname "$0")"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
cargo test --release --workspace
