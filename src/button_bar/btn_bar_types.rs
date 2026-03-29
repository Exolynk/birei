use crate::IcnName;

/// Button entry rendered by [`ButtonBar`](super::ButtonBar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonBarItem {
    /// Stable value emitted when the button is activated.
    pub value: String,
    /// Human-readable label shown for the button trigger.
    pub label: String,
    /// Optional leading icon shown in the visible bar and overflow menu.
    pub icon: Option<IcnName>,
    /// Disables activation for this button.
    pub disabled: bool,
}

impl ButtonBarItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<IcnName>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
