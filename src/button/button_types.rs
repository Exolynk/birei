/// Visual treatment applied to a [`Button`](super::Button).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// High-emphasis action styling.
    #[default]
    Primary,
    /// Lower-emphasis filled or outlined styling.
    Secondary,
    /// No background treatment, only text emphasis.
    Transparent,
}

impl ButtonVariant {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "birei-button--primary",
            Self::Secondary => "birei-button--secondary",
            Self::Transparent => "birei-button--transparent",
        }
    }
}

/// Available button sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Compact button spacing.
    Small,
    /// Default button spacing.
    #[default]
    Medium,
    /// Spacious button spacing.
    Large,
}

impl ButtonSize {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-button--small",
            Self::Medium => "birei-button--medium",
            Self::Large => "birei-button--large",
        }
    }
}

/// Native HTML button behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonType {
    /// Behaves like a plain clickable button.
    #[default]
    Button,
    /// Submits the nearest parent form.
    Submit,
    /// Resets the nearest parent form to its initial values.
    Reset,
}

impl ButtonType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ButtonGroupContext {
    pub(crate) variant: Option<ButtonVariant>,
    pub(crate) size: Option<ButtonSize>,
    pub(crate) disabled: Option<bool>,
}
