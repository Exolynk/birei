use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableAlign {
    #[default]
    Start,
    Center,
    End,
}

impl TableAlign {
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
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Compact => "birei-table--compact",
            Self::Comfortable => "birei-table--comfortable",
        }
    }

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

    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn min_width(mut self, min_width: impl Into<String>) -> Self {
        self.min_width = Some(min_width.into());
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    pub fn header_class(mut self, class: impl Into<String>) -> Self {
        self.header_class = Some(class.into());
        self
    }

    pub fn cell_class(mut self, class: impl Into<String>) -> Self {
        self.cell_class = Some(class.into());
        self
    }

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
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            disabled: false,
            draggable: true,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

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
