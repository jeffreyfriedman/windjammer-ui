#!/bin/bash
# Temporary script to fix generated code issues
# These fixes should eventually be addressed in the Windjammer compiler itself

set -e

cd "$(dirname "$0")"

echo "🔧 Applying fixes to generated code..."

# Fix 1: Remove lib module completely (rustfmt can't handle it)
# Delete the lines entirely and rename the file
sed -i '' '/^pub mod lib;$/d' src/components/generated/mod.rs
sed -i '' '/^pub use lib::\*;$/d' src/components/generated/mod.rs
# Rename lib.rs so it doesn't interfere
if [ -f src/components/generated/lib.rs ]; then
    mv src/components/generated/lib.rs src/components/generated/lib.rs.disabled
fi

# Fix 2: Add desktop feature gates
sed -i '' 's/^pub mod app;$/#[cfg(feature = "desktop")]\npub mod app;/' src/components/generated/mod.rs
sed -i '' 's/^pub mod app_docking;$/#[cfg(feature = "desktop")]\npub mod app_docking;/' src/components/generated/mod.rs
sed -i '' 's/^pub mod app_reactive_eframe;$/#[cfg(feature = "desktop")]\npub mod app_reactive_eframe;/' src/components/generated/mod.rs
sed -i '' 's/^pub mod desktop_app_context;$/#[cfg(feature = "desktop")]\npub mod desktop_app_context;/' src/components/generated/mod.rs
sed -i '' 's/^pub mod desktop_renderer;$/#[cfg(feature = "desktop")]\npub mod desktop_renderer;/' src/components/generated/mod.rs
sed -i '' 's/^pub mod desktop_renderer_v2;$/#[cfg(feature = "desktop")]\npub mod desktop_renderer_v2;/' src/components/generated/mod.rs

sed -i '' 's/^pub use app::\*;$/#[cfg(feature = "desktop")]\npub use app::*;/' src/components/generated/mod.rs
sed -i '' 's/^pub use app_docking::\*;$/#[cfg(feature = "desktop")]\npub use app_docking::*;/' src/components/generated/mod.rs
sed -i '' 's/^pub use app_reactive_eframe::\*;$/#[cfg(feature = "desktop")]\npub use app_reactive_eframe::*;/' src/components/generated/mod.rs
sed -i '' 's/^pub use desktop_app_context::\*;$/#[cfg(feature = "desktop")]\npub use desktop_app_context::*;/' src/components/generated/mod.rs
sed -i '' 's/^pub use desktop_renderer::\*;$/#[cfg(feature = "desktop")]\npub use desktop_renderer::*;/' src/components/generated/mod.rs
sed -i '' 's/^pub use desktop_renderer_v2::\*;$/#[cfg(feature = "desktop")]\npub use desktop_renderer_v2::*;/' src/components/generated/mod.rs

# Fix 3: Delete app_reactive and examples_wasm declarations entirely
sed -i '' '/^pub mod app_reactive;$/d' src/components/generated/mod.rs
sed -i '' '/^pub mod examples_wasm;$/d' src/components/generated/mod.rs
sed -i '' '/^pub use app_reactive::\*;$/d' src/components/generated/mod.rs
sed -i '' '/^pub use examples_wasm::\*;$/d' src/components/generated/mod.rs

# Fix 4: Delete missing dispatcher module declarations
sed -i '' '/^pub mod dispatcher;$/d' src/components/generated/events.rs
sed -i '' '/^pub use dispatcher::ComponentEventDispatcher;$/d' src/components/generated/events.rs

# Fix 5: Add 'static lifetime to Computed Debug impl in signal.rs
sed -i '' 's/impl<T: Clone + std::fmt::Debug> std::fmt::Debug for Computed<T>/impl<T: Clone + std::fmt::Debug + '\''static> std::fmt::Debug for Computed<T>/' src/components/generated/signal.rs

# Fix 6: Cast Rc<F> to Rc<dyn Fn()> in Effect::new
sed -i '' 's/let f = Rc::new(f);$/let f: Rc<dyn Fn()> = Rc::new(f);/' src/components/generated/signal.rs

echo "✅ Fixes applied successfully!"

