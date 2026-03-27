/// Tab entry rendered by [`TabList`](super::TabList).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabItem {
    /// Stable value emitted when the tab is selected.
    pub value: String,
    /// Human-readable label shown for the tab trigger.
    pub label: String,
    /// Disables selection for this tab.
    pub disabled: bool,
}

impl TabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Placement of the tab indicator track relative to the tab labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabLinePosition {
    /// Render the indicator track below the tabs.
    #[default]
    Below,
    /// Render the indicator track above the tabs.
    Above,
}
