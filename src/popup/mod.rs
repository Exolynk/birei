// Popup is kept in a small dedicated module because the public API is a
// single modal component while the view and styling remain self-contained.
mod view;

pub use view::Popup;
