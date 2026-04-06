use crate::IcnName;

/// Action entry rendered by [`ButtonMenu`](super::ButtonMenu).
#[derive(Clone)]
pub struct ButtonMenuItem {
    pub(crate) value: String,
    pub(crate) label: String,
    pub(crate) icon: Option<IcnName>,
    pub(crate) disabled: bool,
}

impl ButtonMenuItem {
    /// Creates a menu item with a stable value and visible label.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            disabled: false,
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
}
