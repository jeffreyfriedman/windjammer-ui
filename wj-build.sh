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
SRC_DIR="$SCRIPT_DIR/src_wj/components"
BUILD_DIR="$SCRIPT_DIR/build_components"
OUT_DIR="$SCRIPT_DIR/src/components/generated"

# Clean and create directories
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
mkdir -p "$OUT_DIR"

echo "📦 Transpiling Windjammer components..."
echo "   Source: $SRC_DIR"
echo "   Output: $OUT_DIR"
echo ""

# Build all components to temp directory
"$WJ_CLI" build "$SRC_DIR" -o "$BUILD_DIR" --target rust

echo ""
echo "📝 Processing generated files (removing main(), adding pub)..."
echo ""

# Count files
SUCCESS=0
FAILED=0

# Process each generated .rs file
for rs_file in "$BUILD_DIR"/*.rs; do
    if [ -f "$rs_file" ]; then
        filename=$(basename "$rs_file")
        
        # Skip if it's not a component file
        if [[ "$filename" == "serve_demo.rs" ]] || [[ "$filename" == "counter.rs" ]] || [[ "$filename" == "contact_form.rs" ]] || [[ "$filename" == "dashboard.rs" ]]; then
            continue
        fi
        
        echo "  📄 Processing $filename..."
        
        # 1. Remove fn main() and its body
        # 2. Add pub to enum/struct/fn declarations (Bug #8 workaround)
        if sed '/^fn main()/,/^}$/d' "$rs_file" | \
           sed 's/^enum /pub enum /' | \
           sed 's/^struct /pub struct /' | \
           sed 's/^fn /pub fn /' \
           > "$OUT_DIR/$filename"; then
            SUCCESS=$((SUCCESS + 1))
            echo "     ✅ Generated $OUT_DIR/$filename"
        else
            FAILED=$((FAILED + 1))
            echo "     ❌ Failed to process"
        fi
    fi
done

# Clean up temp build directory
rm -rf "$BUILD_DIR"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $FAILED -eq 0 ]; then
    echo "✅ All $SUCCESS component(s) transpiled successfully!"
    echo ""
    echo "Generated files in: $OUT_DIR"
    echo ""
    echo "⚠️  Note: Bug #8 workaround applied (manually adding pub keywords)"
    echo ""
    echo "Next: cargo build && cargo test"
    exit 0
else
    echo "❌ $FAILED component(s) failed"
    echo "✅ $SUCCESS component(s) succeeded"
    exit 1
fi
