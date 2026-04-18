// Relation graphs are split into data types, layout, and the interactive view
// so the public API stays small while the layout code remains testable.
mod internal;
mod layout;
mod types;
mod view;

pub use types::{RelationGraphEdge, RelationGraphNode};
pub use view::RelationGraph;
