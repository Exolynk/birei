/// Visual intent variants supported by [`Notification`](crate::Notification) and
/// [`NotificationManager`](crate::NotificationManager).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NotificationVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationVariant {
    pub(crate) fn class_name(self) -> &'static str {
        match self {
            Self::Info => "birei-notification--info",
            Self::Success => "birei-notification--success",
            Self::Warning => "birei-notification--warning",
            Self::Error => "birei-notification--error",
        }
    }

    pub(crate) fn icon_name(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "circle-check",
            Self::Warning => "triangle-alert",
            Self::Error => "circle-x",
        }
    }

    pub(crate) fn default_duration_ms(self) -> i32 {
        match self {
            Self::Info | Self::Success => 4200,
            Self::Warning => 5600,
            Self::Error => 6800,
        }
    }
}

/// Options used to create a new managed notification.
#[derive(Clone, Debug)]
pub struct NotificationOptions {
    pub text: String,
    pub variant: NotificationVariant,
    pub duration_ms: Option<i32>,
}

impl NotificationOptions {
    /// Creates a notification options object with plain text and default `Info` styling.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: NotificationVariant::Info,
            duration_ms: None,
        }
    }

    /// Overrides the visual variant used by the notification.
    pub fn variant(mut self, variant: NotificationVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Overrides the auto-hide timeout in milliseconds.
    pub fn duration_ms(mut self, duration_ms: i32) -> Self {
        self.duration_ms = Some(duration_ms.max(0));
        self
    }
}
