#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Desktop app context for reactive re-rendering
//!
//! This module provides a way for Signals to trigger UI re-renders in desktop apps

use std::cell::RefCell;

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
thread_local! {
    static REPAINT_CALLBACK: RefCell<Option<Box<dyn Fn()>>> = RefCell::new(None);
}

/// Set the repaint callback (called when signals change)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub fn set_repaint_callback<F: Fn() + 'static>(callback: F) {
    REPAINT_CALLBACK.with(|rc| {
        *rc.borrow_mut() = Some(Box::new(callback));
    });
}

/// Trigger a repaint (called by Signal::notify)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub fn trigger_repaint() {
    REPAINT_CALLBACK.with(|rc| {
        if let Some(callback) = rc.borrow().as_ref() {
            callback();
        }
    });
}

/// Clear the repaint callback (for cleanup)
#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub fn clear_repaint_callback() {
    REPAINT_CALLBACK.with(|rc| {
        *rc.borrow_mut() = None;
    });
}
