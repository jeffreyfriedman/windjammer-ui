//! Keyed diff/patch for `simple_vnode::VNode` plus the DOM port it patches through.
//! Everything except `web` is pure and wasm-agnostic. See `MODULE.md`.

pub mod apply;
pub mod diff;
pub mod fake_dom;
pub mod mounted;
pub mod port;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub mod web;

pub use diff::{diff, PatchOp};
pub use mounted::Mounted;
pub use port::DomPort;

/// Child-index path from the tree root. `[]` is the root itself.
pub type Path = Vec<usize>;
