/// Native HTML input types supported by [`Input`](super::Input).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Search,
    Tel,
    Url,
}

impl InputType {
    /// Returns the native `<input type="...">` attribute value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

/// Typed autocomplete values supported by [`Input`](super::Input).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAutocomplete {
    On,
    Off,
    Name,
    GivenName,
    FamilyName,
    Nickname,
    Email,
    Username,
    CurrentPassword,
    NewPassword,
    Tel,
    Url,
    StreetAddress,
    PostalCode,
    Country,
}

impl InputAutocomplete {
    /// Returns the native autocomplete token passed to the browser.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Name => "name",
            Self::GivenName => "given-name",
            Self::FamilyName => "family-name",
            Self::Nickname => "nickname",
            Self::Email => "email",
            Self::Username => "username",
            Self::CurrentPassword => "current-password",
            Self::NewPassword => "new-password",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::StreetAddress => "street-address",
            Self::PostalCode => "postal-code",
            Self::Country => "country",
        }
    }
}
