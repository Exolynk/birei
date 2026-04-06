// The tag component is intentionally tiny, but it still lives behind its own module boundary so
// the public API stays consistent with larger components.
mod tag_view;

pub use tag_view::Tag;
