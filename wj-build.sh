#!/bin/bash
# Pure Windjammer build pipeline for windjammer-ui
# Developers only need to know Windjammer - no Rust/Cargo knowledge required!

set -e

echo "🔨 Building windjammer-ui from pure Windjammer source..."
echo ""

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
WJ_CLI="$SCRIPT_DIR/../windjammer/target/release/wj"

# Check if wj CLI exists
if [ ! -f "$WJ_CLI" ]; then
    echo "❌ Error: wj CLI not found at $WJ_CLI"
    echo "Please build it first: cd ../windjammer && cargo build --release"
    exit 1
fi

echo "✓ Found wj CLI at $WJ_CLI"
echo ""

# Source and output directories  
SRC_DIR="$SCRIPT_DIR/src/components_wj"
OUT_DIR="$SCRIPT_DIR/src/components/generated"

# Create output directory
mkdir -p "$OUT_DIR"

echo "📦 Transpiling Windjammer components..."
echo "   Source: $SRC_DIR"
echo "   Output: $OUT_DIR"
echo ""

# Build all components with new CLI flags:
# --library: Automatically strips main() test functions
# --module-file: Auto-generates mod.rs with pub mod declarations
"$WJ_CLI" build "$SRC_DIR" -o "$OUT_DIR" --target rust --library --module-file

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All components transpiled successfully!"
echo ""
echo "Generated files in: $OUT_DIR"
echo ""
echo "🎉 New CLI features working:"
echo "  - --library mode: main() functions stripped automatically"
echo "  - --module-file: mod.rs generated automatically"
echo ""
echo "Next: cargo build && cargo test"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
