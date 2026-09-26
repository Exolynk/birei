use crate::IcnName;

/// A tree item identified by a value that is unique across the whole tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub(crate) value: String,
    pub(crate) title: String,
    pub(crate) children: Vec<Self>,
    pub(crate) has_children: bool,
    pub(crate) icon: Option<IcnName>,
    pub(crate) meta: Option<String>,
    pub(crate) highlight: Option<String>,
}

impl TreeNode {
    /// Creates a leaf item with a stable value and visible title.
    pub fn new(value: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            children: Vec::new(),
            has_children: false,
            icon: None,
            meta: None,
            highlight: None,
        }
    }

    /// Sets the nested items shown when this item is expanded.
    pub fn children(mut self, children: Vec<Self>) -> Self {
        self.has_children |= !children.is_empty();
        self.children = children;
        self
    }

    /// Shows a disclosure control before child items have been loaded.
    pub fn has_children(mut self, has_children: bool) -> Self {
        self.has_children = has_children;
        self
    }

    /// Adds a leading icon to the row.
    pub fn icon(mut self, icon: impl Into<IcnName>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Adds secondary text at the end of the row.
    pub fn meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    /// Adds a row highlight color as a CSS color string.
    pub fn highlight(mut self, highlight: impl Into<String>) -> Self {
        self.highlight = Some(highlight.into());
        self
    }
}
