// The checkbox module keeps the public export minimal while the implementation
// file owns the native input wiring and animated control rendering.
mod chk;

pub use chk::Checkbox;
