use std::rc::Rc;

use leptos::prelude::*;

use super::types::{CodeCompletionItem, CodeSelection, CompletionResponse, TextEdit};

/// Applies a completion response into the editor state while respecting
/// whether the caller is allowed to open a new popup or only refresh one
/// that is already visible.
pub(crate) fn apply_completion_response(
    response: CompletionResponse,
    completions: RwSignal<Vec<CodeCompletionItem>>,
    completion_replace: RwSignal<Option<std::ops::Range<usize>>>,
    completion_open: RwSignal<bool>,
    completion_index: RwSignal<usize>,
    completion_scroll_request: RwSignal<u64>,
    allow_open: bool,
) {
    let is_empty = response.items.is_empty();
    let _ = completions.try_set(response.items);
    let _ = completion_replace.try_set(response.replace);
    let _ = completion_index.try_set(0);
    let should_open =
        !is_empty && (allow_open || completion_open.try_get_untracked().unwrap_or_default());
    let _ = completion_open.try_set(should_open);
    if should_open {
        let _ = completion_scroll_request.try_update(|value| *value += 1);
    }
}

/// Replaces the currently targeted completion range with the active item.
/// Returns `false` when the popup has nothing actionable to accept.
pub(crate) fn accept_selected_completion(
    apply_edit: Rc<dyn Fn(TextEdit)>,
    completions: RwSignal<Vec<CodeCompletionItem>>,
    completion_replace: RwSignal<Option<std::ops::Range<usize>>>,
    completion_open: RwSignal<bool>,
    completion_index: RwSignal<usize>,
) -> bool {
    if !completion_open.try_get_untracked().unwrap_or_default() {
        return false;
    }

    let items = completions.try_get_untracked().unwrap_or_default();
    let Some(item) = items
        .get(completion_index.try_get_untracked().unwrap_or_default())
        .cloned()
    else {
        return false;
    };
    let Some(range) = completion_replace.try_get_untracked().unwrap_or_default() else {
        return false;
    };

    let cursor = range.start + item.cursor.unwrap_or(item.insert_text.len());
    apply_edit(TextEdit {
        range,
        replacement: item.insert_text,
        cursor: Some(cursor),
    });
    completion_open.set(false);
    true
}

/// Programmatic edits collapse the selection to a single caret position.
pub(crate) fn selection_after_edit(cursor: usize) -> CodeSelection {
    CodeSelection {
        start: cursor,
        end: cursor,
    }
}
