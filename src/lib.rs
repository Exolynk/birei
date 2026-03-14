mod button;
mod common;
mod input;

pub use button::{Button, ButtonGroup, ButtonType, ButtonVariant};
pub use common::Size;
pub use input::{Input, InputAutocomplete, InputType};

#[cfg(feature = "embedded-css")]
pub const CSS: &str = include_str!("../dist/birei.css");
