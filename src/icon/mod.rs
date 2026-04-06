// Icon rendering, generated names, and the typed icon-name wrapper live in
// separate files so generated content does not obscure the handwritten API.
mod icn;
pub mod icn_names;
mod icon_types;

pub use icn::Icon;
pub use icn_names::*;
pub use icon_types::IcnName;
