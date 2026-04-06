use super::types::CodeSelection;

/// Snapshot stored in the editor-owned undo/redo history.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct HistoryEntry {
    pub(crate) text: String,
    pub(crate) selection: CodeSelection,
    pub(crate) scroll_top: i32,
    pub(crate) scroll_left: i32,
}
