// The card keeps its interactive view logic isolated from the module export so
// the crate root only exposes the final component type.
mod crd;

pub use crd::Card;
