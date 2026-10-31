/// Root attribute that forces a color scheme; without it the system preference applies.
const THEME_ATTRIBUTE: &str = "data-theme";

/// Color scheme of the birei theme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    /// Follows the light or dark preference of the operating system.
    #[default]
    System,
    /// Always uses the light theme.
    Light,
    /// Always uses the dark theme.
    Dark,
}

impl ColorScheme {
    /// Parses a stored identifier; unknown or empty values follow the system.
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::System,
        }
    }

    /// Returns the stable identifier, also used as `data-theme` value.
    pub fn ident(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Applies a color scheme to the document root.
///
/// `Light` and `Dark` set `data-theme`, `System` removes it so `prefers-color-scheme` decides.
pub fn set_color_scheme(scheme: ColorScheme) {
    let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    else {
        return;
    };
    let _ = match scheme {
        ColorScheme::System => root.remove_attribute(THEME_ATTRIBUTE),
        ColorScheme::Light | ColorScheme::Dark => {
            root.set_attribute(THEME_ATTRIBUTE, scheme.ident())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::ColorScheme;

    /// Identifiers round-trip and unknown values fall back to the system scheme.
    #[test]
    fn identifiers_round_trip() {
        for scheme in [ColorScheme::System, ColorScheme::Light, ColorScheme::Dark] {
            assert_eq!(ColorScheme::from_ident(scheme.ident()), scheme);
        }
        assert_eq!(ColorScheme::from_ident(""), ColorScheme::System);
        assert_eq!(ColorScheme::from_ident("sepia"), ColorScheme::System);
    }
}
