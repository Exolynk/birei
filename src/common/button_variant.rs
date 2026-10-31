/// Visual treatment applied to buttons and button-like triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Filled with the tone color, for the main action.
    #[default]
    Solid,
    /// Surface background with a border and tone-colored text.
    Outline,
    /// No background or border, only tone-colored text.
    Ghost,
}

impl ButtonVariant {
    /// Returns the shared CSS class name consumed by button-like controls.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Solid => "birei-button--solid",
            Self::Outline => "birei-button--outline",
            Self::Ghost => "birei-button--ghost",
        }
    }
}

/// Color intent of a button, independent of its variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonTone {
    /// Brand accent color.
    #[default]
    Accent,
    /// Neutral gray for secondary actions.
    Neutral,
    /// Positive or confirming actions.
    Success,
    /// Actions that need attention.
    Warning,
    /// Destructive actions such as delete.
    Danger,
}

impl ButtonTone {
    /// Returns the tone CSS class name; the accent tone is the default and needs none.
    pub const fn class_name(self) -> Option<&'static str> {
        match self {
            Self::Accent => None,
            Self::Neutral => Some("birei-button--neutral"),
            Self::Success => Some("birei-button--success"),
            Self::Warning => Some("birei-button--warning"),
            Self::Danger => Some("birei-button--danger"),
        }
    }
}
