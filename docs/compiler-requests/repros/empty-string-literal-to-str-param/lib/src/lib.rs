//! A tiny external Rust crate: a Copy handle type, a function that stores a 'static callback,
//! and a few typed-parameter functions. Nothing here is specific to any framework.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Handle(pub u32);

pub fn new_handle(_initial: i32) -> Handle {
    Handle(0)
}

impl Handle {
    pub fn get(&self) -> i32 {
        self.0 as i32
    }
    pub fn set(&self, _v: i32) {}
}

/// Stores `f` for later: requires 'static, like any callback registry.
pub fn run_later(f: impl Fn() + 'static) {
    f();
}

pub fn set_label(_text: &str) {}

pub fn set_scale(_factor: f64) {}

pub struct Record {
    pub id: i64,
}
