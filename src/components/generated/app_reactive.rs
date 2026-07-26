#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
/// Reactive application runtime for WASM
/// Simpler version without the complex winit+wgpu setup
use crate::reactive_mount::{default_mount_selector, MountTarget};
use crate::simple_vnode::VNode;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Global render callback for triggering re-renders
static mut RENDER_CALLBACK: Option<Box<dyn Fn()>> = None;

pub fn set_render_callback<F: Fn() + 'static>(callback: F) {
    unsafe {
        RENDER_CALLBACK = Some(Box::new(callback));
    }
}

pub fn trigger_rerender() {
    unsafe {
        let callback_ptr = &raw const RENDER_CALLBACK;
        if let Some(callback) = (*callback_ptr).as_ref() {
            callback();
        }
    }
}

/// Reactive application that automatically re-renders when signals change
pub struct ReactiveApp {
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    render_fn: Rc<dyn Fn() -> VNode>,
    mount: MountTarget,
}

impl ReactiveApp {
    pub fn new<F>(title: String, render_fn: F) -> Self
    where
        F: Fn() -> VNode + 'static,
    {
        Self {
            title,
            render_fn: Rc::new(render_fn),
            mount: MountTarget::new(),
        }
    }

    /// Mount into a CSS selector instead of the default `#app`.
    /// Hybrid LedgerKit shell: `.mount_target("#main")` preserves chrome.
    pub fn mount_target(mut self, selector: impl Into<String>) -> Self {
        self.mount = self.mount.mount_target(selector);
        self
    }

    /// Current mount selector (for tests / introspection).
    pub fn mount_selector(&self) -> &str {
        self.mount.selector()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run(self) {
        use wasm_bindgen::JsCast;
        use web_sys::{window, HtmlElement};

        let document = window().unwrap().document().unwrap();
        let selector = self.mount.selector().to_string();
        let root = document
            .query_selector(&selector)
            .expect("query_selector failed")
            .unwrap_or_else(|| panic!("Failed to find mount target '{}'", selector))
            .dyn_into::<HtmlElement>()
            .unwrap();

        let render_fn = self.render_fn.clone();
        let needs_rerender = Rc::new(RefCell::new(true));

        let needs_rerender_clone = needs_rerender.clone();
        set_render_callback(move || {
            *needs_rerender_clone.borrow_mut() = true;
        });

        let render_fn_clone = render_fn.clone();
        let needs_rerender_clone = needs_rerender.clone();

        let render_loop = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
        let render_loop_clone = render_loop.clone();

        *render_loop.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if *needs_rerender_clone.borrow() {
                *needs_rerender_clone.borrow_mut() = false;

                let vnode = render_fn_clone();
                let html = crate::simple_renderer::render_to_html(&vnode);
                root.set_inner_html(&html);
            }

            web_sys::window()
                .unwrap()
                .request_animation_frame(
                    render_loop_clone
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .request_animation_frame(
                render_loop
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(self) {
        let _ = default_mount_selector();
        panic!(
            "ReactiveApp::run() is only available on wasm32 targets.\n\
             For desktop applications, use ReactiveApp from app_reactive_eframe module.\n\
             Example: use windjammer_ui::components::app_reactive_eframe::ReactiveApp;"
        );
    }
}
