use super::types::{TableDropPosition, TableRowMove};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DragState {
    pub(crate) from_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DragTarget {
    pub(crate) key: String,
    pub(crate) position: TableDropPosition,
}

pub(crate) fn drag_target_from_y(
    client_y: f64,
    row_top: f64,
    row_height: f64,
    row_key: &str,
) -> DragTarget {
    let midpoint = row_top + (row_height / 2.0);
    let position = if client_y < midpoint {
        TableDropPosition::Before
    } else {
        TableDropPosition::After
    };

    DragTarget {
        key: row_key.to_owned(),
        position,
    }
}

pub(crate) fn build_row_move(drag_state: &DragState, target: &DragTarget) -> Option<TableRowMove> {
    if drag_state.from_key == target.key {
        return None;
    }

    Some(TableRowMove {
        from_key: drag_state.from_key.clone(),
        to_key: target.key.clone(),
        position: target.position,
    })
}
