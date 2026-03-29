use crate::{ButtonVariant, Size};

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
    pub(crate) size: Option<Size>,
    pub(crate) disabled: Option<bool>,
}
