//! `wj-layout` (layer L2): turns scene `Style` props into rectangles.
//!
//! Flexbox comes from Taffy (adopted, ADR-0008); this crate owns the adapter, incremental
//! synchronisation from the scene's dirty log, and the text-measure seam. Stack, scroll and
//! virtual-list behaviour is expressed in scene styles (`absolute`, `scroll`) and the scene's
//! `virtual_list`. See `MODULE.md`.

mod convert;
mod engine;
mod measure;

pub use engine::LayoutEngine;
pub use measure::{FixedAdvanceMeasure, TextMeasure};
