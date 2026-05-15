use leptos::ev;

use crate::{ArcOneCallback, IcnName};

/// Action entry rendered by [`ButtonMenu`](super::ButtonMenu).
#[derive(Clone)]
pub struct ButtonMenuItem {
    pub(crate) value: String,
    pub(crate) label: String,
    pub(crate) icon: Option<IcnName>,
    pub(crate) disabled: bool,
    pub(crate) on_click: Option<ArcOneCallback<ev::MouseEvent>>,
}

impl ButtonMenuItem {
    /// Creates a menu item with a stable value and visible label.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            on_click: None,
        }
    }

    /// Adds an optional leading icon to the menu item.
    pub fn icon(mut self, icon: impl Into<IcnName>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Marks the item as present but non-interactive.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Adds an item-level click callback fired before the menu-level `on_select`.
    pub fn on_click(mut self, on_click: impl Into<ArcOneCallback<ev::MouseEvent>>) -> Self {
        self.on_click = Some(on_click.into());
        self
    }
}
