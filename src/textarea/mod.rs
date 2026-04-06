// The textarea component is a single Rust file today, but it still uses a module wrapper to match
// the structure of the other form controls.
mod txt;

pub use txt::Textarea;
