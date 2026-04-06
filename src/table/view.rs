use leptos::ev;
use leptos::prelude::*;

use crate::{Icon, Size};

use super::drag::{DragState, DragTarget};
use super::types::{TableColumn, TableDensity, TableDropPosition, TableRowMeta};

pub(crate) fn root_class_name(
    density: TableDensity,
    keyboard_navigation: bool,
    class: Option<&str>,
) -> String {
    // Keep root class assembly in one place so the plain and virtualized tables never drift apart.
    let mut classes = vec!["birei-table", density.class_name()];
    if keyboard_navigation {
        classes.push("birei-table--keyboard");
    }
    if let Some(class) = class {
        classes.push(class);
    }
    classes.join(" ")
}

pub(crate) fn grid_template<Row>(columns: &[TableColumn<Row>], reorderable: bool) -> String
where
    Row: Clone + Send + Sync + 'static,
{
    // The optional drag-handle column is treated as just another grid track so the header and body
    // can share the exact same template string.
    let mut tracks = Vec::new();
    if reorderable {
        tracks.push(String::from("2.75rem"));
    }

    tracks.extend(columns.iter().map(column_track));
    tracks.join(" ")
}

fn column_track<Row>(column: &TableColumn<Row>) -> String
where
    Row: Clone + Send + Sync + 'static,
{
    // Prefer explicit caller sizing, but always fall back to a flexible track so the grid can shrink.
    match (&column.min_width, &column.width) {
        (Some(min), Some(width)) => format!("minmax({min}, {width})"),
        (Some(min), None) => format!("minmax({min}, 1fr)"),
        (None, Some(width)) => width.clone(),
        (None, None) => String::from("minmax(0, 1fr)"),
    }
}

pub(crate) fn header_cell_class<Row>(column: &TableColumn<Row>, clickable: bool) -> String
where
    Row: Clone + Send + Sync + 'static,
{
    // Header classes merge alignment, optional click affordance, and caller overrides.
    let mut classes = vec![
        "birei-table__cell",
        "birei-table__cell--header",
        column.align.class_name(),
    ];
    if clickable {
        classes.push("birei-table__cell--clickable");
    }
    if let Some(class) = column.header_class.as_deref() {
        classes.push(class);
    }
    classes.join(" ")
}

pub(crate) fn body_cell_class<Row>(column: &TableColumn<Row>) -> String
where
    Row: Clone + Send + Sync + 'static,
{
    // Body cells share the same alignment model as headers but without click affordances.
    let mut classes = vec!["birei-table__cell", column.align.class_name()];
    if let Some(class) = column.cell_class.as_deref() {
        classes.push(class);
    }
    classes.join(" ")
}

pub(crate) fn row_class_name(
    active: bool,
    selected: bool,
    disabled: bool,
    is_dragging: bool,
    drag_target: Option<TableDropPosition>,
) -> String {
    // Row state is flattened into classes so CSS can express active/selected/drag visuals declaratively.
    let mut classes = vec!["birei-table__row"];
    if active {
        classes.push("birei-table__row--active");
    }
    if selected {
        classes.push("birei-table__row--selected");
    }
    if disabled {
        classes.push("birei-table__row--disabled");
    }
    if is_dragging {
        classes.push("birei-table__row--dragging");
    }
    if let Some(position) = drag_target {
        classes.push(match position {
            TableDropPosition::Before => "birei-table__row--drop-before",
            TableDropPosition::After => "birei-table__row--drop-after",
        });
    }
    classes.join(" ")
}

pub(crate) fn drag_handle(on_mouse_down: Callback<ev::MouseEvent>) -> AnyView {
    // Reuse the library button tokens for the drag handle so it matches the rest of the design system.
    view! {
        <span
            class="birei-table__drag-handle birei-button birei-button--transparent birei-button--small birei-button--circle"
            tabindex="-1"
            role="button"
            aria-label="Reorder row"
            on:mousedown=move |event| on_mouse_down.run(event)
        >
            <Icon name="grip-vertical" size=Size::Small/>
        </span>
    }
    .into_any()
}

pub(crate) fn row_meta_or_default(
    meta: Option<TableRowMeta>,
    fallback_key: String,
) -> TableRowMeta {
    // Consumers can omit row metadata entirely; the table fills in the minimum required key state.
    meta.unwrap_or_else(|| TableRowMeta::new(fallback_key))
}

pub(crate) fn drag_target_for_row(
    drag_state: Option<DragState>,
    drag_target: Option<DragTarget>,
    row_key: &str,
) -> (bool, Option<TableDropPosition>) {
    // Project the global drag state into row-local flags the row view can render directly.
    let is_dragging = drag_state
        .as_ref()
        .is_some_and(|state| state.from_key == row_key);
    let drop_position = drag_target
        .as_ref()
        .filter(|target| target.key == row_key)
        .map(|target| target.position);

    (is_dragging, drop_position)
}
