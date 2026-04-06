// Input rendering and its typed HTML attribute enums stay split so the public
// API remains small while still exposing strongly typed options.
mod inp;
mod inp_types;

pub use inp::Input;
pub use inp_types::{InputAutocomplete, InputType};
