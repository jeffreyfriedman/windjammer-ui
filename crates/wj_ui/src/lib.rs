//! `wj-ui`: the API Windjammer code calls to build interfaces.
//!
//! Design constraints, taken from what Windjammer's ownership inference needs today:
//!  * every handle is `Copy` ([`Node`], and `wj-reactive`'s `Signal` / `Memo`), so closures capture
//!    them with no `clone` or `move` ceremony in user code;
//!  * builder methods take `self` and return it, so chains read top to bottom;
//!  * numbers are `f64` / `i64`, the types Windjammer literals default to;
//!  * the scene being built is implicit (`use_app`), so `column()` needs no context argument.
//!
//! This is the library-convention form of the component DSL (VISION section 3, R1/R2 prototyped
//! without compiler support). See `MODULE.md`.

mod node;
mod ctx;
mod list;

pub use ctx::{use_app, with_scene};
pub use list::each;
pub use node::{button, button_fn, checkbox, column, heading, row, text, text_field, text_of, Node};
