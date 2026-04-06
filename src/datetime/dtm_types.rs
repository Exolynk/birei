/// Native picker mode used by [`DateTimeInput`](super::DateTimeInput).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateTimeInputMode {
    /// Date only, mapped to native `date`.
    Date,
    /// Time only, mapped to native `time`.
    Time,
    /// Local date and time, mapped to native `datetime-local`.
    #[default]
    DateTime,
}

impl DateTimeInputMode {
    /// Returns the native input type consumed by the hidden picker element.
    pub const fn native_input_type(self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime-local",
        }
    }

    /// Returns the icon name used by the visible picker trigger.
    pub const fn icon_name(self) -> &'static str {
        match self {
            Self::Date => "calendar",
            Self::Time => "clock",
            Self::DateTime => "calendar-clock",
        }
    }
}
