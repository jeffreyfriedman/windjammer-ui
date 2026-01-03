//! App runtime for mounting and running Windjammer UI applications
//!
//! This module provides the `App` struct and runtime system for:
//! - Mounting UI components to the DOM (WASM)
//! - Running the reactive system
//! - Handling the render loop
//! - Managing application lifecycle

use crate::simple_vnode::VNode;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, Element};

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// Global app state for re-rendering
    static APP_STATE: RefCell<Option<AppState>> = RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
struct AppState {
    render_fn: Rc<dyn Fn() -> VNode>,
    root_element: Element,
    document: web_sys::Document,
}

/// Application runtime
pub struct App {
    /// Application title
    pub title: String,
    /// Root UI component (static VNode for now)
    pub root: VNode,
    /// Optional render function for reactive apps
    pub render_fn: Option<Box<dyn Fn() -> VNode>>,
}

impl App {
    /// Create a new application with a static VNode
    pub fn new(title: impl Into<String>, root: VNode) -> Self {
        Self {
            title: title.into(),
            root,
            render_fn: None,
        }
    }

    /// Create a new application with a render function (reactive)
    pub fn new_reactive<F>(title: impl Into<String>, render_fn: F) -> Self
    where
        F: Fn() -> VNode + 'static,
    {
        let initial_vnode = render_fn();
        Self {
            title: title.into(),
            root: initial_vnode,
            render_fn: Some(Box::new(render_fn)),
        }
    }

    /// Run the application (WASM only)
    #[cfg(target_arch = "wasm32")]
    pub fn run(self) {
        use crate::simple_renderer;
        use wasm_bindgen::JsCast;

        // Get the root element from the DOM
        let window = web_sys::window().expect("No window found");
        let document = window.document().expect("No document found");
        let root_el = document
            .get_element_by_id("app")
            .expect("No #app element found in HTML")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Root is not an HTMLElement");

        // Render the initial UI
        let render_fn = self.render_fn;
        let mut current_vnode = self.root;

        // Simple initial render
        let html = simple_renderer::render_to_html(&current_vnode);
        root_el.set_inner_html(&html);

        web_sys::console::log_1(&format!("✅ {} mounted", self.title).into());

        // TODO: Add reactive re-rendering support for WASM
        // For now, just renders the initial state
    }

    /// Run the application (Desktop with egui/eframe)
    #[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
    pub fn run(self) {
        use crate::desktop_renderer::DesktopRenderer;

        let title = self.title;
        let render_fn = self.render_fn;
        let mut current_vnode = self.root;

        let options = eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0])
                .with_title(title.clone()),
            ..Default::default()
        };

        let _ = eframe::run_simple_native(&title, options, move |ctx, _frame| {
            // Set up repaint callback for reactive updates
            let ctx_clone = ctx.clone();
            crate::desktop_app_context::set_repaint_callback(move || {
                ctx_clone.request_repaint();
            });

            // Re-generate VNode if we have a render function (reactive mode)
            if let Some(ref render) = render_fn {
                current_vnode = render();
            }

            // Render the UI
            let mut renderer = DesktopRenderer::new();
            renderer.render(ctx, &current_vnode);
        });

        // Cleanup
        crate::desktop_app_context::clear_repaint_callback();
    }

    /// Run the application (Non-desktop, non-WASM - error)
    #[cfg(all(not(target_arch = "wasm32"), not(feature = "desktop")))]
    pub fn run(self) {
        eprintln!("❌ Error: App::run() requires either:");
        eprintln!("   - WASM target (for browser)");
        eprintln!("   - 'desktop' feature (for native)");
        panic!("Cannot run app without a supported platform");
    }

    /// Internal run method that returns Result
    #[cfg(target_arch = "wasm32")]
    fn run_internal(self) -> Result<(), JsValue> {
        // Set up panic hook for better error messages
        console_error_panic_hook::set_once();

        web_sys::console::log_1(&"🔧 Starting App::run_internal".into());

        // Get the window and document
        let window = window().ok_or("No window found")?;
        let document = window.document().ok_or("No document found")?;

        web_sys::console::log_1(&"✓ Got window and document".into());

        // Set the document title
        document.set_title(&self.title);

        // Get or create the root element
        let root_element = document
            .get_element_by_id("app")
            .or_else(|| document.body().map(|b| b.into()))
            .ok_or("No root element found")?;

        web_sys::console::log_1(&"✓ Got root element".into());

        // Clear existing content
        root_element.set_inner_html("");

        web_sys::console::log_1(&"✓ Cleared root element".into());

        // Render the root VNode
        web_sys::console::log_1(&"🎨 Rendering VNode...".into());
        let rendered = self.root.render(&document)?;

        web_sys::console::log_1(&"✓ VNode rendered".into());

        root_element.append_child(&rendered)?;

        web_sys::console::log_1(&"✅ UI mounted successfully!".into());

        Ok(())
    }
}

/// Mount a component to the DOM (WASM only)
#[cfg(target_arch = "wasm32")]
pub fn mount(root: VNode) {
    App::new("Windjammer App", root).run()
}

/// Mount a component with a custom title (WASM only)
#[cfg(target_arch = "wasm32")]
pub fn mount_with_title(title: impl Into<String>, root: VNode) {
    App::new(title, root).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new("Test App", VNode::Text("Hello".to_string()));
        assert_eq!(app.title, "Test App");
    }
}
