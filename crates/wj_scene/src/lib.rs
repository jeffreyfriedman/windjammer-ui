//! `wj-scene` (layer L1): the retained node tree every Windjammer UI renders from.
//!
//! A [`Scene`] is a `Copy` handle to a per-thread arena of nodes. Each node carries layout props
//! ([`Style`]), paint props ([`Paint`], [`TextProps`]), semantics ([`Semantics`]) and event
//! handlers, plus dirty flags. Reactive *binding sites* ([`Scene::bind`], [`Scene::bind_text`],
//! [`Scene::keyed_children`]) are effects owned by the node that write props directly, so a state
//! change touches exactly the nodes that read it. See `MODULE.md`.

mod bind;
mod dirty;
mod event;
mod geom;
mod interact;
mod semantics;
mod style;
mod tree;
mod virtual_list;

pub use dirty::{Dirty, DirtyEntry};
pub use event::{Event, EventKind, Handler};
pub use geom::{Color, Point, Rect};
pub use semantics::{Action, Role, SemanticNode, SemanticTree, Semantics};
pub use style::{Align, Dim, Direction, Edges, Justify, Paint, Style, TextProps};
pub use tree::{NodeId, Scene};
pub use virtual_list::visible_range;
