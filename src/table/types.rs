use crate::ArcOneCallback;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableAlign {
    #[default]
    Start,
    Center,
    End,
}

impl TableAlign {
    /// Map alignment choices to CSS classes so layout rules stay centralized in the stylesheet.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Start => "birei-table__cell--start",
            Self::Center => "birei-table__cell--center",
            Self::End => "birei-table__cell--end",
        }
    }
}

#[derive(Clone)]
pub struct TableColumn<Row>
where
    Row: Clone + Send + Sync + 'static,
{
    pub(crate) key: String,
    pub(crate) header: String,
    pub(crate) width: Option<String>,
    pub(crate) min_width: Option<String>,
    pub(crate) align: TableAlign,
    pub(crate) header_class: Option<String>,
    pub(crate) cell_class: Option<String>,
    pub(crate) header_view: Option<ArcOneCallback<(), AnyView>>,
    pub(crate) cell: ArcOneCallback<Row, AnyView>,
}

impl<Row> TableColumn<Row>
where
    Row: Clone + Send + Sync + 'static,
{
    /// Minimal constructor: callers provide identity, header text, and a cell renderer.
    pub fn new(
        key: impl Into<String>,
        header: impl Into<String>,
        cell: impl Into<ArcOneCallback<Row, AnyView>>,
    ) -> Self {
        Self {
            key: key.into(),
            header: header.into(),
            width: None,
            min_width: None,
            align: TableAlign::Start,
            header_class: None,
            cell_class: None,
            header_view: None,
            cell: cell.into(),
        }
    }

    /// Optional width lets callers pin a track instead of using the default flexible column.
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Minimum width feeds the grid track calculation used by both table variants.
    pub fn min_width(mut self, min_width: impl Into<String>) -> Self {
        self.min_width = Some(min_width.into());
        self
    }

    /// Per-column alignment is expressed as a class so header and body cells stay consistent.
    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    /// Header and cell class hooks allow domain-specific styling without forking the component.
    pub fn header_class(mut self, class: impl Into<String>) -> Self {
        self.header_class = Some(class.into());
        self
    }

    pub fn cell_class(mut self, class: impl Into<String>) -> Self {
        self.cell_class = Some(class.into());
        self
    }

    /// Custom header views support rich controls like sort indicators while reusing table layout.
    pub fn header_view(mut self, header_view: impl Into<ArcOneCallback<(), AnyView>>) -> Self {
        self.header_view = Some(header_view.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRowMeta {
    pub disabled: bool,
    pub background_color: Option<String>,
}

impl TableRowMeta {
    /// Creates default metadata for an interactive row.
    pub fn new() -> Self {
        Self {
            disabled: false,
            background_color: None,
        }
    }

    /// Disabled rows still render but opt out of click and drag interactions.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a row-local background color without affecting other rows.
    pub fn background_color(mut self, background_color: impl Into<String>) -> Self {
        self.background_color = Some(background_color.into());
        self
    }
}

impl Default for TableRowMeta {
    /// Creates metadata with no row-specific state.
    fn default() -> Self {
        Self::new()
    }
}
