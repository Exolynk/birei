//! Command palette: the launcher component, its result groups, filtering and list scrolling.

mod cmd_plt;
mod cmd_plt_types;
mod filter;
mod groups;
mod scroll;

pub use cmd_plt::CommandPalette;
pub use cmd_plt_types::{
    CommandExecution, CommandItem, CommandParameter, CommandParameterOption, CommandParameterValue,
};
