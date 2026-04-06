/// Visual treatment applied to buttons and button-like triggers.
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
    /// Returns the shared CSS class name consumed by button-like controls.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "birei-button--primary",
            Self::Secondary => "birei-button--secondary",
            Self::Transparent => "birei-button--transparent",
        }
    }
}
