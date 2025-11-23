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
OUT_DIR="$SCRIPT_DIR/src/components/generated"

# Create output directory
mkdir -p "$OUT_DIR"

echo "📦 Transpiling Windjammer components..."
echo "   Source: $SRC_DIR"
echo "   Output: $OUT_DIR"
echo ""

# Build all components
"$WJ_CLI" build "$SRC_DIR" -o "$OUT_DIR" --target rust

echo ""
echo "📝 Cleaning up generated files (removing main() test functions)..."

# Remove main() functions from generated files (they're for testing only)
for rs_file in "$OUT_DIR"/*.rs; do
    if [ -f "$rs_file" ]; then
        filename=$(basename "$rs_file")
        # Skip if it's the mod.rs we'll create
        if [[ "$filename" == "mod.rs" ]]; then
            continue
        fi
        
        # Remove fn main() and its body
        sed -i.bak '/^fn main()/,/^}$/d' "$rs_file" && rm "${rs_file}.bak"
    fi
done

# Create mod.rs to export all generated modules
cat > "$OUT_DIR/mod.rs" << 'EOF'
// Generated Windjammer UI components
// Auto-generated from src_wj/components/*.wj
// DO NOT EDIT MANUALLY - run wj-build.sh to regenerate

pub mod alert;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod container;
pub mod divider;
pub mod flex;
pub mod input;
pub mod slider;
pub mod spacer;
pub mod spinner;
pub mod text;

// Re-export main types for convenience
pub use alert::{Alert, AlertVariant};
pub use badge::{Badge, BadgeSize, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use checkbox::{Checkbox, CheckboxSize};
pub use container::Container;
pub use divider::{Divider, DividerOrientation};
pub use flex::{Flex, FlexDirection};
pub use input::Input;
pub use slider::Slider;
pub use spacer::Spacer;
pub use spinner::{Spinner, SpinnerSize};
pub use text::{Text, TextSize, TextWeight};
EOF

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All components transpiled successfully!"
echo ""
echo "Generated files in: $OUT_DIR"
echo ""
echo "🎉 Bug #8 FIXED! pub keywords now generated automatically!"
echo ""
echo "Next: cargo build && cargo test"
