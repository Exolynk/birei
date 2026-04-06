// Keep the tab trigger implementation and its public item types grouped under one component module.
mod tab_list;
mod tab_types;

pub use tab_list::TabList;
pub use tab_types::{TabItem, TabLinePosition};
