use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement, Node, Range};

#[derive(Clone)]
pub(crate) struct TableSelection {
    pub(crate) table: Element,
    pub(crate) row: Element,
    pub(crate) cell: Element,
    pub(crate) row_index: usize,
    pub(crate) col_index: usize,
    pub(crate) row_count: usize,
    pub(crate) col_count: usize,
}

pub(crate) fn current_table_selection() -> Option<TableSelection> {
    let selection = window().and_then(|window| window.get_selection().ok().flatten())?;
    if selection.range_count() == 0 {
        return None;
    }

    let range = selection.get_range_at(0).ok()?;
    table_selection_from_range(&range)
}

pub(crate) fn table_selection_from_range(range: &Range) -> Option<TableSelection> {
    let cell = closest_matching_ancestor(&range.start_container().ok()?, "td, th")?;
    let row = cell.closest("tr").ok().flatten()?;
    let table = cell.closest("table").ok().flatten()?;
    let rows = table_rows(&table);
    let cells = row_cells(&row);

    Some(TableSelection {
        row_index: rows
            .iter()
            .position(|candidate| candidate.is_same_node(Some(row.as_ref())))
            .unwrap_or(0),
        col_index: cells
            .iter()
            .position(|candidate| candidate.is_same_node(Some(cell.as_ref())))
            .unwrap_or(0),
        row_count: rows.len(),
        col_count: cells.len(),
        table,
        row,
        cell,
    })
}

pub(crate) fn move_to_adjacent_cell(
    selection: &TableSelection,
    backwards: bool,
) -> Option<Element> {
    let rows = table_rows(&selection.table);
    let mut flattened = Vec::<Element>::new();
    for row in &rows {
        flattened.extend(row_cells(row));
    }

    let current_index = flattened
        .iter()
        .position(|cell| cell.is_same_node(Some(selection.cell.as_ref())))?;

    let target = if backwards {
        current_index
            .checked_sub(1)
            .and_then(|index| flattened.get(index).cloned())
    } else {
        flattened.get(current_index + 1).cloned()
    };

    if let Some(target) = target {
        focus_cell(&target);
        return Some(target);
    }

    if backwards {
        return None;
    }

    let new_row = insert_row_like(&selection.row, false)?;
    let first_cell = row_cells(&new_row).into_iter().next()?;
    focus_cell(&first_cell);
    Some(first_cell)
}

pub(crate) fn apply_table_action(selection: &TableSelection, action: &str) -> Option<Element> {
    match action {
        "table-row-above" => {
            let new_row = insert_row_like(&selection.row, true)?;
            let cell = row_cells(&new_row).get(selection.col_index).cloned()?;
            focus_cell(&cell);
            Some(cell)
        }
        "table-row-below" => {
            let new_row = insert_row_like(&selection.row, false)?;
            let cell = row_cells(&new_row).get(selection.col_index).cloned()?;
            focus_cell(&cell);
            Some(cell)
        }
        "table-row-delete" => delete_row(selection),
        "table-col-left" => insert_column(selection, true),
        "table-col-right" => insert_column(selection, false),
        "table-col-delete" => delete_column(selection),
        "table-delete" => delete_table(selection),
        _ => None,
    }
}

pub(crate) fn focus_cell(cell: &Element) {
    if let Some(html) = cell.dyn_ref::<HtmlElement>() {
        let _ = html.focus();
    }

    let Some(document) = cell.owner_document() else {
        return;
    };
    let Ok(range) = document.create_range() else {
        return;
    };
    let _ = range.select_node_contents(cell);
    range.collapse_with_to_start(true);

    if let Some(selection) = window().and_then(|window| window.get_selection().ok().flatten()) {
        let _ = selection.remove_all_ranges();
        let _ = selection.add_range(&range);
    }
}

fn delete_row(selection: &TableSelection) -> Option<Element> {
    if selection.row_count <= 1 {
        return None;
    }

    let parent = selection.row.parent_node()?;
    let _ = parent.remove_child(selection.row.as_ref());
    let rows = table_rows(&selection.table);
    let next_row = rows
        .get(selection.row_index.min(rows.len().saturating_sub(1)))?
        .clone();
    let next_cells = row_cells(&next_row);
    let next_cell = next_cells
        .get(selection.col_index.min(next_cells.len().saturating_sub(1)))?
        .clone();
    focus_cell(&next_cell);
    Some(next_cell)
}

fn insert_column(selection: &TableSelection, insert_left: bool) -> Option<Element> {
    let rows = table_rows(&selection.table);
    let mut focused_cell = None::<Element>;

    for row in rows {
        let cells = row_cells(&row);
        if cells.is_empty() {
            continue;
        }

        let document = row.owner_document()?;
        let reference_index = selection.col_index.min(cells.len().saturating_sub(1));
        let target_index = if insert_left {
            selection.col_index.min(cells.len())
        } else {
            (selection.col_index + 1).min(cells.len())
        };
        let tag_name = cells[reference_index].tag_name().to_ascii_lowercase();
        let new_cell = create_editable_cell(&document, &tag_name)?;

        if let Some(reference_cell) = cells.get(target_index) {
            let _ = row.insert_before(new_cell.as_ref(), Some(reference_cell.as_ref()));
        } else {
            let _ = row.append_child(new_cell.as_ref());
        }

        if row.is_same_node(Some(selection.row.as_ref())) {
            focused_cell = Some(new_cell);
        }
    }

    let focused_cell = focused_cell?;
    focus_cell(&focused_cell);
    Some(focused_cell)
}

fn delete_column(selection: &TableSelection) -> Option<Element> {
    if selection.col_count <= 1 {
        return None;
    }

    for row in table_rows(&selection.table) {
        let cells = row_cells(&row);
        if let Some(cell) = cells.get(selection.col_index) {
            let _ = row.remove_child(cell.as_ref());
        }
    }

    let rows = table_rows(&selection.table);
    let row = rows.get(selection.row_index.min(rows.len().saturating_sub(1)))?;
    let cells = row_cells(row);
    let next_index = selection
        .col_index
        .saturating_sub(1)
        .min(cells.len().saturating_sub(1));
    let next_cell = cells.get(next_index)?.clone();
    focus_cell(&next_cell);
    Some(next_cell)
}

fn delete_table(selection: &TableSelection) -> Option<Element> {
    let editor = selection
        .table
        .closest(r#"[contenteditable="true"]"#)
        .ok()
        .flatten()?;
    let parent = selection.table.parent_node()?;
    let _ = parent.remove_child(selection.table.as_ref());
    focus_editable_root(&editor);
    Some(editor)
}

fn insert_row_like(reference_row: &Element, insert_before: bool) -> Option<Element> {
    let document = reference_row.owner_document()?;
    let new_row = document.create_element("tr").ok()?;
    for cell in row_cells(reference_row) {
        let new_cell = create_editable_cell(&document, &cell.tag_name().to_ascii_lowercase())?;
        let _ = new_row.append_child(new_cell.as_ref());
    }

    let parent = reference_row.parent_node()?;
    if insert_before {
        let _ = parent.insert_before(new_row.as_ref(), Some(reference_row.as_ref()));
    } else {
        let next = reference_row.next_sibling();
        let _ = parent.insert_before(new_row.as_ref(), next.as_ref());
    }

    Some(new_row)
}

fn create_editable_cell(document: &Document, tag_name: &str) -> Option<Element> {
    let cell = document.create_element(tag_name).ok()?;
    let _ = cell.set_attribute("contenteditable", "true");
    Some(cell)
}

fn focus_editable_root(root: &Element) {
    if let Some(html) = root.dyn_ref::<HtmlElement>() {
        let _ = html.focus();
    }

    let Some(document) = root.owner_document() else {
        return;
    };
    let Ok(range) = document.create_range() else {
        return;
    };
    let _ = range.select_node_contents(root);
    range.collapse_with_to_start(false);

    if let Some(selection) = window().and_then(|window| window.get_selection().ok().flatten()) {
        let _ = selection.remove_all_ranges();
        let _ = selection.add_range(&range);
    }
}

fn closest_matching_ancestor(node: &Node, selector: &str) -> Option<Element> {
    if let Some(element) = node.dyn_ref::<Element>() {
        return element.closest(selector).ok().flatten();
    }

    node.parent_element()
        .and_then(|element| element.closest(selector).ok().flatten())
}

fn row_cells(row: &Element) -> Vec<Element> {
    query_selector_all(row, "th, td")
}

fn table_rows(table: &Element) -> Vec<Element> {
    query_selector_all(table, "tr")
}

fn query_selector_all(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all(selector) else {
        return Vec::new();
    };

    let mut elements = Vec::new();
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Some(element) = node.dyn_into::<Element>().ok() else {
            continue;
        };
        elements.push(element);
    }

    elements
}
