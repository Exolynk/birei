use crate::IcnName;

/// Fixed row density used by [`List`](super::List).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListDensity {
    /// Icon, title, and optional trailing meta only.
    #[default]
    Compact,
    /// Title with optional description below.
    Detailed,
}

impl ListDensity {
    /// Returns the density-specific root class name.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Compact => "birei-list--compact",
            Self::Detailed => "birei-list--detailed",
        }
    }

    /// Returns the fixed row height used by list virtualization.
    pub const fn row_height(self) -> f64 {
        match self {
            Self::Compact => 52.0,
            Self::Detailed => 72.0,
        }
    }
}

/// Opinionated row data rendered by [`List`](super::List).
#[derive(Clone)]
pub struct ListEntry {
    pub(crate) value: String,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<IcnName>,
    pub(crate) meta: Option<String>,
}

impl ListEntry {
    /// Creates a list row with the required stable value and visible title.
    pub fn new(value: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            description: None,
            icon: None,
            meta: None,
        }
    }

    /// Adds a secondary line shown in detailed density.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a leading icon rendered on the row.
    pub fn icon(mut self, icon: impl Into<IcnName>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Adds trailing meta text rendered at the row edge.
    pub fn meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}
