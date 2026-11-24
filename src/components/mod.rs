//! Windjammer UI Component Library
//!
//! Components for building web, desktop, and mobile applications. Every component is written in Windjammer
//! (`src/components_wj/`); `generated` is the compiler's Rust output, kept as compiler wrote it (not reformatted
//! or linted by hand) and verified against the sources by `tests/generated_fresh_test.rs`.

#[rustfmt::skip]
#[allow(warnings, clippy::all)]
pub mod generated;

// Re-export all generated components for convenience
pub use generated::*;
