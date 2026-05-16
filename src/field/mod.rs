// Field layout stays separate from the concrete controls so callers can compose
// any existing input-like component with the same label alignment.
mod fld;

pub use fld::Field;
