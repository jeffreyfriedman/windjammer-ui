#!/bin/bash
# Regenerate src/components/generated/ from src/components_wj/ with the `wj` on PATH
# (override with WJ=/path/to/wj).
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WJ_CLI="${WJ:-wj}"

if ! command -v "$WJ_CLI" >/dev/null 2>&1; then
    echo "error: wj compiler not found (put it on PATH or set WJ=/path/to/wj)" >&2
    exit 1
fi

SRC_DIR="$SCRIPT_DIR/src/components_wj"
OUT_DIR="$SCRIPT_DIR/src/components/generated"
mkdir -p "$OUT_DIR"

echo "Transpiling $SRC_DIR -> $OUT_DIR"
# --library strips main(); --module-file generates mod.rs with the pub mod declarations.
"$WJ_CLI" build "$SRC_DIR" -o "$OUT_DIR" --target rust --library --module-file

echo "Done."
