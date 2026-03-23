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
    pub const fn button_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-button--small",
            Self::Medium => "birei-button--medium",
            Self::Large => "birei-button--large",
        }
    }

    pub const fn input_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-input--small",
            Self::Medium => "birei-input--medium",
            Self::Large => "birei-input--large",
        }
    }

    pub const fn textarea_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-textarea--small",
            Self::Medium => "birei-textarea--medium",
            Self::Large => "birei-textarea--large",
        }
    }

    pub const fn select_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-select--small",
            Self::Medium => "birei-select--medium",
            Self::Large => "birei-select--large",
        }
    }

    pub const fn slider_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-slider--small",
            Self::Medium => "birei-slider--medium",
            Self::Large => "birei-slider--large",
        }
    }

    pub const fn icon_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-icon--small",
            Self::Medium => "birei-icon--medium",
            Self::Large => "birei-icon--large",
        }
    }

    pub const fn checkbox_class_name(self) -> &'static str {
        match self {
            Self::Small => "birei-checkbox--small",
            Self::Medium => "birei-checkbox--medium",
            Self::Large => "birei-checkbox--large",
        }
    }
}
