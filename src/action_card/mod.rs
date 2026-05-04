// Action cards expose a single compact highlight component while keeping the
// formatting and animation logic local to the module.
mod upload;
mod view;

pub use upload::ActionCardUpload;
pub use view::ActionCard;
