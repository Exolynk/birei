// Flexible columns keeps its interactive layout logic and semantic column enum
// separate so the public surface stays easy to scan.
mod fcl;
mod fcl_types;

pub use fcl::FlexibleColumns;
pub use fcl_types::FlexibleColumn;
