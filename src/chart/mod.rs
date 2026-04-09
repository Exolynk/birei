// Chart rendering is split into reusable data types, geometry helpers, and a
// single configurable SVG component so the public API stays compact.
mod chart_types;
mod chart_utils;
mod internal;
mod layout;
mod render;
mod view;

pub use chart_types::{ChartData, ChartDatumKind, ChartLegendPosition, ChartType};
pub use view::Chart;
