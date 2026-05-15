use leptos::{ev, prelude::*};

use crate::{ArcOneCallback, IcnName};

/// Button entry rendered by [`ButtonBar`](super::ButtonBar).
#[derive(Clone)]
pub struct ButtonBarItem {
    /// Stable value emitted when the button is activated.
    pub value: String,
    /// Human-readable label shown for the button trigger.
    pub label: MaybeProp<String>,
    /// Optional leading icon shown in the visible bar and overflow menu.
    pub icon: Option<IcnName>,
    /// Disables activation for this button.
    pub disabled: bool,
    /// Callback fired with the click event before the bar-level selection callback.
    pub on_click: Option<ArcOneCallback<ev::MouseEvent>>,
}

impl std::fmt::Debug for ButtonBarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonBarItem")
            .field("value", &self.value)
            .field("label", &self.label.get_untracked())
            .field("icon", &self.icon)
            .field("disabled", &self.disabled)
            .field("on_click", &self.on_click.is_some())
            .finish()
    }
}

impl PartialEq for ButtonBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.icon == other.icon && self.disabled == other.disabled
    }
}

impl Eq for ButtonBarItem {}

impl ButtonBarItem {
    /// Creates a toolbar item with a stable value and visible label.
    pub fn new(value: impl Into<String>, label: impl Into<MaybeProp<String>>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            on_click: None,
        }
    }

    /// Adds a leading icon rendered in both the bar and overflow menu.
    pub fn icon(mut self, icon: impl Into<IcnName>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Marks the item as non-interactive in both visible and overflow states.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Adds an item-level click callback fired before [`ButtonBar`](super::ButtonBar)'s `on_select`.
    pub fn on_click(mut self, on_click: impl Into<ArcOneCallback<ev::MouseEvent>>) -> Self {
        self.on_click = Some(on_click.into());
        self
    }
}
