/// Shared control sizes used by buttons, inputs and other controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    /// Compact control spacing.
    Small,
    /// Default control spacing.
    #[default]
    Medium,
    /// Spacious control spacing.
    Large,
}

impl Size {
    /// Returns the size modifier class shared by all controls.
    ///
    /// Component stylesheets select it as `.birei-<component>:where(.birei-size--<size>)`.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-size--small",
            Self::Medium => "birei-size--medium",
            Self::Large => "birei-size--large",
        }
    }
}
