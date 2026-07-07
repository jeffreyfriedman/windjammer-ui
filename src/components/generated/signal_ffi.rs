#![allow(clippy::all)]
#![allow(noop_method_call)]
#![allow(clippy::all)]
#![allow(noop_method_call)]
//! Signal FFI - Functions for using reactive signals from Windjammer code
//!
//! Provides a stable FFI interface for creating and manipulating typed signals.
//! Uses opaque handles (u64) to reference signals in a thread-local registry,
//! mirroring the VNode FFI pattern.

use crate::reactivity::Signal;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static I32_SIGNALS: RefCell<SignalRegistry<i32>> = RefCell::new(SignalRegistry::new());
    static F32_SIGNALS: RefCell<SignalRegistry<f32>> = RefCell::new(SignalRegistry::new());
    static BOOL_SIGNALS: RefCell<SignalRegistry<bool>> = RefCell::new(SignalRegistry::new());
    static STRING_SIGNALS: RefCell<SignalRegistry<String>> = RefCell::new(SignalRegistry::new());
}

struct SignalRegistry<T: Clone> {
    signals: HashMap<u64, Signal<T>>,
    next_handle: u64,
}

impl<T: Clone> SignalRegistry<T> {
    fn new() -> Self {
        Self {
            signals: HashMap::new(),
            next_handle: 1,
        }
    }

    fn insert(&mut self, signal: Signal<T>) -> u64 {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.signals.insert(handle, signal);
        handle
    }
}

// ============================================================================
// i32 signals
// ============================================================================

pub fn signal_new_i32(value: i32) -> u64 {
    I32_SIGNALS.with(|registry| registry.borrow_mut().insert(Signal::new(value)))
}

pub fn signal_get_i32(handle: u64) -> i32 {
    I32_SIGNALS.with(|registry| {
        registry
            .borrow()
            .signals
            .get(&handle)
            .map(|s| s.get())
            .unwrap_or(0)
    })
}

pub fn signal_set_i32(handle: u64, value: i32) {
    I32_SIGNALS.with(|registry| {
        if let Some(signal) = registry.borrow().signals.get(&handle) {
            signal.set(value);
        }
    });
}

// ============================================================================
// f32 signals
// ============================================================================

pub fn signal_new_f32(value: f32) -> u64 {
    F32_SIGNALS.with(|registry| registry.borrow_mut().insert(Signal::new(value)))
}

pub fn signal_get_f32(handle: u64) -> f32 {
    F32_SIGNALS.with(|registry| {
        registry
            .borrow()
            .signals
            .get(&handle)
            .map(|s| s.get())
            .unwrap_or(0.0)
    })
}

pub fn signal_set_f32(handle: u64, value: f32) {
    F32_SIGNALS.with(|registry| {
        if let Some(signal) = registry.borrow().signals.get(&handle) {
            signal.set(value);
        }
    });
}

// ============================================================================
// bool signals
// ============================================================================

pub fn signal_new_bool(value: bool) -> u64 {
    BOOL_SIGNALS.with(|registry| registry.borrow_mut().insert(Signal::new(value)))
}

pub fn signal_get_bool(handle: u64) -> bool {
    BOOL_SIGNALS.with(|registry| {
        registry
            .borrow()
            .signals
            .get(&handle)
            .map(|s| s.get())
            .unwrap_or(false)
    })
}

pub fn signal_set_bool(handle: u64, value: bool) {
    BOOL_SIGNALS.with(|registry| {
        if let Some(signal) = registry.borrow().signals.get(&handle) {
            signal.set(value);
        }
    });
}

// ============================================================================
// String signals
// ============================================================================

pub fn signal_new_string(value: &str) -> u64 {
    STRING_SIGNALS.with(|registry| registry.borrow_mut().insert(Signal::new(value.to_string())))
}

pub fn signal_get_string(handle: u64) -> String {
    STRING_SIGNALS.with(|registry| {
        registry
            .borrow()
            .signals
            .get(&handle)
            .map(|s| s.get())
            .unwrap_or_default()
    })
}

pub fn signal_set_string(handle: u64, value: &str) {
    STRING_SIGNALS.with(|registry| {
        if let Some(signal) = registry.borrow().signals.get(&handle) {
            signal.set(value.to_string());
        }
    });
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_signal() {
        let handle = signal_new_i32(42);
        assert_eq!(signal_get_i32(handle), 42);
        signal_set_i32(handle, 100);
        assert_eq!(signal_get_i32(handle), 100);
    }

    #[test]
    fn test_f32_signal() {
        let handle = signal_new_f32(3.14);
        let val = signal_get_f32(handle);
        assert!((val - 3.14).abs() < 0.001);
        signal_set_f32(handle, 2.71);
        let val = signal_get_f32(handle);
        assert!((val - 2.71).abs() < 0.001);
    }

    #[test]
    fn test_bool_signal() {
        let handle = signal_new_bool(false);
        assert!(!signal_get_bool(handle));
        signal_set_bool(handle, true);
        assert!(signal_get_bool(handle));
    }

    #[test]
    fn test_string_signal() {
        let handle = signal_new_string("hello");
        assert_eq!(signal_get_string(handle), "hello");
        signal_set_string(handle, "world");
        assert_eq!(signal_get_string(handle), "world");
    }

    #[test]
    fn test_multiple_signals() {
        let a = signal_new_i32(1);
        let b = signal_new_i32(2);
        assert_eq!(signal_get_i32(a), 1);
        assert_eq!(signal_get_i32(b), 2);
        signal_set_i32(a, 10);
        assert_eq!(signal_get_i32(a), 10);
        assert_eq!(signal_get_i32(b), 2);
    }
}
