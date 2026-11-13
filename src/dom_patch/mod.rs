//! Keyed diff/patch for `simple_vnode::VNode` plus the DOM port it patches through.
//! Everything except `web` is pure and wasm-agnostic. See `MODULE.md`.

pub mod apply;
pub mod diff;
pub mod event_info;
pub mod fake_dom;
pub mod mounted;
pub mod port;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub mod web;

pub use diff::{diff, PatchOp};
pub use event_info::{current_event, set_current_event, EventInfo};
pub use mounted::{prevent_default, run_handlers, stop_propagation, DispatchOutcome, Mounted};
pub use port::DomPort;

/// Child-index path from the tree root. `[]` is the root itself.
pub type Path = Vec<usize>;
