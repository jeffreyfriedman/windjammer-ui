//! `wj-a11y` (layer L3): consumes the scene's [`SemanticTree`] and produces
//!  * incremental AccessKit [`TreeUpdate`]s for native platforms ([`AccessKitTree`]), and
//!  * a DOM projection ([`project`], [`ProjectedNode::to_html`]) that a web host mounts as a hidden
//!    mirror for screen readers, find-in-page, selection and SEO.
//!
//! Both are pure functions of the semantic tree, so they run headless and are snapshot-tested.

mod ak;
mod dom;

pub use ak::{scene_parts, AccessKitTree, ROOT_ID};
pub use dom::{project, ProjectedNode};
