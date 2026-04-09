// Crate modules stay private by default so the public API is curated through
// the re-exports below, with `code_editor` and `icon` intentionally exposed as
// namespaces for richer sub-APIs.
mod button;
mod button_bar;
mod button_menu;
mod card;
mod chart;
mod checkbox;
pub mod code_editor;
mod color;
mod common;
mod datetime;
#[cfg(any(feature = "embedded-css", feature = "embedded-icons"))]
mod embed;
mod flexible_columns;
pub mod icon;
mod input;
mod label;
mod list;
mod map;
mod markdown;
mod select;
mod slider;
mod tab;
mod table;
mod tag;
mod textarea;
mod tooltip;

// Public re-exports define the main component-library surface consumed by
// downstream applications.
pub use button::{Button, ButtonGroup, ButtonType};
pub use button_bar::{ButtonBar, ButtonBarItem};
pub use button_menu::{ButtonMenu, ButtonMenuItem};
pub use card::Card;
pub use chart::{Chart, ChartData, ChartDatumKind, ChartLegendPosition, ChartType};
pub use checkbox::Checkbox;
pub use code_editor::CodeEditor;
pub use color::ColorInput;
pub use common::ButtonVariant;
pub use common::Size;
pub use common::TooltipPlacement;
pub use datetime::{DateTimeInput, DateTimeInputMode};
#[cfg(any(feature = "embedded-css", feature = "embedded-icons"))]
pub use embed::embed_assets;
pub use flexible_columns::{FlexibleColumn, FlexibleColumns};
pub use icon::{IcnName, Icon};
pub use input::{Input, InputAutocomplete, InputType};
pub use label::Label;
pub use list::{List, ListDensity, ListEntry};
pub use map::{MapCoordinate, MapViewer};
pub use markdown::{MarkdownEditor, MarkdownImageUploadHandler};
pub use select::{Select, SelectOption};
pub use slider::{Slider, SliderStepLabel};
pub use tab::{TabItem, TabLinePosition, TabList};
pub use table::{
    Table, TableAlign, TableColumn, TableDensity, TableDropPosition, TableList, TableRowMeta,
    TableRowMove,
};
pub use tag::Tag;
pub use textarea::Textarea;
pub use tooltip::Tooltip;
