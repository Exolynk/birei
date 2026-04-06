// The color input keeps its composed control logic in a dedicated file while
// the module export stays small and crate-root friendly.
mod clr;

pub use clr::ColorInput;
