use wasm_bindgen::JsCast;
use web_sys::{Element, KeyboardEvent};

use super::types::{TableColumn, TableRowMeta};

pub(crate) fn keyboard_event_targets_control(event: &KeyboardEvent) -> bool {
    target_is_control(event.target())
}

pub(crate) fn mouse_event_targets_control(event: &web_sys::MouseEvent) -> bool {
    target_is_control(event.target())
}

fn target_is_control(target: Option<web_sys::EventTarget>) -> bool {
    target
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|target| {
            target
                .closest(
                    r#"input, textarea, select, button, [contenteditable="true"], [role="textbox"], [role="combobox"]"#,
                )
                .ok()
                .flatten()
        })
        .is_some()
}

pub(crate) fn root_class_name(keyboard_navigation: bool, class: Option<&str>) -> String {
    // Keep root class assembly in one place so state styles remain consistent.
    let mut classes = vec!["birei-table"];
    if keyboard_navigation {
        classes.push("birei-table--keyboard");
    }
    if let Some(class) = class {
        classes.push(class);
    }
    classes.join(" ")
}

pub(crate) fn grid_template<Row>(columns: &[TableColumn<Row>]) -> String
where
    Row: Clone + Send + Sync + 'static,
{
    columns
        .iter()
        .map(column_track)
        .collect::<Vec<_>>()
        .join(" ")
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

pub(crate) fn row_class_name(active: bool, selected: bool, disabled: bool) -> String {
    // Row state is flattened into classes so CSS can express visuals declaratively.
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
    classes.join(" ")
}

pub(crate) fn row_style(meta: &TableRowMeta) -> String {
    meta.background_color
        .as_deref()
        .filter(|background_color| !background_color.trim().is_empty())
        .map(|background_color| format!("--birei-table-row-background: {background_color};"))
        .unwrap_or_default()
}
