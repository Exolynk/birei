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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableDensity {
    Compact,
    #[default]
    Comfortable,
}

impl TableDensity {
    /// Density is represented by CSS classes so both table variants share the same sizing tokens.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Compact => "birei-table--compact",
            Self::Comfortable => "birei-table--comfortable",
        }
    }

    /// Virtualization needs a deterministic row height to translate scroll offsets into row ranges.
    pub const fn row_height(self) -> f64 {
        match self {
            Self::Compact => 44.0,
            Self::Comfortable => 56.0,
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
    pub(crate) header_view: Option<Callback<(), AnyView>>,
    pub(crate) cell: Callback<Row, AnyView>,
}

impl<Row> TableColumn<Row>
where
    Row: Clone + Send + Sync + 'static,
{
    /// Minimal constructor: callers provide identity, header text, and a cell renderer.
    pub fn new(
        key: impl Into<String>,
        header: impl Into<String>,
        cell: Callback<Row, AnyView>,
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
            cell,
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
    pub fn header_view(mut self, header_view: Callback<(), AnyView>) -> Self {
        self.header_view = Some(header_view);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRowMeta {
    pub key: String,
    pub disabled: bool,
    pub draggable: bool,
}

impl TableRowMeta {
    /// Row metadata keeps optional interaction flags alongside the stable row key.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            disabled: false,
            draggable: true,
        }
    }

    /// Disabled rows still render but opt out of click and drag interactions.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Draggability is configurable per row so reorderable tables can still protect fixed rows.
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDropPosition {
    Before,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRowMove {
    pub from_key: String,
    pub to_key: String,
    pub position: TableDropPosition,
}
