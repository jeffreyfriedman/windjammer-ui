//! `wj-reactive`: the reactive core of Windjammer UI (layer L0).
//!
//! State lives in a per-thread arena. Handles ([`Signal`], [`Memo`], [`Scope`]) are small `Copy`
//! values (`index + generation`), so closures capture them by copy with no `clone`, `move` or `Rc`
//! ceremony in user code. Reads inside [`derived`] / [`effect`] bodies are tracked; reads anywhere
//! else are not. Writes are glitch-free: each effect runs at most once per update or [`batch`].
//!
//! See `MODULE.md` for the module contract and the test strategy.

mod handles;
mod runtime;
mod scope;

pub use handles::{Memo, Signal};
pub use runtime::{batch, derived, effect, reset, state, stats, untrack, Stats};
pub use scope::{create_scope, Scope};
