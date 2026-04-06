// Select keeps the interaction-heavy popup logic separate from its option type
// definitions so the public API stays easy to scan.
mod sel;
mod sel_types;

pub use sel::Select;
pub use sel_types::SelectOption;
