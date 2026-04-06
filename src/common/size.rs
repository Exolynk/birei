/// Shared control sizes used by buttons and inputs.
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
    /// Returns the button size class used by buttons and button-like triggers.
    pub const fn button_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-button--small",
            Self::Medium => "birei-button--medium",
            Self::Large => "birei-button--large",
        }
    }

    /// Returns the text input size class.
    pub const fn input_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-input--small",
            Self::Medium => "birei-input--medium",
            Self::Large => "birei-input--large",
        }
    }

    /// Returns the textarea size class.
    pub const fn textarea_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-textarea--small",
            Self::Medium => "birei-textarea--medium",
            Self::Large => "birei-textarea--large",
        }
    }

    /// Returns the select field size class.
    pub const fn select_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-select--small",
            Self::Medium => "birei-select--medium",
            Self::Large => "birei-select--large",
        }
    }

    /// Returns the slider size class.
    pub const fn slider_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-slider--small",
            Self::Medium => "birei-slider--medium",
            Self::Large => "birei-slider--large",
        }
    }

    /// Returns the icon size class.
    pub const fn icon_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-icon--small",
            Self::Medium => "birei-icon--medium",
            Self::Large => "birei-icon--large",
        }
    }

    /// Returns the checkbox size class.
    pub const fn checkbox_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-checkbox--small",
            Self::Medium => "birei-checkbox--medium",
            Self::Large => "birei-checkbox--large",
        }
    }
}
