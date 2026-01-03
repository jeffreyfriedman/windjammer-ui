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

# Fix 7: Remove unused/ambiguous glob re-exports in mod.rs
sed -i '' 's/^pub use reactivity::\*;$/\/\/ pub use reactivity::*; \/\/ Removed: causes ambiguous re-exports with signal::*/' src/components/generated/mod.rs
sed -i '' 's/^pub use reactivity_tests::\*;$/\/\/ pub use reactivity_tests::*; \/\/ Removed: unused and test-only/' src/components/generated/mod.rs
sed -i '' 's/^pub use signal::\*;$/\/\/ pub use signal::*; \/\/ Removed: causes ambiguous re-exports with reactivity_optimized::*/' src/components/generated/mod.rs
sed -i '' 's/^pub use event_handler::\*;$/\/\/ pub use event_handler::*; \/\/ Removed: causes ambiguous re-exports with events::*/' src/components/generated/mod.rs
sed -i '' 's/^pub use simple_vnode::\*;$/\/\/ pub use simple_vnode::*; \/\/ Removed: causes ambiguous re-exports with vdom::*/' src/components/generated/mod.rs
sed -i '' 's/^pub use vnode::\*;$/\/\/ pub use vnode::*; \/\/ Removed: causes ambiguous re-exports with simple_vnode::*/' src/components/generated/mod.rs

# Fix 8: Remove unused doc comment before thread_local! macro in signal.rs
sed -i '' 's/^\/\/\/ Global reactive context (thread-local for single-threaded WASM)$/\/\/ Global reactive context (thread-local for single-threaded WASM)/' src/components/generated/signal.rs

# Fix 9: Add #[allow(dead_code)] to unused Effect.id field
if ! grep -q "#\[allow(dead_code)\]" src/components/generated/signal.rs; then
    sed -i '' '/^pub struct Effect {$/a\
    #[allow(dead_code)]
' src/components/generated/signal.rs
fi

# Fix 10: Desktop-specific clippy fixes
# Remove desktop_renderer::* to avoid ambiguous re-exports with renderer::*
sed -i '' 's/^pub use desktop_renderer::\*;$/\/\/ pub use desktop_renderer::*; \/\/ Removed: causes ambiguous re-exports with renderer::*/' src/components/generated/mod.rs

# Fix unused variable and field in desktop_renderer_v2.rs
sed -i '' 's/fn render_panel(&mut self, ui: &mut Ui, attrs:/fn render_panel(\&mut self, ui: \&mut Ui, _attrs:/' src/components/generated/desktop_renderer_v2.rs
if grep -q "pub struct DesktopRendererV2" src/components/generated/desktop_renderer_v2.rs; then
    if ! grep -q "event_handlers.*#\[allow(dead_code)\]" src/components/generated/desktop_renderer_v2.rs; then
        sed -i '' '/pub struct DesktopRendererV2 {$/,/event_handlers:/ s/event_handlers:/#[allow(dead_code)]\n    event_handlers:/' src/components/generated/desktop_renderer_v2.rs
    fi
fi

echo "✅ Fixes applied successfully!"

